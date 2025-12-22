use btcturk_websockets::client::{Balance, BalanceResponse};
use serde_json;

#[test]
fn test_balance_deserialization() {
    let json_data = r#"{
        "asset": "BTC",
        "assetname": "Bitcoin",
        "balance": "1.5",
        "locked": "0.2",
        "free": "1.3",
        "orderFund": "0.1",
        "requestFund": "0.1",
        "precision": 8,
        "timestamp": 1634567890
    }"#;
    
    let balance: Balance = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(balance.asset, "BTC");
    assert_eq!(balance.assetname, "Bitcoin");
    assert_eq!(balance.balance, "1.5");
    assert_eq!(balance.locked, "0.2");
    assert_eq!(balance.free, "1.3");
    assert_eq!(balance.order_fund, "0.1");
    assert_eq!(balance.request_fund, "0.1");
    assert_eq!(balance.precision, 8);
    assert_eq!(balance.timestamp, 1634567890);
}

#[test]
fn test_balance_serialization() {
    let balance = Balance {
        asset: "ETH".to_string(),
        assetname: "Ethereum".to_string(),
        balance: "10.0".to_string(),
        locked: "1.0".to_string(),
        free: "9.0".to_string(),
        order_fund: "0.5".to_string(),
        request_fund: "0.5".to_string(),
        precision: 18,
        timestamp: 1634567890,
    };
    
    let serialized = serde_json::to_string(&balance).unwrap();
    let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(json["asset"], "ETH");
    assert_eq!(json["assetname"], "Ethereum");
    assert_eq!(json["balance"], "10.0");
    assert_eq!(json["locked"], "1.0");
    assert_eq!(json["free"], "9.0");
    assert_eq!(json["orderFund"], "0.5");
    assert_eq!(json["requestFund"], "0.5");
    assert_eq!(json["precision"], 18);
    assert_eq!(json["timestamp"], 1634567890);
}

#[test]
fn test_balance_response_deserialization_success() {
    let json_data = r#"{
        "data": [
            {
                "asset": "BTC",
                "assetname": "Bitcoin",
                "balance": "1.5",
                "locked": "0.2",
                "free": "1.3",
                "orderFund": "0.1",
                "requestFund": "0.1",
                "precision": 8,
                "timestamp": 1634567890
            },
            {
                "asset": "TRY",
                "assetname": "Turkish Lira",
                "balance": "50000.0",
                "locked": "5000.0",
                "free": "45000.0",
                "orderFund": "2500.0",
                "requestFund": "2500.0",
                "precision": 2,
                "timestamp": 1634567890
            }
        ],
        "success": true,
        "message": null,
        "code": 0
    }"#;
    
    let response: BalanceResponse = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.success, true);
    assert_eq!(response.message, None);
    assert_eq!(response.code, 0);
    
    let btc_balance = &response.data[0];
    assert_eq!(btc_balance.asset, "BTC");
    assert_eq!(btc_balance.balance, "1.5");
    
    let try_balance = &response.data[1];
    assert_eq!(try_balance.asset, "TRY");
    assert_eq!(try_balance.balance, "50000.0");
}

#[test]
fn test_balance_response_deserialization_error() {
    let json_data = r#"{
        "data": [],
        "success": false,
        "message": "Authentication failed",
        "code": 1001
    }"#;
    
    let response: BalanceResponse = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(response.data.len(), 0);
    assert_eq!(response.success, false);
    assert_eq!(response.message, Some("Authentication failed".to_string()));
    assert_eq!(response.code, 1001);
}

#[test]
fn test_balance_clone() {
    let balance = Balance {
        asset: "BTC".to_string(),
        assetname: "Bitcoin".to_string(),
        balance: "1.0".to_string(),
        locked: "0.1".to_string(),
        free: "0.9".to_string(),
        order_fund: "0.05".to_string(),
        request_fund: "0.05".to_string(),
        precision: 8,
        timestamp: 1634567890,
    };
    
    let cloned_balance = balance.clone();
    
    assert_eq!(balance.asset, cloned_balance.asset);
    assert_eq!(balance.balance, cloned_balance.balance);
    assert_eq!(balance.timestamp, cloned_balance.timestamp);
}

#[test]
fn test_balance_response_clone() {
    let response = BalanceResponse {
        data: vec![],
        success: true,
        message: None,
        code: 0,
    };
    
    let cloned_response = response.clone();
    
    assert_eq!(response.success, cloned_response.success);
    assert_eq!(response.code, cloned_response.code);
    assert_eq!(response.data.len(), cloned_response.data.len());
}

#[test]
fn test_balance_debug_format() {
    let balance = Balance {
        asset: "BTC".to_string(),
        assetname: "Bitcoin".to_string(),
        balance: "1.0".to_string(),
        locked: "0.1".to_string(),
        free: "0.9".to_string(),
        order_fund: "0.05".to_string(),
        request_fund: "0.05".to_string(),
        precision: 8,
        timestamp: 1634567890,
    };
    
    let debug_output = format!("{:?}", balance);
    
    assert!(debug_output.contains("Balance"));
    assert!(debug_output.contains("BTC"));
    assert!(debug_output.contains("Bitcoin"));
}