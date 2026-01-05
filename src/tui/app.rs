//! TUI Application state and logic.

use crate::paper_trading::{PaperTradingEngine, PaperTrade};


/// Active tab in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Markets,
    Trades,
    Strategies,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Dashboard => Tab::Markets,
            Tab::Markets => Tab::Trades,
            Tab::Trades => Tab::Strategies,
            Tab::Strategies => Tab::Dashboard,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Tab::Dashboard => Tab::Strategies,
            Tab::Markets => Tab::Dashboard,
            Tab::Trades => Tab::Markets,
            Tab::Strategies => Tab::Trades,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Markets => "Markets",
            Tab::Trades => "Trades",
            Tab::Strategies => "Strategies",
        }
    }
}

/// Live market data
#[derive(Debug, Clone)]
pub struct MarketData {
    pub name: String,
    pub coin: String,
    pub timeframe: String,
    pub poly_price: Option<f64>,
    pub kalshi_price: Option<f64>,
    pub spread: Option<f64>,
    pub liquidity: f64,
    pub time_to_resolve: String,
}

/// Strategy status
#[derive(Debug, Clone)]
pub struct StrategyStatus {
    pub name: String,
    pub enabled: bool,
    pub trades_today: usize,
    pub pnl_today: f64,
}

/// Top trader info
#[derive(Debug, Clone)]
pub struct TopTrader {
    pub name: String,
    pub address: String,
    pub monthly_pnl: f64,
    pub is_copying: bool,
}

/// Main application state
pub struct App {
    /// Current active tab
    pub active_tab: Tab,
    /// Should the app quit
    pub should_quit: bool,
    /// Paper trading engine
    pub engine: PaperTradingEngine,
    /// Live market data
    pub markets: Vec<MarketData>,
    /// Strategy statuses
    pub strategies: Vec<StrategyStatus>,
    /// Top traders
    pub top_traders: Vec<TopTrader>,
    /// Selected index in current list
    pub selected_index: usize,
    /// Status message
    pub status_message: Option<String>,
    /// Is refreshing data
    pub is_refreshing: bool,
}

impl App {
    /// Create a new app instance.
    pub fn new() -> Self {
        let engine = PaperTradingEngine::new();
        
        // Initialize with default data
        let markets = vec![
            MarketData {
                name: "BTC Up/Down 5PM ET".to_string(),
                coin: "BTC".to_string(),
                timeframe: "Hourly".to_string(),
                poly_price: Some(0.505),
                kalshi_price: Some(0.53),
                spread: Some(0.025),
                liquidity: 72724.0,
                time_to_resolve: "4h".to_string(),
            },
            MarketData {
                name: "BTC Up/Down 8PM ET".to_string(),
                coin: "BTC".to_string(),
                timeframe: "Hourly".to_string(),
                poly_price: Some(0.48),
                kalshi_price: Some(0.51),
                spread: Some(0.03),
                liquidity: 45000.0,
                time_to_resolve: "7h".to_string(),
            },
            MarketData {
                name: "ETH Up/Down 5PM ET".to_string(),
                coin: "ETH".to_string(),
                timeframe: "Hourly".to_string(),
                poly_price: Some(0.52),
                kalshi_price: Some(0.49),
                spread: Some(0.03),
                liquidity: 28000.0,
                time_to_resolve: "4h".to_string(),
            },
        ];

        let strategies = vec![
            StrategyStatus {
                name: "Arbitrage".to_string(),
                enabled: true,
                trades_today: 0,
                pnl_today: 0.0,
            },
            StrategyStatus {
                name: "Copy Trading".to_string(),
                enabled: true,
                trades_today: 0,
                pnl_today: 0.0,
            },
            StrategyStatus {
                name: "Manual".to_string(),
                enabled: true,
                trades_today: 0,
                pnl_today: 0.0,
            },
        ];

        let top_traders = vec![
            TopTrader {
                name: "SeriouslySirius".to_string(),
                address: "0x16b29c50f2439faf627209b2ac0c7bbddaa8a881".to_string(),
                monthly_pnl: 1511844.0,
                is_copying: true,
            },
            TopTrader {
                name: "DrPufferfish".to_string(),
                address: "0xdb27bf2ac5d428a9c63dbc914611036855a6c56e".to_string(),
                monthly_pnl: 772762.0,
                is_copying: true,
            },
            TopTrader {
                name: "kch123".to_string(),
                address: "0x6a72f61820b26b1fe4d956e17b6dc2a1ea3033ee".to_string(),
                monthly_pnl: 693265.0,
                is_copying: false,
            },
            TopTrader {
                name: "Theo4".to_string(),
                address: "0x56687bf447db6ffa42ffe2204a05edaa20f55839".to_string(),
                monthly_pnl: 22000000.0,
                is_copying: false,
            },
        ];

        Self {
            active_tab: Tab::Dashboard,
            should_quit: false,
            engine,
            markets,
            strategies,
            top_traders,
            selected_index: 0,
            status_message: Some("Ready - Press 'h' for help".to_string()),
            is_refreshing: false,
        }
    }

