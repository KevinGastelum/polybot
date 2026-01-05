//! Polymarket top trader analysis module.
//!
//! Analyzes top performing traders on Polymarket to learn from their strategies.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Polymarket trader analyzer
pub struct TraderAnalyzer {
    http: Client,
}

/// Trader profile from leaderboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderProfile {
    pub address: String,
    pub username: Option<String>,
    pub profit: Option<f64>,
    pub volume: Option<f64>,
    pub positions_count: Option<i32>,
    pub markets_traded: Option<i32>,
}

/// Trader position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderPosition {
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub outcome: String,
    #[serde(default)]
    pub size: f64,
    #[serde(default)]
    pub avg_price: f64,
    #[serde(default)]
    pub current_price: f64,
    #[serde(rename = "tokenId", default)]
    pub token_id: Option<String>,
    #[serde(rename = "conditionId", default)]
    pub condition_id: Option<String>,
}

/// Trader activity/trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderActivity {
    #[serde(default)]
    pub action: String,  // "buy" or "sell"
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub outcome: String,
    #[serde(default)]
    pub size: f64,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub timestamp: String,
    #[serde(rename = "tokenId", default)]
    pub token_id: Option<String>,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub proxy_address: String,
    #[serde(default)]
    pub profit_and_loss: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub positions_value: f64,
    #[serde(default)]
    pub num_markets_won: i32,
}

/// Analysis result for a trader
#[derive(Debug, Clone)]
pub struct TraderAnalysis {
    pub address: String,
    pub username: Option<String>,
    pub total_profit: f64,
    pub total_volume: f64,
    pub positions: Vec<TraderPosition>,
    pub recent_activity: Vec<TraderActivity>,
    pub top_markets: Vec<String>,
    pub trading_frequency: String,
    pub avg_position_size: f64,
}

impl TraderAnalyzer {
    /// Create a new trader analyzer.
    pub fn new() -> Self {
        Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Get the leaderboard (top traders by profit).
    pub async fn get_leaderboard(&self, time_period: &str, limit: usize) -> Result<Vec<LeaderboardEntry>> {
        // API: https://data-api.polymarket.com/v1/leaderboard?timePeriod=all&orderBy=VOL&limit=20&category=overall
        let url = format!(
            "https://data-api.polymarket.com/v1/leaderboard?timePeriod={}&orderBy=PNL&limit={}&category=overall",
            time_period, limit
        );
        
        debug!("Fetching leaderboard: {}", url);
        
        let response = self.http.get(&url).send().await
            .context("Failed to fetch leaderboard")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Leaderboard request failed: {} - {}", status, text);
        }
        
        let entries: Vec<LeaderboardEntry> = response.json().await
            .context("Failed to parse leaderboard response")?;
        
        info!("Fetched {} leaderboard entries", entries.len());
        Ok(entries)
    }

    /// Get positions for a specific trader.
    pub async fn get_trader_positions(&self, address: &str, limit: usize) -> Result<Vec<TraderPosition>> {
        let url = format!(
            "https://data-api.polymarket.com/positions?user={}&limit={}",
            address, limit
        );
        
        debug!("Fetching positions for {}", address);
        
        let response = self.http.get(&url).send().await
            .context("Failed to fetch positions")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch positions for {}", address);
            return Ok(vec![]);
        }
        
        let positions: Vec<TraderPosition> = response.json().await
            .unwrap_or_else(|_| vec![]);
        
        Ok(positions)
    }

    /// Get recent activity for a specific trader.
    pub async fn get_trader_activity(&self, address: &str, limit: usize) -> Result<Vec<TraderActivity>> {
        let url = format!(
            "https://data-api.polymarket.com/activity?user={}&limit={}",
            address, limit
        );
        
        debug!("Fetching activity for {}", address);
        
        let response = self.http.get(&url).send().await
            .context("Failed to fetch activity")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch activity for {}", address);
            return Ok(vec![]);
        }
        
        let activity: Vec<TraderActivity> = response.json().await
            .unwrap_or_else(|_| vec![]);
        
        Ok(activity)
    }

    /// Analyze a specific trader's strategy.
    pub async fn analyze_trader(&self, address: &str) -> Result<TraderAnalysis> {
        info!("Analyzing trader: {}", address);
        
        // Fetch positions and activity in parallel
        let (positions, activity) = tokio::join!(
            self.get_trader_positions(address, 50),
            self.get_trader_activity(address, 100)
        );
        
        let positions = positions.unwrap_or_default();
        let activity = activity.unwrap_or_default();
        
        // Calculate metrics
        let total_position_value: f64 = positions.iter()
            .map(|p| p.size * p.current_price)
            .sum();
        
        let avg_position_size = if positions.is_empty() {
            0.0
        } else {
            total_position_value / positions.len() as f64
        };
        
        // Identify top markets by position size
        let mut market_sizes: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for pos in &positions {
            *market_sizes.entry(pos.market.clone()).or_insert(0.0) += pos.size;
        }
        let mut top_markets: Vec<_> = market_sizes.into_iter().collect();
        top_markets.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let top_markets: Vec<String> = top_markets.into_iter().take(5).map(|(m, _)| m).collect();
        
        // Estimate trading frequency
        let trading_frequency = if activity.len() >= 50 {
            "High (50+ trades recently)".to_string()
        } else if activity.len() >= 20 {
            "Medium (20-50 trades recently)".to_string()
        } else if activity.len() >= 5 {
            "Low (5-20 trades recently)".to_string()
        } else {
            "Very Low (<5 trades recently)".to_string()
        };
        
        Ok(TraderAnalysis {
            address: address.to_string(),
            username: None,
            total_profit: 0.0, // Would need PnL API
            total_volume: 0.0, // Would need additional API
            positions,
            recent_activity: activity,
            top_markets,
            trading_frequency,
            avg_position_size,
        })
    }

    /// Get insights from top traders' current positions.
    pub async fn get_top_trader_insights(&self, num_traders: usize) -> Result<Vec<(String, Vec<TraderPosition>)>> {
        let leaderboard = self.get_leaderboard("monthly", num_traders).await?;
        
        let mut insights = Vec::new();
        
        for entry in leaderboard.iter().take(num_traders) {
            if entry.proxy_address.is_empty() {
                continue;
            }
            
            let positions = self.get_trader_positions(&entry.proxy_address, 20).await
                .unwrap_or_default();
            
            if !positions.is_empty() {
                let name = if entry.name.is_empty() {
                    entry.proxy_address.clone()
                } else {
                    entry.name.clone()
                };
                insights.push((name, positions));
            }
        }
        
        Ok(insights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_leaderboard() {
        let analyzer = TraderAnalyzer::new();
        let result = analyzer.get_leaderboard("monthly", 5).await;
        assert!(result.is_ok());
    }
}
