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
//! let store: MemoryStore<String> = MemoryStore::new(MemoryStoreConfig::default());
//!
//! store.insert("key1".to_string(), "value1".to_string());
//! assert_eq!(store.get(&"key1".to_string()), Some("value1".to_string()));
//! assert!(store.contains(&"key1".to_string()));
//!
//! store.remove(&"key1".to_string());
//! assert!(!store.contains(&"key1".to_string()));
//! ```

mod state;
mod stats;

use std::hash::Hash;
use std::sync::{Arc, RwLock};

use state::StoreState;
pub use stats::MemoryStoreStats;
use stats::StatsRecorder;

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
    state: Arc<RwLock<StoreState<K, V>>>,
    config: MemoryStoreConfig,
    stats: StatsRecorder,
}

impl<V, K> Clone for MemoryStore<V, K>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            config: self.config.clone(),
            stats: self.stats.clone(),
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
            state: Arc::new(RwLock::new(StoreState::new(config.max_entries))),
            stats: StatsRecorder::new(config.track_stats),
            config,
        }
    }

    /// Insert a value into the store.
    ///
    /// If the store has a maximum entry limit and is at capacity,
    /// an arbitrary entry will be evicted to make room.
    pub fn insert(&self, key: K, value: V) {
        let outcome = self.state.write().unwrap().insert(key, value);
        if outcome.evicted {
            self.stats.record_eviction();
        }
        self.stats.record_insertion();
    }

    /// Get a value from the store.
    ///
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    pub fn get(&self, key: &K) -> Option<V> {
        let result = self.state.read().unwrap().get_cloned(key);

        if result.is_some() {
            self.stats.record_hit();
        } else {
            self.stats.record_miss();
        }

        result
    }

    /// Check if a key exists in the store.
    pub fn contains(&self, key: &K) -> bool {
        self.state.read().unwrap().contains(key)
    }

    /// Remove a value from the store.
    ///
    /// Returns the removed value if the key existed.
    pub fn remove(&self, key: &K) -> Option<V> {
        let result = self.state.write().unwrap().remove(key);
        if result.is_some() {
            self.stats.record_removal();
        }
        result
    }

    /// Clear all entries from the store.
    pub fn clear(&self) {
        self.state.write().unwrap().clear();
    }

    /// Get the number of entries in the store.
    pub fn len(&self) -> usize {
        self.state.read().unwrap().len()
    }

    /// Check if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.state.read().unwrap().is_empty()
    }

    /// Get all keys in the store.
    pub fn keys(&self) -> Vec<K> {
        self.state.read().unwrap().keys()
    }

    /// Get statistics for the store.
    ///
    /// Only meaningful if `track_stats` was enabled in the config.
    pub fn stats(&self) -> MemoryStoreStats {
        self.stats.snapshot()
    }

    /// Reset statistics.
    pub fn reset_stats(&self) {
        self.stats.reset();
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
        if let Some(value) = self.state.read().unwrap().get_cloned(&key) {
            self.stats.record_hit();
            return value;
        }

        let outcome = self.state.write().unwrap().get_or_insert_with(key, factory);

        if outcome.inserted {
            self.stats.record_miss();
            if outcome.evicted {
                self.stats.record_eviction();
            }
            self.stats.record_insertion();
        } else {
            self.stats.record_miss();
            self.stats.record_hit();
        }

        outcome.value
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