    /// Handle key input.
    pub fn on_key(&mut self, key: char) {
        match key {
            'q' | 'Q' => self.should_quit = true,
            '1' => self.active_tab = Tab::Dashboard,
            '2' => self.active_tab = Tab::Markets,
            '3' => self.active_tab = Tab::Trades,
            '4' => self.active_tab = Tab::Strategies,
            'r' | 'R' => {
                self.is_refreshing = true;
                self.status_message = Some("Refreshing market data...".to_string());
            }
            'j' | 'J' => self.next_item(),
            'k' | 'K' => self.prev_item(),
            'b' | 'B' => self.execute_paper_buy(),
            's' | 'S' => self.execute_paper_sell(),
            't' | 'T' => self.toggle_strategy(),
            _ => {}
        }
    }

    /// Handle special keys.
    pub fn on_special_key(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match key {
            KeyCode::Tab => self.active_tab = self.active_tab.next(),
            KeyCode::BackTab => self.active_tab = self.active_tab.prev(),
            KeyCode::Down => self.next_item(),
            KeyCode::Up => self.prev_item(),
            KeyCode::Enter => self.select_item(),
            KeyCode::Esc => self.status_message = None,
            _ => {}
        }
    }

    fn next_item(&mut self) {
        let max = match self.active_tab {
            Tab::Markets => self.markets.len().saturating_sub(1),
            Tab::Trades => self.engine.trade_log.get_all().len().saturating_sub(1),
            Tab::Strategies => self.strategies.len().saturating_sub(1),
            _ => 0,
        };
        if self.selected_index < max {
            self.selected_index += 1;
        }
    }

    fn prev_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn select_item(&mut self) {
        match self.active_tab {
            Tab::Markets => {
                if let Some(market) = self.markets.get(self.selected_index) {
                    self.status_message = Some(format!("Selected: {}", market.name));
                }
            }
            Tab::Strategies => {
                self.toggle_strategy();
            }
            _ => {}
        }
    }

    fn toggle_strategy(&mut self) {
        if let Some(strategy) = self.strategies.get_mut(self.selected_index) {
            strategy.enabled = !strategy.enabled;
            self.status_message = Some(format!(
                "{} strategy {}",
                strategy.name,
                if strategy.enabled { "enabled" } else { "disabled" }
            ));
        }
    }

    fn execute_paper_buy(&mut self) {
        if self.active_tab != Tab::Markets {
            self.status_message = Some("Switch to Markets tab to buy".to_string());
            return;
        }

        if let Some(market) = self.markets.get(self.selected_index).cloned() {
            let price = market.poly_price.unwrap_or(0.5);
            let size = 10.0; // $10 default size
            
            match self.engine.buy(
                &market.name,
                &market.coin,
                &market.timeframe,
                "polymarket",
                size,
                price,
                "manual",
                0.5,
            ) {
                Ok(_) => {
                    self.status_message = Some(format!(
                        "✅ Bought ${:.0} of {} @ {:.2}",
                        size, market.name, price
                    ));
                }
                Err(e) => {
                    self.status_message = Some(format!("❌ Buy failed: {}", e));
                }
            }
        }
    }

    fn execute_paper_sell(&mut self) {
        if self.active_tab != Tab::Markets {
            self.status_message = Some("Switch to Markets tab to sell".to_string());
            return;
        }

        if let Some(market) = self.markets.get(self.selected_index).cloned() {
            let price = market.poly_price.unwrap_or(0.5);
            
            match self.engine.sell(&market.name, price) {
                Ok(pnl) => {
                    let emoji = if pnl >= 0.0 { "✅" } else { "❌" };
                    self.status_message = Some(format!(
                        "{} Sold {} for ${:.2} P&L",
                        emoji, market.name, pnl
                    ));
                }
                Err(e) => {
                    self.status_message = Some(format!("❌ Sell failed: {}", e));
                }
            }
        }
    }

    /// Get recent trades for display.
    pub fn recent_trades(&self) -> Vec<&PaperTrade> {
        self.engine.trade_log.get_recent(10)
    }

    /// Get open positions.
    pub fn open_positions(&self) -> Vec<(&String, &crate::paper_trading::Position)> {
        self.engine.portfolio.positions.iter().collect()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
