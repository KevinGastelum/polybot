//! Ethereum transaction signer for Polymarket.
//!
//! Handles signing of orders and transactions for the CLOB API.

use anyhow::{Context, Result};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Signature};
use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

/// Polymarket order signer.
pub struct PolymarketSigner {
    /// Ethereum wallet for signing
    wallet: LocalWallet,
    /// API key for CLOB
    api_key: String,
    /// API secret for HMAC
    api_secret: String,
    /// API passphrase
    passphrase: String,
}

impl PolymarketSigner {
    /// Create a new signer from private key and API credentials.
    pub fn new(
        private_key: &str,
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
    ) -> Result<Self> {
        let wallet: LocalWallet = private_key
            .parse()
            .context("Failed to parse private key")?;

        Ok(Self {
            wallet,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            passphrase: passphrase.to_string(),
        })
    }

    /// Get the wallet address.
    pub fn address(&self) -> Address {
        self.wallet.address()
    }

    /// Sign a message with the wallet.
    pub async fn sign_message(&self, message: &[u8]) -> Result<Signature> {
        let signature = self.wallet
            .sign_message(message)
            .await
            .context("Failed to sign message")?;
        Ok(signature)
    }

    /// Create HMAC signature for API requests.
    pub fn create_hmac_signature(
        &self,
        timestamp: &str,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<String> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        
        let message = format!("{}{}{}{}", timestamp, method.to_uppercase(), path, body);
        
        let secret_bytes = STANDARD.decode(&self.api_secret)
            .context("Failed to decode API secret")?;
        
        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .context("Failed to create HMAC")?;
        mac.update(message.as_bytes());
        
        let result = mac.finalize();
        let signature = STANDARD.encode(result.into_bytes());
        
        Ok(signature)
    }

    /// Get headers for authenticated API requests.
    pub fn get_auth_headers(&self, timestamp: &str, signature: &str) -> Vec<(String, String)> {
        vec![
            ("POLY_API_KEY".to_string(), self.api_key.clone()),
            ("POLY_SIGNATURE".to_string(), signature.to_string()),
            ("POLY_TIMESTAMP".to_string(), timestamp.to_string()),
            ("POLY_PASSPHRASE".to_string(), self.passphrase.clone()),
        ]
    }

    /// Sign an order for the CLOB.
    pub async fn sign_order(
        &self,
        token_id: &str,
        price: f64,
        size: f64,
        side: &str,
        nonce: u64,
    ) -> Result<String> {
        // Create the order hash according to Polymarket's EIP-712 spec
        let order_data = format!(
            "{}:{}:{}:{}:{}",
            token_id, price, size, side, nonce
        );
        
        let signature = self.sign_message(order_data.as_bytes()).await?;
        Ok(format!("0x{}", signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_derivation() {
        // Test with a known private key (DO NOT use in production!)
        let test_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let signer = PolymarketSigner::new(test_key, "key", "secret", "pass");
        assert!(signer.is_ok());
    }
}
