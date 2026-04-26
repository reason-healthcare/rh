use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub(super) struct StoreState<K, V> {
    data: HashMap<K, V>,
    max_entries: usize,
}

#[derive(Debug, Default)]
pub(super) struct InsertOutcome {
    pub(super) evicted: bool,
}

#[derive(Debug)]
pub(super) struct GetOrInsertOutcome<V> {
    pub(super) value: V,
    pub(super) inserted: bool,
    pub(super) evicted: bool,
}

impl<K, V> StoreState<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub(super) fn new(max_entries: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_entries,
        }
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> InsertOutcome {
        let evicted = if self.data.contains_key(&key) {
            false
        } else {
            self.evict_if_at_capacity()
        };
        self.data.insert(key, value);
        InsertOutcome { evicted }
    }

    pub(super) fn get_cloned(&self, key: &K) -> Option<V> {
        self.data.get(key).cloned()
    }

    pub(super) fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub(super) fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    pub(super) fn clear(&mut self) {
        self.data.clear();
    }

    pub(super) fn len(&self) -> usize {
        self.data.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub(super) fn keys(&self) -> Vec<K> {
        self.data.keys().cloned().collect()
    }

    pub(super) fn get_or_insert_with<F>(&mut self, key: K, factory: F) -> GetOrInsertOutcome<V>
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.data.get(&key) {
            return GetOrInsertOutcome {
                value: value.clone(),
                inserted: false,
                evicted: false,
            };
        }

        let value = factory();
        let evicted = self.evict_if_at_capacity();
        self.data.insert(key, value.clone());

        GetOrInsertOutcome {
            value,
            inserted: true,
            evicted,
        }
    }

    fn evict_if_at_capacity(&mut self) -> bool {
        if self.max_entries == 0 || self.data.len() < self.max_entries {
            return false;
        }

        if let Some(first_key) = self.data.keys().next().cloned() {
            self.data.remove(&first_key);
            return true;
        }

        false
    }
}
