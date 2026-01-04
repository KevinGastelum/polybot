//! Arbitrage detector module.
//!
//! Monitors prices on both platforms and identifies profitable spreads.

use anyhow::Result;
use tracing::{info, debug};

use crate::polymarket::PolymarketClient;
use crate::kalshi::KalshiClient;
use super::market_matcher::MarketMatcher;

/// Analyzes market data for arbitrage opportunities.
pub struct ArbitrageDetector {
    /// Polymarket client
    poly_client: PolymarketClient,
    /// Kalshi client
    kalshi_client: KalshiClient,
    /// Market matcher
    matcher: MarketMatcher,
    /// Minimum profit threshold (e.g., 0.02 for 2%)
    min_profit: f64,
}

impl ArbitrageDetector {
    /// Create a new detector.
    pub fn new(
        poly_client: PolymarketClient,
        kalshi_client: KalshiClient,
        matcher: MarketMatcher,
        min_profit: f64,
    ) -> Self {
        Self {
            poly_client,
            kalshi_client,
            matcher,
            min_profit,
        }
    }

    /// Run a single detection pass across all matched markets.
    pub async fn check_all_opportunities(&self) -> Result<()> {
        let matches = self.matcher.get_all();
        
        for matched in matches {
            self.check_opportunity(matched).await?;
        }
        
        Ok(())
    }

    /// Check for arbitrage on a specific matched pair.
    pub async fn check_opportunity(&self, matched: &crate::arbitrage::market_matcher::MatchedMarket) -> Result<()> {
        debug!("Checking opportunity: {}", matched.name);

        // Get prices from Polymarket
        let (poly_bid, poly_ask) = self.poly_client.get_best_prices(&matched.polymarket_id).await?;
        
        // Get prices from Kalshi
        let (kalshi_bid, kalshi_ask) = self.kalshi_client.get_best_prices(&matched.kalshi_ticker).await?;

        // 1. Buy Kalshi, Sell Polymarket
        if let (Some(k_ask), Some(p_bid)) = (kalshi_ask, poly_bid) {
            let spread = p_bid - k_ask;
            if spread > self.min_profit {
                info!(
                    "🔥 ARB OPPORTUNITY FOUND: Buy Kalshi @ {:.3}, Sell Poly @ {:.3} | Spread: {:.2}% ({})",
                    k_ask, p_bid, spread * 100.0, matched.name
                );
            }
        }

        // 2. Buy Polymarket, Sell Kalshi
        if let (Some(p_ask), Some(k_bid)) = (poly_ask, kalshi_bid) {
            let spread = k_bid - p_ask;
            if spread > self.min_profit {
                info!(
                    "🔥 ARB OPPORTUNITY FOUND: Buy Poly @ {:.3}, Sell Kalshi @ {:.3} | Spread: {:.2}% ({})",
                    p_ask, k_bid, spread * 100.0, matched.name
                );
            }
        }

        Ok(())
    }
}
