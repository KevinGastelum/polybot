//! Cache module.
//!
//! Basic in-memory cache for market data.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct CacheItem<T> {
    data: T,
    expiry: Instant,
}

/// Simple TTL cache.
pub struct Cache<T> {
    items: Mutex<HashMap<String, CacheItem<T>>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    /// Create a new cache with specific TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    /// Get item from cache if not expired.
    pub fn get(&self, key: &str) -> Option<T> {
        let mut items = self.items.lock().unwrap();
        if let Some(item) = items.get(key) {
            if item.expiry > Instant::now() {
                return Some(item.data.clone());
            } else {
                items.remove(key);
            }
        }
        None
    }

    /// Insert item into cache.
    pub fn set(&self, key: &str, data: T) {
        let mut items = self.items.lock().unwrap();
        items.insert(key.to_string(), CacheItem {
            data,
            expiry: Instant::now() + self.ttl,
        });
    }
}
