use dotenv::dotenv;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use sha2::{Sha256};
use hmac::{Hmac, Mac};
use chrono::prelude::*;
use std::borrow::Borrow;

// Working main function as an example for the library
#[tokio::main]
async fn main() {
    // TODO create a package
    // TODO Initialize the config

    dotenv().ok();
    let btc_public_key = std::env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY must be set.");
    let btc_private_key = std::env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY must be set.");
    let connect_addr = std::env::var("BTCTURK_WEBSOCKET_ADDRESS").expect("BTCTURK_PRIVATE_KEY must be set.");

    // TODO This is a separate function
    let nonce = 3000;
    let timestamp = Utc::now().timestamp_millis().to_string();
    let mut mac = Hmac::<Sha256>::new_from_slice(&base64::decode(btc_private_key).unwrap()).unwrap();
    mac.update((btc_public_key.clone() + &timestamp).as_bytes());
    let signature: String = base64::encode(mac.finalize().into_bytes());
    let url = url::Url::parse(&connect_addr).unwrap();
    let message = Message::from(format!("[114,{{\"type\":114, \"publicKey\":\"{}\", \"timestamp\":{}, \"nonce\":{}, \"signature\": \"{}\"}}]", btc_public_key, timestamp, nonce, signature));

    // TODO This should be in a separate function
    let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");

    let (mut write, read) = ws_stream.split();
    write.send(message.clone()).await.unwrap();
    let subscription_message = Message::from("[151,{\"type\":151, \"channel\":\"ticker\", \"event\":\"all\", \"join\":true}]");

    write.send(subscription_message).await.unwrap();

    let read_from_socket = read.for_each(|message| async {
        let message = message.unwrap();
        println!("Received a message from the server: {:?}", message);
    });
    tokio::spawn(read_from_socket).await;
}
