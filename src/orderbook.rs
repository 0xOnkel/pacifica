use reqwest::Url;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{API_BASE, ApiResponse, Client, Result};

const GET_ORDERBOOK: &str = "book";

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBook {
    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Bids [0] and Asks [1]
    ///
    /// Each inner array is a list of levels, and each level is deserialized into `OrderBookLevel`.
    #[serde(rename = "l")]
    levels: [Vec<OrderBookLevel>; 2],

    /// Response timestamp (milliseconds since epoch)
    #[serde(rename = "t")]
    pub timestamp: u128,
}

impl OrderBook {
    pub fn bids(&self) -> &[OrderBookLevel] {
        &self.levels[0]
    }

    pub fn asks(&self) -> &[OrderBookLevel] {
        &self.levels[1]
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel {
    /// Price level
    #[serde(rename = "p")]
    pub price: Decimal,

    /// Total amount at price level
    #[serde(rename = "a")]
    pub amount: Decimal,

    /// Number of orders at level
    #[serde(rename = "n")]
    pub orders: u64,
}

impl Client {
    pub async fn get_orderbook(&self, symbol: &str) -> Result<ApiResponse<OrderBook>> {
        let mut url = Url::parse(&format!("{API_BASE}/{GET_ORDERBOOK}"))?;
        url.query_pairs_mut().append_pair("symbol", symbol);

        Ok(self.client.get(url).send().await?.json().await?)
    }
}
