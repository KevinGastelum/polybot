//! Polymarket CLOB API client.
//!
//! Handles all HTTP communication with Polymarket's Central Limit Order Book.

use anyhow::{Context, Result};
use reqwest::Client;
use tracing::{debug, info, warn};

use super::signer::PolymarketSigner;
use super::types::*;
use crate::config::Config;

/// Base URL for Polymarket CLOB API.
const CLOB_API_URL: &str = "https://clob.polymarket.com";

/// Polymarket API client.
pub struct PolymarketClient {
    /// HTTP client
    http: Client,
    /// Order signer
    signer: Option<PolymarketSigner>,
    /// Whether in dry-run mode
    dry_run: bool,
}

impl PolymarketClient {
    /// Create a new Polymarket client from configuration.
    pub fn new(config: &Config) -> Result<Self> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        let signer = if config.has_polymarket_credentials() {
            Some(PolymarketSigner::new(
                &config.polymarket_private_key,
                &config.polymarket_api_key,
                &config.polymarket_secret,
                &config.polymarket_passphrase,
            )?)
        } else {
            warn!("Polymarket credentials not configured - read-only mode");
            None
        };

        Ok(Self {
            http,
            signer,
            dry_run: config.dry_run,
        })
    }

    /// Get all active markets.
    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/markets", CLOB_API_URL);
        
        debug!("Fetching markets from {}", url);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch markets")?;

        let markets: Vec<Market> = response
            .json()
            .await
            .context("Failed to parse markets response")?;

        info!("Fetched {} markets", markets.len());
        Ok(markets)
    }

    /// Get order book for a specific token.
    pub async fn get_orderbook(&self, token_id: &str) -> Result<OrderBook> {
        let url = format!("{}/book?token_id={}", CLOB_API_URL, token_id);
        
        debug!("Fetching orderbook for token {}", token_id);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .context("Failed to fetch orderbook")?;

        let status = response.status();
        let text = response.text().await
            .context("Failed to read orderbook response")?;
        
        debug!("Orderbook response ({}): {}", status, &text[..text.len().min(500)]);

        // Parse the response - Polymarket CLOB returns a specific format
        let book: OrderBook = serde_json::from_str(&text)
            .context("Failed to parse orderbook response")?;

        Ok(book)
    }

    /// Get the best bid and ask prices for a token.
    pub async fn get_best_prices(&self, token_id: &str) -> Result<(Option<f64>, Option<f64>)> {
        let book = self.get_orderbook(token_id).await?;
        Ok((book.best_bid(), book.best_ask()))
    }

    /// Place an order on the CLOB.
    pub async fn place_order(&self, order: Order) -> Result<OrderResponse> {
        let signer = self.signer.as_ref()
            .context("Cannot place orders without credentials")?;

        if self.dry_run {
            info!(
                "DRY RUN: Would place {:?} order for {} shares at {} on token {}",
                order.side, order.size, order.price, order.token_id
            );
            return Ok(OrderResponse {
                order_id: Some("DRY_RUN_ORDER".to_string()),
                success: true,
                error: None,
                executions: None,
            });
        }

        let url = format!("{}/order", CLOB_API_URL);
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let body = serde_json::to_string(&order)?;
        
        let signature = signer.create_hmac_signature(
            &timestamp,
            "POST",
            "/order",
            &body,
        )?;

        let auth_headers = signer.get_auth_headers(&timestamp, &signature);
        
        let mut request = self.http
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body);

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request
            .send()
            .await
            .context("Failed to place order")?;

        let order_response: OrderResponse = response
            .json()
            .await
            .context("Failed to parse order response")?;

        if order_response.success {
            info!("Order placed successfully: {:?}", order_response.order_id);
        } else {
            warn!("Order failed: {:?}", order_response.error);
        }

        Ok(order_response)
    }

    /// Cancel an open order.
    pub async fn cancel_order(&self, order_id: &str) -> Result<bool> {
        let signer = self.signer.as_ref()
            .context("Cannot cancel orders without credentials")?;

        if self.dry_run {
            info!("DRY RUN: Would cancel order {}", order_id);
            return Ok(true);
        }

        let url = format!("{}/order/{}", CLOB_API_URL, order_id);
        let timestamp = chrono::Utc::now().timestamp().to_string();
        
        let signature = signer.create_hmac_signature(
            &timestamp,
            "DELETE",
            &format!("/order/{}", order_id),
            "",
        )?;

        let auth_headers = signer.get_auth_headers(&timestamp, &signature);
        
        let mut request = self.http.delete(&url);

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request
            .send()
            .await
            .context("Failed to cancel order")?;

        let success = response.status().is_success();
        
        if success {
            info!("Order {} cancelled successfully", order_id);
        } else {
            warn!("Failed to cancel order {}", order_id);
        }

        Ok(success)
    }

    /// Check if the client has write access (credentials configured).
    pub fn can_trade(&self) -> bool {
        self.signer.is_some()
    }
}
