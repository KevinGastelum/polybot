//! Paper trading module for simulating trades without real money.

pub mod engine;
pub mod portfolio;
pub mod trade_log;

pub use engine::PaperTradingEngine;
pub use portfolio::{Portfolio, Position};
pub use trade_log::{PaperTrade, TradeLog, TradeStatus, Side};
