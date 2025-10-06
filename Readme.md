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

## 🔐 Authentication

- **Public channels** (`ticker`, `orderbook`) do **not** require real API keys.
- **Private channels** (not yet implemented) will use the `generate_token_message` method
  with your API key/secret.

---

## 🧩 Channels

| Channel | Description | Example Method |
|----------|--------------|----------------|
| `ticker` | Real-time market prices, volume, and last trades | `subscribe_ticker()` |
| `orderbook` | Order book snapshots and updates | `subscribe_orderbook()` |

---

## 🧱 Architecture

- **WebSocket backend:** [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite)
- **Async runtime:** [`tokio`](https://crates.io/crates/tokio)
- **Serialization:** [`serde` / `serde_json`](https://serde.rs/)
- **Auth signing:** [`hmac` + `sha2` + `base64`]

All messages are deserialized into typed Rust structs:
```rust
TickerEvent { pair_symbol, bid, ask, last, volume, ... }
OrderBookEvent { bids, asks, pair_symbol, ... }
```

---

## 🧠 Roadmap

- [ ] Add trade history (recent trades) channel  
- [ ] Add private user data channels (balance, order updates)  
- [ ] Add reconnection + heartbeat support  
- [ ] Add optional REST OHLC fetcher  

---

## 🔧 Code Review & Tasks

### 🛠️ Code Quality Improvements

- [ ] **Add comprehensive error types** - Create custom error enum instead of using `Box<dyn std::error::Error>`
- [ ] **Improve documentation** - Add rustdoc comments for all public APIs and structs
- [ ] **Add unit tests** - Create test module with tests for parsing, authentication, and connection logic
- [ ] **Add integration tests** - Test actual WebSocket connections with mock server
- [ ] **Fix hardcoded nonce in authentication** - `generate_token_message()` uses hardcoded nonce value `3000` which should be random/unique per request
- [ ] **Improve error handling** - Replace `unwrap()` calls with proper error propagation, especially in `create_connection()` and base64 decoding
- [ ] **Add input validation** - Validate API keys format and WebSocket URL before connection attempts
- [ ] **Fix potential panic in base64 decoding** - `general_purpose::STANDARD.decode(&self.keys.private_key).unwrap()` can panic

### 🏗️ Architecture Enhancements

- [ ] **Implement connection pooling** - Allow multiple concurrent subscriptions without creating new connections
- [ ] **Add reconnection logic** - Implement automatic reconnection with exponential backoff
- [ ] **Add heartbeat/ping mechanism** - Keep connections alive with periodic ping messages
- [ ] **Implement graceful shutdown** - Add proper cleanup and connection termination
- [ ] **Add connection state management** - Track connection status and provide state queries

### 🔒 Security & Robustness

- [ ] **Secure API key storage** - Consider using `secrecy` crate for sensitive data
- [ ] **Add rate limiting** - Implement rate limiting for subscription requests
- [ ] **Validate message integrity** - Add checksum validation for incoming messages
- [ ] **Add connection timeout** - Implement connection timeout and retry logic

### 📊 Data Handling Improvements

- [ ] **Add data validation** - Validate incoming JSON structure before deserialization
- [ ] **Improve type safety** - Use `Decimal` type for price/amount fields instead of `String`
- [ ] **Add data filtering** - Allow filtering messages by pair or event type
- [ ] **Implement message buffering** - Add optional message buffering for high-frequency updates

### 🧪 Testing & Documentation

- [ ] **Add example with error handling** - Create example showing proper error handling patterns
- [ ] **Add performance benchmarks** - Benchmark message parsing and connection performance
- [ ] **Add API documentation** - Generate and host comprehensive API documentation
- [ ] **Add usage examples** - Create more comprehensive usage examples for different scenarios

### 🚀 Performance Optimizations

- [ ] **Optimize JSON parsing** - Use streaming JSON parser for large messages
- [ ] **Add message compression** - Support WebSocket compression for bandwidth optimization
- [ ] **Implement message batching** - Batch multiple updates into single handler calls
- [ ] **Add memory usage monitoring** - Track and optimize memory usage for long-running connections

### 🔧 Configuration & Flexibility

- [ ] **Add configuration struct** - Create `ClientConfig` for customizable connection settings
- [ ] **Add environment variable support** - Allow configuration via environment variables
- [ ] **Add logging support** - Integrate with `log` crate for better debugging
- [ ] **Add metrics collection** - Add optional metrics collection for monitoring

### 📱 API Improvements

- [ ] **Add subscription management** - Allow subscribing/unsubscribing from multiple channels
- [ ] **Add message filtering** - Filter messages by pair, event type, or custom criteria
- [ ] **Add callback error handling** - Allow handlers to return errors and handle them gracefully
- [ ] **Add async handlers** - Support async handlers for complex processing

---

---

**Made with 🦀 in Rust** – All contributions welcome!
