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
