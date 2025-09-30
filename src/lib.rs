use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use base58::FromBase58;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod api;
mod json;
mod market;

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

#[derive(Debug, Serialize)]
pub struct SignatureHeader<T = Value>
where
    T: Serialize,
{
    timestamp: u128,
    expiry_window: u64,
    r#type: Operation,
    data: T,
}

impl Default for SignatureHeader<Value> {
    fn default() -> Self {
        Self {
            timestamp: 0,
            expiry_window: 0,
            r#type: Operation::ListApiKeys,
            data: empty_value(),
        }
    }
}

fn empty_value() -> Value {
    Value::Object(Default::default())
}

#[derive(Debug, Serialize)]
pub struct RequestHeaders<'a> {
    account: &'a str,
    agent_wallet: Option<String>,
    signature: String,
    timestamp: u128,
    expiry_window: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "String")]
pub enum Operation {
    ListApiKeys,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::ListApiKeys => f.write_str("list_api_keys"),
        }
    }
}

impl From<Operation> for String {
    fn from(value: Operation) -> Self {
        value.to_string()
    }
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
