# 📦 btcturk_websockets

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-green.svg)  
[![Crates.io](https://img.shields.io/crates/v/btcturk_websockets.svg)](https://crates.io/crates/btcturk_websockets)  
[![Docs.rs](https://docs.rs/btcturk_websockets/badge.svg)](https://docs.rs/btcturk_websockets)  
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

A Rust client for the **BtcTurk WebSocket API**.  
Subscribe to live **ticker** data today; designed to extend to **depth (order book)** and **OHLC** channels.

---

## 🚀 Installation

Add this to your `Cargo.toml` (use your published minor series):

```toml
[dependencies]
btcturk_websockets = "0.4.1"
```

> Check the latest version on crates.io if newer is available.

---

## 🔧 Quickstart (works with current API)

The current public API exposes `Client::get_ticker_with_handler` which yields raw WebSocket messages.  
For public data, dummy keys are fine (BtcTurk doesn’t require auth for ticker).

```rust
use btcturk_websockets::{Client, ApiKeys};
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Public feeds don't need real keys; the Client struct requires them, so any strings work here.
    let api_keys = ApiKeys::new("dummy_public", "dummy_private");

    // Connect to BtcTurk feed
    let mut client = Client::new(
        "wss://ws-feed-pro.btcturk.com/".to_string(),
        api_keys,
    );

    // Subscribe to BTC/USDT ticker and print raw messages
    client
        .get_ticker_with_handler("BTCUSDT", |msg: Message| {
            println!("Ticker: {:?}", msg);
        })
        .await?;

    Ok(())
}
```

### Run as an example

Create `examples/ticker.rs` with the code above, then:

```bash
cargo run --example ticker
```

---

## 🧱 Roadmap (optional API you may add)

If/when you add a unified subscribe function and a `Channel` enum, usage could look like this:

```rust
// hypothetically provided by a newer crate version
use btcturk_websockets::{Client, ApiKeys, Channel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_keys = ApiKeys::new("dummy_public", "dummy_private");
    let mut client = Client::new("wss://ws-feed-pro.btcturk.com/".into(), api_keys);

    // Fallback to ticker if None
    client.subscribe_with_handler("BTCUSDT", None, |msg| {
        println!("Ticker (fallback): {:?}", msg);
    }).await?;

    // Depth (order book)
    client.subscribe_with_handler("BTCUSDT", Some(Channel::Depth), |msg| {
        println!("Depth: {:?}", msg);
    }).await?;

    // OHLC (candlesticks)
    client.subscribe_with_handler("BTCUSDT", Some(Channel::Ohlc), |msg| {
        println!("OHLC: {:?}", msg);
    }).await?;

    Ok(())
}
```

> Until that API lands, use `get_ticker_with_handler` as shown in **Quickstart**.

---

## 🔐 Authentication

- **Public channels** (ticker, depth, ohlc) do **not** require real API keys.  
- **Private channels** (if implemented in the future) will require valid keys and signature.

---

## 📜 License

This project is licensed under the [MIT license](./LICENSE).
