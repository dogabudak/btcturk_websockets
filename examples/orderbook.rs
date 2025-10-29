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
