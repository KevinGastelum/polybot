//! Portfolio management for paper trading.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A position in a market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub market: String,
    pub coin: String,
    pub platform: String,
    pub size: f64,          // Number of shares
    pub avg_price: f64,     // Average entry price
    pub current_price: f64, // Current market price
    pub unrealized_pnl: f64,
}

impl Position {
    /// Calculate unrealized P&L.
    pub fn update_pnl(&mut self, current_price: f64) {
        self.current_price = current_price;
        self.unrealized_pnl = self.size * (current_price - self.avg_price);
    }

    /// Current value of the position.
    pub fn current_value(&self) -> f64 {
        self.size * self.current_price
    }

    /// Initial value of the position.
    pub fn initial_value(&self) -> f64 {
        self.size * self.avg_price
    }
}

/// Virtual portfolio for paper trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Starting balance (for reference)
    pub initial_balance: f64,
    /// Current cash balance (USDC)
    pub cash_balance: f64,
    /// Open positions (keyed by market ID)
    pub positions: HashMap<String, Position>,
    /// Total realized P&L
    pub realized_pnl: f64,
    /// File path for persistence
    #[serde(skip)]
    file_path: Option<String>,
}

impl Portfolio {
    /// Create a new portfolio with initial balance.
    pub fn new(initial_balance: f64) -> Self {
        Self {
            initial_balance,
            cash_balance: initial_balance,
            positions: HashMap::new(),
            realized_pnl: 0.0,
            file_path: None,
        }
    }

    /// Load portfolio from file or create new.
    pub fn load_or_create(file_path: &str, initial_balance: f64) -> Self {
        if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path).unwrap_or_default();
            if let Ok(mut portfolio) = serde_json::from_str::<Portfolio>(&content) {
                portfolio.file_path = Some(file_path.to_string());
                return portfolio;
            }
        }

        let mut portfolio = Self::new(initial_balance);
        portfolio.file_path = Some(file_path.to_string());
        portfolio.save();
        portfolio
    }

    /// Open a new position or add to existing.
    pub fn open_position(
        &mut self,
        market: &str,
        coin: &str,
        platform: &str,
        size_usd: f64,
        price: f64,
    ) -> Result<(), String> {
        // Check if we have enough cash
        if size_usd > self.cash_balance {
            return Err(format!(
                "Insufficient balance: ${:.2} available, ${:.2} needed",
                self.cash_balance, size_usd
            ));
        }

        // Deduct from cash
        self.cash_balance -= size_usd;

        // Calculate shares (size in shares = USD / price)
        let shares = size_usd / price;

        if let Some(pos) = self.positions.get_mut(market) {
            // Add to existing position
            let total_shares = pos.size + shares;
            let total_value = (pos.size * pos.avg_price) + size_usd;
            pos.avg_price = total_value / total_shares;
            pos.size = total_shares;
        } else {
            // Create new position
            self.positions.insert(
                market.to_string(),
                Position {
                    market: market.to_string(),
                    coin: coin.to_string(),
                    platform: platform.to_string(),
                    size: shares,
                    avg_price: price,
                    current_price: price,
                    unrealized_pnl: 0.0,
                },
            );
        }

        self.save();
        Ok(())
    }

    /// Close a position (or part of it).
    pub fn close_position(&mut self, market: &str, exit_price: f64) -> Result<f64, String> {
        let position = self.positions.remove(market)
            .ok_or_else(|| format!("No position found for {}", market))?;

        // Calculate P&L
        let pnl = position.size * (exit_price - position.avg_price);
        
        // Return cash + P&L
        let exit_value = position.size * exit_price;
        self.cash_balance += exit_value;
        self.realized_pnl += pnl;

        self.save();
        Ok(pnl)
    }

    /// Update all positions with current prices.
    pub fn update_prices(&mut self, prices: &HashMap<String, f64>) {
        for (market, position) in &mut self.positions {
            if let Some(&price) = prices.get(market) {
                position.update_pnl(price);
            }
        }
        self.save();
    }

    /// Total portfolio value (cash + positions).
    pub fn total_value(&self) -> f64 {
        let positions_value: f64 = self.positions.values()
            .map(|p| p.current_value())
            .sum();
        self.cash_balance + positions_value
    }

    /// Total unrealized P&L.
    pub fn unrealized_pnl(&self) -> f64 {
        self.positions.values()
            .map(|p| p.unrealized_pnl)
            .sum()
    }

    /// Total P&L (realized + unrealized).
    pub fn total_pnl(&self) -> f64 {
        self.realized_pnl + self.unrealized_pnl()
    }

    /// P&L percentage.
    pub fn pnl_percent(&self) -> f64 {
        if self.initial_balance == 0.0 {
            return 0.0;
        }
        (self.total_pnl() / self.initial_balance) * 100.0
    }

    /// Number of open positions.
    pub fn position_count(&self) -> usize {
        self.positions.len()
    }

    /// Save portfolio to file.
    fn save(&self) {
        if let Some(ref path) = self.file_path {
            if let Ok(content) = serde_json::to_string_pretty(&self) {
                let _ = fs::write(path, content);
            }
        }
    }

    /// Reset portfolio to initial state.
    pub fn reset(&mut self) {
        self.cash_balance = self.initial_balance;
        self.positions.clear();
        self.realized_pnl = 0.0;
        self.save();
    }
}
