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
    #[serde(default)]
    pub markets: Vec<KalshiMarket>,
}

/// Kalshi market (a specific contract within an event).
/// Using serde(default) for optional fields to handle API response variations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KalshiMarket {
    /// Market ticker
    #[serde(default)]
    pub ticker: String,
    /// Event ticker this market belongs to
    #[serde(default)]
    pub event_ticker: Option<String>,
    /// Market type (e.g., "binary")
    #[serde(default)]
    pub market_type: Option<String>,
    /// Market title/question
    #[serde(default)]
    pub title: String,
    /// Subtitle
    #[serde(default)]
    pub subtitle: Option<String>,
    /// Yes subtitle
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    /// No subtitle
    #[serde(default)]
    pub no_sub_title: Option<String>,
    /// Status (open, closed, settled, etc.)
    #[serde(default)]
    pub status: String,
    /// Category
    #[serde(default)]
    pub category: Option<String>,
    /// Created time (ISO 8601)
    #[serde(default)]
    pub created_time: Option<String>,
    /// Open time (ISO 8601)
    #[serde(default)]
    pub open_time: Option<String>,
    /// Close time (ISO 8601)
    #[serde(default)]
    pub close_time: Option<String>,
    /// Expiration time (ISO 8601)
    #[serde(default)]
    pub expiration_time: Option<String>,
    /// Latest expiration time (ISO 8601)
    #[serde(default)]
    pub latest_expiration_time: Option<String>,
    /// Settlement time (ISO 8601)
    #[serde(default)]
    pub settlement_time: Option<String>,
    /// Last trade price (0-100 representing cents)
    #[serde(default)]
    pub last_price: Option<i32>,
    /// Best yes bid (0-100 cents)
    #[serde(default)]
    pub yes_bid: Option<i32>,
    /// Best yes ask (0-100 cents)
    #[serde(default)]
    pub yes_ask: Option<i32>,
    /// Best no bid (0-100 cents)
    #[serde(default)]
    pub no_bid: Option<i32>,
    /// Best no ask (0-100 cents)
    #[serde(default)]
    pub no_ask: Option<i32>,
    /// Volume
    #[serde(default)]
    pub volume: Option<i64>,
    /// 24h volume
    #[serde(default)]
    pub volume_24h: Option<i64>,
    /// Liquidity
    #[serde(default)]
    pub liquidity: Option<i64>,
    /// Open interest
    #[serde(default)]
    pub open_interest: Option<i64>,
    /// Result ("yes", "no", or null)
    #[serde(default)]
    pub result: Option<String>,
    /// Can close early
    #[serde(default)]
    pub can_close_early: Option<bool>,
    /// Cap strike
    #[serde(default)]
    pub cap_strike: Option<f64>,
    /// Risk limit in cents
    #[serde(default)]
    pub risk_limit_cents: Option<i64>,
    /// Tick size
    #[serde(default)]
    pub tick_size: Option<i32>,
    /// Rules primary
    #[serde(default)]
    pub rules_primary: Option<String>,
    /// Rules secondary
    #[serde(default)]
    pub rules_secondary: Option<String>,
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
