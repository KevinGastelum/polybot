//! Position tracker module.
//!
//! Keeps track of current holdings and P&L.

use std::collections::HashMap;
use std::sync::Mutex;

/// Current state of a position.
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub ticker: String,
    pub quantity: i32,
    pub avg_price: f64,
}

/// Tracks positions across platforms.
pub struct PositionTracker {
    /// Platform Name -> Ticker -> Position
    positions: Mutex<HashMap<String, HashMap<String, Position>>>,
}

impl PositionTracker {
    /// Create a new position tracker.
    pub fn new() -> Self {
        Self {
            positions: Mutex::new(HashMap::new()),
        }
    }

    /// Update position for a specific platform.
    pub fn update_position(&self, platform: &str, ticker: &str, quantity: i32, price: f64) {
        let mut all_positions = self.positions.lock().unwrap();
        let platform_map = all_positions.entry(platform.to_string()).or_default();
        
        let pos = platform_map.entry(ticker.to_string()).or_insert(Position {
            ticker: ticker.to_string(),
            ..Default::default()
        });

        // Simple weighted average for new buys
        if quantity > 0 {
            let total_qty = pos.quantity + quantity;
            if total_qty > 0 {
                pos.avg_price = (pos.avg_price * pos.quantity as f64 + price * quantity as f64) / total_qty as f64;
            }
            pos.quantity = total_qty;
        } else {
            pos.quantity += quantity;
        }
    }

    /// Get current position.
    pub fn get_position(&self, platform: &str, ticker: &str) -> Option<Position> {
        let all_positions = self.positions.lock().unwrap();
        all_positions.get(platform)?.get(ticker).cloned()
    }
}
