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

// =============================================================================
// CompiledLibrary - Wrapper for ELM Library with resolved references
// =============================================================================

use crate::elm::{
    AccessModifier, CodeDef, CodeSystemDef, ConceptDef, ContextDef, ExpressionDef, FunctionDef,
    IncludeDef, Library, OperandDef, ParameterDef, UsingDef, ValueSetDef, VersionedIdentifier,
};

/// A compiled CQL library with convenient lookup methods.
///
/// `CompiledLibrary` wraps an ELM [`Library`] and provides efficient access to
/// its definitions by name. It also tracks the source location and resolved
/// dependencies.
///
/// # Example
///
/// ```
/// use rh_cql::library::{CompiledLibrary, LibraryIdentifier};
/// use rh_cql::elm::Library;
///
/// // Create from an ELM library
/// let elm = Library::default();
/// let compiled = CompiledLibrary::new(elm);
///
/// // Look up expressions by name
/// if let Some(expr_def) = compiled.get_expression("InPopulation") {
///     println!("Found expression: {:?}", expr_def.name);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CompiledLibrary {
    /// The underlying ELM library.
    library: Library,
    /// Source location (file path or URI) if known.
    source_location: Option<String>,
}

impl CompiledLibrary {
    /// Create a new compiled library from an ELM library.
    pub fn new(library: Library) -> Self {
        Self {
            library,
            source_location: None,
        }
    }

    /// Create a compiled library with source location metadata.
    pub fn with_source_location(library: Library, location: impl Into<String>) -> Self {
        Self {
            library,
            source_location: Some(location.into()),
        }
    }

    /// Get the underlying ELM library.
    pub fn library(&self) -> &Library {
        &self.library
    }

    /// Get the source location if known.
    pub fn source_location(&self) -> Option<&str> {
        self.source_location.as_deref()
    }

    /// Get the library identifier.
    pub fn identifier(&self) -> Option<&VersionedIdentifier> {
        self.library.identifier.as_ref()
    }

    /// Get the library name.
    pub fn name(&self) -> Option<&str> {
        self.library
            .identifier
            .as_ref()
            .and_then(|id| id.id.as_deref())
    }

    /// Get the library version.
    pub fn version(&self) -> Option<&str> {
        self.library
            .identifier
            .as_ref()
            .and_then(|id| id.version.as_deref())
    }

    /// Convert to a LibraryIdentifier.
    pub fn to_library_identifier(&self) -> LibraryIdentifier {
        LibraryIdentifier::new(self.name().unwrap_or("unknown"), self.version())
    }

    // =========================================================================
    // Using declarations
    // =========================================================================

