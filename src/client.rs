use crate::{
    ApiKeys,
    Channel,
    types::{Event, TickerEvent, OrderBookEvent},
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
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};
use reqwest;

/// Represents an API client that can handle both WebSocket and REST calls.
#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    keys: ApiKeys,
    http_client: reqwest::Client,
}

/// A single asset balance from the /users/balances endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    pub asset: String,
    pub assetname: String,
    pub balance: String,
    pub locked: String,
    pub free: String,
    #[serde(rename = "orderFund")]
    pub order_fund: String,
    #[serde(rename = "requestFund")]
    pub request_fund: String,
    pub precision: i32,
    pub timestamp: i64,
}

/// Response wrapper for /users/balances
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BalanceResponse {
    pub data: Vec<Balance>,
    pub success: bool,
    pub message: Option<String>,
    pub code: i32,
}

impl Client {
    pub fn new(address: impl Into<String>, keys: ApiKeys) -> Self {
        Self {
            address: address.into(),
            keys,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn set_keys(&mut self, keys: ApiKeys) {
        self.keys = keys;
    }

    fn auth_headers(&self) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
        use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    
        let timestamp = Utc::now().timestamp_millis();
        let data = format!("{}{}", self.keys.public_key, timestamp);
    
        // 🧩 Decode API secret (supports both Base64 variants)
        let decoded_secret = match general_purpose::STANDARD.decode(&self.keys.private_key) {
            Ok(bytes) => bytes,
            Err(_) => general_purpose::URL_SAFE_NO_PAD
                .decode(&self.keys.private_key)
                .map_err(|_| "Failed to decode API secret as Base64")?,
        };
    
        let mut mac = Hmac::<Sha256>::new_from_slice(&decoded_secret)?;
        mac.update(data.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
    
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-PCK", HeaderValue::from_str(&self.keys.public_key)?);
        headers.insert("X-Stamp", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-Signature", HeaderValue::from_str(&signature)?);
    
        Ok(headers)
    }

    pub async fn get_account_balance(
        &self,
    ) -> Result<BalanceResponse, Box<dyn std::error::Error>> {
        let url = "https://api.btcturk.com/api/v1/users/balances";
        let headers = self.auth_headers()?;

        let res = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let body: BalanceResponse = res.json().await?;
        Ok(body)
    }

    /// Generates WebSocket authentication token message
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
            "[114,{{\"type\":114,\"publicKey\":\"{}\",\"timestamp\":{},\"nonce\":{},\"signature\":\"{}\"}}]",
            self.keys.public_key, timestamp, nonce, signature
        ))
    }

    pub async fn create_connection(&self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let url = url::Url::parse(&self.address).unwrap();
        let (ws_stream, _response) = connect_async(url)
            .await
            .expect("❌ Failed to connect to BtcTurk WebSocket");
        ws_stream
    }

    /// Generic subscription entry point
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
        let subscribe_msg = Message::from(format!(
            "[151,{{\"type\":151,\"channel\":\"{}\",\"event\":\"{}\",\"join\":true}}]",
            channel.as_str(),
            pair
        ));

        write.send(subscribe_msg).await?;
        println!("✅ Subscribed to {}:{}", channel.as_str(), pair);

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
                                    "orderbook" => {
                                        if let Ok(ev) =
                                            serde_json::from_value::<OrderBookEvent>(data.clone())
                                        {
                                            handler(Event::OrderBook(ev));
                                            continue;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    if !text.starts_with("[991")
                        && !text.starts_with("[100")
                        && !text.starts_with("[101")
                    {
                        eprintln!("⚠️ Unparsed message: {text}");
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

    /// Subscribe to ticker updates
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

    /// Subscribe to live orderbook updates
    pub async fn subscribe_orderbook<F>(
        &mut self,
        pair: &str,
        handler: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(OrderBookEvent) + Send + 'static,
    {
        let handler = Arc::new(Mutex::new(handler));

        self.subscribe_with_handler(pair, Some(Channel::Orderbook), {
            let handler = Arc::clone(&handler);
            move |event| {
                if let Event::OrderBook(d) = event {
                    if let Ok(mut h) = handler.lock() {
                        (h)(d);
                    }
                }
            }
        })
        .await
    }
}
