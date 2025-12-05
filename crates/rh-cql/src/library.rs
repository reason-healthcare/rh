//! Library source providers and compiled library management.
//!
//! This module provides infrastructure for resolving CQL library dependencies:
//!
//! - [`LibrarySourceProvider`] trait for locating and loading CQL source code
//! - [`MemoryLibrarySourceProvider`] for in-memory storage (WASM-compatible)
//! - [`FileLibrarySourceProvider`] for filesystem-based loading (non-WASM)
//!
//! # Example
//!
//! ```
//! use rh_cql::library::{LibrarySourceProvider, MemoryLibrarySourceProvider, LibraryIdentifier};
//!
//! // Create a memory provider
//! let provider = MemoryLibrarySourceProvider::new();
//!
//! // Register a library
//! let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
//! provider.register_source(id.clone(), r#"
//!     library FHIRHelpers version '4.0.1'
//!     using FHIR version '4.0.1'
//!     define function ToQuantity(value FHIR.Quantity): System.Quantity { ... }
//! "#.to_string());
//!
//! // Look up the library
//! let source = provider.get_source(&id);
//! assert!(source.is_some());
//! ```

use std::path::{Path, PathBuf};

use rh_foundation::{MemoryStore, MemoryStoreConfig};

/// Identifier for a CQL library (name + optional version).
///
/// Used as a key for library lookup and dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryIdentifier {
    /// The library name/path.
    pub name: String,
    /// The library version (optional).
    pub version: Option<String>,
}

impl LibraryIdentifier {
    /// Create a new library identifier.
    pub fn new(name: impl Into<String>, version: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            version: version.map(|v| v.into()),
        }
    }

    /// Create an identifier without a version.
    pub fn unversioned(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    /// Create a key string for storage.
    pub fn to_key(&self) -> String {
        match &self.version {
            Some(v) => format!("{}|{}", self.name, v),
            None => self.name.clone(),
        }
    }

    /// Parse a key string back into an identifier.
    pub fn from_key(key: &str) -> Self {
        match key.split_once('|') {
            Some((name, version)) => Self::new(name, Some(version)),
            None => Self::unversioned(key),
        }
    }

    /// Check if this identifier matches another, considering version compatibility.
    ///
    /// A request without a version matches any version of the same library.
    /// A request with a version only matches that exact version.
    pub fn matches(&self, other: &LibraryIdentifier) -> bool {
        if self.name != other.name {
            return false;
        }

        match (&self.version, &other.version) {
            // Both have versions - must match exactly
            (Some(v1), Some(v2)) => v1 == v2,
            // Request has no version - matches any
            (None, _) => true,
            // Request has version but candidate doesn't - no match
            (Some(_), None) => false,
        }
    }
}

impl std::fmt::Display for LibraryIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            Some(v) => write!(f, "{} version '{}'", self.name, v),
            None => write!(f, "{}", self.name),
        }
    }
}

/// CQL source code with metadata.
#[derive(Debug, Clone)]
pub struct LibrarySource {
    /// The library identifier.
    pub identifier: LibraryIdentifier,
    /// The CQL source code.
    pub source: String,
    /// Optional source location (file path or URI).
    pub location: Option<String>,
}

impl LibrarySource {
    /// Create a new library source.
    pub fn new(
        identifier: LibraryIdentifier,
        source: impl Into<String>,
        location: Option<impl Into<String>>,
    ) -> Self {
        Self {
            identifier,
            source: source.into(),
            location: location.map(|l| l.into()),
        }
    }
}

/// A provider for CQL library source code.
///
/// Implementations of this trait provide access to CQL source files
/// for compilation and dependency resolution.
pub trait LibrarySourceProvider: Send + Sync {
    /// Get the source code for a library.
    ///
    /// Returns `Some(source)` if the library is found, `None` otherwise.
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource>;

    /// Check if a library is available.
    fn has_library(&self, identifier: &LibraryIdentifier) -> bool {
        self.get_source(identifier).is_some()
    }

    /// List all available library identifiers.
    fn list_libraries(&self) -> Vec<LibraryIdentifier>;

    /// Find libraries by name (any version).
    fn find_by_name(&self, name: &str) -> Vec<LibraryIdentifier> {
        self.list_libraries()
            .into_iter()
            .filter(|id| id.name == name)
            .collect()
    }
}

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

