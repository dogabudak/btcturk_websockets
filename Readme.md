# 📦 btcturk_websockets

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-green.svg)
[![Crates.io](https://img.shields.io/crates/v/btcturk_websockets.svg)](https://crates.io/crates/btcturk_websockets)
[![Docs.rs](https://docs.rs/btcturk_websockets/badge.svg)](https://docs.rs/btcturk_websockets)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

A lightweight **Rust client** for the [BtcTurk WebSocket API](https://docs.btcturk.com/).  
Easily subscribe to **real-time ticker**, **order book (depth)** channels, and **execute trades** with full order management support.

---

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
btcturk_websockets = "1.0.0"
```

> Check [crates.io](https://crates.io/crates/btcturk_websockets) for the latest version.

---

## ⚡ Quickstart

```rust
use btcturk_websockets::{ApiKeys, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Public channels don't require real keys
    let api_keys = ApiKeys::new("dummy_public", "dummy_private");
    let mut client = Client::new("wss://ws-feed-pro.btcturk.com/".to_string(), api_keys);

    // Subscribe to live ticker updates
    client
        .subscribe_ticker("BTCTRY", |t| {
            println!(
                "✅ {} → last: {}, bid: {}, ask: {}",
                t.pair_symbol, t.last, t.bid, t.ask
            );
        })
        .await?;

    Ok(())
}
```

---

## 📊 Example: Order Book (Depth)

```rust
use btcturk_websockets::{ApiKeys, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_keys = ApiKeys::new("dummy_public", "dummy_private");
    let mut client = Client::new("wss://ws-feed-pro.btcturk.com/".to_string(), api_keys);

    client
        .subscribe_orderbook("BTCTRY", |ob| {
            if let (Some(bid), Some(ask)) = (ob.bids.first(), ob.asks.first()) {
                println!(
                    "📊 {} → Best Bid: {} @ {}, Best Ask: {} @ {}",
                    ob.pair_symbol, bid.amount, bid.price, ask.amount, ask.price
                );
            }
        })
        .await?;

    Ok(())
}
```

---

### 🔐 Private API Example

```rust
use btcturk_websockets::{ApiKeys, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let public_key = env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY not set");
    let private_key = env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY not set");

    let api_keys = ApiKeys::new(public_key, private_key);
    let client = Client::new("wss://ws-feed-pro.btcturk.com/".to_string(), api_keys);

    // Get account balance using REST API
    let balances = client.get_account_balance().await?;
    println!("✅ Balances: {:?}", balances);

    Ok(())
}
```

---

### 📈 Trading: Submit an Order

Place new orders using the `submit_order` method. You'll need to provide your API keys via environment variables.

```rust
use btcturk_websockets::{ApiKeys, Client, SubmitOrderRequest, OrderMethod, OrderType};
use std::env;

#[tokio::main]
async fn main() {
    let public_key = env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY not set");
    let private_key = env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY not set");

    let keys = ApiKeys::new(public_key, private_key);
    // Use the sandbox environment for testing trades
    let client = Client::new("wss://ws-feed-sandbox.btcturk.com/", keys);

    let order_request = SubmitOrderRequest {
        quantity: "10".to_string(),
        price: Some("50000".to_string()),
        stop_price: None,
        order_method: OrderMethod::Limit,
        order_type: OrderType::Buy,
        pair_symbol: "BTCTRY".to_string(),
        new_order_client_id: None,
    };

    match client.submit_order(order_request).await {
        Ok(response) => {
            println!("✅ Order submitted successfully: {:?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to submit order: {}", e);
        }
    }
}
```

---

## 🧩 Channels & Features

| Feature | Description | Example Method |
|---------|-------------|----------------|
| **Market Data** | | |
| `ticker` | Real-time market prices, volume, and last trades | `subscribe_ticker()` |
| `orderbook` | Order book snapshots and updates | `subscribe_orderbook()` |
| **Private API** | | |
| Account Balance | Get user account balances | `get_account_balance()` |
| Submit Order | Place a new buy or sell order | `submit_order()` |
---

## 🧱 Architecture

- **WebSocket backend:** [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite)
- **HTTP client:** [`reqwest`](https://crates.io/crates/reqwest) for REST API trading calls
- **Async runtime:** [`tokio`](https://crates.io/crates/tokio)
- **Serialization:** [`serde` / `serde_json`](https://serde.rs/)
- **Auth signing:** [`hmac` + `sha2` + `base64`]

All messages are deserialized into typed Rust structs:
```rust
TickerEvent { pair_symbol, bid, ask, last, volume, ... }
OrderBookEvent { bids, asks, pair_symbol, ... }
```


**Made with 🦀 in Rust** – All contributions welcome!
