use std::time::{SystemTime, UNIX_EPOCH};

use base58::FromBase58;
use ed25519_dalek::SigningKey;
use serde::Deserialize;

mod api;
mod json;
pub mod kline;
mod market;
pub mod order;
mod orderbook;
mod signature;

pub const API_BASE: &str = "https://api.pacifica.fi/api/v1";
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub error: Option<String>,
    pub code: Option<String>,
}

pub fn timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub struct Client {
    account: String,
    client: reqwest::Client,
    signing_key: SigningKey,
}

pub struct ClientBuilder {
    account: String,
    signing_key: SigningKey,
}

impl Client {
    pub fn builder(api_key: String, account: String) -> ClientBuilder {
        let signing_key = SigningKey::from_keypair_bytes(
            api_key
                .from_base58()
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        ClientBuilder {
            account,
            signing_key,
        }
    }
}

impl ClientBuilder {
    pub fn build(self) -> Client {
        Client {
            account: self.account,
            client: reqwest::Client::new(),
            signing_key: self.signing_key,
        }
    }
}
