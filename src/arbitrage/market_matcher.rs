//! Market matching module.
//!
//! Maps equivalent markets between Polymarket and Kalshi.

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
    /// Map from Kalshi ticker to matched market.
    matches: HashMap<String, MatchedMarket>,
}

impl MarketMatcher {
    /// Create a new market matcher with verified active market pairs.
    pub fn new() -> Self {
        let mut matches = HashMap::new();

        // ---------------------------------------------------------------------
        // SHORT-TERM MARKETS (Jan 4, 2026 - 5pm EST resolution)
        // These are ACTIVE markets from the KXBTCD-26JAN0417 event.
        // Ticker format: KXBTCD-26JAN0417-T{strike-0.01}
        // ---------------------------------------------------------------------

        // 1. Bitcoin Above $97,750 (Jan 4, 5pm EST)
        // Kalshi ticker verified: KXBTCD-26JAN0417-T97749.99
        let btc_97750_jan4 = MatchedMarket {
            name: "BTC Above $97,750 (Jan 4 5pm)".to_string(),
            // Placeholder Polymarket ID - need to find matching market
            polymarket_id: "69247515319465768372083049433252998824082264566479049746766827838123736211528".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T97749.99".to_string(),
        };
        matches.insert(btc_97750_jan4.polymarket_id.clone(), btc_97750_jan4);

        // 2. Bitcoin Above $98,250 (Jan 4, 5pm EST)
        let btc_98250_jan4 = MatchedMarket {
            name: "BTC Above $98,250 (Jan 4 5pm)".to_string(),
            polymarket_id: "41926693962996844375240544873659045678453567376343419315993827729886928943511".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T98249.99".to_string(),
        };
        matches.insert(btc_98250_jan4.polymarket_id.clone(), btc_98250_jan4);

        // 3. Bitcoin Above $98,750 (Jan 4, 5pm EST)
        let btc_98750_jan4 = MatchedMarket {
            name: "BTC Above $98,750 (Jan 4 5pm)".to_string(),
            polymarket_id: "102408286949673884647958077490729153220746101153545802177147488106380288306606".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0417-T98749.99".to_string(),
        };
        matches.insert(btc_98750_jan4.polymarket_id.clone(), btc_98750_jan4);

        // ---------------------------------------------------------------------
        // LONG-TERM MILESTONES (currently returning 404 - needs ticker verification)
        // ---------------------------------------------------------------------

        // TODO: Find correct ticker format for Bitcoin $200k milestone
        // let btc_200k = MatchedMarket {
        //     name: "BTC $200k Milestone".to_string(),
        //     polymarket_id: "61368943128255287414565270336856615453000675377332178800733742873558311943412".to_string(),
        //     kalshi_ticker: "BTCMAXY-26DEC31-B199999.99".to_string(),
        // };
        // matches.insert(btc_200k.polymarket_id.clone(), btc_200k);

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
