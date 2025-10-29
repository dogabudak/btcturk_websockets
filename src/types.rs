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
    #[serde(alias = "timestamp")]
    pub datetime: u64,
    pub r#type: String,
    pub method: String,
    pub price: String,
    #[serde(alias = "amount")]
    pub quantity: String,
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

// Helper function to deserialize numbers or strings as strings
fn deserialize_number_or_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct NumberOrStringVisitor;

    impl<'de> Visitor<'de> for NumberOrStringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number or a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(NumberOrStringVisitor)
}

#[derive(Debug, Deserialize)]
pub struct TickerRestData {
    #[serde(rename = "pair")]
    pub pair_symbol: String,
    #[serde(rename = "bid", deserialize_with = "deserialize_number_or_string")]
    pub bid: String,
    #[serde(rename = "ask", deserialize_with = "deserialize_number_or_string")]
    pub ask: String,
    #[serde(rename = "last", deserialize_with = "deserialize_number_or_string")]
    pub last: String,
    #[serde(rename = "high", deserialize_with = "deserialize_number_or_string")]
    pub high: String,
    #[serde(rename = "low", deserialize_with = "deserialize_number_or_string")]
    pub low: String,
    #[serde(rename = "volume", deserialize_with = "deserialize_number_or_string")]
    pub volume: String,
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
    #[serde(rename = "maximumLimitOrderPrice", deserialize_with = "deserialize_option_number_or_string")]
    pub maximum_limit_order_price: Option<String>,
    #[serde(rename = "minimumLimitOrderPrice", deserialize_with = "deserialize_option_number_or_string")]
    pub minimum_limit_order_price: Option<String>,
}

// Helper function to deserialize optional numbers or strings as strings
fn deserialize_option_number_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct OptionNumberOrStringVisitor;

    impl<'de> Visitor<'de> for OptionNumberOrStringVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional number or a string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserialize_number_or_string(deserializer).map(Some)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }
    }

    deserializer.deserialize_option(OptionNumberOrStringVisitor)
}
