use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TickerEvent {
    #[serde(rename = "type")]
    pub type_field: i32,
    pub channel: String,
    pub event: String,

    #[serde(rename = "B")]
    pub bid: String,
    #[serde(rename = "A")]
    pub ask: String,
    #[serde(rename = "BA")]
    pub bid_amount: String,
    #[serde(rename = "AA")]
    pub ask_amount: String,
    #[serde(rename = "PS")]
    pub pair_symbol: String,
    #[serde(rename = "H")]
    pub high: String,
    #[serde(rename = "L")]
    pub low: String,
    #[serde(rename = "LA")]
    pub last: String,
    #[serde(rename = "O")]
    pub open: String,
    #[serde(rename = "V")]
    pub volume: String,
    #[serde(rename = "AV")]
    pub average: String,
    #[serde(rename = "D")]
    pub change_amount: String,
    #[serde(rename = "DP")]
    pub change_percent: String,
    #[serde(rename = "DS")]
    pub denominator_symbol: String,
    #[serde(rename = "NS")]
    pub numerator_symbol: String,
    #[serde(rename = "PId")]
    pub pair_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct DepthEvent {
    #[serde(rename = "type")]
    pub type_field: i32,
    pub channel: String,
    pub event: String,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

#[derive(Debug)]
pub enum Event {
    Ticker(TickerEvent),
    Depth(DepthEvent),
    Unknown(serde_json::Value),
}