    /// Get all using declarations.
    pub fn usings(&self) -> &[UsingDef] {
        self.library
            .usings
            .as_ref()
            .map(|u| u.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a using declaration by local identifier.
    pub fn get_using(&self, local_identifier: &str) -> Option<&UsingDef> {
        self.usings()
            .iter()
            .find(|u| u.local_identifier.as_deref() == Some(local_identifier))
    }

    // =========================================================================
    // Include declarations
    // =========================================================================

    /// Get all include declarations.
    pub fn includes(&self) -> &[IncludeDef] {
        self.library
            .includes
            .as_ref()
            .map(|i| i.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get an include declaration by local identifier.
    pub fn get_include(&self, local_identifier: &str) -> Option<&IncludeDef> {
        self.includes()
            .iter()
            .find(|i| i.local_identifier.as_deref() == Some(local_identifier))
    }

    /// Get the library identifiers for all includes.
    pub fn include_identifiers(&self) -> Vec<LibraryIdentifier> {
        self.includes()
            .iter()
            .filter_map(|inc| {
                inc.path
                    .as_ref()
                    .map(|path| LibraryIdentifier::new(path.clone(), inc.version.clone()))
            })
            .collect()
    }

    // =========================================================================
    // Parameter definitions
    // =========================================================================

    /// Get all parameter definitions.
    pub fn parameters(&self) -> &[ParameterDef] {
        self.library
            .parameters
            .as_ref()
            .map(|p| p.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a parameter definition by name.
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterDef> {
        self.parameters()
            .iter()
            .find(|p| p.name.as_deref() == Some(name))
    }

    /// Get all public parameter definitions.
    pub fn public_parameters(&self) -> Vec<&ParameterDef> {
        self.parameters()
            .iter()
            .filter(|p| p.access_level != Some(AccessModifier::Private))
            .collect()
    }

    // =========================================================================
    // Code system definitions
    // =========================================================================

    /// Get all code system definitions.
    pub fn code_systems(&self) -> &[CodeSystemDef] {
        self.library
            .code_systems
            .as_ref()
            .map(|cs| cs.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a code system definition by name.
    pub fn get_code_system(&self, name: &str) -> Option<&CodeSystemDef> {
        self.code_systems()
            .iter()
            .find(|cs| cs.name.as_deref() == Some(name))
    }

    // =========================================================================
    // Value set definitions
    // =========================================================================

    /// Get all value set definitions.
    pub fn value_sets(&self) -> &[ValueSetDef] {
        self.library
            .value_sets
            .as_ref()
            .map(|vs| vs.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a value set definition by name.
    pub fn get_value_set(&self, name: &str) -> Option<&ValueSetDef> {
        self.value_sets()
            .iter()
            .find(|vs| vs.name.as_deref() == Some(name))
    }

    // =========================================================================
    // Code definitions
    // =========================================================================

    /// Get all code definitions.
    pub fn codes(&self) -> &[CodeDef] {
        self.library
            .codes
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a code definition by name.
    pub fn get_code(&self, name: &str) -> Option<&CodeDef> {
        self.codes()
            .iter()
            .find(|c| c.name.as_deref() == Some(name))
    }

    // =========================================================================
    // Concept definitions
    // =========================================================================

    /// Get all concept definitions.
    pub fn concepts(&self) -> &[ConceptDef] {
        self.library
            .concepts
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a concept definition by name.
    pub fn get_concept(&self, name: &str) -> Option<&ConceptDef> {
        self.concepts()
            .iter()
            .find(|c| c.name.as_deref() == Some(name))
    }

    // =========================================================================
    // Context definitions
    // =========================================================================

    /// Get all context definitions.
    pub fn contexts(&self) -> &[ContextDef] {
        self.library
            .contexts
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a context definition by name.
    pub fn get_context(&self, name: &str) -> Option<&ContextDef> {
        self.contexts()
            .iter()
            .find(|c| c.name.as_deref() == Some(name))
    }

    // =========================================================================
    // Expression definitions
    // =========================================================================

    /// Get all expression definitions (statements).
    pub fn expressions(&self) -> &[ExpressionDef] {
        self.library
            .statements
            .as_ref()
            .map(|s| s.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get an expression definition by name.
    pub fn get_expression(&self, name: &str) -> Option<&ExpressionDef> {
        self.expressions()
            .iter()
            .find(|e| e.name.as_deref() == Some(name))
    }

    /// Get all public expression definitions.
    pub fn public_expressions(&self) -> Vec<&ExpressionDef> {
        self.expressions()
            .iter()
            .filter(|e| e.access_level != Some(AccessModifier::Private))
            .collect()
    }

    /// Get all expression definitions for a specific context.
    pub fn expressions_for_context(&self, context: &str) -> Vec<&ExpressionDef> {
        self.expressions()
            .iter()
            .filter(|e| e.context.as_deref() == Some(context))
            .collect()
    }

    // =========================================================================
    // Function definitions
    // =========================================================================

    /// Get all function definitions.
    ///
    /// Note: In ELM, functions are stored alongside expression definitions
    /// in the statements section, distinguished by having operands.
    /// This method returns definitions that have the structure of functions.
    pub fn functions(&self) -> Vec<FunctionRef<'_>> {
        // In ELM JSON, FunctionDef and ExpressionDef are separate, but both
        // can appear in statements. We need to look at the raw structure.
        // For now, we'll return expression defs that could be functions.
        // A more complete implementation would parse the raw JSON differently.
        Vec::new()
    }

    /// Get a function definition by name.
    ///
    /// If there are multiple overloads, returns the first match.
    /// Use `get_function_by_signature` for specific overload resolution.
    pub fn get_function(&self, name: &str) -> Option<FunctionRef<'_>> {
        // Placeholder - function lookup will be enhanced in Phase 4
        let _ = name;
        None
    }

    /// Get a function definition by name and operand types.
    ///
    /// This performs basic signature matching for function overload resolution.
    pub fn get_function_by_signature(
        &self,
        name: &str,
        operand_types: &[&str],
    ) -> Option<FunctionRef<'_>> {
        // Placeholder - full signature matching requires type system integration
        let _ = (name, operand_types);
        None
    }

    /// Get all functions with a given name (all overloads).
    pub fn get_function_overloads(&self, name: &str) -> Vec<FunctionRef<'_>> {
        let _ = name;
        Vec::new()
    }

    // =========================================================================
    // Definition lookup (any type)
    // =========================================================================

    /// Look up any definition by name.
    ///
    /// Searches expressions, parameters, code systems, value sets, codes,
    /// concepts, and contexts.
    pub fn get_definition(&self, name: &str) -> Option<DefinitionRef<'_>> {
        // Check expressions first (most common)
        if let Some(expr) = self.get_expression(name) {
            return Some(DefinitionRef::Expression(expr));
        }
        if let Some(param) = self.get_parameter(name) {
            return Some(DefinitionRef::Parameter(param));
        }
        if let Some(cs) = self.get_code_system(name) {
            return Some(DefinitionRef::CodeSystem(cs));
        }
        if let Some(vs) = self.get_value_set(name) {
            return Some(DefinitionRef::ValueSet(vs));
        }
        if let Some(code) = self.get_code(name) {
            return Some(DefinitionRef::Code(code));
        }
        if let Some(concept) = self.get_concept(name) {
            return Some(DefinitionRef::Concept(concept));
        }
        if let Some(ctx) = self.get_context(name) {
            return Some(DefinitionRef::Context(ctx));
        }
        None
    }

    /// Check if a definition with the given name exists.
    pub fn has_definition(&self, name: &str) -> bool {
        self.get_definition(name).is_some()
    }

    /// Get all definition names in this library.
    pub fn definition_names(&self) -> Vec<&str> {
        let mut names = Vec::new();

        for expr in self.expressions() {
            if let Some(name) = expr.name.as_deref() {
                names.push(name);
            }
        }
        for param in self.parameters() {
            if let Some(name) = param.name.as_deref() {
                names.push(name);
            }
        }
        for cs in self.code_systems() {
            if let Some(name) = cs.name.as_deref() {
                names.push(name);
            }
        }
        for vs in self.value_sets() {
            if let Some(name) = vs.name.as_deref() {
                names.push(name);
            }
        }
        for code in self.codes() {
            if let Some(name) = code.name.as_deref() {
                names.push(name);
            }
        }
        for concept in self.concepts() {
            if let Some(name) = concept.name.as_deref() {
                names.push(name);
            }
        }

        names
    }
}

impl From<Library> for CompiledLibrary {
    fn from(library: Library) -> Self {
        Self::new(library)
    }
}

/// A reference to a function definition.
///
/// This is a placeholder type that will be expanded when function definitions
/// are properly parsed from ELM JSON.
#[derive(Debug, Clone)]
pub struct FunctionRef<'a> {
    /// Function name.
    pub name: &'a str,
    /// Function operands.
    pub operands: &'a [OperandDef],
    /// The underlying definition (if available).
    pub def: Option<&'a FunctionDef>,
}

/// A reference to any definition in a library.
#[derive(Debug, Clone)]
pub enum DefinitionRef<'a> {
    /// An expression definition.
    Expression(&'a ExpressionDef),
    /// A parameter definition.
    Parameter(&'a ParameterDef),
    /// A code system definition.
    CodeSystem(&'a CodeSystemDef),
    /// A value set definition.
    ValueSet(&'a ValueSetDef),
    /// A code definition.
    Code(&'a CodeDef),
    /// A concept definition.
    Concept(&'a ConceptDef),
    /// A context definition.
    Context(&'a ContextDef),
}

impl<'a> DefinitionRef<'a> {
    /// Get the name of this definition.
    pub fn name(&self) -> Option<&str> {
        match self {
            DefinitionRef::Expression(e) => e.name.as_deref(),
            DefinitionRef::Parameter(p) => p.name.as_deref(),
            DefinitionRef::CodeSystem(cs) => cs.name.as_deref(),
            DefinitionRef::ValueSet(vs) => vs.name.as_deref(),
            DefinitionRef::Code(c) => c.name.as_deref(),
            DefinitionRef::Concept(c) => c.name.as_deref(),
            DefinitionRef::Context(c) => c.name.as_deref(),
        }
    }

    /// Get the access level of this definition.
    pub fn access_level(&self) -> Option<AccessModifier> {
        match self {
            DefinitionRef::Expression(e) => e.access_level.clone(),
            DefinitionRef::Parameter(p) => p.access_level.clone(),
            DefinitionRef::CodeSystem(cs) => cs.access_level.clone(),
            DefinitionRef::ValueSet(vs) => vs.access_level.clone(),
            DefinitionRef::Code(c) => c.access_level.clone(),
            DefinitionRef::Concept(c) => c.access_level.clone(),
            DefinitionRef::Context(_) => None, // Contexts don't have access levels
        }
    }

    /// Check if this definition is public.
    pub fn is_public(&self) -> bool {
        self.access_level() != Some(AccessModifier::Private)
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

    // ===========================================
    // CompiledLibrary tests
    // ===========================================

    use crate::elm::{
        ExpressionDef, ExpressionDefs, IncludeDef, IncludeDefs, ParameterDef, ParameterDefs,
        UsingDef, UsingDefs,
    };

    fn create_test_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TestLibrary".to_string()),
                version: Some("1.0.0".to_string()),
                system: None,
            }),
            usings: Some(UsingDefs {
                defs: vec![UsingDef {
                    local_identifier: Some("FHIR".to_string()),
                    uri: Some("http://hl7.org/fhir".to_string()),
                    version: Some("4.0.1".to_string()),
                }],
            }),
            includes: Some(IncludeDefs {
                defs: vec![IncludeDef {
                    local_identifier: Some("FHIRHelpers".to_string()),
                    path: Some("FHIRHelpers".to_string()),
                    version: Some("4.0.1".to_string()),
                }],
            }),
            parameters: Some(ParameterDefs {
                defs: vec![ParameterDef {
                    name: Some("MeasurementPeriod".to_string()),
                    access_level: Some(AccessModifier::Public),
                    ..Default::default()
                }],
            }),
            statements: Some(ExpressionDefs {
                defs: vec![
                    ExpressionDef {
                        name: Some("InPopulation".to_string()),
                        context: Some("Patient".to_string()),
                        access_level: Some(AccessModifier::Public),
                        ..Default::default()
                    },
                    ExpressionDef {
                        name: Some("PrivateHelper".to_string()),
                        context: Some("Patient".to_string()),
                        access_level: Some(AccessModifier::Private),
                        ..Default::default()
                    },
                    ExpressionDef {
                        name: Some("UnfilteredExpression".to_string()),
                        context: Some("Unfiltered".to_string()),
                        access_level: Some(AccessModifier::Public),
                        ..Default::default()
                    },
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_compiled_library_new() {
        let library = Library::default();
        let compiled = CompiledLibrary::new(library);
        assert!(compiled.source_location().is_none());
    }

    #[test]
    fn test_compiled_library_with_source_location() {
        let library = Library::default();
        let compiled = CompiledLibrary::with_source_location(library, "/path/to/file.cql");
        assert_eq!(compiled.source_location(), Some("/path/to/file.cql"));
    }

    #[test]
    fn test_compiled_library_identifier() {
        let compiled = CompiledLibrary::new(create_test_library());
        assert_eq!(compiled.name(), Some("TestLibrary"));
        assert_eq!(compiled.version(), Some("1.0.0"));

        let lib_id = compiled.to_library_identifier();
        assert_eq!(lib_id.name, "TestLibrary");
        assert_eq!(lib_id.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_compiled_library_usings() {
        let compiled = CompiledLibrary::new(create_test_library());

        let usings = compiled.usings();
        assert_eq!(usings.len(), 1);

        let fhir = compiled.get_using("FHIR");
        assert!(fhir.is_some());
        assert_eq!(fhir.unwrap().uri, Some("http://hl7.org/fhir".to_string()));

        assert!(compiled.get_using("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_includes() {
        let compiled = CompiledLibrary::new(create_test_library());

        let includes = compiled.includes();
        assert_eq!(includes.len(), 1);

        let fhir_helpers = compiled.get_include("FHIRHelpers");
        assert!(fhir_helpers.is_some());

        let include_ids = compiled.include_identifiers();
        assert_eq!(include_ids.len(), 1);
        assert_eq!(include_ids[0].name, "FHIRHelpers");
        assert_eq!(include_ids[0].version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_compiled_library_parameters() {
        let compiled = CompiledLibrary::new(create_test_library());

        let params = compiled.parameters();
        assert_eq!(params.len(), 1);

        let mp = compiled.get_parameter("MeasurementPeriod");
        assert!(mp.is_some());

        assert!(compiled.get_parameter("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_expressions() {
        let compiled = CompiledLibrary::new(create_test_library());

        let exprs = compiled.expressions();
        assert_eq!(exprs.len(), 3);

        let in_pop = compiled.get_expression("InPopulation");
        assert!(in_pop.is_some());
        assert_eq!(in_pop.unwrap().context, Some("Patient".to_string()));

        assert!(compiled.get_expression("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_public_expressions() {
        let compiled = CompiledLibrary::new(create_test_library());

        let public = compiled.public_expressions();
        assert_eq!(public.len(), 2); // InPopulation and UnfilteredExpression
        assert!(public
            .iter()
            .any(|e| e.name.as_deref() == Some("InPopulation")));
        assert!(!public
            .iter()
            .any(|e| e.name.as_deref() == Some("PrivateHelper")));
    }

    #[test]
    fn test_compiled_library_expressions_for_context() {
        let compiled = CompiledLibrary::new(create_test_library());

        let patient_exprs = compiled.expressions_for_context("Patient");
        assert_eq!(patient_exprs.len(), 2);

        let unfiltered_exprs = compiled.expressions_for_context("Unfiltered");
        assert_eq!(unfiltered_exprs.len(), 1);

        let other_exprs = compiled.expressions_for_context("Other");
        assert_eq!(other_exprs.len(), 0);
    }

    #[test]
    fn test_compiled_library_get_definition() {
        let compiled = CompiledLibrary::new(create_test_library());

        // Expression
        let def = compiled.get_definition("InPopulation");
        assert!(matches!(def, Some(DefinitionRef::Expression(_))));

        // Parameter
        let def = compiled.get_definition("MeasurementPeriod");
        assert!(matches!(def, Some(DefinitionRef::Parameter(_))));

        // Not found
        assert!(compiled.get_definition("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_has_definition() {
        let compiled = CompiledLibrary::new(create_test_library());

        assert!(compiled.has_definition("InPopulation"));
        assert!(compiled.has_definition("MeasurementPeriod"));
        assert!(!compiled.has_definition("NonExistent"));
    }

    #[test]
    fn test_compiled_library_definition_names() {
        let compiled = CompiledLibrary::new(create_test_library());

        let names = compiled.definition_names();
        assert!(names.contains(&"InPopulation"));
        assert!(names.contains(&"PrivateHelper"));
        assert!(names.contains(&"MeasurementPeriod"));
    }

    #[test]
    fn test_definition_ref_name() {
        let compiled = CompiledLibrary::new(create_test_library());

        let def = compiled.get_definition("InPopulation").unwrap();
        assert_eq!(def.name(), Some("InPopulation"));
    }

    #[test]
    fn test_definition_ref_access_level() {
        let compiled = CompiledLibrary::new(create_test_library());

        let public_def = compiled.get_definition("InPopulation").unwrap();
        assert!(public_def.is_public());

        let private_def = compiled.get_definition("PrivateHelper").unwrap();
        assert!(!private_def.is_public());
    }

    #[test]
    fn test_compiled_library_from_library() {
        let library = create_test_library();
        let compiled: CompiledLibrary = library.into();
        assert_eq!(compiled.name(), Some("TestLibrary"));
    }

    #[test]
    fn test_compiled_library_empty() {
        let compiled = CompiledLibrary::new(Library::default());

        assert!(compiled.name().is_none());
        assert!(compiled.usings().is_empty());
        assert!(compiled.includes().is_empty());
        assert!(compiled.parameters().is_empty());
        assert!(compiled.expressions().is_empty());
        assert!(compiled.definition_names().is_empty());
    }

    // ===========================================
    // Terminology definition tests
    // ===========================================

    use crate::elm::{
        CodeDef, CodeDefs, CodeSystemDef, CodeSystemDefs, ConceptDef, ConceptDefs, ValueSetDef,
        ValueSetDefs,
    };

    fn create_terminology_library() -> Library {
        Library {
            identifier: Some(VersionedIdentifier {
                id: Some("TerminologyLibrary".to_string()),
                version: Some("1.0.0".to_string()),
                system: None,
            }),
            code_systems: Some(CodeSystemDefs {
                defs: vec![
                    CodeSystemDef {
                        name: Some("LOINC".to_string()),
                        id: Some("http://loinc.org".to_string()),
                        version: Some("2.73".to_string()),
                        access_level: Some(AccessModifier::Public),
                    },
                    CodeSystemDef {
                        name: Some("SNOMED".to_string()),
                        id: Some("http://snomed.info/sct".to_string()),
                        version: Some("2023-09".to_string()),
                        access_level: Some(AccessModifier::Public),
                    },
                    CodeSystemDef {
                        name: Some("InternalCodes".to_string()),
                        id: Some("http://example.org/internal".to_string()),
                        version: None,
                        access_level: Some(AccessModifier::Private),
                    },
                ],
            }),
            value_sets: Some(ValueSetDefs {
                defs: vec![
                    ValueSetDef {
                        name: Some("DiabetesCodes".to_string()),
                        id: Some("http://cts.nlm.nih.gov/fhir/ValueSet/2.16.840.1.113883.3.464.1003.103.12.1001".to_string()),
                        version: Some("20230101".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: vec![],
                    },
                    ValueSetDef {
                        name: Some("PrivateValueSet".to_string()),
                        id: Some("http://example.org/private".to_string()),
                        version: None,
                        access_level: Some(AccessModifier::Private),
                        code_system: vec![],
                    },
                ],
            }),
            codes: Some(CodeDefs {
                defs: vec![
                    CodeDef {
                        name: Some("HbA1c".to_string()),
                        id: Some("4548-4".to_string()),
                        display: Some("Hemoglobin A1c".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: None,
                    },
                    CodeDef {
                        name: Some("GlucoseLevel".to_string()),
                        id: Some("2339-0".to_string()),
                        display: Some("Glucose [Mass/volume] in Blood".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code_system: None,
                    },
                    CodeDef {
                        name: Some("InternalCode".to_string()),
                        id: Some("INT-001".to_string()),
                        display: None,
                        access_level: Some(AccessModifier::Private),
                        code_system: None,
                    },
                ],
            }),
            concepts: Some(ConceptDefs {
                defs: vec![
                    ConceptDef {
                        name: Some("DiabetesLabTests".to_string()),
                        display: Some("Diabetes-related laboratory tests".to_string()),
                        access_level: Some(AccessModifier::Public),
                        code: vec![],
                    },
                    ConceptDef {
                        name: Some("PrivateConcept".to_string()),
                        display: None,
                        access_level: Some(AccessModifier::Private),
                        code: vec![],
                    },
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_compiled_library_code_systems() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let code_systems = compiled.code_systems();
        assert_eq!(code_systems.len(), 3);

        let loinc = compiled.get_code_system("LOINC");
        assert!(loinc.is_some());
        assert_eq!(loinc.unwrap().id, Some("http://loinc.org".to_string()));
        assert_eq!(loinc.unwrap().version, Some("2.73".to_string()));

        let snomed = compiled.get_code_system("SNOMED");
        assert!(snomed.is_some());

        assert!(compiled.get_code_system("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_value_sets() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let value_sets = compiled.value_sets();
        assert_eq!(value_sets.len(), 2);

        let diabetes = compiled.get_value_set("DiabetesCodes");
        assert!(diabetes.is_some());
        assert!(diabetes.unwrap().id.as_ref().unwrap().contains("2.16.840"));

        assert!(compiled.get_value_set("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_codes() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let codes = compiled.codes();
        assert_eq!(codes.len(), 3);

        let hba1c = compiled.get_code("HbA1c");
        assert!(hba1c.is_some());
        assert_eq!(hba1c.unwrap().id, Some("4548-4".to_string()));
        assert_eq!(hba1c.unwrap().display, Some("Hemoglobin A1c".to_string()));

        assert!(compiled.get_code("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_concepts() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let concepts = compiled.concepts();
        assert_eq!(concepts.len(), 2);

        let diabetes_labs = compiled.get_concept("DiabetesLabTests");
        assert!(diabetes_labs.is_some());
        assert_eq!(
            diabetes_labs.unwrap().display,
            Some("Diabetes-related laboratory tests".to_string())
        );

        assert!(compiled.get_concept("NonExistent").is_none());
    }

    #[test]
    fn test_compiled_library_get_definition_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        // CodeSystem
        let def = compiled.get_definition("LOINC");
        assert!(matches!(def, Some(DefinitionRef::CodeSystem(_))));

        // ValueSet
        let def = compiled.get_definition("DiabetesCodes");
        assert!(matches!(def, Some(DefinitionRef::ValueSet(_))));

        // Code
        let def = compiled.get_definition("HbA1c");
        assert!(matches!(def, Some(DefinitionRef::Code(_))));

        // Concept
        let def = compiled.get_definition("DiabetesLabTests");
        assert!(matches!(def, Some(DefinitionRef::Concept(_))));
    }

    #[test]
    fn test_compiled_library_has_definition_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        assert!(compiled.has_definition("LOINC"));
        assert!(compiled.has_definition("DiabetesCodes"));
        assert!(compiled.has_definition("HbA1c"));
        assert!(compiled.has_definition("DiabetesLabTests"));
        assert!(!compiled.has_definition("NonExistent"));
    }

    #[test]
    fn test_compiled_library_definition_names_includes_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let names = compiled.definition_names();
        assert!(names.contains(&"LOINC"));
        assert!(names.contains(&"SNOMED"));
        assert!(names.contains(&"DiabetesCodes"));
        assert!(names.contains(&"HbA1c"));
        assert!(names.contains(&"GlucoseLevel"));
        assert!(names.contains(&"DiabetesLabTests"));
    }

    #[test]
    fn test_definition_ref_access_level_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        // Public code system
        let def = compiled.get_definition("LOINC").unwrap();
        assert!(def.is_public());
        assert_eq!(def.access_level(), Some(AccessModifier::Public));

        // Private code system
        let def = compiled.get_definition("InternalCodes").unwrap();
        assert!(!def.is_public());
        assert_eq!(def.access_level(), Some(AccessModifier::Private));

        // Public value set
        let def = compiled.get_definition("DiabetesCodes").unwrap();
        assert!(def.is_public());

        // Private value set
        let def = compiled.get_definition("PrivateValueSet").unwrap();
        assert!(!def.is_public());

        // Public code
        let def = compiled.get_definition("HbA1c").unwrap();
        assert!(def.is_public());

        // Private code
        let def = compiled.get_definition("InternalCode").unwrap();
        assert!(!def.is_public());

        // Public concept
        let def = compiled.get_definition("DiabetesLabTests").unwrap();
        assert!(def.is_public());

        // Private concept
        let def = compiled.get_definition("PrivateConcept").unwrap();
        assert!(!def.is_public());
    }

    #[test]
    fn test_definition_ref_name_terminology() {
        let compiled = CompiledLibrary::new(create_terminology_library());

        let cs = compiled.get_definition("LOINC").unwrap();
        assert_eq!(cs.name(), Some("LOINC"));

        let vs = compiled.get_definition("DiabetesCodes").unwrap();
        assert_eq!(vs.name(), Some("DiabetesCodes"));

        let code = compiled.get_definition("HbA1c").unwrap();
        assert_eq!(code.name(), Some("HbA1c"));

        let concept = compiled.get_definition("DiabetesLabTests").unwrap();
        assert_eq!(concept.name(), Some("DiabetesLabTests"));
    }

    #[test]
    fn test_compiled_library_empty_terminology() {
        let compiled = CompiledLibrary::new(Library::default());

        assert!(compiled.code_systems().is_empty());
        assert!(compiled.value_sets().is_empty());
        assert!(compiled.codes().is_empty());
        assert!(compiled.concepts().is_empty());
    }

    #[test]
    fn test_compiled_library_mixed_definitions() {
        // Test a library with both expressions/parameters and terminology
        let mut library = create_test_library();
        library.code_systems = Some(CodeSystemDefs {
            defs: vec![CodeSystemDef {
                name: Some("ICD10".to_string()),
                id: Some("http://hl7.org/fhir/sid/icd-10".to_string()),
                version: None,
                access_level: Some(AccessModifier::Public),
            }],
        });

        let compiled = CompiledLibrary::new(library);

        // Check expressions still work
        assert!(compiled.get_expression("InPopulation").is_some());
        assert!(compiled.get_parameter("MeasurementPeriod").is_some());

        // Check code system also works
        assert!(compiled.get_code_system("ICD10").is_some());

        // Check definition_names includes all types
        let names = compiled.definition_names();
        assert!(names.contains(&"InPopulation"));
        assert!(names.contains(&"MeasurementPeriod"));
        assert!(names.contains(&"ICD10"));
    }
}
