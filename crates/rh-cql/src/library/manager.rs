//! Library resolution, caching, and dependency management.
//!
//! Provides [`LibraryError`], [`LibraryResult`], and [`LibraryManager`] for
//! resolving CQL library dependencies with cycle detection and caching.

use rh_foundation::{MemoryStore, MemoryStoreConfig, MemoryStoreStats};

use crate::elm::{Library, VersionedIdentifier};

use super::compiled::CompiledLibrary;
use super::identifiers::LibraryIdentifier;
use super::providers::LibrarySourceProvider;
use super::sources::LibrarySource;

// =============================================================================
// Error and result types
// =============================================================================

/// Error type for library resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryError {
    /// Library not found.
    NotFound(LibraryIdentifier),
    /// Circular dependency detected.
    CircularDependency(Vec<LibraryIdentifier>),
    /// Parse error.
    ParseError {
        library: LibraryIdentifier,
        message: String,
    },
    /// Compilation error.
    CompileError {
        library: LibraryIdentifier,
        message: String,
    },
    /// Version conflict.
    VersionConflict {
        library: String,
        requested: Option<String>,
        found: Option<String>,
    },
}

impl std::fmt::Display for LibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibraryError::NotFound(id) => {
                write!(f, "Library not found: {id}")
            }
            LibraryError::CircularDependency(cycle) => {
                let path: Vec<_> = cycle.iter().map(|id| id.to_string()).collect();
                write!(f, "Circular dependency detected: {}", path.join(" -> "))
            }
            LibraryError::ParseError { library, message } => {
                write!(f, "Parse error in library {library}: {message}")
            }
            LibraryError::CompileError { library, message } => {
                write!(f, "Compile error in library {library}: {message}")
            }
            LibraryError::VersionConflict {
                library,
                requested,
                found,
            } => {
                write!(
                    f,
                    "Version conflict for library '{library}': requested {requested:?}, found {found:?}",
                )
            }
        }
    }
}

impl std::error::Error for LibraryError {}

/// Result type for library operations.
pub type LibraryResult<T> = Result<T, LibraryError>;

// =============================================================================
// LibraryManager
// =============================================================================

/// Manages library resolution, caching, and dependency tracking.
///
/// The `LibraryManager` provides:
/// - Library caching to avoid recompilation
/// - Dependency resolution with cycle detection
/// - Support for multiple library source providers
///
/// # Example
///
/// ```
/// use rh_cql::library::{LibraryManager, MemoryLibrarySourceProvider, LibraryIdentifier};
///
/// // Create a source provider with library source code
/// let provider = MemoryLibrarySourceProvider::new();
/// provider.register_source(
///     LibraryIdentifier::new("FHIRHelpers", Some("4.0.1")),
///     "library FHIRHelpers version '4.0.1'".to_string(),
/// );
///
/// // Create library manager with the provider
/// let manager = LibraryManager::new(provider);
///
/// // Check if library is available
/// assert!(manager.has_source(&LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"))));
/// ```
pub struct LibraryManager<P: LibrarySourceProvider> {
    /// The source provider for loading library source code.
    source_provider: P,
    /// Cache of compiled libraries.
    cache: MemoryStore<CompiledLibrary, String>,
}

impl<P: LibrarySourceProvider> LibraryManager<P> {
    /// Create a new library manager with the given source provider.
    pub fn new(source_provider: P) -> Self {
        Self {
            source_provider,
            cache: MemoryStore::new(MemoryStoreConfig::default()),
        }
    }

    /// Create a new library manager with custom cache configuration.
    pub fn with_cache_config(source_provider: P, config: MemoryStoreConfig) -> Self {
        Self {
            source_provider,
            cache: MemoryStore::new(config),
        }
    }

    /// Get the source provider.
    pub fn source_provider(&self) -> &P {
        &self.source_provider
    }

    /// Check if a library source is available.
    pub fn has_source(&self, id: &LibraryIdentifier) -> bool {
        self.source_provider.has_library(id)
    }

