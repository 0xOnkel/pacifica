use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use tokio::{sync::mpsc::Sender, time::interval};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
};

use crate::{Client, Result, WS_BASE};

#[derive(Debug, Clone, Serialize)]
pub struct SubscribeMessage<T> {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubscribePrices;

impl SubscribePrices {
    fn params(self) -> Value {
        json!({ "source": "prices" })
    }
}

impl Client {
    /// Subscribes to a WebSocket stream and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let ws_url = WS_BASE;
        let (mut ws_stream, _) = connect_async(ws_url)
            .await
            .expect("Error connecting to WebSocket");

        let subscribe_message = SubscribeMessage {
            method: "subscribe".to_string(),
            params: Some(SubscribePrices.params()),
        };

        ws_stream
            .send(Message::Text(Utf8Bytes::from(serde_json::to_string(
                &subscribe_message,
            )?)))
            .await
            .expect("Error subscribing to WebSocket");

        let mut tick = interval(Duration::from_secs(30));
        // we skip the first tick
        tick.tick().await;

        loop {
            tokio::select!(
                Some(message) = ws_stream.next() => handle_message(message, &tx).await?,
                _ = tick.tick() => {
                    tracing::info!("TICK");
                    ws_stream.send(ping_msg()?).await?;
                }
            )
        }
    }
}

fn ping_msg() -> Result<Message> {
    Ok(Message::text(serde_json::to_string(&SubscribeMessage::<
        Value,
    > {
        method: "ping".to_string(),
        params: None,
    })?))
}

async fn handle_message<T>(
    message: std::result::Result<Message, tokio_tungstenite::tungstenite::Error>,
    tx: &Sender<T>,
) -> Result<()>
where
    T: DeserializeOwned + Send + 'static,
{
    match message {
        Ok(msg) => match msg {
            Message::Text(text) => {
                if let Ok(data) = serde_json::from_str::<T>(&text) {
                    if tx.send(data).await.is_err() {
                        tracing::error!("Failed to send message through the channel");
                    }
                }
            }
            Message::Close(_) => {
                tracing::error!("close message received");
                return Err("closed".into());
            }
            _ => {}
        },
        Err(error) => tracing::error!("WebSocket error: {}", error),
    }

    Ok(())
}
