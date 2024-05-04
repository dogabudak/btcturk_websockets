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

## Example

```rust
use btcturk_websockets::{Client, ApiKeys};

#[tokio::main]
async fn main() {
// Retrieve API keys and WebSocket address from environment variables
let btc_public_key = std::env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY must be set.");
let btc_private_key = std::env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY must be set.");
let connect_addr = std::env::var("BTCTURK_WEBSOCKET_ADDRESS").expect("BTCTURK_WEBSOCKET_ADDRESS must be set.");

    // Initialize API keys
    let api_keys = ApiKeys::new(btc_public_key, btc_private_key);
    
    // Initialize WebSocket client
    let client = Client::new(connect_addr, api_keys);
    
    // Generate token message for authentication
    let token = client.clone().generate_token_message();
    
    // Create WebSocket connection
    let connection = client.clone().create_connection().await;
    
    // Retrieve ticker information
    let ticker = client.clone().get_ticker().await;
}
```
