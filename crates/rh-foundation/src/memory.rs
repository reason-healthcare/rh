//! In-memory storage utilities for WASM-compatible caching.
//!
//! This module provides a generic `MemoryStore` that can be used to cache
//! data in memory, particularly useful for WASM environments where filesystem
//! access is limited or unavailable.
//!
//! # Example
//!
//! ```
//! use rh_foundation::memory::{MemoryStore, MemoryStoreConfig};
//!
//! // Create a store with default config
//! let store: MemoryStore<String> = MemoryStore::new(MemoryStoreConfig::default());
//!
//! // Insert and retrieve values
//! store.insert("key1".to_string(), "value1".to_string());
//! assert_eq!(store.get(&"key1".to_string()), Some("value1".to_string()));
//!
//! // Check if key exists
//! assert!(store.contains(&"key1".to_string()));
//!
//! // Remove a value
//! store.remove(&"key1".to_string());
//! assert!(!store.contains(&"key1".to_string()));
//! ```

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

/// Configuration for a memory store.
#[derive(Debug, Clone, Default)]
pub struct MemoryStoreConfig {
    /// Maximum number of entries to store (0 = unlimited).
    pub max_entries: usize,
    /// Whether to track access statistics.
    pub track_stats: bool,
}

impl MemoryStoreConfig {
    /// Create a config with a maximum entry limit.
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            max_entries,
            ..Default::default()
        }
    }

    /// Enable statistics tracking.
    pub fn with_stats(mut self) -> Self {
        self.track_stats = true;
        self
    }
}

/// Statistics for a memory store.
#[derive(Debug, Clone, Default)]
pub struct MemoryStoreStats {
    /// Number of cache hits.
    pub hits: u64,
    /// Number of cache misses.
    pub misses: u64,
    /// Number of insertions.
    pub insertions: u64,
    /// Number of removals.
    pub removals: u64,
    /// Number of evictions due to capacity limits.
    pub evictions: u64,
}

impl MemoryStoreStats {
    /// Calculate the hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// A thread-safe in-memory key-value store.
///
/// This store is designed to be WASM-compatible and can be used to cache
/// any type of data that implements `Clone`. It uses interior mutability
/// via `RwLock` to allow concurrent read access.
///
/// # Type Parameters
///
/// - `K`: The key type, must implement `Eq + Hash + Clone`
/// - `V`: The value type, must implement `Clone`
#[derive(Debug)]
pub struct MemoryStore<V, K = String>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    data: Arc<RwLock<HashMap<K, V>>>,
    config: MemoryStoreConfig,
    stats: Arc<RwLock<MemoryStoreStats>>,
}

impl<V, K> Clone for MemoryStore<V, K>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            config: self.config.clone(),
            stats: Arc::clone(&self.stats),
        }
    }
}

impl<V> Default for MemoryStore<V, String>
where
    V: Clone,
{
    fn default() -> Self {
        Self::new(MemoryStoreConfig::default())
    }
}

