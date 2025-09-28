use btcturk_websockets::{ApiKeys, Client};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_keys = ApiKeys::new("dummy_public", "dummy_private");
    let mut client = Client::new(
        "wss://ws-feed-pro.btcturk.com/".to_string(),
        api_keys,
    );

    client
    .subscribe_with_handler("BTCUSDT", None, |msg| {
        println!("Depth: {:?}", msg);
    })
    .await?;

    Ok(())
}
