//! Market matching module.
//!
//! Maps equivalent markets between Polymarket and Kalshi.
//!
//! Note: Polymarket and Kalshi have different market structures:
//! - Polymarket: "Bitcoin Up or Down" hourly markets (50/50 binary)
//! - Kalshi: "Bitcoin above $X" threshold markets
//!
//! For arbitrage, we compare implied probabilities between platforms.

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
    /// Create a new market matcher with active market pairs.
    ///
    /// Updated: January 5, 2026
    /// Markets are "Bitcoin Up or Down" hourly resolution markets.
    pub fn new() -> Self {
        let mut matches = HashMap::new();

        // ---------------------------------------------------------------------
        // BITCOIN UP OR DOWN - January 5, 2026 (Hourly Markets)
        // These resolve at the end of each hour based on Binance BTC/USDT
        // clobTokenId is for the "Up" (YES) outcome
        // ---------------------------------------------------------------------

        // 1. Bitcoin Up or Down - 3PM ET (resolves 4PM ET)
        let btc_3pm = MatchedMarket {
            name: "BTC Up/Down 3PM ET (Jan 5)".to_string(),
            polymarket_id: "19624172204178867270299534492363892804243098884958805437588142691739650752818".to_string(),
            // Using 5pm EST Kalshi market for comparison
            kalshi_ticker: "KXBTCD-26JAN0517-T94249.99".to_string(),
        };
        matches.insert(btc_3pm.polymarket_id.clone(), btc_3pm);

        // 2. Bitcoin Up or Down - 5PM ET (resolves 6PM ET)
        let btc_5pm = MatchedMarket {
            name: "BTC Up/Down 5PM ET (Jan 5)".to_string(),
            polymarket_id: "64331692285920497167043827511734089895966734302171910924386164102158120192515".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0517-T94249.99".to_string(),
        };
        matches.insert(btc_5pm.polymarket_id.clone(), btc_5pm);

        // 3. Bitcoin Up or Down - 8PM ET (resolves 9PM ET)
        let btc_8pm = MatchedMarket {
            name: "BTC Up/Down 8PM ET (Jan 5)".to_string(),
            polymarket_id: "63501553680907011398404492052704199683744111807521098612450074584385523964810".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0517-T94249.99".to_string(),
        };
        matches.insert(btc_8pm.polymarket_id.clone(), btc_8pm);

        // 4. Bitcoin Up or Down - 11PM ET (resolves 12AM ET next day)
        let btc_11pm = MatchedMarket {
            name: "BTC Up/Down 11PM ET (Jan 5)".to_string(),
            polymarket_id: "11322761507222986303977493587384536158539335638025200075639546051812934376948".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0517-T94249.99".to_string(),
        };
        matches.insert(btc_11pm.polymarket_id.clone(), btc_11pm);

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
