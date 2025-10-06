use std::fmt::Display;

use base58::ToBase58;
use ed25519_dalek::ed25519::signature::SignerMut;
use serde::Serialize;
use serde_json::Value;

use crate::{Client, json::to_sorted_json, timestamp_ms};

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

#[derive(Debug, Clone, Serialize)]
#[serde(into = "String")]
pub enum Operation {
    CreateMarketOrder,
    CreateLimitOrder,
    ListApiKeys,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::CreateMarketOrder => f.write_str("create_market_order"),
            Operation::CreateLimitOrder => f.write_str("create_order"),
            Operation::ListApiKeys => f.write_str("list_api_keys"),
        }
    }
}

impl From<Operation> for String {
    fn from(value: Operation) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Serialize)]
pub struct RequestHeaders<'a> {
    account: &'a str,
    agent_wallet: Option<String>,
    signature: String,
    timestamp: u128,
    expiry_window: u64,
}

fn merge<T: Serialize, U: Serialize>(a: &T, b: &U) -> serde_json::Value {
    let mut a_map = serde_json::to_value(a)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();

    let b_map = serde_json::to_value(b).unwrap();

    a_map.extend(b_map.as_object().unwrap().clone());
    Value::Object(a_map)
}

impl Client {
    pub fn get_request_header<'a, T>(&'a mut self, operation: Operation, data: T) -> Value
    where
        T: Serialize + Clone,
    {
        let signature_header = SignatureHeader {
            timestamp: timestamp_ms(),
            expiry_window: 5000,
            r#type: operation,
            data: data.clone(),
        };

        let sorted_json = to_sorted_json(&signature_header);
        let signed_header = self.signing_key.sign(sorted_json.as_bytes());
        let signature = signed_header.to_vec().as_slice().to_base58();
        let request_headers = RequestHeaders {
            account: &self.account,
            agent_wallet: Some(
                self.signing_key
                    .verifying_key()
                    .to_bytes()
                    .as_slice()
                    .to_base58(),
            ),
            signature,
            timestamp: signature_header.timestamp,
            expiry_window: signature_header.expiry_window,
        };
        merge(&request_headers, &data)
    }
}
