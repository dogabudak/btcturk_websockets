use btcturk_websockets::Event;
use rust_decimal_macros::dec;

const TICKER_FRAME: &str = r#"[402,{"B":"34.706","A":"34.738","BA":"190.3053","AA":"1481.3956","PS":"ADATRY","H":"36.7","L":"34.555","LA":"34.76","O":"34.911","V":"473088.27239993","AV":"35.701201400000","D":"-0.173","DP":"-0.43","DS":"TRY","NS":"ADA","PId":57,"channel":"ticker","event":"ADATRY","type":402}]"#;

const ORDERBOOK_FRAME: &str = r#"[431,{"CS":461280,"PS":"BTCTRY",
  "AO":[{"A":"0.01699937","P":"5171945"},{"A":"0.01366444","P":"5171946"}],
  "BO":[{"A":"0.00795068","P":"5168031"},{"A":"0.00735303","P":"5168030"}],
  "channel":"orderbook","event":"BTCTRY","type":431}]"#;

#[test]
fn parses_ticker_frame() {
    match Event::parse(TICKER_FRAME) {
        Some(Event::Ticker(t)) => {
            assert_eq!(t.pair_symbol, "ADATRY");
            assert_eq!(t.bid, dec!(34.706));
            assert_eq!(t.pair_id, 57);
        }
        other => panic!("expected ticker event, got {:?}", other),
    }
}

#[test]
fn parses_orderbook_frame() {
    match Event::parse(ORDERBOOK_FRAME) {
        Some(Event::OrderBook(ob)) => {
            assert_eq!(ob.pair_symbol, "BTCTRY");
            assert_eq!(ob.change_seq, Some(461280));
            assert_eq!(ob.asks[0].price, dec!(5171945));
            assert_eq!(ob.bids[1].amount, dec!(0.00735303));
        }
        other => panic!("expected orderbook event, got {:?}", other),
    }
}

#[test]
fn parses_channel_name_with_escapes() {
    // Spell "ticker" with a JSON unicode escape for the `e`, so the parser can't borrow it.
    let escaped = format!(r#""channel":"tick{}u0065r""#, '\\');
    let frame = TICKER_FRAME.replace(r#""channel":"ticker""#, &escaped);
    assert!(!frame.contains(r#""channel":"ticker""#));
    assert!(matches!(Event::parse(&frame), Some(Event::Ticker(_))));
}

#[test]
fn ignores_control_and_unknown_frames() {
    assert!(Event::parse(r#"[991,{"type":991,"current":"5.1.0","min":"2.3.0"}]"#).is_none());
    assert!(Event::parse(r#"[100,{"ok":true,"message":"join|ticker:BTCTRY","type":100}]"#).is_none());
    assert!(Event::parse(r#"[421,{"channel":"trade","event":"BTCTRY","type":421}]"#).is_none());
}

#[test]
fn rejects_malformed_frames() {
    assert!(Event::parse("").is_none());
    assert!(Event::parse("not json").is_none());
    assert!(Event::parse("[402]").is_none());
    assert!(Event::parse(r#"{"channel":"ticker"}"#).is_none());
    assert!(Event::parse(&TICKER_FRAME[..TICKER_FRAME.len() - 2]).is_none());
}

#[test]
fn rejects_known_channel_with_invalid_payload() {
    assert!(Event::parse(r#"[402,{"channel":"ticker","event":"BTCTRY","type":402}]"#).is_none());
    let bad_price = ORDERBOOK_FRAME.replace(r#""P":"5171945""#, r#""P":"oops""#);
    assert!(Event::parse(&bad_price).is_none());
}
