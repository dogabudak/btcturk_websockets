# Rust WebSocket Crate

![Rust Version](https://img.shields.io/badge/rust-1.65+-green.svg)
![WebSocket Version](https://img.shields.io/badge/websocket-0.0.1-blue.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
btcturk_websockets = "0.0.1"
```

## Example Usage

Here's an example of how to use the `btcturk_websockets` library:

```rust
use btcturk_websockets::{Client, ApiKeys};
use dotenv::dotenv;

#[tokio::test]
async fn general_test() {
    dotenv().ok(); // Load environment variables from .env file
    let btc_public_key = std::env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY must be set.");
    let btc_private_key = std::env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY must be set.");
    let connect_addr = std::env::var("BTCTURK_WEBSOCKET_ADDRESS").expect("BTCTURK_PRIVATE_KEY must be set.");
    let api_keys=ApiKeys::new(btc_public_key, btc_private_key);
    let client = Client::new(connect_addr, api_keys);
    let token = client.clone().generate_token_message();
    let connection = client.clone().create_connection().await;
    let ticker = client.clone().get_ticker("BTCTRY").await;
}
```

Make sure to set up your `.env` file with the necessary environment variables before running the example.
