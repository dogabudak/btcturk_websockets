use btcturk_websockets::{ApiKeys, Client, Error};
use reqwest::StatusCode;

fn client(address: &str, private_key: &str) -> Client {
    Client::new(address, ApiKeys::new("public", private_key))
}

#[test]
fn token_message_rejects_invalid_secret() {
    let mut client = client("wss://ws-feed-pro.btcturk.com/", "not base64!!");
    assert!(matches!(client.generate_token_message(), Err(Error::InvalidSecret(_))));
}

#[test]
fn token_message_accepts_standard_and_url_safe_secrets() {
    assert!(client("wss://x", "c2VjcmV0K/8=").generate_token_message().is_ok());
    assert!(client("wss://x", "c2VjcmV0K_8").generate_token_message().is_ok());
}

#[tokio::test]
async fn private_endpoint_fails_on_invalid_secret_before_any_request() {
    let client = client("https://api.btcturk.com", "not base64!!");
    assert!(matches!(client.get_account_balance().await, Err(Error::InvalidSecret(_))));
}

#[tokio::test]
async fn create_connection_rejects_invalid_url() {
    let client = client("not a url", "c2VjcmV0");
    assert!(matches!(client.create_connection().await, Err(Error::InvalidUrl(_))));
}

#[tokio::test]
async fn create_connection_reports_connect_failure() {
    // Port 1 on localhost is closed, so the connection is refused immediately.
    let client = client("ws://127.0.0.1:1/", "c2VjcmV0");
    assert!(matches!(client.create_connection().await, Err(Error::WebSocket(_))));
}

#[tokio::test]
async fn subscribe_returns_error_instead_of_panicking() {
    let mut client = client("not a url", "c2VjcmV0");
    let result = client.subscribe_ticker("BTCTRY", |_| {}).await;
    assert!(matches!(result, Err(Error::InvalidUrl(_))));
}

#[test]
fn api_error_display_includes_status_and_body() {
    let err = Error::Api {
        status: StatusCode::UNAUTHORIZED,
        body: r#"{"success":false,"message":"Unauthorized","code":401}"#.to_string(),
    };
    let message = err.to_string();
    assert!(message.contains("401 Unauthorized"));
    assert!(message.contains(r#""message":"Unauthorized""#));
}

#[test]
fn decode_error_keeps_body_and_source() {
    let source = serde_json::from_str::<u64>("\"nope\"").unwrap_err();
    let err = Error::Decode { source, body: "\"nope\"".to_string() };
    assert!(err.to_string().starts_with("failed to decode response:"));
    assert!(std::error::Error::source(&err).is_some());
    if let Error::Decode { body, .. } = err {
        assert_eq!(body, "\"nope\"");
    }
}

#[test]
fn error_is_thread_safe_and_boxable() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<Error>();

    let boxed: Box<dyn std::error::Error + Send + Sync> = Error::InvalidSecret(base64_err()).into();
    assert!(boxed.to_string().contains("not valid Base64"));
}

fn base64_err() -> base64::DecodeError {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.decode("!").unwrap_err()
}
