//! Trade log for recording paper trades.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Trade direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
        }
    }
}

/// Trade status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeStatus {
    Open,
    Closed,
    Cancelled,
}

/// A paper trade record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperTrade {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub market: String,
    pub coin: String,
    pub timeframe: String,
    pub platform: String,  // "polymarket" or "kalshi"
    pub side: Side,
    pub size: f64,         // USD amount
    pub entry_price: f64,  // 0.0 - 1.0
    pub exit_price: Option<f64>,
    pub pnl: Option<f64>,
    pub status: TradeStatus,
    pub strategy: String,  // "arbitrage", "copy_trade", "manual"
    pub confidence: f64,   // 0.0 - 1.0
    pub notes: Option<String>,
}

impl PaperTrade {
    /// Create a new paper trade.
    pub fn new(
        market: &str,
        coin: &str,
        timeframe: &str,
        platform: &str,
        side: Side,
        size: f64,
        entry_price: f64,
        strategy: &str,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            market: market.to_string(),
            coin: coin.to_string(),
            timeframe: timeframe.to_string(),
            platform: platform.to_string(),
            side,
            size,
            entry_price,
            exit_price: None,
            pnl: None,
            status: TradeStatus::Open,
            strategy: strategy.to_string(),
            confidence,
            notes: None,
        }
    }

    /// Close the trade with an exit price.
    pub fn close(&mut self, exit_price: f64) {
        self.exit_price = Some(exit_price);
        self.status = TradeStatus::Closed;
        
        // Calculate P&L
        // For a YES position (buy): profit = size * (exit - entry)
        // For a NO position (sell): profit = size * (entry - exit)
        let pnl = match self.side {
            Side::Buy => self.size * (exit_price - self.entry_price),
            Side::Sell => self.size * (self.entry_price - exit_price),
        };
        self.pnl = Some(pnl);
    }

    /// Check if trade is profitable.
    pub fn is_profitable(&self) -> bool {
        self.pnl.map(|p| p > 0.0).unwrap_or(false)
    }
}

/// Trade log that persists trades to disk.
pub struct TradeLog {
    trades: Vec<PaperTrade>,
    file_path: String,
}

impl TradeLog {
    /// Create or load a trade log from file.
    pub fn new(file_path: &str) -> Self {
        let trades = if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        Self {
            trades,
            file_path: file_path.to_string(),
        }
    }

    /// Add a new trade.
    pub fn add_trade(&mut self, trade: PaperTrade) {
        self.trades.push(trade);
        self.save();
    }

    /// Get all trades.
    pub fn get_all(&self) -> &[PaperTrade] {
        &self.trades
    }

    /// Get open trades.
    pub fn get_open(&self) -> Vec<&PaperTrade> {
        self.trades.iter().filter(|t| t.status == TradeStatus::Open).collect()
    }

    /// Get closed trades.
    pub fn get_closed(&self) -> Vec<&PaperTrade> {
        self.trades.iter().filter(|t| t.status == TradeStatus::Closed).collect()
    }

    /// Get recent trades (last N).
    pub fn get_recent(&self, n: usize) -> Vec<&PaperTrade> {
        self.trades.iter().rev().take(n).collect()
    }

    /// Close a trade by ID.
    pub fn close_trade(&mut self, id: &str, exit_price: f64) -> bool {
        if let Some(trade) = self.trades.iter_mut().find(|t| t.id == id) {
            trade.close(exit_price);
            self.save();
            true
        } else {
            false
        }
    }

    /// Calculate total realized P&L.
    pub fn total_pnl(&self) -> f64 {
        self.trades.iter()
            .filter_map(|t| t.pnl)
            .sum()
    }

    /// Calculate win rate.
    pub fn win_rate(&self) -> (f64, usize, usize) {
        let closed: Vec<_> = self.get_closed();
        if closed.is_empty() {
            return (0.0, 0, 0);
        }
        let wins = closed.iter().filter(|t| t.is_profitable()).count();
        let rate = wins as f64 / closed.len() as f64;
        (rate, wins, closed.len())
    }

    /// Get best trade.
    pub fn best_trade(&self) -> Option<&PaperTrade> {
        self.trades.iter()
            .filter(|t| t.pnl.is_some())
            .max_by(|a, b| a.pnl.partial_cmp(&b.pnl).unwrap())
    }

    /// Get worst trade.
    pub fn worst_trade(&self) -> Option<&PaperTrade> {
        self.trades.iter()
            .filter(|t| t.pnl.is_some())
            .min_by(|a, b| a.pnl.partial_cmp(&b.pnl).unwrap())
    }

    /// Save trades to file.
    fn save(&self) {
        if let Ok(content) = serde_json::to_string_pretty(&self.trades) {
            let _ = fs::write(&self.file_path, content);
        }
    }
}
