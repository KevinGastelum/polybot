//! Copy trading strategy module.
//!
//! Monitors top traders on Polymarket and mirrors their positions.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use std::collections::HashSet;

/// Configuration for copy trading
#[derive(Debug, Clone)]
pub struct CopyTraderConfig {
    /// Addresses of traders to copy
    pub target_traders: Vec<String>,
    /// Maximum position size in USD
    pub max_position_size: f64,
    /// Minimum trade size to copy (filter dust trades)
    pub min_trade_size: f64,
    /// Our wallet/proxy address
    pub our_address: String,
    /// Whether to actually execute trades
    pub dry_run: bool,
}

/// A trade activity from a trader
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeActivity {
    #[serde(default)]
    pub proxy_wallet: String,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub condition_id: String,
    #[serde(default, rename = "type")]
    pub activity_type: String,
    #[serde(default)]
    pub size: f64,
    #[serde(default)]
    pub usdc_size: f64,
    #[serde(default)]
    pub transaction_hash: String,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub asset: String,
    #[serde(default)]
    pub side: String,  // "BUY" or "SELL"
    #[serde(default)]
    pub outcome_index: i32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub event_slug: String,
    #[serde(default)]
    pub outcome: String,
}

/// A position held by a trader
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraderPosition {
    #[serde(default)]
    pub proxy_wallet: String,
    #[serde(default)]
    pub asset: String,
    #[serde(default)]
    pub condition_id: String,
    #[serde(default)]
    pub size: f64,
    #[serde(default)]
    pub avg_price: f64,
    #[serde(default)]
    pub initial_value: f64,
    #[serde(default)]
    pub current_value: f64,
    #[serde(default)]
    pub cash_pnl: f64,
    #[serde(default)]
    pub percent_pnl: f64,
    #[serde(default)]
    pub cur_price: f64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub event_slug: String,
    #[serde(default)]
    pub outcome: String,
    #[serde(default)]
    pub outcome_index: i32,
}

/// Represents a trade we want to copy
#[derive(Debug, Clone)]
pub struct CopyTrade {
    pub trader_address: String,
    pub condition_id: String,
    pub asset: String,
    pub side: String,
    pub original_size: f64,
    pub our_size: f64,  // Scaled based on our balance
    pub price: f64,
    pub title: String,
    pub event_slug: String,
}

/// Copy trader that monitors and copies trades
pub struct CopyTrader {
    http: Client,
    config: CopyTraderConfig,
    /// Track trades we've already processed to avoid duplicates
    processed_trades: HashSet<String>,
}