    /// Get the source for a library.
    pub fn get_source(&self, id: &LibraryIdentifier) -> Option<LibrarySource> {
        self.source_provider.get_source(id)
    }

    /// Check if a compiled library is in the cache.
    pub fn is_cached(&self, id: &LibraryIdentifier) -> bool {
        self.cache.contains(&id.to_key())
    }

    /// Get a compiled library from the cache.
    pub fn get_cached(&self, id: &LibraryIdentifier) -> Option<CompiledLibrary> {
        self.cache.get(&id.to_key())
    }

    /// Store a compiled library in the cache.
    pub fn cache_library(&self, id: &LibraryIdentifier, library: CompiledLibrary) {
        self.cache.insert(id.to_key(), library);
    }

    /// Remove a library from the cache.
    pub fn invalidate(&self, id: &LibraryIdentifier) {
        self.cache.remove(&id.to_key());
    }

    /// Clear all cached libraries.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Get cache statistics.
    pub fn cache_stats(&self) -> MemoryStoreStats {
        self.cache.stats()
    }

    /// List all cached library identifiers.
    pub fn cached_libraries(&self) -> Vec<LibraryIdentifier> {
        self.cache
            .keys()
            .into_iter()
            .map(|k| LibraryIdentifier::from_key(&k))
            .collect()
    }

    /// Resolve a library and its dependencies.
    ///
    /// This method:
    /// 1. Checks the cache first
    /// 2. If not cached, loads the source
    /// 3. Detects circular dependencies
    /// 4. Returns the resolved library
    ///
    /// Note: This is a placeholder that returns the cached library or an error.
    pub fn resolve(&self, id: &LibraryIdentifier) -> LibraryResult<CompiledLibrary> {
        self.resolve_with_stack(id, &mut Vec::new())
    }

    /// Resolve a library with dependency tracking.
    fn resolve_with_stack(
        &self,
        id: &LibraryIdentifier,
        resolution_stack: &mut Vec<LibraryIdentifier>,
    ) -> LibraryResult<CompiledLibrary> {
        // Check for circular dependency
        if resolution_stack.iter().any(|r| r.matches(id)) {
            let mut cycle = resolution_stack.clone();
            cycle.push(id.clone());
            return Err(LibraryError::CircularDependency(cycle));
        }

        // Check cache first
        if let Some(cached) = self.get_cached(id) {
            return Ok(cached);
        }

        // Get source
        let source = self
            .source_provider
            .get_source(id)
            .ok_or_else(|| LibraryError::NotFound(id.clone()))?;

        // Track resolution for cycle detection
        resolution_stack.push(id.clone());

        // For now, create a placeholder compiled library.
        // Full parsing/compilation will be implemented in Phase 4.
        let library = Library {
            identifier: Some(VersionedIdentifier {
                id: Some(id.name.clone()),
                version: id.version.clone(),
                system: None,
            }),
            ..Default::default()
        };

        let compiled = match &source.location {
            Some(loc) => CompiledLibrary::with_source_location(library, loc.clone()),
            None => CompiledLibrary::new(library),
        };

        // Cache the result
        self.cache_library(id, compiled.clone());

        // Pop from resolution stack
        resolution_stack.pop();

        Ok(compiled)
    }

    /// Resolve multiple libraries.
    pub fn resolve_all(&self, ids: &[LibraryIdentifier]) -> LibraryResult<Vec<CompiledLibrary>> {
        let mut resolution_stack = Vec::new();
        let mut results = Vec::with_capacity(ids.len());

        for id in ids {
            results.push(self.resolve_with_stack(id, &mut resolution_stack)?);
        }

        Ok(results)
    }

    /// Check if a library has a circular dependency.
    ///
    /// Returns the cycle path if one exists.
    pub fn detect_cycle(&self, id: &LibraryIdentifier) -> Option<Vec<LibraryIdentifier>> {
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        self.detect_cycle_dfs(id, &mut visited, &mut path)
    }

