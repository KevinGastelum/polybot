//! Paper trading engine - coordinates trading simulation.

use super::{Portfolio, TradeLog, PaperTrade, Side};
use anyhow::Result;

/// Default data directory
const DATA_DIR: &str = "data";
const DEFAULT_BALANCE: f64 = 1000.0;

/// Paper trading engine that coordinates the simulation.
pub struct PaperTradingEngine {
    pub portfolio: Portfolio,
    pub trade_log: TradeLog,
}

impl PaperTradingEngine {
    /// Create a new paper trading engine.
    pub fn new() -> Self {
        // Ensure data directory exists
        let _ = std::fs::create_dir_all(DATA_DIR);

        let portfolio = Portfolio::load_or_create(
            &format!("{}/portfolio.json", DATA_DIR),
            DEFAULT_BALANCE,
        );
        let trade_log = TradeLog::new(&format!("{}/paper_trades.json", DATA_DIR));

        Self {
            portfolio,
            trade_log,
        }
    }

    /// Create with custom initial balance.
    pub fn with_balance(initial_balance: f64) -> Self {
        let _ = std::fs::create_dir_all(DATA_DIR);

        let portfolio = Portfolio::load_or_create(
            &format!("{}/portfolio.json", DATA_DIR),
            initial_balance,
        );
        let trade_log = TradeLog::new(&format!("{}/paper_trades.json", DATA_DIR));

        Self {
            portfolio,
            trade_log,
        }
    }

    /// Execute a paper trade (buy).
    pub fn buy(
        &mut self,
        market: &str,
        coin: &str,
        timeframe: &str,
        platform: &str,
        size_usd: f64,
        price: f64,
        strategy: &str,
        confidence: f64,
    ) -> Result<String> {
        // Open position in portfolio
        self.portfolio.open_position(market, coin, platform, size_usd, price)
            .map_err(|e| anyhow::anyhow!(e))?;

        // Log the trade
        let trade = PaperTrade::new(
            market,
            coin,
            timeframe,
            platform,
            Side::Buy,
            size_usd,
            price,
            strategy,
            confidence,
        );
        let trade_id = trade.id.clone();
        self.trade_log.add_trade(trade);

        Ok(trade_id)
    }

    /// Close a position (sell).
    pub fn sell(&mut self, market: &str, exit_price: f64) -> Result<f64> {
        // Close position in portfolio
        let pnl = self.portfolio.close_position(market, exit_price)
            .map_err(|e| anyhow::anyhow!(e))?;

        // Find and close the trade in log
        // Clone the ID first to avoid borrow issues
        let trade_id: Option<String> = self.trade_log.get_open()
            .iter()
            .find(|t| t.market == market)
            .map(|t| t.id.clone());
        
        if let Some(id) = trade_id {
            self.trade_log.close_trade(&id, exit_price);
        }

        Ok(pnl)
    }

    /// Get current portfolio summary.
    pub fn summary(&self) -> PortfolioSummary {
        let (win_rate, wins, total) = self.trade_log.win_rate();
        
        PortfolioSummary {
            total_value: self.portfolio.total_value(),
            cash_balance: self.portfolio.cash_balance,
            positions_count: self.portfolio.position_count(),
            realized_pnl: self.portfolio.realized_pnl,
            unrealized_pnl: self.portfolio.unrealized_pnl(),
            total_pnl: self.portfolio.total_pnl(),
            pnl_percent: self.portfolio.pnl_percent(),
            win_rate,
            wins,
            total_trades: total,
            best_trade_pnl: self.trade_log.best_trade().and_then(|t| t.pnl),
            worst_trade_pnl: self.trade_log.worst_trade().and_then(|t| t.pnl),
        }
    }

    /// Reset the engine (clear all trades and positions).
    pub fn reset(&mut self) {
        self.portfolio.reset();
        // Note: Trade log is not cleared, for historical reference
    }
}

/// Summary of portfolio performance.
#[derive(Debug, Clone)]
pub struct PortfolioSummary {
    pub total_value: f64,
    pub cash_balance: f64,
    pub positions_count: usize,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_pnl: f64,
    pub pnl_percent: f64,
    pub win_rate: f64,
    pub wins: usize,
    pub total_trades: usize,
    pub best_trade_pnl: Option<f64>,
    pub worst_trade_pnl: Option<f64>,
}

impl Default for PaperTradingEngine {
    fn default() -> Self {
        Self::new()
    }
}
