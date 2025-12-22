use btcturk_websockets::Channel;

#[test]
fn test_ticker_channel_as_str() {
    let channel = Channel::Ticker;
    assert_eq!(channel.as_str(), "ticker");
}

#[test]
fn test_orderbook_channel_as_str() {
    let channel = Channel::Orderbook;
    assert_eq!(channel.as_str(), "orderbook");
}

#[test]
fn test_channel_clone() {
    let channel = Channel::Ticker;
    let cloned_channel = channel.clone();
    
    assert_eq!(channel.as_str(), cloned_channel.as_str());
}

#[test]
fn test_channel_debug_format() {
    let ticker_channel = Channel::Ticker;
    let orderbook_channel = Channel::Orderbook;
    
    let ticker_debug = format!("{:?}", ticker_channel);
    let orderbook_debug = format!("{:?}", orderbook_channel);
    
    assert_eq!(ticker_debug, "Ticker");
    assert_eq!(orderbook_debug, "Orderbook");
}

#[test]
fn test_all_channel_variants() {
    let channels = vec![Channel::Ticker, Channel::Orderbook];
    let expected_strings = vec!["ticker", "orderbook"];
    
    for (channel, expected) in channels.iter().zip(expected_strings.iter()) {
        assert_eq!(channel.as_str(), *expected);
    }
}