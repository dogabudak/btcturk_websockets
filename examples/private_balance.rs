use btcturk_websockets::{ApiKeys, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_keys = ApiKeys::new("YOUR_PUBLIC_KEY", "YOUR_PRIVATE_KEY");
    let client = Client::new("wss://ws-feed-pro.btcturk.com/".to_string(), api_keys);

    let balances = client.get_account_balance().await?;
    println!("✅ Balances: {:?}", balances);

    Ok(())
}
