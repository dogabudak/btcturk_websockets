# Gemini Project: BtcTurk Websockets Rust Client

This document provides a Gemini-friendly overview of the project, its structure, and common commands.

## Project Overview

This is a Rust client for the BtcTurk WebSocket API. It allows for real-time data streaming from BtcTurk, focusing on ticker and orderbook channels. It also includes functionality for private channels that require authentication.

## Project Structure

```
/
├───.gitignore
├───Cargo.lock
├───Cargo.toml      # Project manifest and dependencies
├───Readme.md
├───Tasks.md
├───Gemini.md       # This file
├───examples/       # Example usage of the client
│   ├───orderbook.rs
│   ├───private_balance.rs
│   └───ticker.rs
├───src/            # Main source code
│   ├───api_keys.rs # Handles API key authentication
│   ├───channel.rs  # Defines the websocket channels
│   ├───client.rs   # The main websocket client logic
│   ├───lib.rs      # Library entry point
│   └───types.rs    # Data structures and types
└───tests/          # Integration and parsing tests
    ├───live_orderbook.rs
    ├───live_ticker.rs
    ├───parse_orderbook.rs
    └───parse_ticker.rs
```

## Development Commands

### Building the Project

To build the project, use the standard Cargo command:
```bash
cargo build
```

### Running Tests

To run the test suite:
```bash
cargo test
```

### Running Examples

To run a specific example, use `cargo run --example <name>`.

- **Ticker Example:**
  ```bash
  cargo run --example ticker
  ```
- **Orderbook Example:**
  ```bash
  cargo run --example orderbook
  ```
- **Private Balance Example (requires API keys):**
  ```bash
  cargo run --example private_balance
  ```

## Key Dependencies

The project relies on the following major crates:

- **`tokio`**: Asynchronous runtime for network applications.
- **`tokio-tungstenite`**: WebSocket library for `tokio`.
- **`serde` & `serde_json`**: For serialization and deserialization of JSON data.
- **`futures-util`**: Utilities for working with futures.
- **`hmac`, `sha2`, `base64`**: For API key authentication signature generation.
- **`chrono`**: For handling timestamps.
- **`reqwest`**: For making HTTP requests (likely for authentication or initial data).
- **`url`**: For URL parsing and manipulation.