impl<V, K> MemoryStore<V, K>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new memory store with the given configuration.
    pub fn new(config: MemoryStoreConfig) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(MemoryStoreStats::default())),
        }
    }

    /// Insert a value into the store.
    ///
    /// If the store has a maximum entry limit and is at capacity,
    /// an arbitrary entry will be evicted to make room.
    pub fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().unwrap();

        // Check capacity and evict if necessary
        if self.config.max_entries > 0 && data.len() >= self.config.max_entries {
            // Simple eviction: remove first entry (not LRU, but simple and fast)
            if let Some(first_key) = data.keys().next().cloned() {
                data.remove(&first_key);
                if self.config.track_stats {
                    self.stats.write().unwrap().evictions += 1;
                }
            }
        }

        data.insert(key, value);

        if self.config.track_stats {
            self.stats.write().unwrap().insertions += 1;
        }
    }

    /// Get a value from the store.
    ///
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    pub fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().unwrap();
        let result = data.get(key).cloned();

        if self.config.track_stats {
            let mut stats = self.stats.write().unwrap();
            if result.is_some() {
                stats.hits += 1;
            } else {
                stats.misses += 1;
            }
        }

        result
    }

    /// Check if a key exists in the store.
    pub fn contains(&self, key: &K) -> bool {
        self.data.read().unwrap().contains_key(key)
    }

    /// Remove a value from the store.
    ///
    /// Returns the removed value if the key existed.
    pub fn remove(&self, key: &K) -> Option<V> {
        let result = self.data.write().unwrap().remove(key);

        if self.config.track_stats && result.is_some() {
            self.stats.write().unwrap().removals += 1;
        }

        result
    }

    /// Clear all entries from the store.
    pub fn clear(&self) {
        self.data.write().unwrap().clear();
    }

    /// Get the number of entries in the store.
    pub fn len(&self) -> usize {
        self.data.read().unwrap().len()
    }

    /// Check if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.data.read().unwrap().is_empty()
    }

    /// Get all keys in the store.
    pub fn keys(&self) -> Vec<K> {
        self.data.read().unwrap().keys().cloned().collect()
    }

    /// Get statistics for the store.
    ///
    /// Only meaningful if `track_stats` was enabled in the config.
    pub fn stats(&self) -> MemoryStoreStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics.
    pub fn reset_stats(&self) {
        *self.stats.write().unwrap() = MemoryStoreStats::default();
    }

    /// Get or insert a value using a factory function.
    ///
    /// If the key exists, returns the existing value.
    /// If the key doesn't exist, calls the factory function to create
    /// a value, inserts it, and returns it.
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> V
    where
        F: FnOnce() -> V,
    {
        // Try to get first (read lock only)
        if let Some(value) = self.get(&key) {
            return value;
        }

        // Need to insert (write lock)
        let mut data = self.data.write().unwrap();

        // Double-check after acquiring write lock
        if let Some(value) = data.get(&key) {
            if self.config.track_stats {
                self.stats.write().unwrap().hits += 1;
            }
            return value.clone();
        }

        // Create and insert
        let value = factory();

        // Check capacity and evict if necessary
        if self.config.max_entries > 0 && data.len() >= self.config.max_entries {
            if let Some(first_key) = data.keys().next().cloned() {
                data.remove(&first_key);
                if self.config.track_stats {
                    self.stats.write().unwrap().evictions += 1;
                }
            }
        }

        data.insert(key, value.clone());

        if self.config.track_stats {
            let mut stats = self.stats.write().unwrap();
            stats.misses += 1;
            stats.insertions += 1;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let store: MemoryStore<String> = MemoryStore::default();

        store.insert("key1".to_string(), "value1".to_string());
        assert_eq!(store.get(&"key1".to_string()), Some("value1".to_string()));
        assert!(store.contains(&"key1".to_string()));
        assert_eq!(store.len(), 1);

        store.remove(&"key1".to_string());
        assert!(!store.contains(&"key1".to_string()));
        assert!(store.is_empty());
    }

    #[test]
    fn test_max_entries() {
        let config = MemoryStoreConfig::with_max_entries(2).with_stats();
        let store: MemoryStore<i32> = MemoryStore::new(config);

        store.insert("a".to_string(), 1);
        store.insert("b".to_string(), 2);
        assert_eq!(store.len(), 2);

        // This should evict one entry
        store.insert("c".to_string(), 3);
        assert_eq!(store.len(), 2);
        assert_eq!(store.stats().evictions, 1);
    }

    #[test]
    fn test_stats_tracking() {
        let config = MemoryStoreConfig::default().with_stats();
        let store: MemoryStore<String> = MemoryStore::new(config);

        store.insert("key1".to_string(), "value1".to_string());
        assert_eq!(store.stats().insertions, 1);

        store.get(&"key1".to_string());
        assert_eq!(store.stats().hits, 1);

        store.get(&"nonexistent".to_string());
        assert_eq!(store.stats().misses, 1);

        let rate = store.stats().hit_rate();
        assert!((rate - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_get_or_insert_with() {
        let store: MemoryStore<i32> = MemoryStore::default();

        let value = store.get_or_insert_with("key1".to_string(), || 42);
        assert_eq!(value, 42);

        // Should return cached value, not call factory
        let value = store.get_or_insert_with("key1".to_string(), || 100);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_clear() {
        let store: MemoryStore<String> = MemoryStore::default();

        store.insert("a".to_string(), "1".to_string());
        store.insert("b".to_string(), "2".to_string());
        assert_eq!(store.len(), 2);

        store.clear();
        assert!(store.is_empty());
    }

    #[test]
    fn test_keys() {
        let store: MemoryStore<i32> = MemoryStore::default();

        store.insert("a".to_string(), 1);
        store.insert("b".to_string(), 2);

        let mut keys = store.keys();
        keys.sort();
        assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_clone_shares_data() {
        let store1: MemoryStore<String> = MemoryStore::default();
        store1.insert("key".to_string(), "value".to_string());

        let store2 = store1.clone();
        assert_eq!(store2.get(&"key".to_string()), Some("value".to_string()));

        // Changes in store2 should be visible in store1
        store2.insert("key2".to_string(), "value2".to_string());
        assert_eq!(store1.get(&"key2".to_string()), Some("value2".to_string()));
    }

    #[test]
    fn test_custom_key_type() {
        let store: MemoryStore<String, i32> = MemoryStore::new(MemoryStoreConfig::default());

        store.insert(1, "one".to_string());
        store.insert(2, "two".to_string());

        assert_eq!(store.get(&1), Some("one".to_string()));
        assert_eq!(store.get(&2), Some("two".to_string()));
    }
}
