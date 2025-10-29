use btcturk_websockets::{ApiKeys, Client, SubmitOrderRequest, OrderMethod, OrderType};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let public_key = env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY not set");
    let private_key = env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY not set");

    let keys = ApiKeys::new(public_key, private_key);
    let client = Client::new("wss://ws-feed-sandbox.btcturk.com/", keys);
    let balances = client.get_account_balance().await.unwrap();
    let try_balance = balances
        .data
        .iter()
        .find(|b| b.asset == "TRY")
        .map(|b| b.free.replace(",", ".").parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);

    println!("💰 TRY balance: {}", try_balance);

    // Original example code
    println!("=== Original Submit Order Example ===");
    let order_request = SubmitOrderRequest {
        quantity: "1.800".to_string(),
        price: Some("50000".to_string()),
        stop_price: None,
        order_method: OrderMethod::Limit,
        order_type: OrderType::Buy,
        pair_symbol: "USDTTRY".to_string(),
        new_order_client_id: None,
    };

    match client.submit_order(order_request).await {
        Ok(response) => {
            println!("✅ Order submitted successfully: {:?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to submit order: {}", e);
        }
    }
}
