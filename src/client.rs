use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

const DEFAULT_API_VERSION: &str = "v2";

/// 404.001	Unknown not found error.
/// 404.002	Order not found for corresponding request.
/// 404.003	Currency pair not found for corresponding request.
/// 404.004	Trade account not found for provided API key.
/// 404.005	Order book not found.
/// 404.006	Currency not found for corresponding request.
/// 404.007	Market not found for corresponding request.
#[derive(Debug)]
#[allow(dead_code)]
pub enum BitstampError {
    UnknownNotFound,
    OrderNotFound,
    CurrencyPairNotFound,
    TradeAccountNotFound,
    OrderBookNotFound,
    CurrencyNotFound,
    MarketNotFound,
}

impl fmt::Display for BitstampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitstampError::UnknownNotFound => write!(f, "Unknown not found error (404.001)"),
            BitstampError::OrderNotFound => {
                write!(f, "Order not found for corresponding request (404.002)")
            }
            BitstampError::CurrencyPairNotFound => write!(
                f,
                "Currency pair not found for corresponding request (404.003)"
            ),
            BitstampError::TradeAccountNotFound => {
                write!(f, "Trade account not found for provided API key (404.004)")
            }
            BitstampError::OrderBookNotFound => write!(f, "Order book not found (404.005)"),
            BitstampError::CurrencyNotFound => {
                write!(f, "Currency not found for corresponding request (404.006)")
            }
            BitstampError::MarketNotFound => {
                write!(f, "Market not found for corresponding request (404.007)")
            }
        }
    }
}

impl Error for BitstampError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: Url,
    #[serde(rename = "api_version", default = "default_api_version")]
    pub version: String,
    pub rate_limit_sec: u16,
    pub rate_limit_min: u16,
    pub timeout_ms: u16,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub struct Client {
    config: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    timestamp: String,
    open: String,
    high: String,
    low: String,
    last: String,
    volume: String,
    vwap: String,
    bid: String,
    ask: String,
    side: String,
    open_24: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent_change_24: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pair: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub currency: String,
    #[serde(rename = "type")]
    pub currency_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub decimals: u8,
    pub logo: String,
    pub available_supply: String,
    pub deposit: String,
    pub withdrawal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<Network>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub network: String,
    pub withdrawal_decimals: u8,
    pub deposit: String,
    pub withdrawal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_minimum_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub name: String,
    pub market_symbol: String,
    pub base_currency: String,
    pub base_decimals: u8,
    pub counter_currency: String,
    pub counter_decimals: u8,
    pub minimum_order_value: String,
    pub trading: String,
    pub instant_order_counter_decimals: u8,
    pub instant_and_market_orders: String,
    pub description: String,
    pub market_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: String,
    pub amount: String,
}

impl ApiConfig {
    pub fn from_json(path_to_config_file: &str) -> Self {
        let config = std::fs::read_to_string(path_to_config_file).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}

impl Client {
    pub fn new(config: &ApiConfig) -> Self {
        Client {
            config: config.clone(),
        }
    }

    pub fn get_ticker(&self, market_symbol: &str) -> Result<Ticker, Box<dyn Error>> {
        let url = self.build_url(format!("ticker/{}", market_symbol).as_str());
        let body = self.get(url.as_str())?;

        // Try to parse as array first
        if let Ok(tickers) = serde_json::from_str::<Vec<Ticker>>(&body) {
            if let Some(ticker) = tickers.first() {
                return Ok(ticker.clone());
            }
        } else if let Err(error) = serde_json::from_str::<Vec<Ticker>>(&body) {
            println!("Error: {}", error);
        }

        // If not an array, try to parse as single ticker
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_currencies(&self) -> Result<Vec<Currency>, Box<dyn Error>> {
        let url = self.build_url("currencies");
        let body = self.get(url.as_str())?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_markets(&self) -> Result<Vec<Market>, Box<dyn Error>> {
        let url = self.build_url("markets");
        let body = self.get(url.as_str())?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn get_order_book(&self, market_symbol: &str) -> Result<OrderBook, Box<dyn Error>> {
        let url = self.build_url(format!("order_book/{}", market_symbol).as_str());
        let body = self.get(url.as_str())?;
        Ok(serde_json::from_str(&body)?)
    }

    fn get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let mut res = ureq::get(url).call()?;
        let body = res.body_mut().read_to_string()?;
        Ok(body)
    }

    fn build_url(&self, service: &str) -> String {
        format!(
            "{}/{}/{}",
            self.config.base_url, self.config.version, service
        )
    }
}

fn default_api_version() -> String {
    String::from(DEFAULT_API_VERSION)
}