/// A filesystem-based library source provider.
///
/// This provider loads CQL source files from the filesystem. It supports
/// configurable search paths and file extensions.
///
/// **Note**: This provider is not available in WASM environments.
///
/// # Example
///
/// ```no_run
/// use rh_cql::library::{FileLibrarySourceProvider, LibraryIdentifier, LibrarySourceProvider};
///
/// let provider = FileLibrarySourceProvider::new()
///     .with_path("./cql")
///     .with_path("./libs");
///
/// // Will search for Common.cql or Common-1.0.0.cql in ./cql and ./libs
/// let id = LibraryIdentifier::new("Common", Some("1.0.0"));
/// let source = provider.get_source(&id);
/// ```
#[derive(Debug, Clone)]
pub struct FileLibrarySourceProvider {
    /// Search paths for CQL files.
    paths: Vec<PathBuf>,
    /// File extension to search for (default: "cql").
    extension: String,
    /// Cache of loaded sources.
    cache: MemoryStore<LibrarySource>,
}

impl Default for FileLibrarySourceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileLibrarySourceProvider {
    /// Create a new provider with no search paths.
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            extension: "cql".to_string(),
            cache: MemoryStore::new(MemoryStoreConfig::default()),
        }
    }

    /// Add a search path.
    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Add multiple search paths.
    pub fn with_paths(mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        for path in paths {
            self.paths.push(path.as_ref().to_path_buf());
        }
        self
    }

    /// Set the file extension to search for.
    pub fn with_extension(mut self, ext: impl Into<String>) -> Self {
        self.extension = ext.into();
        self
    }

    /// Get the configured search paths.
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Generate possible filenames for a library identifier.
    ///
    /// This is useful for understanding how libraries are resolved from the filesystem.
    pub fn possible_filenames(&self, identifier: &LibraryIdentifier) -> Vec<String> {
        let mut names = Vec::new();

        // Try versioned filename first: LibraryName-version.cql
        if let Some(version) = &identifier.version {
            names.push(format!(
                "{}-{}.{}",
                identifier.name, version, self.extension
            ));
        }

        // Then try unversioned: LibraryName.cql
        names.push(format!("{}.{}", identifier.name, self.extension));

        names
    }

    /// Find and load a library file.
    fn load_from_disk(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        let filenames = self.possible_filenames(identifier);

        for search_path in &self.paths {
            for filename in &filenames {
                let file_path = search_path.join(filename);
                if file_path.exists() {
                    match std::fs::read_to_string(&file_path) {
                        Ok(content) => {
                            let location = file_path.to_string_lossy().to_string();
                            return Some(LibrarySource::new(
                                identifier.clone(),
                                content,
                                Some(location),
                            ));
                        }
                        Err(_) => continue,
                    }
                }
            }
        }

        None
    }
}

impl LibrarySourceProvider for FileLibrarySourceProvider {
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        // Check cache first
        let key = identifier.to_key();
        if let Some(source) = self.cache.get(&key) {
            return Some(source);
        }

        // Load from disk
        if let Some(source) = self.load_from_disk(identifier) {
            // Cache it
            self.cache.insert(key, source.clone());
            return Some(source);
        }

        None
    }

    fn has_library(&self, identifier: &LibraryIdentifier) -> bool {
        // Check cache
        if self.cache.contains(&identifier.to_key()) {
            return true;
        }

        // Check filesystem
        let filenames = self.possible_filenames(identifier);
        for search_path in &self.paths {
            for filename in &filenames {
                if search_path.join(filename).exists() {
                    return true;
                }
            }
        }

        false
    }

    fn list_libraries(&self) -> Vec<LibraryIdentifier> {
        let mut libraries = Vec::new();

        for search_path in &self.paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == self.extension.as_str() {
                                if let Some(stem) = path.file_stem() {
                                    let name = stem.to_string_lossy().to_string();
                                    // Try to extract version from filename (Name-version.cql)
                                    let (lib_name, version) = if let Some((n, v)) =
                                        name.rsplit_once('-')
                                    {
                                        // Check if the part after - looks like a version
                                        if v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                                            (n.to_string(), Some(v.to_string()))
                                        } else {
                                            (name, None)
                                        }
                                    } else {
                                        (name, None)
                                    };
                                    libraries.push(LibraryIdentifier::new(lib_name, version));
                                }
                            }
                        }
                    }
                }
            }
        }

        libraries
    }
}

/// A composite provider that searches multiple providers in order.
///
/// Useful for layering providers, e.g., check in-memory first, then filesystem.
#[derive(Default)]
pub struct CompositeLibrarySourceProvider {
    providers: Vec<Box<dyn LibrarySourceProvider>>,
}

impl CompositeLibrarySourceProvider {
    /// Create a new empty composite provider.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Add a provider to the chain.
    pub fn add_provider(mut self, provider: impl LibrarySourceProvider + 'static) -> Self {
        self.providers.push(Box::new(provider));
        self
    }
}

