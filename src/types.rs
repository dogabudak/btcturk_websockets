use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum OrderMethod {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "stoplimit")]
    StopLimit,
    #[serde(rename = "stopmarket")]
    StopMarket,
}

#[derive(Debug, Serialize)]
pub enum OrderType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitOrderRequest {
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    pub order_method: OrderMethod,
    pub order_type: OrderType,
    pub pair_symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitOrderResponse {
    pub id: u64,
    pub timestamp: u64,
    pub r#type: String,
    pub method: String,
    pub price: String,
    pub amount: String,
    pub pair_symbol: String,
    pub new_order_client_id: String,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookOrder {
    #[serde(rename = "A")]
    pub amount: String,
    #[serde(rename = "P")]
    pub price: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookEvent {
    #[serde(rename = "type")]
    pub type_field: i32,
    pub channel: String,
    pub event: String,

    #[serde(rename = "PS")]
    pub pair_symbol: String,

    #[serde(rename = "CS")]
    pub change_seq: Option<i64>,

    #[serde(rename = "AO")]
    pub asks: Vec<OrderBookOrder>,

    #[serde(rename = "BO")]
    pub bids: Vec<OrderBookOrder>,
}

#[derive(Debug)]
pub enum Event {
    Ticker(TickerEvent),
    OrderBook(OrderBookEvent),
    Unknown(serde_json::Value),
}

#[derive(Debug, Deserialize)]
pub struct TickerRestResponse {
    pub data: Vec<TickerRestData>,
}

#[derive(Debug, Deserialize)]
pub struct TickerRestData {
    #[serde(rename = "pair")]
    pub pair_symbol: String,
    #[serde(rename = "bid")]
    pub bid: String,
    #[serde(rename = "ask")]
    pub ask: String,
    #[serde(rename = "last")]
    pub last: String,
    #[serde(rename = "high")]
    pub high: String,
    #[serde(rename = "low")]
    pub low: String,
    #[serde(rename = "volume")]
    pub volume: String,
}
