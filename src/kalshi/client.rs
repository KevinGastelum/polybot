//! Kalshi REST API client.
//!
//! Handles all HTTP communication with Kalshi's trading API.
//! Uses RSA-PSS signature-based authentication.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, info, warn};
use std::time::{SystemTime, UNIX_EPOCH};

use super::types::*;
use crate::config::Config;

/// Base URL for Kalshi API (production - new endpoint).
const KALSHI_API_URL: &str = "https://api.elections.kalshi.com/trade-api/v2";

/// Kalshi API client with RSA-PSS authentication.
pub struct KalshiClient {
    /// HTTP client
    http: Client,
    /// API Key ID
    api_key_id: Option<String>,
    /// API Secret (for HMAC or simpler auth if available)
    api_secret: Option<String>,
    /// Email for legacy login (deprecated)
    email: Option<String>,
    /// Password for legacy login (deprecated)
    password: Option<String>,
    /// Whether in dry-run mode
    dry_run: bool,
}

impl KalshiClient {
    /// Create a new Kalshi client from configuration.
    pub fn new(config: &Config) -> Result<Self> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            http,
            api_key_id: config.kalshi_api_key.clone(),
            api_secret: config.kalshi_api_secret.clone(),
            email: config.kalshi_email.clone(),
            password: config.kalshi_password.clone(),
            dry_run: config.dry_run,
        })
    }

    /// Get current timestamp in milliseconds.
    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    /// Generate authentication headers for API requests.
    /// 
    /// Note: Full RSA-PSS signing requires the private key file.
    /// For now, we'll use a simpler approach if API key/secret is available.
    fn auth_headers(&self, _method: &str, _path: &str) -> Option<Vec<(&'static str, String)>> {
        let api_key = self.api_key_id.as_ref()?;
        let timestamp = Self::current_timestamp_ms().to_string();
        
        // Basic API key auth headers
        Some(vec![
            ("KALSHI-ACCESS-KEY", api_key.clone()),
            ("KALSHI-ACCESS-TIMESTAMP", timestamp),
            // Note: Full implementation needs RSA-PSS signature
            // ("KALSHI-ACCESS-SIGNATURE", signature),
        ])
    }

    /// Authenticate/login is not needed with API key auth.
    /// This method now just validates that credentials are configured.
    pub async fn login(&mut self) -> Result<bool> {
        // Check if we have API key credentials
        if self.api_key_id.is_some() {
            info!("Kalshi API key configured - testing connection...");
            
            // Test the connection by fetching exchange status
            match self.get_exchange_status().await {
                Ok(status) => {
                    info!("Kalshi connection successful: {}", status);
                    Ok(true)
                }
                Err(e) => {
                    warn!("Kalshi connection test failed: {}", e);
                    Ok(false)
                }
            }
        } else if self.email.is_some() && self.password.is_some() {
            // Legacy email/password auth - no longer supported
            warn!("Email/password authentication is deprecated by Kalshi.");
            warn!("Please generate API keys at https://kalshi.com/account/api");
            Ok(false)
        } else {
            warn!("No Kalshi credentials configured");
            Ok(false)
        }
    }

    /// Get exchange status (public endpoint, no auth required).
    pub async fn get_exchange_status(&self) -> Result<String> {
        let url = format!("{}/exchange/status", KALSHI_API_URL);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch exchange status")?;

        let status = response.status();
        let text = response.text().await
            .context("Failed to read exchange status response")?;
        
        if status.is_success() {
            Ok(text)
        } else {
            anyhow::bail!("Exchange status check failed: {}", text)
        }
    }

    /// Get all events/markets.
    pub async fn get_events(&self, limit: Option<i32>) -> Result<Vec<KalshiEvent>> {
        let url = format!(
            "{}/events?limit={}&status=open",
            KALSHI_API_URL,
            limit.unwrap_or(100)
        );
        
        debug!("Fetching Kalshi events");

        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch events")?;

        #[derive(Deserialize)]
        struct EventsResponse {
            events: Vec<KalshiEvent>,
        }

        let events_resp: EventsResponse = response
            .json()
            .await
            .context("Failed to parse events response")?;

        info!("Fetched {} Kalshi events", events_resp.events.len());
        Ok(events_resp.events)
    }

    /// Get a specific market by ticker.
    pub async fn get_market(&self, ticker: &str) -> Result<KalshiMarket> {
        let url = format!("{}/markets/{}", KALSHI_API_URL, ticker);
        
        debug!("Fetching Kalshi market {}", ticker);

        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch market")?;

        let status = response.status();
        let text = response.text().await
            .context("Failed to read market response body")?;
        
        debug!("Kalshi market response ({}): {}", status, &text[..text.len().min(500)]);

        if !status.is_success() {
            anyhow::bail!("Kalshi market request failed: {}", text);
        }

        #[derive(Deserialize)]
        struct MarketResponse {
            market: KalshiMarket,
        }

        let market_resp: MarketResponse = serde_json::from_str(&text)
            .context("Failed to parse market response")?;

        Ok(market_resp.market)
    }

    /// Get order book for a market.
    pub async fn get_orderbook(&self, ticker: &str) -> Result<KalshiOrderBook> {
        let url = format!("{}/markets/{}/orderbook", KALSHI_API_URL, ticker);
        
        debug!("Fetching Kalshi orderbook for {}", ticker);

        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch orderbook")?;

        #[derive(Deserialize)]
        struct OrderBookResponse {
            orderbook: KalshiOrderBook,
        }

        let book_resp: OrderBookResponse = response
            .json()
            .await
            .context("Failed to parse orderbook response")?;

        Ok(book_resp.orderbook)
    }

    /// Get best prices for a market (converted to 0.0-1.0 scale).
    pub async fn get_best_prices(&self, ticker: &str) -> Result<(Option<f64>, Option<f64>)> {
        let market = self.get_market(ticker).await?;
        
        // Convert from cents (0-100) to probability (0.0-1.0)
        let yes_bid = market.yes_bid.map(|p| p as f64 / 100.0);
        let yes_ask = market.yes_ask.map(|p| p as f64 / 100.0);
        
        Ok((yes_bid, yes_ask))
    }

    /// Place an order.
    pub async fn place_order(&self, order: KalshiOrderRequest) -> Result<KalshiOrderResponse> {
        // For now, orders require full RSA-PSS auth which we don't have yet
        if !self.api_key_id.is_some() {
            anyhow::bail!("API key required for placing orders");
        }

        if self.dry_run {
            info!(
                "DRY RUN: Would place {} {} order for {} contracts on {}",
                order.action, order.side, order.count, order.ticker
            );
            return Ok(KalshiOrderResponse {
                order_id: Some("DRY_RUN_ORDER".to_string()),
                status: Some("filled".to_string()),
                error: None,
            });
        }

        let url = format!("{}/portfolio/orders", KALSHI_API_URL);

        // TODO: Add proper RSA-PSS signature auth headers here
        let response = self.http
            .post(&url)
            .json(&order)
            .send()
            .await
            .context("Failed to place order")?;

        let order_resp: KalshiOrderResponse = response
            .json()
            .await
            .context("Failed to parse order response")?;

        if order_resp.order_id.is_some() {
            info!("Kalshi order placed: {:?}", order_resp.order_id);
        } else {
            warn!("Kalshi order failed: {:?}", order_resp.error);
        }

        Ok(order_resp)
    }

    /// Get current positions.
    pub async fn get_positions(&self) -> Result<Vec<KalshiPosition>> {
        if !self.api_key_id.is_some() {
            anyhow::bail!("API key required for fetching positions");
        }

        let url = format!("{}/portfolio/positions", KALSHI_API_URL);

        // TODO: Add proper RSA-PSS signature auth headers here
        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch positions")?;

        #[derive(Deserialize)]
        struct PositionsResponse {
            positions: Vec<KalshiPosition>,
        }

        let positions_resp: PositionsResponse = response
            .json()
            .await
            .context("Failed to parse positions response")?;

        Ok(positions_resp.positions)
    }

    /// Get account balance.
    pub async fn get_balance(&self) -> Result<KalshiBalance> {
        if !self.api_key_id.is_some() {
            anyhow::bail!("API key required for fetching balance");
        }

        let url = format!("{}/portfolio/balance", KALSHI_API_URL);

        // TODO: Add proper RSA-PSS signature auth headers here
        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch balance")?;

        let balance: KalshiBalance = response
            .json()
            .await
            .context("Failed to parse balance response")?;

        Ok(balance)
    }

    /// Check if authenticated (API key configured).
    pub fn is_authenticated(&self) -> bool {
        self.api_key_id.is_some()
    }

    /// Check if credentials are configured.
    pub fn has_credentials(&self) -> bool {
        self.api_key_id.is_some() || (self.email.is_some() && self.password.is_some())
    }
}
