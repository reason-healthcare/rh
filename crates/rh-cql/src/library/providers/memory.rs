//! In-memory library source provider.

use rh_foundation::{MemoryStore, MemoryStoreConfig};

use super::LibrarySourceProvider;
use crate::library::identifiers::LibraryIdentifier;
use crate::library::sources::LibrarySource;

/// A memory-based library source provider using `MemoryStore`.
///
/// This provider stores CQL source code in memory and is suitable for WASM
/// environments where filesystem access is not available.
///
/// # Example
///
/// ```
/// use rh_cql::library::{MemoryLibrarySourceProvider, LibraryIdentifier, LibrarySourceProvider};
///
/// let provider = MemoryLibrarySourceProvider::new();
///
/// // Register a library
/// let id = LibraryIdentifier::new("Common", Some("1.0.0"));
/// provider.register_source(id.clone(), "library Common version '1.0.0'".to_string());
///
/// // Retrieve it
/// let source = provider.get_source(&id);
/// assert!(source.is_some());
/// ```
#[derive(Debug, Clone)]
pub struct MemoryLibrarySourceProvider {
    store: MemoryStore<LibrarySource>,
}

impl Default for MemoryLibrarySourceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryLibrarySourceProvider {
    /// Create a new empty provider.
    pub fn new() -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::default()),
        }
    }

    /// Create a provider with a maximum number of cached libraries.
    pub fn with_max_libraries(max: usize) -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::with_max_entries(max)),
        }
    }

    /// Create a provider with statistics tracking enabled.
    pub fn with_stats() -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::default().with_stats()),
        }
    }

    /// Register a library source.
    pub fn register_source(&self, identifier: LibraryIdentifier, source: String) {
        let key = identifier.to_key();
        let lib_source = LibrarySource::new(identifier, source, None::<String>);
        self.store.insert(key, lib_source);
    }

    /// Register a library source with location metadata.
    pub fn register_source_with_location(
        &self,
        identifier: LibraryIdentifier,
        source: String,
        location: String,
    ) {
        let key = identifier.to_key();
        let lib_source = LibrarySource::new(identifier, source, Some(location));
        self.store.insert(key, lib_source);
    }

    /// Register a `LibrarySource` directly.
    pub fn register(&self, source: LibrarySource) {
        let key = source.identifier.to_key();
        self.store.insert(key, source);
    }

    /// Remove a library from the provider.
    pub fn remove(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        self.store.remove(&identifier.to_key())
    }

    /// Clear all libraries from the provider.
    pub fn clear(&self) {
        self.store.clear();
    }

    /// Get the number of registered libraries.
    pub fn library_count(&self) -> usize {
        self.store.len()
    }

    /// Get cache statistics (if tracking is enabled).
    pub fn stats(&self) -> rh_foundation::MemoryStoreStats {
        self.store.stats()
    }
}

impl LibrarySourceProvider for MemoryLibrarySourceProvider {
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        // Try exact match first
        let key = identifier.to_key();
        if let Some(source) = self.store.get(&key) {
            return Some(source);
        }

        // If no version specified, find any matching library
        if identifier.version.is_none() {
            for lib_id in self.list_libraries() {
                if lib_id.name == identifier.name {
                    return self.store.get(&lib_id.to_key());
                }
            }
        }

        None
    }

    fn list_libraries(&self) -> Vec<LibraryIdentifier> {
        self.store
            .keys()
            .iter()
            .map(|k| LibraryIdentifier::from_key(k))
            .collect()
    }
}
