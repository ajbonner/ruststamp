use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use ureq::Agent;
use ureq::http::StatusCode;
use url::Url;

const DEFAULT_API_VERSION: &str = "v2";

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

pub struct RestClient {
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

impl RestClient {
    pub fn new(config: &ApiConfig) -> Self {
        RestClient {
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
        let agent = self.ureq_agent_with_response_for_status_error();
        let res= agent.get(url).call();

        match res {
            Ok(mut res) => {
                let status_code = res.status();
                match status_code {
                    StatusCode::OK => {
                        let body = res.body_mut().read_to_string()?;
                        Ok(body)
                    },
                    StatusCode::NOT_FOUND => {
                        // let body = res.body_mut().read_to_string()?;
                        Err("404 Not Found".into())
                    },
                    _ => {
                        // let body = res.body_mut().read_to_string()?;
                        Err(status_code.to_string().into())
                    }
                }
            },
            Err(e) => {
                // Handle everything else transport errors (connection failed, timeout, etc.)
                Err(e.into())
            }
        }
    }

    fn build_url(&self, service: &str) -> String {
        format!(
            "{}/{}/{}",
            self.config.base_url, self.config.version, service
        )
    }

    fn ureq_agent_with_response_for_status_error(&self) -> ureq::Agent {
        let config = Agent::config_builder()
            .timeout_global(Option::from(Duration::from_secs(5)))
            .http_status_as_error(false)
            .build();

        Agent::new_with_config(config)
    }
}

fn default_api_version() -> String {
    String::from(DEFAULT_API_VERSION)
}
