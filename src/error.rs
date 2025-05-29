use std::error::Error;
use std::fmt;

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