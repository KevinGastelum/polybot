//! Polymarket integration module.
//!
//! Provides client for interacting with Polymarket's CLOB API and smart contracts.

pub mod client;
pub mod types;
pub mod signer;

pub use client::PolymarketClient;
