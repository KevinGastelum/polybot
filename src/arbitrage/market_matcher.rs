//! Market matcher module.
//!
//! Responsible for mapping equivalent markets between Polymarket and Kalshi.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a matched pair of markets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedMarket {
    /// Friendly name for the matched market
    pub name: String,
    /// Polymarket token ID (for a specific outcome, e.g., YES)
    pub polymarket_id: String,
    /// Kalshi ticker (for the same outcome)
    pub kalshi_ticker: String,
}

/// Market matcher handles the mapping between platforms.
pub struct MarketMatcher {
    /// Map of Polymarket ID -> MatchedMarket
    matches: HashMap<String, MatchedMarket>,
}

impl MarketMatcher {
    /// Create a new market matcher.
    pub fn new() -> Self {
        let mut matches = HashMap::new();

        // ---------------------------------------------------------------------
        // SHORT-TERM MARKETS (Jan 4 - Jan 5, 2026)
        // ---------------------------------------------------------------------

        // 1. Bitcoin Above $90,000 (Jan 4, 9 AM EST)
        let btc_90k_jan4 = MatchedMarket {
            name: "BTC Above $90k (Jan 4)".to_string(),
            polymarket_id: "69247515319465768372083049433252998824082264566479049746766827838123736211528".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0409-T89999".to_string(),
        };
        matches.insert(btc_90k_jan4.polymarket_id.clone(), btc_90k_jan4);

        // 2. Bitcoin Above $92,000 (Jan 4, 9 AM EST)
        let btc_92k_jan4 = MatchedMarket {
            name: "BTC Above $92k (Jan 4)".to_string(),
            polymarket_id: "41926693962996844375240544873659045678453567376343419315993827729886928943511".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0409-T91999".to_string(),
        };
        matches.insert(btc_92k_jan4.polymarket_id.clone(), btc_92k_jan4);

        // 3. Bitcoin Above $90,000 (Jan 5, 9 AM EST)
        let btc_90k_jan5 = MatchedMarket {
            name: "BTC Above $90k (Jan 5)".to_string(),
            polymarket_id: "102408286949673884647958077490729153220746101153545802177147488106380288306606".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0509-T89999".to_string(),
        };
        matches.insert(btc_90k_jan5.polymarket_id.clone(), btc_90k_jan5);

        // 4. Bitcoin Above $92,000 (Jan 5, 9 AM EST)
        let btc_92k_jan5 = MatchedMarket {
            name: "BTC Above $92k (Jan 5)".to_string(),
            polymarket_id: "96810459394593527443983742996467887973884848456728623117973078553343049397227".to_string(),
            kalshi_ticker: "KXBTCD-26JAN0509-T91999".to_string(),
        };
        matches.insert(btc_92k_jan5.polymarket_id.clone(), btc_92k_jan5);

        // ---------------------------------------------------------------------
        // LONG-TERM MILESTONES
        // ---------------------------------------------------------------------

        // Bitcoin $200k by December 31, 2026
        let btc_200k = MatchedMarket {
            name: "BTC $200k Milestone".to_string(),
            polymarket_id: "61368943128255287414565270336856615453000675377332178800733742873558311943412".to_string(),
            kalshi_ticker: "BTCMAXY-26DEC31-B199999.99".to_string(),
        };
        matches.insert(btc_200k.polymarket_id.clone(), btc_200k);

        Self { matches }
    }

    /// Get matched market by Polymarket ID.
    pub fn get_by_polymarket(&self, id: &str) -> Option<&MatchedMarket> {
        self.matches.get(id)
    }

    /// Get all matches.
    pub fn get_all(&self) -> Vec<&MatchedMarket> {
        self.matches.values().collect()
    }
    
    /// Add a new match.
    pub fn add_match(&mut self, matched: MatchedMarket) {
        self.matches.insert(matched.polymarket_id.clone(), matched);
    }
}
