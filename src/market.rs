use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{API_BASE, ApiResponse, Client, Result};

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

impl Client {
    pub async fn get_market_info(&self) -> Result<ApiResponse<Vec<Market>>> {
        Ok(self
            .client
            .get(format!("{API_BASE}/info"))
            .send()
            .await?
            .json()
            .await?)
    }
}