impl CopyTrader {
    /// Create a new copy trader.
    pub fn new(config: CopyTraderConfig) -> Self {
        Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            config,
            processed_trades: HashSet::new(),
        }
    }

    /// Fetch recent trades for a trader.
    pub async fn get_trader_activity(&self, address: &str, limit: usize) -> Result<Vec<TradeActivity>> {
        let url = format!(
            "https://data-api.polymarket.com/activity?user={}&type=TRADE&limit={}",
            address, limit
        );

        debug!("Fetching activity for {}", address);

        let response = self.http.get(&url).send().await
            .context("Failed to fetch trader activity")?;

        if !response.status().is_success() {
            warn!("Failed to fetch activity for {}: {}", address, response.status());
            return Ok(vec![]);
        }

        let activities: Vec<TradeActivity> = response.json().await
            .unwrap_or_else(|e| {
                warn!("Failed to parse activity: {}", e);
                vec![]
            });

        Ok(activities)
    }

    /// Fetch positions for a trader.
    pub async fn get_trader_positions(&self, address: &str) -> Result<Vec<TraderPosition>> {
        let url = format!(
            "https://data-api.polymarket.com/positions?user={}",
            address
        );

        debug!("Fetching positions for {}", address);

        let response = self.http.get(&url).send().await
            .context("Failed to fetch trader positions")?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let positions: Vec<TraderPosition> = response.json().await
            .unwrap_or_else(|_| vec![]);

        Ok(positions)
    }

    /// Calculate trader's total portfolio value.
    pub async fn get_trader_portfolio_value(&self, address: &str) -> Result<f64> {
        let positions = self.get_trader_positions(address).await?;
        let total: f64 = positions.iter().map(|p| p.current_value).sum();
        Ok(total)
    }

    /// Get our portfolio value.
    pub async fn get_our_portfolio_value(&self) -> Result<f64> {
        self.get_trader_portfolio_value(&self.config.our_address).await
    }

    /// Scan for new trades from target traders.
    pub async fn scan_for_new_trades(&mut self) -> Result<Vec<CopyTrade>> {
        let mut trades_to_copy = Vec::new();
        let our_value = self.get_our_portfolio_value().await.unwrap_or(1000.0);

        for trader_address in &self.config.target_traders.clone() {
            // Get recent activity
            let activities = self.get_trader_activity(trader_address, 25).await?;
            
            // Get trader's portfolio value for sizing
            let trader_value = self.get_trader_portfolio_value(trader_address).await
                .unwrap_or(100000.0);  // Default to 100k if unknown

            // Calculate size ratio
            let size_ratio = our_value / trader_value;
            debug!("Size ratio for {}: {} (our: ${}, trader: ${})", 
                   trader_address, size_ratio, our_value, trader_value);

            for activity in activities {
                // Skip if already processed
                if self.processed_trades.contains(&activity.transaction_hash) {
                    continue;
                }

                // Skip if too small
                if activity.usdc_size < self.config.min_trade_size {
                    continue;
                }

                // Skip if too old (more than 1 hour)
                let now = chrono::Utc::now().timestamp_millis();
                let age_hours = (now - activity.timestamp) as f64 / (1000.0 * 60.0 * 60.0);
                if age_hours > 1.0 {
                    continue;
                }

                // Calculate our position size
                let mut our_size = activity.usdc_size * size_ratio;
                
                // Apply max position limit
                if our_size > self.config.max_position_size {
                    our_size = self.config.max_position_size;
                }

                // Mark as processed
                self.processed_trades.insert(activity.transaction_hash.clone());

                info!(
                    "📋 New trade to copy from {}: {} {} @ ${:.4} (${:.2} -> ${:.2})",
                    &trader_address[..8],
                    activity.side,
                    activity.outcome,
                    activity.price,
                    activity.usdc_size,
                    our_size
                );

                trades_to_copy.push(CopyTrade {
                    trader_address: trader_address.clone(),
                    condition_id: activity.condition_id,
                    asset: activity.asset,
                    side: activity.side,
                    original_size: activity.usdc_size,
                    our_size,
                    price: activity.price,
                    title: activity.title,
                    event_slug: activity.event_slug,
                });
            }
        }

        Ok(trades_to_copy)
    }

    /// Get summary of traders being monitored.
    pub async fn get_trader_summaries(&self) -> Vec<(String, f64, usize)> {
        let mut summaries = Vec::new();

        for address in &self.config.target_traders {
            let positions = self.get_trader_positions(address).await.unwrap_or_default();
            let total_value: f64 = positions.iter().map(|p| p.current_value).sum();
            summaries.push((address.clone(), total_value, positions.len()));
        }

        summaries
    }
}

impl Default for CopyTraderConfig {
    fn default() -> Self {
        Self {
            target_traders: vec![
                // Top performers from leaderboard
                "0x16b29c50f2439faf627209b2ac0c7bbddaa8a881".to_string(), // SeriouslySirius
                "0xdb27bf2ac5d428a9c63dbc914611036855a6c56e".to_string(), // DrPufferfish
            ],
            max_position_size: 50.0,  // Start small
            min_trade_size: 5.0,      // Filter dust trades
            our_address: String::new(),
            dry_run: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_trader_activity() {
        let config = CopyTraderConfig {
            target_traders: vec!["0x16b29c50f2439faf627209b2ac0c7bbddaa8a881".to_string()],
            ..Default::default()
        };
        let trader = CopyTrader::new(config);
        let result = trader.get_trader_activity(
            "0x16b29c50f2439faf627209b2ac0c7bbddaa8a881",
            5
        ).await;
        assert!(result.is_ok());
    }
}
