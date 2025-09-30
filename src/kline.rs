use reqwest::Url;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{API_BASE, ApiResponse, Client, Result};

const GET_KLINE_DATA: &str = "kline";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KlineInterval {
    #[serde(rename = "1m")]
    I1m,
    #[serde(rename = "3m")]
    I3m,
    #[serde(rename = "5m")]
    I5m,
    #[serde(rename = "15m")]
    I15m,
    #[serde(rename = "30m")]
    I30m,
    #[serde(rename = "1h")]
    I1h,
    #[serde(rename = "2h")]
    I2h,
    #[serde(rename = "4h")]
    I4h,
    #[serde(rename = "8h")]
    I8h,
    #[serde(rename = "12h")]
    I12h,
    #[serde(rename = "1d")]
    I1d,
}

impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KlineInterval::I1m => "1m",
            KlineInterval::I3m => "3m",
            KlineInterval::I5m => "5m",
            KlineInterval::I15m => "15m",
            KlineInterval::I30m => "30m",
            KlineInterval::I1h => "1h",
            KlineInterval::I2h => "2h",
            KlineInterval::I4h => "4h",
            KlineInterval::I8h => "8h",
            KlineInterval::I12h => "12h",
            KlineInterval::I1d => "1d",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Kline {
    // Candle start time
    #[serde(rename = "t")]
    pub start_time: u128,

    // Candle end time
    #[serde(rename = "T")]
    pub end_time: u128,

    // Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Time interval of candles
    #[serde(rename = "i")]
    pub interval: KlineInterval,

    /// Open price
    #[serde(rename = "o")]
    pub open: Decimal,

    /// Close price
    #[serde(rename = "c")]
    pub close: Decimal,

    /// High price
    #[serde(rename = "h")]
    pub high: Decimal,

    /// Low price
    #[serde(rename = "l")]
    pub low: Decimal,

    /// Volume
    #[serde(rename = "v")]
    pub volume: Decimal,

    /// Number of trades
    #[serde(rename = "n")]
    pub trades: u64,
}

impl Client {
    // This endpoint allows users to get historical price candles for
    // a specific market and time interval.
    pub async fn get_kline_data(
        &self,
        // Trading pair symbol
        symbol: &str,
        // Candlestick interval
        interval: KlineInterval,
        // Start time in milliseconds
        start_time: u128,
        // End time in milliseconds, defaults to current time if not provided
        end_time: Option<u128>,
    ) -> Result<ApiResponse<Vec<Kline>>> {
        let mut url = Url::parse(&format!("{API_BASE}/{GET_KLINE_DATA}"))?;
        url.query_pairs_mut()
            .append_pair("symbol", &symbol)
            .append_pair("interval", &interval.to_string())
            .append_pair("start_time", &start_time.to_string());
        if let Some(end_time) = end_time {
            url.query_pairs_mut()
                .append_pair("end_time", &end_time.to_string());
        }

        Ok(self.client.get(url).send().await?.json().await?)
    }
}
