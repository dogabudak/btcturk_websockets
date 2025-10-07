use btcturk_websockets::types::{OrderBookEvent};

#[test]
fn parses_btcturk_orderbook_payload() {
    let raw = r#"
    [431,{"CS":461280,"PS":"BTCTRY",
      "AO":[{"A":"0.01699937","P":"5171945"},{"A":"0.01366444","P":"5171946"}],
      "BO":[{"A":"0.00795068","P":"5168031"},{"A":"0.00735303","P":"5168030"}],
      "channel":"orderbook","event":"BTCTRY","type":431}]
    "#;

    let v: serde_json::Value = serde_json::from_str(raw).unwrap();
    let payload = v.get(1).expect("payload at index 1").clone();

    let ob: OrderBookEvent = serde_json::from_value(payload).unwrap();
    assert_eq!(ob.channel, "orderbook");
    assert_eq!(ob.event, "BTCTRY");
    assert_eq!(ob.pair_symbol, "BTCTRY");
    assert_eq!(ob.type_field, 431);
    assert!(ob.asks.len() >= 2);
    assert!(ob.bids.len() >= 2);
    assert_eq!(ob.asks[0].price, "5171945");
    assert_eq!(ob.bids[0].price, "5168031");
}
