use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PriceUpdateMsg {
    channel: String,
    data: Vec<PriceUpdate>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PriceUpdate {
    pub symbol: String,
    pub funding: Decimal,
    pub next_funding: Decimal,
    pub oracle: Decimal,
    pub mark: Decimal,
    pub mid: Decimal,
    pub yesterday_price: Decimal,
    pub open_interest: Decimal,
    pub volume_24h: Decimal,
    pub timestamp: u128,
}
// {
// 			"symbol": "ASTER",
// 			"funding": "0.00005647",
// 			"next_funding": "0.00008587",
// 			"oracle": "1.85083",
// 			"mark": "1.852394",
// 			"mid": "1.85525",
// 			"yesterday_price": "2.04135",
// 			"open_interest": "1739320.62",
// 			"volume_24h": "13298021.1951094",
// 			"timestamp": 1759728574569
// 		},
