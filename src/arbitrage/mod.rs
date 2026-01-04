//! Arbitrage engine module.
//!
//! Contains logic for detecting and executing arbitrage opportunities.

pub mod detector;
pub mod executor;
pub mod market_matcher;

pub use detector::ArbitrageDetector;
pub use executor::TradeExecutor;
pub use market_matcher::MarketMatcher;
