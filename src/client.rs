use crate::{
    ApiKeys,
    Channel,
    types::{Event, TickerEvent, DepthEvent},
};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use base64::{engine::general_purpose, Engine as _};
use serde_json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    keys: ApiKeys,
}

impl Client {
    pub fn new(address: impl Into<String>, keys: ApiKeys) -> Self {
        Self {
            address: address.into(),
            keys,
        }
    }
    pub fn set_keys(&mut self, keys: ApiKeys) {
        self.keys = keys;
    }
    pub fn generate_token_message(&mut self) -> Message {
        let nonce = 3000;
        let timestamp = Utc::now().timestamp_millis().to_string();
        let mut mac = Hmac::<Sha256>::new_from_slice(
            &general_purpose::STANDARD.decode(&self.keys.private_key).unwrap(),
        )
        .unwrap();

        mac.update((self.keys.public_key.clone() + &timestamp).as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        Message::from(format!(
            "[114,{{\"type\":114, \"publicKey\":\"{}\", \"timestamp\":{}, \"nonce\":{}, \"signature\":\"{}\"}}]",
            self.keys.public_key, timestamp, nonce, signature
        ))
    }

    pub async fn create_connection(&self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let url = url::Url::parse(&self.address).unwrap();
        let (ws_stream, _response) = connect_async(url)
            .await
            .expect("Failed to connect to BtcTurk WebSocket");
        ws_stream
    }

    pub async fn subscribe_with_handler<F>(
        &mut self,
        pair: &str,
        channel: Option<Channel>,
        mut handler: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(Event) + Send + 'static,
    {
        let ws_stream = self.create_connection().await;
        let (mut write, mut read) = ws_stream.split();

        let channel = channel.unwrap_or(Channel::Ticker);
        let subscription_message = Message::from(format!(
            "[151,{{\"type\":151, \"channel\":\"{}\", \"event\":\"{}\", \"join\":true}}]",
            channel.as_str(),
            pair
        ));

        write.send(subscription_message).await?;

        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(text) => {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(data) = json.get(1) {
                            if let Some(chan) = data.get("channel").and_then(|v| v.as_str()) {
                                match chan {
                                    "ticker" => {
                                        if let Ok(ev) =
                                            serde_json::from_value::<TickerEvent>(data.clone())
                                        {
                                            handler(Event::Ticker(ev));
                                            continue;
                                        }
                                    }
                                    "depth" => {
                                        if let Ok(ev) =
                                            serde_json::from_value::<DepthEvent>(data.clone())
                                        {
                                            handler(Event::Depth(ev));
                                            continue;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // Skip system messages like [991] or [100]
                    if !text.starts_with("[991")
                        && !text.starts_with("[100")
                        && !text.starts_with("[101")
                    {
                        eprintln!("⚠️ Unparsed message: {}", text);
                    }
                }
                Message::Close(_) => {
                    eprintln!("🔌 Connection closed by server");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub async fn subscribe_ticker<F>(
        &mut self,
        pair: &str,
        handler: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(TickerEvent) + Send + 'static,
    {
        let handler = Arc::new(Mutex::new(handler));

        self.subscribe_with_handler(pair, Some(Channel::Ticker), {
            let handler = Arc::clone(&handler);
            move |event| {
                if let Event::Ticker(t) = event {
                    if let Ok(mut h) = handler.lock() {
                        (h)(t);
                    }
                }
            }
        })
        .await
    }

    /// Subscribes only to order book (depth) updates for a given pair.
    pub async fn subscribe_depth<F>(
        &mut self,
        pair: &str,
        handler: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(DepthEvent) + Send + 'static,
    {
        let handler = Arc::new(Mutex::new(handler));

        self.subscribe_with_handler(pair, Some(Channel::Depth), {
            let handler = Arc::clone(&handler);
            move |event| {
                if let Event::Depth(d) = event {
                    if let Ok(mut h) = handler.lock() {
                        (h)(d);
                    }
                }
            }
        })
        .await
    }
}
