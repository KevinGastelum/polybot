use anyhow::Result;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod config;
pub mod polymarket;
pub mod kalshi;
pub mod arbitrage;
pub mod utils;
