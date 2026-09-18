//! Filesystem-based library source provider.

use std::path::{Path, PathBuf};

use rh_foundation::{MemoryStore, MemoryStoreConfig};
use serde::Deserialize;

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
/// // Will search for Common-1.0.0.cql in ./cql and ./libs
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

        if identifier.version.is_none() {
            names.push(format!("{}.{}", identifier.name, self.extension));
        }

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

    fn load_precompiled_elm(
        &self,
        identifier: &LibraryIdentifier,
    ) -> Result<Option<crate::elm::Library>, String> {
        let Some(version) = identifier.version.as_deref() else {
            return Ok(None);
        };
        let filename = format!("{}-{}.json", identifier.name, version);
        let candidates: Vec<PathBuf> = self
            .paths
            .iter()
            .map(|path| path.join("elm").join(&filename))
            .filter(|path| path.is_file())
            .collect();
        if candidates.len() > 1 {
            return Err(format!(
                "ambiguous precompiled ELM dependency for {identifier}: {}",
                candidates
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        let Some(path) = candidates.first() else {
            return Ok(None);
        };
        let contents = std::fs::read_to_string(path).map_err(|error| {
            format!(
                "cannot read precompiled ELM dependency {}: {error}",
                path.display()
            )
        })?;
        #[derive(Deserialize)]
        struct ElmDocument {
            library: crate::elm::Library,
        }
        let document: ElmDocument = serde_json::from_str(&contents).map_err(|error| {
            format!(
                "invalid precompiled ELM dependency {}: {error}",
                path.display()
            )
        })?;
        let matched = document
            .library
            .identifier
            .as_ref()
            .and_then(|identity| identity.id.as_deref().zip(identity.version.as_deref()))
            .is_some_and(|(name, actual_version)| {
                name == identifier.name && actual_version == version
            });
        if !matched {
            return Err(format!(
                "precompiled ELM dependency {} does not declare exact identifier {}",
                path.display(),
                identifier
            ));
        }
        Ok(Some(document.library))
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

    fn get_precompiled_elm(
        &self,
        identifier: &LibraryIdentifier,
    ) -> Result<Option<crate::elm::Library>, String> {
        self.load_precompiled_elm(identifier)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{Library, VersionedIdentifier};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rh-cql-{name}-{nonce}"))
    }

    fn write_elm(root: &Path, name: &str, version: &str, declared_name: &str) {
        let path = root.join("elm");
        std::fs::create_dir_all(&path).expect("create elm directory");
        let library = Library {
            identifier: Some(VersionedIdentifier {
                id: Some(declared_name.to_string()),
                version: Some(version.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        std::fs::write(
            path.join(format!("{name}-{version}.json")),
            serde_json::json!({ "library": library }).to_string(),
        )
        .expect("write elm");
    }

    #[test]
    fn precompiled_elm_requires_exact_versioned_identity() {
        let root = temp_dir("precompiled-mismatch");
        write_elm(&root, "Helper", "1.0.0", "Other");
        let provider = FileLibrarySourceProvider::new().with_path(&root);
        let result = provider.get_precompiled_elm(&LibraryIdentifier::new("Helper", Some("1.0.0")));
        assert!(
            matches!(result, Err(message) if message.contains("does not declare exact identifier"))
        );
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn precompiled_elm_is_versioned_only() {
        let root = temp_dir("precompiled-unversioned");
        write_elm(&root, "Helper", "1.0.0", "Helper");
        let provider = FileLibrarySourceProvider::new().with_path(&root);
        assert!(provider
            .get_precompiled_elm(&LibraryIdentifier::unversioned("Helper"))
            .expect("unversioned lookup is not an error")
            .is_none());
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn precompiled_elm_loads_exact_versioned_document() {
        let root = temp_dir("precompiled-valid");
        write_elm(&root, "Helper", "1.0.0", "Helper");
        let provider = FileLibrarySourceProvider::new().with_path(&root);
        let library = provider
            .get_precompiled_elm(&LibraryIdentifier::new("Helper", Some("1.0.0")))
            .expect("valid precompiled dependency")
            .expect("versioned ELM found");
        assert_eq!(library.identifier.unwrap().id.as_deref(), Some("Helper"));
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn malformed_selected_precompiled_elm_does_not_fall_back_to_source() {
        let root = temp_dir("precompiled-no-fallback");
        std::fs::create_dir_all(&root).expect("create temp directory");
        std::fs::write(
            root.join("Helper-1.0.0.cql"),
            "library Helper version '1.0.0' define Answer: 42",
        )
        .expect("write source fallback candidate");
        write_elm(&root, "Helper", "1.0.0", "Other");
        let provider = FileLibrarySourceProvider::new().with_path(&root);
        let main = "library Main version '1.0.0' include Helper version '1.0.0' called H define X: H.Answer";
        let result = crate::compile_with_libraries(main, None, &provider);
        assert!(
            matches!(result, Err(crate::CompilationError::PrecompiledLibrary(message)) if message.contains("does not declare exact identifier"))
        );
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }

    #[test]
    fn fluent_call_to_precompiled_function_preserves_receiver_alias_and_signature() {
        let root = temp_dir("precompiled-fluent");
        let elm_dir = root.join("elm");
        std::fs::create_dir_all(&elm_dir).expect("create elm directory");
        let named_type = |name: &str| {
            crate::elm::TypeSpecifier::Named(crate::elm::NamedTypeSpecifier {
                name: name.to_string(),
                ..Default::default()
            })
        };
        let helper = Library {
            identifier: Some(VersionedIdentifier {
                id: Some("Helper".into()),
                version: Some("1.0.0".into()),
                ..Default::default()
            }),
            statements: Some(crate::elm::ExpressionDefs {
                defs: vec![crate::elm::StatementDef::Function(
                    crate::elm::FunctionDef {
                        name: Some("references".into()),
                        fluent: Some(true),
                        operand: vec![
                            crate::elm::OperandDef {
                                name: Some("reference".into()),
                                operand_type_specifier: Some(named_type(
                                    "{http://hl7.org/fhir}Reference",
                                )),
                                ..Default::default()
                            },
                            crate::elm::OperandDef {
                                name: Some("resource".into()),
                                operand_type_specifier: Some(named_type(
                                    "{http://hl7.org/fhir}Resource",
                                )),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                )],
            }),
            ..Default::default()
        };
        std::fs::write(
            elm_dir.join("Helper-1.0.0.json"),
            serde_json::json!({ "library": helper }).to_string(),
        )
        .expect("write helper ELM");
        let provider = FileLibrarySourceProvider::new().with_path(&root);
        let main = "library Main version '1.0.0' include Helper version '1.0.0' define X: ref.value.references(resource)";
        let output = crate::compile_with_libraries(main, None, &provider)
            .expect("compile with precompiled helper");
        let defs = &output.result.library.statements.expect("statements").defs;
        let crate::elm::StatementDef::Expression(definition) = &defs[0] else {
            panic!("expected expression definition");
        };
        let Some(expression) = &definition.expression else {
            panic!("expected expression body");
        };
        let crate::elm::Expression::FunctionRef(call) = expression.as_ref() else {
            panic!("expected emitted FunctionRef");
        };
        assert_eq!(call.library_name.as_deref(), Some("Helper"));
        assert_eq!(call.operand.len(), 2);
        assert_eq!(call.signature.len(), 2);
        assert_eq!(
            call.signature[0],
            named_type("{http://hl7.org/fhir}Reference")
        );
        std::fs::remove_dir_all(root).expect("remove temp directory");
    }
}
