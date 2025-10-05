#[derive(Debug, Clone)]
pub enum Channel {
    Ticker,
    Depth
}

impl Channel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Ticker => "ticker",
            Channel::Depth => "depth"
        }
    }
}
