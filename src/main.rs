use std::borrow::Borrow;
use dotenv::dotenv;

use futures_util::{future, pin_mut, Sink, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{tungstenite, connect_async, tungstenite::protocol::Message};
use sha2::{Sha256};
use hmac::{Hmac, Mac};
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    // Initialize the config

    dotenv().ok();
    let btc_public_key = std::env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY must be set.");
    let btc_private_key = std::env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY must be set.");
    let connect_addr = std::env::var("BTCTURK_WEBSOCKET_ADDRESS").expect("BTCTURK_PRIVATE_KEY must be set.");
    let url = url::Url::parse(&connect_addr).unwrap();

    // Create a new thread for writing
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    // Create a new token for messages
    let nonce = 3000;
    let timestamp = Utc::now().timestamp_millis().to_string();
    let mut mac = Hmac::<Sha256>::new_from_slice(&base64::decode(btc_private_key).unwrap()).unwrap();
    mac.update((btc_public_key.clone() + &timestamp).as_bytes());
    let signature: String = base64::encode(mac.finalize().into_bytes());
    let message = Message::from(format!("[114,{{\"type\":114, \"publicKey\":\"{}\", \"timestamp\":{}, \"nonce\":{}, \"signature\": \"{}\"}}]", btc_public_key, timestamp, nonce, signature));


    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let ( write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);


    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}