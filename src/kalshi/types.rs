//! Kalshi data types.

use serde::{Deserialize, Serialize};

/// Kalshi market event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiEvent {
    /// Event ticker (e.g., "INXW-24JAN10-B5175")
    pub ticker: String,
    /// Event title
    pub title: String,
    /// Category
    pub category: Option<String>,
    /// Markets under this event
    pub markets: Vec<KalshiMarket>,
}

/// Kalshi market (a specific contract within an event).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiMarket {
    /// Market ticker
    pub ticker: String,
    /// Market title/question
    pub title: String,
    /// Status (active, settled, etc.)
    pub status: String,
    /// Expiration time
    pub expiration_time: Option<String>,
    /// Last trade price (0-100 representing cents)
    pub last_price: Option<i32>,
    /// Best yes bid
    pub yes_bid: Option<i32>,
    /// Best yes ask
    pub yes_ask: Option<i32>,
    /// Best no bid
    pub no_bid: Option<i32>,
    /// Best no ask
    pub no_ask: Option<i32>,
    /// Volume
    pub volume: Option<i64>,
    /// Open interest
    pub open_interest: Option<i64>,
}

/// Kalshi order book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiOrderBook {
    /// Market ticker
    pub ticker: String,
    /// Yes bids (price in cents, quantity)
    pub yes_bids: Vec<(i32, i32)>,
    /// Yes asks (price in cents, quantity)
    pub yes_asks: Vec<(i32, i32)>,
    /// No bids (price in cents, quantity)  
    pub no_bids: Vec<(i32, i32)>,
    /// No asks (price in cents, quantity)
    pub no_asks: Vec<(i32, i32)>,
}

/// Kalshi order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiOrderRequest {
    /// Market ticker
    pub ticker: String,
    /// "yes" or "no"
    pub side: String,
    /// "buy" or "sell"
    pub action: String,
    /// Number of contracts
    pub count: i32,
    /// Limit price in cents (1-99)
    pub yes_price: Option<i32>,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
}

/// Kalshi order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiOrderResponse {
    /// Order ID
    pub order_id: Option<String>,
    /// Order status
    pub status: Option<String>,
    /// Error message if any
    pub error: Option<KalshiError>,
}

/// Kalshi error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiError {
    pub code: String,
    pub message: String,
}

/// Kalshi authentication response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiAuthResponse {
    /// JWT token for authenticated requests
    pub token: Option<String>,
    /// Member ID
    pub member_id: Option<String>,
    /// Error if login failed
    pub error: Option<KalshiError>,
}

/// Kalshi position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiPosition {
    /// Market ticker
    pub ticker: String,
    /// Position count (positive = yes, negative = no)
    pub position: i32,
    /// Average entry price
    pub average_price: Option<f64>,
    /// Realized P&L
    pub realized_pnl: Option<f64>,
}

/// Kalshi balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalshiBalance {
    /// Available balance in cents
    pub balance: i64,
    /// Pending balance
    pub pending: Option<i64>,
}
