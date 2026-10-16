use rust_decimal::Decimal;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::borrow::Cow;

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
    pub quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,
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
    #[serde(alias = "timestamp")]
    pub datetime: u64,
    pub r#type: String,
    pub method: String,
    pub price: Decimal,
    #[serde(alias = "amount")]
    pub quantity: Decimal,
    pub pair_symbol: String,
    #[serde(rename = "newOrderClientId")]
    pub new_order_client_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerEvent {
    #[serde(rename = "type")]
    pub type_field: i32,
    pub channel: String,
    pub event: String,

    #[serde(rename = "B")]
    pub bid: Decimal,
    #[serde(rename = "A")]
    pub ask: Decimal,
    #[serde(rename = "BA")]
    pub bid_amount: Decimal,
    #[serde(rename = "AA")]
    pub ask_amount: Decimal,
    #[serde(rename = "PS")]
    pub pair_symbol: String,
    #[serde(rename = "H")]
    pub high: Decimal,
    #[serde(rename = "L")]
    pub low: Decimal,
    #[serde(rename = "LA")]
    pub last: Decimal,
    #[serde(rename = "O")]
    pub open: Decimal,
    #[serde(rename = "V")]
    pub volume: Decimal,
    #[serde(rename = "AV")]
    pub average: Decimal,
    #[serde(rename = "D")]
    pub change_amount: Decimal,
    #[serde(rename = "DP")]
    pub change_percent: Decimal,
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
    pub amount: Decimal,
    #[serde(rename = "P")]
    pub price: Decimal,
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

impl Event {
    /// Parses a raw BtcTurk WebSocket frame (`[<type>, {...}]`) into an `Event`.
    ///
    /// Returns `None` for frames that aren't ticker or orderbook payloads, or that fail
    /// to deserialize. The payload is deserialized straight from the frame text, without
    /// building or cloning an intermediate `serde_json::Value` tree.
    pub fn parse(text: &str) -> Option<Event> {
        #[derive(Deserialize)]
        struct ChannelPeek<'a> {
            #[serde(borrow)]
            channel: Option<Cow<'a, str>>,
        }

        let (_, payload): (IgnoredAny, &RawValue) = serde_json::from_str(text).ok()?;
        let peek: ChannelPeek = serde_json::from_str(payload.get()).ok()?;

        match peek.channel.as_deref()? {
            "ticker" => serde_json::from_str(payload.get()).ok().map(Event::Ticker),
            "orderbook" => serde_json::from_str(payload.get()).ok().map(Event::OrderBook),
            _ => None,
        }
    }
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
    pub bid: Decimal,
    #[serde(rename = "ask")]
    pub ask: Decimal,
    #[serde(rename = "last")]
    pub last: Decimal,
    #[serde(rename = "high")]
    pub high: Decimal,
    #[serde(rename = "low")]
    pub low: Decimal,
    #[serde(rename = "volume")]
    pub volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse {
    pub data: ExchangeInfoData,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfoData {
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    pub id: i32,
    pub name: String,
    #[serde(rename = "nameNormalized")]
    pub name_normalized: String,
    #[serde(rename = "numeratorScale")]
    pub numerator_scale: i32,
    #[serde(rename = "denominatorScale")]
    pub denominator_scale: i32,
    #[serde(rename = "maximumLimitOrderPrice")]
    pub maximum_limit_order_price: Option<Decimal>,
    #[serde(rename = "minimumLimitOrderPrice")]
    pub minimum_limit_order_price: Option<Decimal>,
}
