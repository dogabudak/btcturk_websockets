#[derive(Debug, Clone)]
pub enum Channel {
    Ticker,
    Orderbook
}

impl Channel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Ticker => "ticker",
            Channel::Orderbook => "orderbook"
        }
    }
}
