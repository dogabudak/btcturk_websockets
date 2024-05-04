use btcturk_websockets::{Client, ApiKeys};

#[tokio::test]
async fn general_test() {
    let btc_public_key = std::env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY must be set.");
    let btc_private_key = std::env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY must be set.");
    let connect_addr = std::env::var("BTCTURK_WEBSOCKET_ADDRESS").expect("BTCTURK_PRIVATE_KEY must be set.");
    let api_keys=ApiKeys::new(btc_public_key, btc_private_key);
    let client = Client::new(connect_addr, api_keys);
    let token = client.clone().generate_token_message();
    let connection = client.clone().create_connection().await;
    let ticker = client.clone().get_ticker().await;
}
