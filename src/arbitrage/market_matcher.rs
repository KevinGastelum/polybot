//! Market matching module.
//!
//! Maps equivalent markets between Polymarket and Kalshi.
//! 
//! IMPORTANT: Polymarket uses $2,000 increments (e.g., $94k, $96k, $98k)
//! while Kalshi uses $250-$500 increments (e.g., $97,750, $98,250, $98,750).
//! This means exact arbitrage is not possible, but we can compare nearby thresholds.

use std::collections::HashMap;

/// Represents a matched pair of markets on different platforms.
#[derive(Debug, Clone)]
pub struct MatchedMarket {
    /// Human-readable name for the market pair.
    pub name: String,
    /// Polymarket token ID (the clobTokenId for YES outcome).
    pub polymarket_id: String,
    /// Kalshi market ticker.
    pub kalshi_ticker: String,
}

/// Market matcher that maps equivalent markets.
pub struct MarketMatcher {
    /// Map from Polymarket ID to matched market.
    matches: HashMap<String, MatchedMarket>,
}

impl MarketMatcher {
    /// Create a new market matcher with verified active market pairs.
    /// 
    /// Note: These are "approximate" matches due to different granularities:
    /// - Polymarket: $2,000 increments
    /// - Kalshi: $250-$500 increments
    pub fn new() -> Self {
        let mut matches = HashMap::new();

        // ---------------------------------------------------------------------
        // BITCOIN PRICE MARKETS - January 4, 2026
        // Polymarket resolution: 12:00 PM ET (Binance 1-minute candle)
        // Kalshi resolution: 5:00 PM EST (CF Benchmarks RTI average)
        // ---------------------------------------------------------------------

        // 1. Bitcoin Above $98,000 (closest match to Kalshi $98,250)
        // Polymarket: "Bitcoin above 98,000 on January 4?" - resolves 12pm ET
        // Kalshi: "KXBTCD-26JAN0417-T98249.99" - resolves 5pm EST
        let btc_98k = MatchedMarket {
            name: "BTC Above ~$98k (Jan 4)".to_string(),
            // YES clobTokenId for "Bitcoin above 98,000 on January 4?"
            polymarket_id: "112281706743127882541430899708477543478860369766089047798338771401447150750990".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T98249.99".to_string(),
        };
        matches.insert(btc_98k.polymarket_id.clone(), btc_98k);

        // 2. Bitcoin Above $96,000 (closest match to Kalshi $97,750)
        // Polymarket: "Bitcoin above 96,000 on January 4?" - resolves 12pm ET
        // Kalshi: "KXBTCD-26JAN0417-T97749.99" - resolves 5pm EST
        let btc_96k = MatchedMarket {
            name: "BTC Above ~$96k-$97.75k (Jan 4)".to_string(),
            // YES clobTokenId for "Bitcoin above 96,000 on January 4?"
            polymarket_id: "41888813420182332299310344861513525293633211919331684128442282650474680953091".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T97749.99".to_string(),
        };
        matches.insert(btc_96k.polymarket_id.clone(), btc_96k);

        // 3. Additional Kalshi market for spread analysis
        // Kalshi: "KXBTCD-26JAN0417-T98749.99" ($98,750 threshold)
        // No direct Polymarket equivalent - using $98k for comparison
        let btc_98_75k = MatchedMarket {
            name: "BTC Above $98,750 (Kalshi only)".to_string(),
            // Reusing $98k Polymarket ID for comparison
            polymarket_id: "112281706743127882541430899708477543478860369766089047798338771401447150750990".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T98749.99".to_string(),
        };
        // Note: Don't insert duplicate - just for reference
        // matches.insert(btc_98_75k.polymarket_id.clone(), btc_98_75k);
        let _ = btc_98_75k; // suppress warning

        Self { matches }
    }

    /// Get all matched markets.
    pub fn get_all(&self) -> Vec<&MatchedMarket> {
        self.matches.values().collect()
    }

    /// Get a matched market by Polymarket ID.
    pub fn get_by_polymarket(&self, id: &str) -> Option<&MatchedMarket> {
        self.matches.get(id)
    }

    /// Get a matched market by Kalshi ticker.
    pub fn get_by_kalshi(&self, ticker: &str) -> Option<&MatchedMarket> {
        self.matches.values().find(|m| m.kalshi_ticker == ticker)
    }

    /// Add a new matched market.
    pub fn add_match(&mut self, matched: MatchedMarket) {
        self.matches.insert(matched.polymarket_id.clone(), matched);
    }
}
