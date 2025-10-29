use btcturk_websockets::{Client, ApiKeys};
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
