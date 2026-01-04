use anyhow::Result;
use polymarket_kalshi_arbitrage_bot::config::Config;
use polymarket_kalshi_arbitrage_bot::polymarket::PolymarketClient;
use polymarket_kalshi_arbitrage_bot::kalshi::KalshiClient;
use dotenvy::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    // Initialize logging with INFO level
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();
    
    println!("Starting API Key Verification...");

    // Load configuration
    let config = Config::from_env()?;

    // 1. Verify Polymarket
    println!("\n--- Polymarket Verification ---");
    if config.polymarket_api_key.is_empty() {
        println!("❌ Polymarket keys missing in .env");
    } else {
        match PolymarketClient::new(&config) {
            Ok(poly_client) => {
                if poly_client.can_trade() {
                    println!("✅ Polymarket Client Initialized (Credentials found)");
                } else {
                    println!("⚠️  Polymarket: Read-only mode (credentials missing)");
                }
            },
            Err(e) => println!("❌ Polymarket Initialization Error: {}", e),
        }
    }

    // 2. Verify Kalshi
    println!("\n--- Kalshi Verification ---");
    println!("Email configured: {}", config.kalshi_email.is_some());
    println!("Password configured: {}", config.kalshi_password.is_some());
    
    if !config.has_kalshi_credentials() {
        println!("❌ Kalshi credentials missing in .env");
    } else {
        match KalshiClient::new(&config) {
            Ok(mut kalshi_client) => {
                println!("Attempting Kalshi login...");
                match kalshi_client.login().await {
                    Ok(true) => println!("✅ Kalshi: Logged in successfully"),
                    Ok(false) => println!("❌ Kalshi: Login failed (Check credentials/URL)"),
                    Err(e) => println!("❌ Kalshi Error: {}", e),
                }
            },
            Err(e) => println!("❌ Kalshi Initialization Error: {}", e),
        }
    }

    println!("\nVerification Complete.");
    Ok(())
}
