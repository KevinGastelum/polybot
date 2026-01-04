//! Polymarket data types.

use serde::{Deserialize, Serialize};

/// Represents a market on Polymarket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    /// Unique market identifier
    pub condition_id: String,
    /// Human-readable question
    pub question: String,
    /// Market description
    pub description: Option<String>,
    /// End date for the market
    pub end_date_iso: Option<String>,
    /// Whether the market is active
    pub active: bool,
    /// Whether the market is closed
    pub closed: bool,
    /// Market tokens (YES/NO)
    pub tokens: Vec<Token>,
}

/// Represents a token (YES or NO outcome) in a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Token ID used for trading
    pub token_id: String,
    /// Outcome name (e.g., "Yes", "No")
    pub outcome: String,
    /// Current price (0.0 - 1.0)
    pub price: Option<f64>,
}

/// Price level in the order book (as returned by Polymarket CLOB API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    /// Price as a string (e.g., "0.55")
    pub price: String,
    /// Size as a string (e.g., "100.50")
    pub size: String,
}

/// Order book for a specific token (matching Polymarket CLOB API format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Market condition ID
    pub market: String,
    /// Asset/token ID
    pub asset_id: String,
    /// Timestamp in milliseconds
    pub timestamp: String,
    /// Hash of the orderbook
    pub hash: String,
    /// Bid orders (price, size)
    pub bids: Vec<PriceLevel>,
    /// Ask orders (price, size)
    pub asks: Vec<PriceLevel>,
}

impl OrderBook {
    /// Get the best bid price as f64.
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first()
            .and_then(|level| level.price.parse::<f64>().ok())
    }

    /// Get the best ask price as f64.
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first()
            .and_then(|level| level.price.parse::<f64>().ok())
    }
}

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

/// Order to place on Polymarket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Token ID to trade
    pub token_id: String,
    /// Side (buy/sell)
    pub side: Side,
    /// Price (0.01 - 0.99)
    pub price: f64,
    /// Size in shares
    pub size: f64,
    /// Order type
    pub order_type: OrderType,
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Good till cancelled
    Gtc,
    /// Fill or kill
    Fok,
    /// Immediate or cancel
    Ioc,
}

/// Response from placing an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Order ID
    pub order_id: Option<String>,
    /// Whether the order was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution details
    pub executions: Option<Vec<Execution>>,
}

/// Trade execution details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    /// Price executed at
    pub price: f64,
    /// Size executed
    pub size: f64,
    /// Timestamp
    pub timestamp: String,
}

/// CLOB API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}
