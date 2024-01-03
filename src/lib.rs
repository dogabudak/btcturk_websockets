use hmac::{Hmac, Mac};
use chrono::prelude::*;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use sha2::{Sha256};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Client {
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
    pub fn new(
        public_key: impl Into<String>,
        private_key: impl Into<String>,
    ) -> Self {
        Self {
            public_key: public_key.into().clone(),
            private_key: private_key.into().clone(),
        }
    }
}

impl<'i> Client {
    pub fn new(
        address: String,
        keys: ApiKeys,
    ) -> Self {
        Self {
            address,
            keys,
        }
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
        return ws_stream;
    }
    pub async fn get_ticker(&mut self) -> () {
        let ws_stream = Self::create_connection(&self);
        let (mut write, read) = ws_stream.await.split();
        let subscription_message = Message::from("[151,{\"type\":151, \"channel\":\"ticker\", \"event\":\"all\", \"join\":true}]");
        write.send(subscription_message).await.unwrap();
        let read_from_socket = read.for_each(|message| async {
            let message = message.unwrap();
            println!("Received a message from the server: {:?}", message);
        });
        tokio::spawn(read_from_socket).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::Utc;
    use sha2::Sha256;
    use hmac::{Hmac, Mac};
    use base64;

    #[test]
    fn create_api_keys() {

        let created_keys = ApiKeys::new("asd", "fgh");
        assert_eq!(created_keys.private_key, "fgh");
        assert_eq!(created_keys.public_key, "asd");
    }
        #[test]
        fn test_generate_token_message() {
            let api_keys=ApiKeys::new(String::from("dG9rZW4="), String::from("dG9rZW4="));
            let client = Client::new(String::from("address"), api_keys);

            let token = client.clone().generate_token_message();
            let nonce = 3000;
            let timestamp = Utc::now().timestamp_millis().to_string();
            let mut mac = Hmac::<Sha256>::new_from_slice(&base64::decode(client.keys.private_key.clone()).unwrap()).unwrap();
            mac.update((client.keys.public_key.clone() + &timestamp).as_bytes());
            let expected_signature: String = base64::encode(mac.finalize().into_bytes());
            let expected_message = Message::from(format!("[114,{{\"type\":114, \"publicKey\":\"{}\", \"timestamp\":{}, \"nonce\":{}, \"signature\": \"{}\"}}]", client.keys.public_key.clone(), timestamp, nonce, expected_signature));

            assert_eq!(token, expected_message);
        }
}
