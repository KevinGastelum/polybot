mod config;
mod polymarket;
mod kalshi;
mod arbitrage;
mod utils;

use anyhow::Result;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use config::Config;
use polymarket::PolymarketClient;
use kalshi::KalshiClient;
use arbitrage::{ArbitrageDetector, MarketMatcher};
use utils::CircuitBreaker;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .pretty()
        .init();

    info!("🚀 Starting Polymarket-Kalshi Arbitrage Bot");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Load configuration
    let config = Config::from_env()?;
    
    if config.dry_run {
        info!("⚠️  DRY RUN MODE - No real trades will be executed");
    }

    info!("📊 Min Profit Threshold: {:.2}%", config.min_profit_threshold * 100.0);
    info!("💰 Max Position Size: ${}", config.max_position_size);

    // Initialize Safety
    let circuit_breaker = CircuitBreaker::new();

    // Initialize Polymarket client
    info!("🔌 Connecting to Polymarket...");
    let poly_client = PolymarketClient::new(&config)?;
    
    if poly_client.can_trade() {
        info!("✅ Polymarket: Trading enabled");
    } else {
        warn!("⚠️  Polymarket: Read-only mode (no credentials)");
    }

    // Initialize Kalshi client
    info!("🔌 Connecting to Kalshi...");
    let mut kalshi_client = KalshiClient::new(&config)?;
    
    if kalshi_client.has_credentials() {
        match kalshi_client.login().await {
            Ok(true) => info!("✅ Kalshi: Logged in successfully"),
            Ok(false) => warn!("⚠️  Kalshi: Login failed"),
            Err(e) => warn!("⚠️  Kalshi: Login error: {}", e),
        }
    } else {
        warn!("⚠️  Kalshi: No credentials configured");
    }

    // Initialize Market Matcher
    let matcher = MarketMatcher::new();
    info!("📚 Initialized {} market pairs", matcher.get_all().len());

    // Initialize Arbitrage Detector
    let detector = ArbitrageDetector::new(
        poly_client,
        kalshi_client,
        matcher,
        config.min_profit_threshold,
    );

    info!("👀 Monitoring for arbitrage opportunities...");
    info!("");

    // Simple monitoring loop
    while circuit_breaker.is_allowed() {
        if let Err(e) = detector.check_all_opportunities().await {
            warn!("Error in detection pass: {}", e);
            // If we hit too many sequential errors, trip the breaker
            // circuit_breaker.trip("Too many API errors"); 
        }
        
        // Wait before next pass
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }

    info!("🛑 Bot halted by circuit breaker. Shutting down...");
    Ok(())
}
