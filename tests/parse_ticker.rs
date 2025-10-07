use btcturk_websockets::types::TickerEvent;

#[test]
fn parses_btcturk_ticker_payload() {
    let raw = r#"
    [402,{
      "B":"34.706","A":"34.738","BA":"190.3053","AA":"1481.3956",
      "PS":"ADATRY","H":"36.7","L":"34.555","LA":"34.76","O":"34.911",
      "V":"473088.27239993","AV":"35.701201400000","D":"-0.173","DP":"-0.43",
      "DS":"TRY","NS":"ADA","PId":57,"channel":"ticker","event":"ADATRY","type":402
    }]
    "#;

    let v: serde_json::Value = serde_json::from_str(raw).unwrap();
    let payload = v.get(1).expect("payload at index 1").clone();

    let t: TickerEvent = serde_json::from_value(payload).unwrap();
    assert_eq!(t.channel, "ticker");
    assert_eq!(t.event, "ADATRY");
    assert_eq!(t.type_field, 402);
    assert_eq!(t.bid, "34.706");
    assert_eq!(t.ask, "34.738");
    assert_eq!(t.pair_symbol, "ADATRY");
}
