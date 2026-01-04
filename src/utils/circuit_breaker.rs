//! Circuit breaker module.
//!
//! Automatically halts trading if certain conditions (e.g., error rate, large losses) are met.

use std::sync::atomic::{AtomicBool, Ordering};
use tracing::warn;

/// Circuit breaker state.
pub struct CircuitBreaker {
    /// Whether the breaker is tripped (true = halted)
    tripped: AtomicBool,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new() -> Self {
        Self {
            tripped: AtomicBool::new(false),
        }
    }

    /// Trip the breaker, halting all trades.
    pub fn trip(&self, reason: &str) {
        if !self.tripped.swap(true, Ordering::SeqCst) {
            warn!("🛑 CIRCUIT BREAKER TRIPPED: {}", reason);
        }
    }

    /// Reset the breaker.
    pub fn reset(&self) {
        self.tripped.store(false, Ordering::SeqCst);
        warn!("🟢 Circuit breaker reset");
    }

    /// Check if trading is allowed.
    pub fn is_allowed(&self) -> bool {
        !self.tripped.load(Ordering::SeqCst)
    }
}
