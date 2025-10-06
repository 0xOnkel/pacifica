use rust_decimal::Decimal;
use serde::Serialize;

use crate::{API_BASE, Client, Result};

pub const CREATE_MARKET_ORDER: &str = "orders/create_market";
pub const CREATE_LIMIT_ORDER: &str = "orders/create";

#[derive(Debug, Clone, Serialize)]
pub enum Side {
    Ask,
    Bid,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarketOrder<'a> {
    /// Trading pair symbol
    pub symbol: &'a str,
    /// Order amount
    pub amount: Decimal,
    /// Order side (bid/ask)
    pub side: Side,
    /// Maximum slippage tolerance in percentage, e.g. "0.5" means 0.5% max slippage
    pub slippage_percent: Decimal,
    /// Whether the order is reduce-only
    pub reduce_only: bool,
}

#[derive(Debug, Clone, Serialize)]
pub enum Tif {
    /// Good Til Cancled
    Gtc,
    /// Immediate or Cancel
    Ioc,
    /// Post Only
    Alo,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateLimitOrder<'a> {
    /// Trading pair symbol
    pub symbol: &'a str,
    /// Order price
    pub price: Decimal,
    /// Order amount
    pub amount: Decimal,
    /// Order side (bid/ask)
    pub side: Side,
    /// Time in force (GTC, IOC, ALO)
    pub tif: Tif,
    /// Whether the order is reduce-only
    pub reduce_only: bool,
}

impl Client {
    // This endpoint allows users to create a new market order with optional
    // take profit and stop loss levels.
    pub async fn create_market_order<'a>(
        &mut self,
        create_market_order: CreateMarketOrder<'a>,
    ) -> Result<()> {
        let response = reqwest::Client::new()
            .post(format!("{API_BASE}/{CREATE_MARKET_ORDER}"))
            .json(&self.get_request_header(
                crate::signature::Operation::CreateMarketOrder,
                create_market_order,
            ))
            .header("Content-Type", "application/json")
            .send()
            .await;

        println!("{response:#?}");
        println!("{:?}", response.unwrap().text().await);
        Ok(())
    }

    // This endpoint allows users to create a new limit order with optional
    // take profit and stop loss levels.
    pub async fn create_limit_order<'a>(
        &mut self,
        create_limit_order: CreateLimitOrder<'a>,
    ) -> Result<()> {
        let response = reqwest::Client::new()
            .post(format!("{API_BASE}/{CREATE_LIMIT_ORDER}"))
            .json(&self.get_request_header(
                crate::signature::Operation::CreateLimitOrder,
                create_limit_order,
            ))
            .header("Content-Type", "application/json")
            .send()
            .await;

        println!("{response:#?}");
        println!("{:?}", response.unwrap().text().await);
        Ok(())
    }
}