    fn detect_cycle_dfs(
        &self,
        id: &LibraryIdentifier,
        visited: &mut std::collections::HashSet<String>,
        path: &mut Vec<LibraryIdentifier>,
    ) -> Option<Vec<LibraryIdentifier>> {
        let key = id.to_key();

        // Check if we've completed this node (no cycle through it)
        if visited.contains(&key) {
            return None;
        }

        // Check if we're currently visiting this node (cycle!)
        if let Some(pos) = path.iter().position(|p| p.matches(id)) {
            let mut cycle = path[pos..].to_vec();
            cycle.push(id.clone());
            return Some(cycle);
        }

        // Add to current path
        path.push(id.clone());

        // Get the library to check its dependencies
        if let Some(cached) = self.get_cached(id) {
            for dep_id in cached.include_identifiers() {
                if let Some(cycle) = self.detect_cycle_dfs(&dep_id, visited, path) {
                    return Some(cycle);
                }
            }
        }

        // Remove from current path, mark as visited
        path.pop();
        visited.insert(key);

        None
    }

    /// Get the dependency graph for a library.
    ///
    /// Returns a map of library identifiers to their direct dependencies.
    pub fn dependency_graph(
        &self,
        id: &LibraryIdentifier,
    ) -> std::collections::HashMap<LibraryIdentifier, Vec<LibraryIdentifier>> {
        let mut graph = std::collections::HashMap::new();
        let mut visited = std::collections::HashSet::new();
        self.build_dependency_graph(id, &mut graph, &mut visited);
        graph
    }

    fn build_dependency_graph(
        &self,
        id: &LibraryIdentifier,
        graph: &mut std::collections::HashMap<LibraryIdentifier, Vec<LibraryIdentifier>>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        let key = id.to_key();
        if visited.contains(&key) {
            return;
        }
        visited.insert(key);

        if let Some(cached) = self.get_cached(id) {
            let deps = cached.include_identifiers();
            for dep in &deps {
                self.build_dependency_graph(dep, graph, visited);
            }
            graph.insert(id.clone(), deps);
        } else {
            graph.insert(id.clone(), Vec::new());
        }
    }

    /// Get topologically sorted dependencies for a library.
    ///
    /// Returns dependencies in order such that each library appears
    /// after all its dependencies (suitable for compilation order).
    pub fn topological_sort(
        &self,
        id: &LibraryIdentifier,
    ) -> LibraryResult<Vec<LibraryIdentifier>> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();

        self.topological_sort_dfs(id, &mut result, &mut visited, &mut path)?;

        Ok(result)
    }

    fn topological_sort_dfs(
        &self,
        id: &LibraryIdentifier,
        result: &mut Vec<LibraryIdentifier>,
        visited: &mut std::collections::HashSet<String>,
        path: &mut Vec<LibraryIdentifier>,
    ) -> LibraryResult<()> {
        let key = id.to_key();

        // Already processed
        if visited.contains(&key) {
            return Ok(());
        }

        // Cycle detection
        if path.iter().any(|p| p.matches(id)) {
            let mut cycle = path.clone();
            cycle.push(id.clone());
            return Err(LibraryError::CircularDependency(cycle));
        }

        path.push(id.clone());

        // Process dependencies first
        if let Some(cached) = self.get_cached(id) {
            for dep in cached.include_identifiers() {
                self.topological_sort_dfs(&dep, result, visited, path)?;
            }
        }

        path.pop();
        visited.insert(key);
        result.push(id.clone());

        Ok(())
    }
}

impl<P: LibrarySourceProvider + Default> Default for LibraryManager<P> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: LibrarySourceProvider + std::fmt::Debug> std::fmt::Debug for LibraryManager<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryManager")
            .field("source_provider", &self.source_provider)
            .field("cache_size", &self.cache.len())
            .finish()
    }
}
