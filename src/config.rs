//! Configuration module for the arbitrage bot.
//!
//! Loads settings from environment variables using dotenvy.

use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

/// Bot configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    // Polymarket settings
    pub polymarket_api_key: String,
    pub polymarket_secret: String,
    pub polymarket_passphrase: String,
    pub polymarket_private_key: String,
    pub polygon_rpc_url: String,

    // Kalshi settings
    pub kalshi_email: Option<String>,
    pub kalshi_password: Option<String>,
    pub kalshi_api_key: Option<String>,
    pub kalshi_api_secret: Option<String>,

    // Bot settings
    pub min_profit_threshold: f64,
    pub max_position_size: f64,
    pub dry_run: bool,
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        // Load .env file if present
        let _ = dotenv();

        Ok(Self {
            // Polymarket
            polymarket_api_key: env::var("POLYMARKET_API_KEY")
                .unwrap_or_default(),
            polymarket_secret: env::var("POLYMARKET_SECRET")
                .unwrap_or_default(),
            polymarket_passphrase: env::var("POLYMARKET_PASSPHRASE")
                .unwrap_or_default(),
            polymarket_private_key: env::var("POLYMARKET_PRIVATE_KEY")
                .unwrap_or_default(),
            polygon_rpc_url: env::var("POLYGON_RPC_URL")
                .unwrap_or_else(|_| "https://polygon-rpc.com".to_string()),

            // Kalshi
            kalshi_email: env::var("KALSHI_EMAIL").ok(),
            kalshi_password: env::var("KALSHI_PASSWORD").ok(),
            kalshi_api_key: env::var("KALSHI_API_KEY").ok(),
            kalshi_api_secret: env::var("KALSHI_API_SECRET").ok(),

            // Bot settings
            min_profit_threshold: env::var("MIN_PROFIT_THRESHOLD")
                .unwrap_or_else(|_| "0.02".to_string())
                .parse()
                .context("Invalid MIN_PROFIT_THRESHOLD")?,
            max_position_size: env::var("MAX_POSITION_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .context("Invalid MAX_POSITION_SIZE")?,
            dry_run: env::var("DRY_RUN")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "INFO".to_string()),
        })
    }

    /// Check if Polymarket credentials are configured.
    pub fn has_polymarket_credentials(&self) -> bool {
        !self.polymarket_api_key.is_empty()
            && !self.polymarket_secret.is_empty()
            && !self.polymarket_private_key.is_empty()
    }

    /// Check if Kalshi credentials are configured.
    pub fn has_kalshi_credentials(&self) -> bool {
        (self.kalshi_email.is_some() && self.kalshi_password.is_some())
            || (self.kalshi_api_key.is_some() && self.kalshi_api_secret.is_some())
    }
}
