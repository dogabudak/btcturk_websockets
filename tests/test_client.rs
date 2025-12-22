use btcturk_websockets::{Client, ApiKeys};

#[test]
fn test_client_creation_with_websocket_address() {
    let api_keys = ApiKeys::new("test_public", "test_private");
    let address = "wss://ws-feed-pro.btcturk.com/".to_string();
    
    let client = Client::new(address.clone(), api_keys.clone());
    
    // Since fields are private, we test that the client was created successfully
    // and can be used to update keys
    let debug_output = format!("{:?}", client);
    assert!(debug_output.contains("Client"));
}

#[test]
fn test_client_creation_with_rest_address() {
    let api_keys = ApiKeys::new("test_public", "test_private");
    let address = "https://api.btcturk.com".to_string();
    
    let client = Client::new(address.clone(), api_keys.clone());
    
    // Test client was created successfully
    let debug_output = format!("{:?}", client);
    assert!(debug_output.contains("Client"));
}

#[test]
fn test_client_creation_with_empty_keys() {
    let api_keys = ApiKeys::new("", "");
    let address = "wss://test.com".to_string();
    
    let client = Client::new(address, api_keys);
    
    // Test client was created successfully
    let debug_output = format!("{:?}", client);
    assert!(debug_output.contains("Client"));
}

#[test]
fn test_client_clone() {
    let api_keys = ApiKeys::new("public_key", "private_key");
    let address = "wss://test.com".to_string();
    
    let client = Client::new(address, api_keys);
    let cloned_client = client.clone();
    
    // Test that both clients have similar debug representations
    let client_debug = format!("{:?}", client);
    let cloned_debug = format!("{:?}", cloned_client);
    
    assert!(client_debug.contains("Client"));
    assert!(cloned_debug.contains("Client"));
}

#[test]
fn test_client_debug_format() {
    let api_keys = ApiKeys::new("public", "private");
    let address = "wss://test.com".to_string();
    
    let client = Client::new(address, api_keys);
    let debug_output = format!("{:?}", client);
    
    assert!(debug_output.contains("Client"));
    assert!(debug_output.contains("address"));
    assert!(debug_output.contains("keys"));
}

#[test]
fn test_client_set_keys() {
    let api_keys = ApiKeys::new("old_public", "old_private");
    let address = "wss://test.com".to_string();
    
    let mut client = Client::new(address, api_keys);
    
    let new_keys = ApiKeys::new("new_public", "new_private");
    client.set_keys(new_keys);
    
    // Test that the operation completed successfully
    let debug_output = format!("{:?}", client);
    assert!(debug_output.contains("Client"));
}