impl LibrarySourceProvider for CompositeLibrarySourceProvider {
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        for provider in &self.providers {
            if let Some(source) = provider.get_source(identifier) {
                return Some(source);
            }
        }
        None
    }

    fn has_library(&self, identifier: &LibraryIdentifier) -> bool {
        self.providers.iter().any(|p| p.has_library(identifier))
    }

    fn list_libraries(&self) -> Vec<LibraryIdentifier> {
        let mut libraries = Vec::new();
        for provider in &self.providers {
            for id in provider.list_libraries() {
                if !libraries.contains(&id) {
                    libraries.push(id);
                }
            }
        }
        libraries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // LibraryIdentifier tests
    // ===========================================

    #[test]
    fn test_library_identifier_new() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(id.name, "FHIRHelpers");
        assert_eq!(id.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_library_identifier_unversioned() {
        let id = LibraryIdentifier::unversioned("Common");
        assert_eq!(id.name, "Common");
        assert_eq!(id.version, None);
    }

    #[test]
    fn test_library_identifier_key() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(id.to_key(), "FHIRHelpers|4.0.1");
        assert_eq!(LibraryIdentifier::from_key("FHIRHelpers|4.0.1"), id);

        let id2 = LibraryIdentifier::unversioned("Common");
        assert_eq!(id2.to_key(), "Common");
        assert_eq!(LibraryIdentifier::from_key("Common"), id2);
    }

    #[test]
    fn test_library_identifier_key_with_special_chars() {
        // Version can have dots, dashes, etc.
        let id = LibraryIdentifier::new("MyLib", Some("1.2.3-beta.1"));
        assert_eq!(id.to_key(), "MyLib|1.2.3-beta.1");
        let parsed = LibraryIdentifier::from_key("MyLib|1.2.3-beta.1");
        assert_eq!(parsed.name, "MyLib");
        assert_eq!(parsed.version, Some("1.2.3-beta.1".to_string()));
    }

    #[test]
    fn test_library_identifier_display() {
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert_eq!(format!("{id}"), "FHIRHelpers version '4.0.1'");

        let id2 = LibraryIdentifier::unversioned("Common");
        assert_eq!(format!("{id2}"), "Common");
    }

    #[test]
    fn test_library_identifier_matches() {
        let versioned = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let unversioned = LibraryIdentifier::unversioned("FHIRHelpers");
        let different_version = LibraryIdentifier::new("FHIRHelpers", Some("4.0.0"));
        let different_name = LibraryIdentifier::new("Common", Some("4.0.1"));

        // Unversioned matches any version of same library
        assert!(unversioned.matches(&versioned));
        assert!(unversioned.matches(&different_version));
        assert!(!unversioned.matches(&different_name));

        // Versioned only matches exact version
        assert!(versioned.matches(&versioned));
        assert!(!versioned.matches(&different_version));
        assert!(!versioned.matches(&different_name));

        // Versioned doesn't match unversioned candidate
        assert!(!versioned.matches(&unversioned));
    }

    #[test]
    fn test_library_identifier_equality() {
        let id1 = LibraryIdentifier::new("Test", Some("1.0"));
        let id2 = LibraryIdentifier::new("Test", Some("1.0"));
        let id3 = LibraryIdentifier::new("Test", Some("2.0"));

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_library_identifier_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(LibraryIdentifier::new("A", Some("1.0")));
        set.insert(LibraryIdentifier::new("B", Some("1.0")));
        set.insert(LibraryIdentifier::new("A", Some("1.0"))); // duplicate

        assert_eq!(set.len(), 2);
    }

    // ===========================================
    // LibrarySource tests
    // ===========================================

    #[test]
    fn test_library_source_new() {
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        let source = LibrarySource::new(id.clone(), "library Test", None::<String>);

        assert_eq!(source.identifier, id);
        assert_eq!(source.source, "library Test");
        assert!(source.location.is_none());
    }

    #[test]
    fn test_library_source_with_location() {
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        let source = LibrarySource::new(id, "library Test", Some("/path/to/file.cql"));

        assert_eq!(source.location, Some("/path/to/file.cql".to_string()));
    }

    // ===========================================
    // MemoryLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_memory_provider_new() {
        let provider = MemoryLibrarySourceProvider::new();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_default() {
        let provider = MemoryLibrarySourceProvider::default();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_with_max_libraries() {
        let provider = MemoryLibrarySourceProvider::with_max_libraries(10);
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_with_stats() {
        let provider = MemoryLibrarySourceProvider::with_stats();
        let id = LibraryIdentifier::new("Test", Some("1.0"));
        provider.register_source(id.clone(), "test".to_string());

        let _ = provider.get_source(&id);
        let stats = provider.stats();
        assert!(stats.hits > 0 || stats.insertions > 0);
    }

    #[test]
    fn test_memory_provider_register_and_get() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id.clone(), "library TestLib version '1.0'".to_string());

        assert!(provider.has_library(&id));
        assert_eq!(provider.library_count(), 1);

        let source = provider.get_source(&id).unwrap();
        assert_eq!(source.source, "library TestLib version '1.0'");
        assert!(source.location.is_none());
    }

    #[test]
    fn test_memory_provider_get_unversioned() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id, "library TestLib version '1.0'".to_string());

        // Look up without version
        let unversioned_id = LibraryIdentifier::unversioned("TestLib");
        let source = provider.get_source(&unversioned_id).unwrap();
        assert_eq!(source.source, "library TestLib version '1.0'");
    }

    #[test]
    fn test_memory_provider_get_nonexistent() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("NotFound", Some("1.0"));
        assert!(provider.get_source(&id).is_none());
        assert!(!provider.has_library(&id));
    }

    #[test]
    fn test_memory_provider_with_location() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source_with_location(
            id.clone(),
            "library TestLib version '1.0'".to_string(),
            "/path/to/TestLib-1.0.cql".to_string(),
        );

        let source = provider.get_source(&id).unwrap();
        assert_eq!(
            source.location,
            Some("/path/to/TestLib-1.0.cql".to_string())
        );
    }

    #[test]
    fn test_memory_provider_register_library_source() {
        let provider = MemoryLibrarySourceProvider::new();

        let source = LibrarySource::new(
            LibraryIdentifier::new("Direct", Some("1.0")),
            "library Direct",
            Some("/path/file.cql"),
        );
        provider.register(source);

        let id = LibraryIdentifier::new("Direct", Some("1.0"));
        let retrieved = provider.get_source(&id).unwrap();
        assert_eq!(retrieved.source, "library Direct");
        assert_eq!(retrieved.location, Some("/path/file.cql".to_string()));
    }

    #[test]
    fn test_memory_provider_list_libraries() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(
            LibraryIdentifier::new("LibA", Some("1.0")),
            "library LibA".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("LibB", Some("2.0")),
            "library LibB".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::unversioned("LibC"),
            "library LibC".to_string(),
        );

        let libraries = provider.list_libraries();
        assert_eq!(libraries.len(), 3);
    }

    #[test]
    fn test_memory_provider_find_by_name() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(
            LibraryIdentifier::new("Common", Some("1.0")),
            "library Common v1".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("Common", Some("2.0")),
            "library Common v2".to_string(),
        );
        provider.register_source(
            LibraryIdentifier::new("Other", Some("1.0")),
            "library Other".to_string(),
        );

        let common_libs = provider.find_by_name("Common");
        assert_eq!(common_libs.len(), 2);
        assert!(common_libs.iter().all(|id| id.name == "Common"));

        let other_libs = provider.find_by_name("Other");
        assert_eq!(other_libs.len(), 1);

        let none_libs = provider.find_by_name("NotFound");
        assert_eq!(none_libs.len(), 0);
    }

    #[test]
    fn test_memory_provider_remove() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        provider.register_source(id.clone(), "library TestLib".to_string());
        assert!(provider.has_library(&id));

        let removed = provider.remove(&id);
        assert!(removed.is_some());
        assert!(!provider.has_library(&id));

        // Remove again returns None
        let removed_again = provider.remove(&id);
        assert!(removed_again.is_none());
    }

    #[test]
    fn test_memory_provider_clear() {
        let provider = MemoryLibrarySourceProvider::new();

        provider.register_source(LibraryIdentifier::new("A", Some("1.0")), "a".to_string());
        provider.register_source(LibraryIdentifier::new("B", Some("1.0")), "b".to_string());
        assert_eq!(provider.library_count(), 2);

        provider.clear();
        assert_eq!(provider.library_count(), 0);
    }

    #[test]
    fn test_memory_provider_overwrite() {
        let provider = MemoryLibrarySourceProvider::new();

        let id = LibraryIdentifier::new("Test", Some("1.0"));
        provider.register_source(id.clone(), "original".to_string());
        provider.register_source(id.clone(), "updated".to_string());

        let source = provider.get_source(&id).unwrap();
        assert_eq!(source.source, "updated");
        assert_eq!(provider.library_count(), 1);
    }

    // ===========================================
    // FileLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_file_provider_new() {
        let provider = FileLibrarySourceProvider::new();
        assert!(provider.paths().is_empty());
    }

    #[test]
    fn test_file_provider_default() {
        let provider = FileLibrarySourceProvider::default();
        assert!(provider.paths().is_empty());
    }

    #[test]
    fn test_file_provider_with_path() {
        let provider = FileLibrarySourceProvider::new()
            .with_path("./cql")
            .with_path("./libs");

        assert_eq!(provider.paths().len(), 2);
    }

    #[test]
    fn test_file_provider_with_paths() {
        let provider = FileLibrarySourceProvider::new().with_paths(["./cql", "./libs", "./other"]);

        assert_eq!(provider.paths().len(), 3);
    }

    #[test]
    fn test_file_provider_possible_filenames() {
        let provider = FileLibrarySourceProvider::new();

        let versioned = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let filenames = provider.possible_filenames(&versioned);
        assert_eq!(filenames, vec!["FHIRHelpers-4.0.1.cql", "FHIRHelpers.cql"]);

        let unversioned = LibraryIdentifier::unversioned("Common");
        let filenames = provider.possible_filenames(&unversioned);
        assert_eq!(filenames, vec!["Common.cql"]);
    }

    #[test]
    fn test_file_provider_custom_extension() {
        let provider = FileLibrarySourceProvider::new().with_extension("txt");

        let id = LibraryIdentifier::new("TestLib", Some("1.0"));
        let filenames = provider.possible_filenames(&id);
        assert_eq!(filenames, vec!["TestLib-1.0.txt", "TestLib.txt"]);
    }

    #[test]
    fn test_file_provider_nonexistent_path() {
        let provider = FileLibrarySourceProvider::new().with_path("/nonexistent/path");

        let id = LibraryIdentifier::new("Test", Some("1.0"));
        assert!(!provider.has_library(&id));
        assert!(provider.get_source(&id).is_none());
    }

    // ===========================================
    // CompositeLibrarySourceProvider tests
    // ===========================================

    #[test]
    fn test_composite_provider_new() {
        let composite = CompositeLibrarySourceProvider::new();
        assert!(composite.list_libraries().is_empty());
    }

    #[test]
    fn test_composite_provider_default() {
        let composite = CompositeLibrarySourceProvider::default();
        assert!(composite.list_libraries().is_empty());
    }

    #[test]
    fn test_composite_provider_single() {
        let memory = MemoryLibrarySourceProvider::new();
        memory.register_source(
            LibraryIdentifier::new("MemoryLib", Some("1.0")),
            "from memory".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new().add_provider(memory);

        let id = LibraryIdentifier::new("MemoryLib", Some("1.0"));
        let source = composite.get_source(&id).unwrap();
        assert_eq!(source.source, "from memory");

        let libraries = composite.list_libraries();
        assert_eq!(libraries.len(), 1);
    }

    #[test]
    fn test_composite_provider_priority() {
        // First provider takes precedence
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from first".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from second".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        let id = LibraryIdentifier::new("Shared", Some("1.0"));
        let source = composite.get_source(&id).unwrap();
        assert_eq!(source.source, "from first");
    }

    #[test]
    fn test_composite_provider_fallback() {
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("OnlyInFirst", Some("1.0")),
            "from first".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("OnlyInSecond", Some("1.0")),
            "from second".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        // Found in first
        let id1 = LibraryIdentifier::new("OnlyInFirst", Some("1.0"));
        assert!(composite.has_library(&id1));

        // Found in second (fallback)
        let id2 = LibraryIdentifier::new("OnlyInSecond", Some("1.0"));
        assert!(composite.has_library(&id2));

        // Not found anywhere
        let id3 = LibraryIdentifier::new("NotFound", Some("1.0"));
        assert!(!composite.has_library(&id3));
    }

    #[test]
    fn test_composite_provider_list_deduplicates() {
        let memory1 = MemoryLibrarySourceProvider::new();
        memory1.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from first".to_string(),
        );
        memory1.register_source(
            LibraryIdentifier::new("OnlyFirst", Some("1.0")),
            "first only".to_string(),
        );

        let memory2 = MemoryLibrarySourceProvider::new();
        memory2.register_source(
            LibraryIdentifier::new("Shared", Some("1.0")),
            "from second".to_string(),
        );
        memory2.register_source(
            LibraryIdentifier::new("OnlySecond", Some("1.0")),
            "second only".to_string(),
        );

        let composite = CompositeLibrarySourceProvider::new()
            .add_provider(memory1)
            .add_provider(memory2);

        let libraries = composite.list_libraries();
        // Should have 3 unique: Shared, OnlyFirst, OnlySecond
        assert_eq!(libraries.len(), 3);
    }
}
