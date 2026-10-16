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
btcturk_websockets = "2.0.0"
dotenvy = "0.15" # For loading .env files in examples
```

> Check [crates.io](https://crates.io/crates/btcturk_websockets) for the latest version.

---

## ⚡ Quickstart

```rust
use btcturk_websockets::{ApiKeys, Client};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    // Public channels don't require real keys, but we'll use env vars if available
    let public_key = env::var("BTCTURK_PUBLIC_KEY").unwrap_or_else(|_| "dummy_public".to_string());
    let private_key = env::var("BTCTURK_PRIVATE_KEY").unwrap_or_else(|_| "dummy_private".to_string());

    let api_keys = ApiKeys::new(public_key, private_key);
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
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let public_key = env::var("BTCTURK_PUBLIC_KEY").unwrap_or_else(|_| "dummy_public".to_string());
    let private_key = env::var("BTCTURK_PRIVATE_KEY").unwrap_or_else(|_| "dummy_private".to_string());

    let api_keys = ApiKeys::new(public_key, private_key);
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
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

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
use btcturk_websockets::{ApiKeys, Client, Decimal, SubmitOrderRequest, OrderMethod, OrderType};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let public_key = env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY not set");
    let private_key = env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY not set");

    let keys = ApiKeys::new(public_key, private_key);
    // Use the sandbox environment for testing trades
    let client = Client::new("wss://ws-feed-sandbox.btcturk.com/", keys);

    let order_request = SubmitOrderRequest {
        quantity: Decimal::from(10),
        price: Some(Decimal::from(50000)),
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

### 🌎 Public REST API: Get Ticker

Fetch real-time ticker data for a specific pair or all available pairs using the `get_ticker` method. API keys are not required for this public endpoint.

```rust
use btcturk_websockets::{Client, ApiKeys, TickerRestResponse};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    // API keys are not strictly required for public endpoints, but we'll initialize them for consistency
    let public_key = env::var("BTCTURK_PUBLIC_KEY").unwrap_or_else(|_| "dummy_public".to_string());
    let private_key = env::var("BTCTURK_PRIVATE_KEY").unwrap_or_else(|_| "dummy_private".to_string());

    let api_keys = ApiKeys::new(public_key, private_key);
    let client = Client::new("https://api.btcturk.com", api_keys);

    // Fetch ticker for a specific pair
    println!("Fetching ticker for BTCTRY:");
    match client.get_ticker(Some("BTCTRY")).await {
        Ok(response) => {
            for ticker in response.data {
                println!("  Pair: {}, Last: {}, Bid: {}, Ask: {}", ticker.pair_symbol, ticker.last, ticker.bid, ticker.ask);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch BTCTRY ticker: {}", e);
        }
    }

    println!("\nFetching ticker for all pairs:");
    // Fetch ticker for all pairs
    match client.get_ticker(None).await {
        Ok(response) => {
            for ticker in response.data {
                println!("  Pair: {}, Last: {}, Bid: {}, Ask: {}", ticker.pair_symbol, ticker.last, ticker.bid, ticker.ask);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch all tickers: {}", e);
        }
    }

    Ok(())
}
```

---

### 🔢 Decimal Amounts

All prices, amounts, and balances are [`Decimal`](https://docs.rs/rust_decimal) values, so they can be compared and calculated exactly with no floating-point rounding. `Decimal` is re-exported, so you don't need to add `rust_decimal` yourself:

```rust
use btcturk_websockets::Decimal;

client
    .subscribe_orderbook("BTCTRY", |ob| {
        if let (Some(bid), Some(ask)) = (ob.bids.first(), ob.asks.first()) {
            let spread: Decimal = ask.price - bid.price;
            println!("Spread: {spread} TRY");
        }
    })
    .await?;
```

Create values with `Decimal::from(10)` or `"0.001".parse::<Decimal>()?`. For rounding helpers, use the re-exported crate: `btcturk_websockets::rust_decimal::RoundingStrategy`.

---

### 🚨 Error Handling

Every fallible method returns `btcturk_websockets::Result<T>`, using the crate's `Error` enum. Match on it to handle specific failures:

```rust
use btcturk_websockets::Error;

match client.submit_order(order_request).await {
    Ok(order) => println!("✅ Order {} placed", order.id),
    Err(Error::Api { status, body }) => eprintln!("BtcTurk rejected the order ({status}): {body}"),
    Err(Error::InvalidSecret(_)) => eprintln!("BTCTURK_PRIVATE_KEY is not valid Base64"),
    Err(e) => eprintln!("Request failed: {e}"),
}
```

| Variant | When |
|---|---|
| `InvalidSecret` | The private key is not valid Base64 |
| `InvalidHeader` | An API key contains characters not allowed in HTTP headers |
| `InvalidUrl` | The address passed to `Client::new` can't be parsed |
| `Http` | The HTTP request failed or the body couldn't be read |
| `Api { status, body }` | BtcTurk answered with a non-success status |
| `Decode { source, body }` | The response JSON didn't match the expected shape |
| `WebSocket` | Connecting, sending, or reading on the WebSocket failed |

`Error` is `Send + Sync`, so `?` also works in functions returning `Box<dyn std::error::Error>` or `anyhow::Result`.

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
| **Public REST API** | | |
| Get Ticker | Fetch current ticker data | `get_ticker()` |
| Exchange Info | Fetch trading pairs, scales, and price limits | `get_exchange_info()` |
| **Utilities** | | |
| Frame Parsing | Parse a raw BtcTurk WebSocket frame into an `Event` | `Event::parse()` |

---

## 🧱 Architecture

- **WebSocket backend:** [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite)
- **HTTP client:** [`reqwest`](https://crates.io/crates/reqwest) for REST API trading calls
- **Async runtime:** [`tokio`](https://crates.io/crates/tokio)
- **Serialization:** [`serde` / `serde_json`](https://serde.rs/), with WebSocket frames deserialized straight from the frame text
- **Numbers:** [`rust_decimal`](https://crates.io/crates/rust_decimal) for exact prices and amounts
- **Errors:** [`thiserror`](https://crates.io/crates/thiserror)-based `Error` enum
- **Auth signing:** [`hmac` + `sha2` + `base64`]

All messages are deserialized into typed Rust structs:
```rust
TickerEvent { pair_symbol: String, bid: Decimal, ask: Decimal, last: Decimal, volume: Decimal, ... }
OrderBookEvent { pair_symbol: String, bids: Vec<OrderBookOrder>, asks: Vec<OrderBookOrder>, ... }
OrderBookOrder { price: Decimal, amount: Decimal }
TickerRestData { pair_symbol: String, bid: Decimal, ask: Decimal, last: Decimal, high: Decimal, low: Decimal, volume: Decimal }
```

---

## ⬆️ Upgrading from 1.x

- **Numeric fields are `Decimal`, not `String`.** Build orders with `quantity: Decimal::from(10)` instead of `"10".to_string()`. Compare against `Decimal` values (for example `"0.5".parse::<Decimal>()?`) instead of string literals, and drop any `.parse::<f64>()` calls.
- **Methods return `btcturk_websockets::Result<T>`.** Code that uses `?` inside `Box<dyn Error>` functions keeps working. Matching on error strings should switch to matching `Error` variants.
- **`create_connection()` and `generate_token_message()` return `Result`.** Add a `?` where you call them; they no longer panic.


**Made with 🦀 in Rust** – All contributions welcome!
