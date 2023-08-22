use hmac::{Hmac, Mac};
use chrono::prelude::*;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use sha2::{Sha256};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Client{
    address: String,
    keys: ApiKeys,
}
#[derive(Debug, Clone)]
pub struct ApiKeys {
    public_key: String,
    private_key: String,
}
impl ApiKeys {
    /// Creates new API keys object by the given public/private keys.
    /// # Errors
    /// [`PrivateKey`][error::PrivateKey] error occurs if the private key length
    /// is invalid.
    pub fn new(
        public_key: impl Into<String>,
        private_key: impl Into<String>,
    ) -> Result<Self, ()> {
        Ok(Self {
            public_key: public_key.into().clone(),
            private_key: private_key.into().clone(),
        })
    }
}
impl<'i> Client {

    pub fn new(
        address: String,
        keys: ApiKeys,
    ) -> Result<Self, Client> {
        Ok(Self {
            address,
            keys,
        })
    }

    /// Set the client's API keys.
    pub fn set_keys(&mut self, keys: ApiKeys) {
        self.keys = keys;
    }

    pub fn generate_token_message(
        &mut self,
    ) -> Message {
        let nonce = 3000;
        let timestamp = Utc::now().timestamp_millis().to_string();
        let mut mac = Hmac::<Sha256>::new_from_slice(&base64::decode(self.keys.private_key.clone()).unwrap()).unwrap();
        mac.update((self.keys.public_key.clone() + &timestamp).as_bytes());
        let signature: String = base64::encode(mac.finalize().into_bytes());
        let message = Message::from(format!("[114,{{\"type\":114, \"publicKey\":\"{}\", \"timestamp\":{}, \"nonce\":{}, \"signature\": \"{}\"}}]", self.keys.public_key.clone(), timestamp, nonce, signature));
        return message;
    }
    pub async fn create_connection(&self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        let url = url::Url::parse(&self.address).unwrap();
        let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
        return ws_stream
    }
    pub async fn get_ticker(&mut self) -> () {
        let ws_stream = Self::create_connection(&self);
        let ( mut write, read) = ws_stream.await.split();
        let subscription_message = Message::from("[151,{\"type\":151, \"channel\":\"ticker\", \"event\":\"all\", \"join\":true}]");
        write.send(subscription_message).await.unwrap();
        read.for_each(|message| async {
            let message = message.unwrap();
            println!("Received a message from the server: {:?}", message);
        }).await;
    }
}
