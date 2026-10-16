use reqwest::StatusCode;
use tokio_tungstenite::tungstenite;

/// Errors returned by [`Client`](crate::Client).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API private key is not valid Base64.
    #[error("API private key is not valid Base64: {0}")]
    InvalidSecret(#[source] base64::DecodeError),

    /// An API key or signature can't be sent as an HTTP header value.
    #[error("API key contains characters that are not allowed in HTTP headers")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// A URL (such as the WebSocket address given to [`Client::new`](crate::Client::new)) could not be parsed.
    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// The HTTP request failed before a response was received, or the response body could not be read.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The BtcTurk REST API answered with a non-success status code.
    #[error("BtcTurk API returned {status}: {body}")]
    Api {
        status: StatusCode,
        /// Raw response body, which usually holds BtcTurk's error code and message.
        body: String,
    },

    /// A response body did not match the expected JSON structure.
    #[error("failed to decode response: {source}")]
    Decode {
        #[source]
        source: serde_json::Error,
        /// Raw response body that failed to decode.
        body: String,
    },

    /// Connecting to, sending on, or reading from the WebSocket failed.
    #[error("WebSocket error: {0}")]
    WebSocket(#[source] Box<tungstenite::Error>),
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::WebSocket(Box::new(err))
    }
}

/// Shorthand for `Result<T, btcturk_websockets::Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
