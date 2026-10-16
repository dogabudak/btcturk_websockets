use crate::{
    ApiKeys,
    Channel,
    error::{Error, Result},
    types::{Event, TickerEvent, OrderBookEvent, SubmitOrderRequest, SubmitOrderResponse, TickerRestResponse},
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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};
use reqwest;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    keys: ApiKeys,
    http_client: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    pub asset: String,
    pub assetname: String,
    pub balance: Decimal,
    pub locked: Decimal,
    pub free: Decimal,
    #[serde(rename = "orderFund")]
    pub order_fund: Decimal,
    #[serde(rename = "requestFund")]
    pub request_fund: Decimal,
    pub precision: i32,
    pub timestamp: i64,
}

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

    /// Signs `public_key + timestamp` with the Base64-decoded private key.
    fn sign(&self, timestamp: &str) -> Result<String> {
        let secret = general_purpose::STANDARD
            .decode(&self.keys.private_key)
            .or_else(|err| {
                general_purpose::URL_SAFE_NO_PAD
                    .decode(&self.keys.private_key)
                    .map_err(|_| Error::InvalidSecret(err))
            })?;

        let mut mac = Hmac::<Sha256>::new_from_slice(&secret)
            .expect("HMAC accepts keys of any length");
        mac.update(self.keys.public_key.as_bytes());
        mac.update(timestamp.as_bytes());
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    fn auth_headers(&self) -> Result<reqwest::header::HeaderMap> {
        use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

        let timestamp = Utc::now().timestamp_millis().to_string();
        let signature = self.sign(&timestamp)?;
    
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-PCK", HeaderValue::from_str(&self.keys.public_key)?);
        headers.insert("X-Stamp", HeaderValue::from_str(&timestamp)?);
        headers.insert("X-Signature", HeaderValue::from_str(&signature)?);
    
        Ok(headers)
    }

    pub async fn get_account_balance(&self) -> Result<BalanceResponse> {
        let url = "https://api.btcturk.com/api/v1/users/balances";
        let headers = self.auth_headers()?;

        let res = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        decode(success_body(res).await?)
    }

    pub async fn submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse> {
        let url = "https://api.btcturk.com/api/v1/order";
        let headers = self.auth_headers()?;

        let res = self
            .http_client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        let body = success_body(res).await?;
        parse_order_response(&body).map_err(|source| Error::Decode { source, body })
    }

    pub async fn get_ticker(
        &self,
        pair_symbol: Option<&str>,
    ) -> Result<TickerRestResponse> {
        let mut url = reqwest::Url::parse("https://api.btcturk.com/api/v2/ticker")?;

        if let Some(pair) = pair_symbol {
            url.query_pairs_mut().append_pair("pairSymbol", pair);
        }

        let res = self
            .http_client
            .get(url)
            .send()
            .await?;

        decode(success_body(res).await?)
    }

    pub async fn get_exchange_info(&self) -> Result<crate::types::ExchangeInfoResponse> {
        let url = "https://api.btcturk.com/api/v2/server/exchangeinfo";
        
        let res = self
            .http_client
            .get(url)
            .send()
            .await?;

        decode(success_body(res).await?)
    }

    pub fn get_symbol_info<'a>(&self, exchange_info: &'a crate::types::ExchangeInfoResponse, pair_symbol: &str) -> Option<&'a crate::types::SymbolInfo> {
        exchange_info.data.symbols.iter().find(|s| s.name == pair_symbol)
    }

    /// Generates WebSocket authentication token message
    pub fn generate_token_message(&mut self) -> Result<Message> {
        let nonce = 3000;
        let timestamp = Utc::now().timestamp_millis().to_string();
        let signature = self.sign(&timestamp)?;

        Ok(Message::from(format!(
            "[114,{{\"type\":114,\"publicKey\":\"{}\",\"timestamp\":{},\"nonce\":{},\"signature\":\"{}\"}}]",
            self.keys.public_key, timestamp, nonce, signature
        )))
    }

    pub async fn create_connection(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let url = url::Url::parse(&self.address)?;
        let (ws_stream, _response) = connect_async(url).await?;
        Ok(ws_stream)
    }

    pub async fn subscribe_with_handler<F>(
        &mut self,
        pair: &str,
        channel: Option<Channel>,
        mut handler: F,
    ) -> Result<()>
    where
        F: FnMut(Event) + Send + 'static,
    {
        let ws_stream = self.create_connection().await?;
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
                    if let Some(event) = Event::parse(&text) {
                        handler(event);
                        continue;
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
    ) -> Result<()>
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
    ) -> Result<()>
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

/// Returns the response body, or [`Error::Api`] if the status is not a success.
async fn success_body(res: reqwest::Response) -> Result<String> {
    let status = res.status();
    let body = res.text().await?;
    if !status.is_success() {
        return Err(Error::Api { status, body });
    }
    Ok(body)
}

fn decode<T: DeserializeOwned>(body: String) -> Result<T> {
    serde_json::from_str(&body).map_err(|source| Error::Decode { source, body })
}

/// Order responses may arrive wrapped in a `data` field or bare.
fn parse_order_response(body: &str) -> serde_json::Result<SubmitOrderResponse> {
    #[derive(Deserialize)]
    struct Envelope<'a> {
        #[serde(borrow)]
        data: Option<&'a serde_json::value::RawValue>,
    }

    match serde_json::from_str::<Envelope>(body)?.data {
        Some(data) => serde_json::from_str(data.get()),
        None => serde_json::from_str(body),
    }
}
