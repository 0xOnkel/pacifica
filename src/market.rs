use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{API_BASE, ApiResponse, Client, Result};

const GET_MARKET_INFO: &str = "info";
const GET_PRICES: &str = "info/prices";

#[derive(Debug, Clone, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub tick_size: Decimal,
    pub min_tick: Decimal,
    pub max_tick: Decimal,
    pub lot_size: Decimal,
    pub max_leverage: u16,
    pub isolated_only: bool,
    pub min_order_size: Decimal,
    pub max_order_size: Decimal,
    pub funding_rate: Decimal,
    pub next_funding_rate: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    // Trading pair symbol
    pub symbol: String,
    // funding rate paid in the past funding epoch (hour)
    pub funding: Decimal,
    // Mark price, as defined above
    pub mark: Decimal,
    // Mid price, defined as the average of the best bid and best ask price
    pub mid: Decimal,
    // estimated funding rate to be paid in the next funding epoch (hour)
    pub next_funding: Decimal,
    // The current open interest on this symbol (in USD)
    pub open_interest: Decimal,
    // Oracle price, as defined above
    pub oracle: Decimal,
    // Timestamp in Milliseconds
    pub timestamp: u128,
    // Volume (USD) of this market in the past 24 hours
    pub volume_24h: Decimal,
    // Oracle price of this market 24 hours ago (USD)
    pub yesterday_price: Decimal,
}

impl Client {
    pub async fn get_market_info(&self) -> Result<ApiResponse<Vec<Market>>> {
        Ok(self
            .client
            .get(format!("{API_BASE}/{GET_MARKET_INFO}"))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_prices(&self) -> Result<ApiResponse<Vec<Price>>> {
        Ok(self
            .client
            .get(format!("{API_BASE}/{GET_PRICES}"))
            .send()
            .await?
            .json()
            .await?)
    }
}
