pub mod client;
pub mod error;
pub use error::{Error, Result};
pub mod types;
pub use client::Client;
pub use types::{
    Event,
    TickerEvent,
    OrderBookEvent,
    SubmitOrderRequest,
    SubmitOrderResponse,
    OrderMethod,
    OrderType,
    TickerRestResponse,
    TickerRestData,
};
pub mod channel;
pub use channel::Channel;
pub mod api_keys;
pub use api_keys::ApiKeys;
pub use rust_decimal;
pub use rust_decimal::Decimal;
