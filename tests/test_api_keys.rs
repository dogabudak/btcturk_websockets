use btcturk_websockets::ApiKeys;

#[test]
fn test_api_keys_creation_with_strings() {
    let public_key = "test_public_key".to_string();
    let private_key = "test_private_key".to_string();
    
    let api_keys = ApiKeys::new(public_key.clone(), private_key.clone());
    
    assert_eq!(api_keys.public_key, public_key);
    assert_eq!(api_keys.private_key, private_key);
}

#[test]
fn test_api_keys_creation_with_str() {
    let public_key = "test_public_key";
    let private_key = "test_private_key";
    
    let api_keys = ApiKeys::new(public_key, private_key);
    
    assert_eq!(api_keys.public_key, public_key);
    assert_eq!(api_keys.private_key, private_key);
}

#[test]
fn test_api_keys_creation_with_mixed_types() {
    let public_key = "test_public_key".to_string();
    let private_key = "test_private_key";
    
    let api_keys = ApiKeys::new(public_key.clone(), private_key);
    
    assert_eq!(api_keys.public_key, public_key);
    assert_eq!(api_keys.private_key, private_key);
}

#[test]
fn test_api_keys_empty_strings() {
    let api_keys = ApiKeys::new("", "");
    
    assert_eq!(api_keys.public_key, "");
    assert_eq!(api_keys.private_key, "");
}

#[test]
fn test_api_keys_clone() {
    let api_keys = ApiKeys::new("public", "private");
    let cloned_keys = api_keys.clone();
    
    assert_eq!(api_keys.public_key, cloned_keys.public_key);
    assert_eq!(api_keys.private_key, cloned_keys.private_key);
}

#[test]
fn test_api_keys_debug_format() {
    let api_keys = ApiKeys::new("public", "private");
    let debug_output = format!("{:?}", api_keys);
    
    assert!(debug_output.contains("ApiKeys"));
    assert!(debug_output.contains("public_key"));
    assert!(debug_output.contains("private_key"));
}