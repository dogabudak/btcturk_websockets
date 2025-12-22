use btcturk_websockets::types::*;
use serde_json;

#[test]
fn test_order_method_serialization() {
    assert_eq!(serde_json::to_string(&OrderMethod::Limit).unwrap(), "\"limit\"");
    assert_eq!(serde_json::to_string(&OrderMethod::Market).unwrap(), "\"market\"");
    assert_eq!(serde_json::to_string(&OrderMethod::StopLimit).unwrap(), "\"stoplimit\"");
    assert_eq!(serde_json::to_string(&OrderMethod::StopMarket).unwrap(), "\"stopmarket\"");
}

#[test]
fn test_order_type_serialization() {
    assert_eq!(serde_json::to_string(&OrderType::Buy).unwrap(), "\"buy\"");
    assert_eq!(serde_json::to_string(&OrderType::Sell).unwrap(), "\"sell\"");
}

#[test]
fn test_submit_order_request_serialization() {
    let order = SubmitOrderRequest {
        quantity: "10.5".to_string(),
        price: Some("50000.0".to_string()),
        stop_price: None,
        order_method: OrderMethod::Limit,
        order_type: OrderType::Buy,
        pair_symbol: "BTCTRY".to_string(),
        new_order_client_id: Some("client123".to_string()),
    };
    
    let serialized = serde_json::to_string(&order).unwrap();
    let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(json["quantity"], "10.5");
    assert_eq!(json["price"], "50000.0");
    assert_eq!(json["orderMethod"], "limit");
    assert_eq!(json["orderType"], "buy");
    assert_eq!(json["pairSymbol"], "BTCTRY");
    assert_eq!(json["newOrderClientId"], "client123");
    assert!(json["stopPrice"].is_null());
}

#[test]
fn test_submit_order_request_without_optional_fields() {
    let order = SubmitOrderRequest {
        quantity: "10.5".to_string(),
        price: None,
        stop_price: None,
        order_method: OrderMethod::Market,
        order_type: OrderType::Sell,
        pair_symbol: "ETHTRY".to_string(),
        new_order_client_id: None,
    };
    
    let serialized = serde_json::to_string(&order).unwrap();
    let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(json["quantity"], "10.5");
    assert_eq!(json["orderMethod"], "market");
    assert_eq!(json["orderType"], "sell");
    assert_eq!(json["pairSymbol"], "ETHTRY");
    assert!(!json.as_object().unwrap().contains_key("price"));
    assert!(!json.as_object().unwrap().contains_key("stopPrice"));
    assert!(!json.as_object().unwrap().contains_key("newOrderClientId"));
}

#[test]
fn test_submit_order_response_deserialization() {
    let json_data = r#"{
        "id": 12345,
        "datetime": 1634567890,
        "type": "buy",
        "method": "limit",
        "price": "50000.0",
        "quantity": "0.1",
        "pairSymbol": "BTCTRY",
        "newOrderClientId": "client123"
    }"#;
    
    let response: SubmitOrderResponse = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(response.id, 12345);
    assert_eq!(response.datetime, 1634567890);
    assert_eq!(response.r#type, "buy");
    assert_eq!(response.method, "limit");
    assert_eq!(response.price, "50000.0");
    assert_eq!(response.quantity, "0.1");
    assert_eq!(response.pair_symbol, "BTCTRY");
    assert_eq!(response.new_order_client_id, "client123");
}

#[test]
fn test_ticker_event_deserialization() {
    let json_data = r#"{
        "type": 402,
        "channel": "ticker",
        "event": "BTCTRY",
        "B": "1500000",
        "A": "1500100",
        "BA": "0.5",
        "AA": "1.2",
        "PS": "BTCTRY",
        "H": "1600000",
        "L": "1400000",
        "LA": "1500050",
        "O": "1450000",
        "V": "100.5",
        "AV": "1500000",
        "D": "50050",
        "DP": "3.45",
        "DS": "TRY",
        "NS": "BTC",
        "PId": 1
    }"#;
    
    let ticker: TickerEvent = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(ticker.type_field, 402);
    assert_eq!(ticker.channel, "ticker");
    assert_eq!(ticker.event, "BTCTRY");
    assert_eq!(ticker.bid, "1500000");
    assert_eq!(ticker.ask, "1500100");
    assert_eq!(ticker.pair_symbol, "BTCTRY");
    assert_eq!(ticker.high, "1600000");
    assert_eq!(ticker.low, "1400000");
    assert_eq!(ticker.last, "1500050");
    assert_eq!(ticker.pair_id, 1);
}

#[test]
fn test_order_book_order_deserialization() {
    let json_data = r#"{
        "A": "0.5",
        "P": "1500000"
    }"#;
    
    let order: OrderBookOrder = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(order.amount, "0.5");
    assert_eq!(order.price, "1500000");
}

#[test]
fn test_order_book_event_deserialization() {
    let json_data = r#"{
        "type": 431,
        "channel": "orderbook",
        "event": "BTCTRY",
        "PS": "BTCTRY",
        "CS": 12345,
        "AO": [
            {"A": "0.5", "P": "1500100"},
            {"A": "1.0", "P": "1500200"}
        ],
        "BO": [
            {"A": "0.8", "P": "1500000"},
            {"A": "1.2", "P": "1499900"}
        ]
    }"#;
    
    let orderbook: OrderBookEvent = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(orderbook.type_field, 431);
    assert_eq!(orderbook.channel, "orderbook");
    assert_eq!(orderbook.event, "BTCTRY");
    assert_eq!(orderbook.pair_symbol, "BTCTRY");
    assert_eq!(orderbook.change_seq, Some(12345));
    assert_eq!(orderbook.asks.len(), 2);
    assert_eq!(orderbook.bids.len(), 2);
    assert_eq!(orderbook.asks[0].price, "1500100");
    assert_eq!(orderbook.bids[0].price, "1500000");
}

#[test]
fn test_ticker_rest_response_deserialization() {
    let json_data = r#"{
        "data": [
            {
                "pair": "BTCTRY",
                "bid": 1500000,
                "ask": 1500100,
                "last": 1500050,
                "high": 1600000,
                "low": 1400000,
                "volume": 100.5
            }
        ]
    }"#;
    
    let response: TickerRestResponse = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(response.data.len(), 1);
    let ticker = &response.data[0];
    assert_eq!(ticker.pair_symbol, "BTCTRY");
    assert_eq!(ticker.bid, "1500000");
    assert_eq!(ticker.ask, "1500100");
    assert_eq!(ticker.last, "1500050");
}

#[test]
fn test_ticker_rest_data_with_string_values() {
    let json_data = r#"{
        "pair": "ETHTRY",
        "bid": "100000",
        "ask": "100100",
        "last": "100050",
        "high": "110000",
        "low": "95000",
        "volume": "250.75"
    }"#;
    
    let ticker: TickerRestData = serde_json::from_str(json_data).unwrap();
    
    assert_eq!(ticker.pair_symbol, "ETHTRY");
    assert_eq!(ticker.bid, "100000");
    assert_eq!(ticker.ask, "100100");
    assert_eq!(ticker.last, "100050");
    assert_eq!(ticker.high, "110000");
    assert_eq!(ticker.low, "95000");
    assert_eq!(ticker.volume, "250.75");
}