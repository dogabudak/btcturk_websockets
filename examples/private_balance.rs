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

    let balances = client.get_account_balance().await?;
    println!("✅ Balances: {:?}", balances);

    Ok(())
}
