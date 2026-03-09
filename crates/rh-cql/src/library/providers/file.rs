//! Filesystem-based library source provider.

use std::path::{Path, PathBuf};

use rh_foundation::{MemoryStore, MemoryStoreConfig};

use super::LibrarySourceProvider;
use crate::library::identifiers::LibraryIdentifier;
use crate::library::sources::LibrarySource;

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
