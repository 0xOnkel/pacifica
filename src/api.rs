use serde_json::{Value, json};

use crate::{API_BASE, Client};

const LIST_API_KEYS: &str = "account/api_keys";

impl Client {
    pub async fn list_api_keys(&mut self) {
        let response = reqwest::Client::new()
            .post(format!("{API_BASE}/{LIST_API_KEYS}"))
            .json(&self.get_request_header(crate::signature::Operation::ListApiKeys, json!({})))
            .header("Content-Type", "application/json")
            .send()
            .await;
        println!("{:?}", response.unwrap().text().await);
    }
}
