use std::sync::{Arc, RwLock};

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

#[derive(Debug, Clone)]
pub(super) struct StatsRecorder {
    enabled: bool,
    stats: Arc<RwLock<MemoryStoreStats>>,
}

impl StatsRecorder {
    pub(super) fn new(enabled: bool) -> Self {
        Self {
            enabled,
            stats: Arc::new(RwLock::new(MemoryStoreStats::default())),
        }
    }

    pub(super) fn snapshot(&self) -> MemoryStoreStats {
        self.stats.read().unwrap().clone()
    }

    pub(super) fn reset(&self) {
        *self.stats.write().unwrap() = MemoryStoreStats::default();
    }

    pub(super) fn record_hit(&self) {
        self.update(|stats| stats.hits += 1);
    }

    pub(super) fn record_miss(&self) {
        self.update(|stats| stats.misses += 1);
    }

    pub(super) fn record_insertion(&self) {
        self.update(|stats| stats.insertions += 1);
    }

    pub(super) fn record_removal(&self) {
        self.update(|stats| stats.removals += 1);
    }

    pub(super) fn record_eviction(&self) {
        self.update(|stats| stats.evictions += 1);
    }

    fn update(&self, apply: impl FnOnce(&mut MemoryStoreStats)) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.write().unwrap();
        apply(&mut stats);
    }
}
