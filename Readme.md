# 📦 btcturk_websockets

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-green.svg)
[![Crates.io](https://img.shields.io/crates/v/btcturk_websockets.svg)](https://crates.io/crates/btcturk_websockets)
[![Docs.rs](https://docs.rs/btcturk_websockets/badge.svg)](https://docs.rs/btcturk_websockets)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

A lightweight **Rust client** for the [BtcTurk WebSocket API](https://docs.btcturk.com/).  
Easily subscribe to **real-time ticker** and **order book (depth)** channels — no authentication required.

---

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
btcturk_websockets = "0.4.1"
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
        .subscribe_depth("BTCTRY", |d| {
            println!("📊 Depth update for {}:", d.event);

            if let (Some(bid), Some(ask)) = (d.bids.first(), d.asks.first()) {
                println!("  🟩 Best bid: {} @ {}", bid[1], bid[0]);
                println!("  🟥 Best ask: {} @ {}", ask[1], ask[0]);
            }

            println!("  ({} bids / {} asks)", d.bids.len(), d.asks.len());
            println!("------------------------------------");
        })
        .await?;

    Ok(())
}
```

---

## 🔐 Authentication

- **Public channels** (`ticker`, `depth`) do **not** require real API keys.
- **Private channels** (not yet implemented) will use the `generate_token_message` method
  with your API key/secret.

---

## 🧩 Channels

| Channel | Description | Example Method |
|----------|--------------|----------------|
| `ticker` | Real-time market prices, volume, and last trades | `subscribe_ticker()` |
| `depth` | Order book snapshots and changes | `subscribe_depth()` |

---

## 🧱 Architecture

- **WebSocket backend:** [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite)
- **Async runtime:** [`tokio`](https://crates.io/crates/tokio)
- **Serialization:** [`serde` / `serde_json`](https://serde.rs/)
- **Auth signing:** [`hmac` + `sha2` + `base64`]

All messages are deserialized into typed Rust structs:
```rust
TickerEvent { pair_symbol, bid, ask, last, volume, ... }
DepthEvent  { bids, asks, pair_id, ... }
```

---

## 🧠 Roadmap

- [ ] Add trade history (recent trades) channel  
- [ ] Add private user data channels (balance, order updates)  
- [ ] Add reconnection + heartbeat support  
- [ ] Add optional REST OHLC fetcher  

---

## 📜 License

This project is licensed under the [MIT License](./LICENSE).

---

**Made with 🦀 in Rust** – contributions welcome!
