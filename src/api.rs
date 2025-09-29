use base58::ToBase58;
use ed25519_dalek::ed25519::signature::SignerMut;

use crate::{
    API_BASE, Client, RequestHeaders, SignatureHeader, json::to_sorted_json, timestamp_ms,
};

impl Client {
    pub async fn list_api_keys(&mut self) {
        let url = "/account/api_keys";

        let signature_header = SignatureHeader {
            timestamp: timestamp_ms(),
            expiry_window: 5000,
            r#type: crate::Operation::ListApiKeys,
            ..Default::default()
        };

        let data = to_sorted_json(&signature_header);
        let data = self.signing_key.sign(data.as_bytes());
        let signature = data.to_vec().as_slice().to_base58();
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
        let response = reqwest::Client::new()
            .post(format!("{API_BASE}{url}"))
            .json(&request_headers)
            .header("Content-Type", "application/json")
            .send()
            .await;
        println!("{:?}", response.unwrap().text().await);
    }
}
