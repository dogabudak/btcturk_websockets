use btcturk_websockets::{ApiKeys, Client};
use tokio::time::{timeout, Duration};
use tokio::sync::mpsc;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore] // run manually with: cargo test -- --ignored --test live_ticker
async fn live_ticker_emits_within_15s() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut client = Client::new(
            "wss://ws-feed-pro.btcturk.com/".to_string(),
            ApiKeys::new("dummy_public", "dummy_private"),
        );

        let _ = client.subscribe_ticker("BTCTRY", move |t| {
            let _ = tx.send(t);
        }).await;
    });

    let first = timeout(Duration::from_secs(15), rx.recv()).await?;
    assert!(first.is_some(), "no ticker within 15s");
    Ok(())
}
