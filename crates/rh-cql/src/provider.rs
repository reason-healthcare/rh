//! Model information providers for CQL type resolution.
//!
//! This module provides traits and implementations for accessing ModelInfo
//! definitions. The primary implementation uses `MemoryStore` from rh-foundation
//! for WASM-compatible in-memory caching.
//!
//! # Example
//!
//! ```
//! use rh_cql::provider::{ModelInfoProvider, MemoryModelInfoProvider};
//! use rh_cql::modelinfo::{ModelInfo, TypeInfo, ClassInfo};
//!
//! // Create a provider
//! let provider = MemoryModelInfoProvider::new();
//!
//! // Register a model
//! let model = ModelInfo {
//!     name: Some("FHIR".to_string()),
//!     version: Some("4.0.1".to_string()),
//!     ..Default::default()
//! };
//! provider.register_model(model);
//!
//! // Look up the model
//! let found = provider.get_model("FHIR", Some("4.0.1"));
//! assert!(found.is_some());
//! ```

use rh_foundation::{MemoryStore, MemoryStoreConfig};

use crate::modelinfo::{ClassInfo, ModelInfo, TypeInfo};

/// A provider for ModelInfo definitions.
///
/// Implementations of this trait provide access to model information
/// for CQL type resolution and query execution.
pub trait ModelInfoProvider: Send + Sync {
    /// Get a model by name and optional version.
    ///
    /// If version is `None`, returns the latest/default version.
    fn get_model(&self, name: &str, version: Option<&str>) -> Option<ModelInfo>;

    /// Check if a model is registered.
    fn has_model(&self, name: &str, version: Option<&str>) -> bool;

    /// Get all registered model names.
    fn list_models(&self) -> Vec<String>;

    /// Resolve a type by name within a model.
    ///
    /// Returns the `TypeInfo` for the given type name, searching
    /// the specified model (or default model if not specified).
    fn resolve_type(
        &self,
        model_name: &str,
        version: Option<&str>,
        type_name: &str,
    ) -> Option<TypeInfo>;

    /// Resolve a class by name within a model.
    ///
    /// Convenience method that returns only if the type is a `ClassInfo`.
    /// For `ProfileInfo`, converts to a compatible `ClassInfo` structure.
    fn resolve_class(
        &self,
        model_name: &str,
        version: Option<&str>,
        class_name: &str,
    ) -> Option<ClassInfo> {
        self.resolve_type(model_name, version, class_name)
            .and_then(|ti| match ti {
                TypeInfo::ClassInfo(ci) => Some(ci),
                TypeInfo::ProfileInfo(pi) => Some(ClassInfo {
                    namespace: pi.namespace,
                    name: pi.name,
                    identifier: pi.identifier,
                    label: pi.label,
                    base_type: pi.base_type,
                    base_type_specifier: pi.base_type_specifier,
                    retrievable: pi.retrievable,
                    primary_code_path: pi.primary_code_path,
                    element: pi.element,
                    ..Default::default()
                }),
                _ => None,
            })
    }

    /// Get the patient class for a model.
    ///
    /// Returns the class info for the patient type if defined.
    fn get_patient_class(&self, model_name: &str, version: Option<&str>) -> Option<ClassInfo> {
        let model = self.get_model(model_name, version)?;
        let patient_class_name = model.patient_class_name.as_ref()?;
        self.resolve_class(model_name, version, patient_class_name)
    }
}

/// A memory-based ModelInfo provider using `MemoryStore`.
///
/// This provider stores models in memory and is suitable for WASM
/// environments where filesystem access is not available.
#[derive(Debug, Clone)]
pub struct MemoryModelInfoProvider {
    store: MemoryStore<ModelInfo>,
}

impl Default for MemoryModelInfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryModelInfoProvider {
    /// Create a new empty provider.
    pub fn new() -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::default()),
        }
    }

    /// Create a provider with a maximum number of cached models.
    pub fn with_max_models(max_models: usize) -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::with_max_entries(max_models)),
        }
    }

    /// Create a provider with statistics tracking enabled.
    pub fn with_stats() -> Self {
        Self {
            store: MemoryStore::new(MemoryStoreConfig::default().with_stats()),
        }
    }

    /// Register a model with the provider.
    ///
    /// The model is stored using a key derived from its name and version.
    pub fn register_model(&self, model: ModelInfo) {
        let key = Self::make_key(
            model.name.as_deref().unwrap_or("unknown"),
            model.version.as_deref(),
        );
        self.store.insert(key, model);
    }

    /// Register multiple models at once.
    pub fn register_models(&self, models: impl IntoIterator<Item = ModelInfo>) {
        for model in models {
            self.register_model(model);
        }
    }

    /// Remove a model from the provider.
    pub fn remove_model(&self, name: &str, version: Option<&str>) -> Option<ModelInfo> {
        let key = Self::make_key(name, version);
        self.store.remove(&key)
    }

    /// Clear all models from the provider.
    pub fn clear(&self) {
        self.store.clear();
    }

    /// Get the number of registered models.
    pub fn model_count(&self) -> usize {
        self.store.len()
    }

    /// Get cache statistics (if tracking is enabled).
    pub fn stats(&self) -> rh_foundation::MemoryStoreStats {
        self.store.stats()
    }

    fn make_key(name: &str, version: Option<&str>) -> String {
        match version {
            Some(v) => format!("{name}|{v}"),
            None => name.to_string(),
        }
    }

    fn parse_key(key: &str) -> (&str, Option<&str>) {
        match key.split_once('|') {
            Some((name, version)) => (name, Some(version)),
            None => (key, None),
        }
    }
}

impl ModelInfoProvider for MemoryModelInfoProvider {
    fn get_model(&self, name: &str, version: Option<&str>) -> Option<ModelInfo> {
        // Try exact match first
        let key = Self::make_key(name, version);
        if let Some(model) = self.store.get(&key) {
            return Some(model);
        }

        // If version was specified but not found, no fallback
        if version.is_some() {
            return None;
        }

        // Try to find any version of this model
        for stored_key in self.store.keys() {
            let (stored_name, _) = Self::parse_key(&stored_key);
            if stored_name == name {
                return self.store.get(&stored_key);
            }
        }

        None
    }

    fn has_model(&self, name: &str, version: Option<&str>) -> bool {
        self.get_model(name, version).is_some()
    }

    fn list_models(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .store
            .keys()
            .iter()
            .map(|k| Self::parse_key(k).0.to_string())
            .collect();
        names.sort();
        names.dedup();
        names
    }

    fn resolve_type(
        &self,
        model_name: &str,
        version: Option<&str>,
        type_name: &str,
    ) -> Option<TypeInfo> {
        let model = self.get_model(model_name, version)?;

        // Handle qualified names like "FHIR.Patient" or just "Patient"
        let search_name = type_name
            .strip_prefix(&format!("{model_name}."))
            .unwrap_or(type_name);

        for type_info in &model.type_info {
            match type_info {
                TypeInfo::SimpleTypeInfo(si) => {
                    if si.name.as_deref() == Some(search_name) {
                        return Some(type_info.clone());
                    }
                }
                TypeInfo::ClassInfo(ci) => {
                    if ci.name.as_deref() == Some(search_name) {
                        return Some(type_info.clone());
                    }
                    // Also check identifier for profiles
                    if ci.identifier.as_deref() == Some(type_name) {
                        return Some(type_info.clone());
                    }
                }
                TypeInfo::ProfileInfo(pi) => {
                    if pi.name.as_deref() == Some(search_name) {
                        return Some(type_info.clone());
                    }
                    if pi.identifier.as_deref() == Some(type_name) {
                        return Some(type_info.clone());
                    }
                }
                // These types are anonymous/structural and don't have names
                TypeInfo::IntervalTypeInfo(_)
                | TypeInfo::ListTypeInfo(_)
                | TypeInfo::TupleTypeInfo(_)
                | TypeInfo::ChoiceTypeInfo(_) => {}
            }
        }

        None
    }
}

/// Pre-built FHIR R4 ModelInfo.
///
/// This provides a basic ModelInfo for FHIR R4 with the core resource types.
/// For full FHIR support, load the complete ModelInfo from the FHIR-ModelInfo
/// specification.
pub fn fhir_r4_model_info() -> ModelInfo {
    use crate::modelinfo::{ClassInfo, ClassInfoElement, SimpleTypeInfo};

    ModelInfo {
        name: Some("FHIR".to_string()),
        version: Some("4.0.1".to_string()),
        url: Some("http://hl7.org/fhir".to_string()),
        patient_class_name: Some("Patient".to_string()),
        patient_birth_date_property_name: Some("birthDate".to_string()),
        default_context: Some("Patient".to_string()),
        case_sensitive: Some(false),
        type_info: vec![
            // System primitives
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Any".to_string()),
                base_type: None,
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Boolean".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Integer".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Long".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Decimal".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("String".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("DateTime".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Date".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Time".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Quantity".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Code".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("System".to_string()),
                name: Some("Concept".to_string()),
                base_type: Some("System.Any".to_string()),
                base_type_specifier: None,
            }),
            // FHIR base types
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Element".to_string()),
                base_type: Some("System.Any".to_string()),
                retrievable: Some(false),
                element: vec![ClassInfoElement {
                    name: Some("id".to_string()),
                    element_type: Some("System.String".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Resource".to_string()),
                base_type: Some("System.Any".to_string()),
                retrievable: Some(true),
                element: vec![
                    ClassInfoElement {
                        name: Some("id".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("meta".to_string()),
                        element_type: Some("FHIR.Meta".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("DomainResource".to_string()),
                base_type: Some("FHIR.Resource".to_string()),
                retrievable: Some(true),
                element: vec![ClassInfoElement {
                    name: Some("text".to_string()),
                    element_type: Some("FHIR.Narrative".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            // Patient
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Patient".to_string()),
                identifier: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
                base_type: Some("FHIR.DomainResource".to_string()),
                retrievable: Some(true),
                primary_code_path: None,
                element: vec![
                    ClassInfoElement {
                        name: Some("identifier".to_string()),
                        element_type: Some("FHIR.Identifier".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("active".to_string()),
                        element_type: Some("System.Boolean".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("name".to_string()),
                        element_type: Some("FHIR.HumanName".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("gender".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("birthDate".to_string()),
                        element_type: Some("System.Date".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            // Observation
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Observation".to_string()),
                identifier: Some("http://hl7.org/fhir/StructureDefinition/Observation".to_string()),
                base_type: Some("FHIR.DomainResource".to_string()),
                retrievable: Some(true),
                primary_code_path: Some("code".to_string()),
                element: vec![
                    ClassInfoElement {
                        name: Some("status".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("code".to_string()),
                        element_type: Some("FHIR.CodeableConcept".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("subject".to_string()),
                        element_type: Some("FHIR.Reference".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("value".to_string()),
                        element_type: Some("System.Any".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            // Condition
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Condition".to_string()),
                identifier: Some("http://hl7.org/fhir/StructureDefinition/Condition".to_string()),
                base_type: Some("FHIR.DomainResource".to_string()),
                retrievable: Some(true),
                primary_code_path: Some("code".to_string()),
                element: vec![
                    ClassInfoElement {
                        name: Some("clinicalStatus".to_string()),
                        element_type: Some("FHIR.CodeableConcept".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("verificationStatus".to_string()),
                        element_type: Some("FHIR.CodeableConcept".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("code".to_string()),
                        element_type: Some("FHIR.CodeableConcept".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("subject".to_string()),
                        element_type: Some("FHIR.Reference".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            // Encounter
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("Encounter".to_string()),
                identifier: Some("http://hl7.org/fhir/StructureDefinition/Encounter".to_string()),
                base_type: Some("FHIR.DomainResource".to_string()),
                retrievable: Some(true),
                primary_code_path: Some("type".to_string()),
                element: vec![
                    ClassInfoElement {
                        name: Some("status".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("class".to_string()),
                        element_type: Some("FHIR.Coding".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("type".to_string()),
                        element_type: Some("FHIR.CodeableConcept".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("subject".to_string()),
                        element_type: Some("FHIR.Reference".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("period".to_string()),
                        element_type: Some("FHIR.Period".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            // MedicationRequest
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("FHIR".to_string()),
                name: Some("MedicationRequest".to_string()),
                identifier: Some(
                    "http://hl7.org/fhir/StructureDefinition/MedicationRequest".to_string(),
                ),
                base_type: Some("FHIR.DomainResource".to_string()),
                retrievable: Some(true),
                primary_code_path: Some("medication".to_string()),
                element: vec![
                    ClassInfoElement {
                        name: Some("status".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("medication".to_string()),
                        element_type: Some("System.Any".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("subject".to_string()),
                        element_type: Some("FHIR.Reference".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("authoredOn".to_string()),
                        element_type: Some("System.DateTime".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

/// Create a provider pre-loaded with the FHIR R4 model.
pub fn fhir_r4_provider() -> MemoryModelInfoProvider {
    let provider = MemoryModelInfoProvider::new();
    provider.register_model(fhir_r4_model_info());
    provider
}

// =============================================================================
// FHIR Package ModelInfo Loading
// =============================================================================

use std::path::Path;

/// Load ModelInfo from a FHIR package directory.
///
/// This function searches for Library resources in the package that contain
/// ModelInfo XML content (base64 encoded in the `content` field).
///
/// # Arguments
///
/// * `package_dir` - Path to the extracted FHIR package directory
///   (e.g., `~/.fhir/packages/fhir.cqf.common#4.0.1`)
///
/// # Returns
///
/// The parsed `ModelInfo` if found, or an error if not found or parsing failed.
///
/// # Example
///
/// ```rust,no_run
/// use rh_cql::provider::load_modelinfo_from_package;
/// use std::path::Path;
///
/// let package_dir = Path::new("/Users/user/.fhir/packages/fhir.cqf.common#4.0.1");
/// let model_info = load_modelinfo_from_package(package_dir).unwrap();
/// assert_eq!(model_info.name, Some("FHIR".to_string()));
/// ```
pub fn load_modelinfo_from_package(package_dir: &Path) -> anyhow::Result<ModelInfo> {
    // Look for Library-FHIR-ModelInfo.json or similar files
    let package_path = package_dir.join("package");
    let search_dir = if package_path.exists() {
        package_path
    } else {
        package_dir.to_path_buf()
    };

    // Search for files that might contain ModelInfo
    let modelinfo_patterns = [
        "Library-FHIR-ModelInfo.json",
        "Library-FHIRModelInfo.json",
        "FHIR-ModelInfo.json",
    ];

    for pattern in &modelinfo_patterns {
        let file_path = search_dir.join(pattern);
        if file_path.exists() {
            return load_modelinfo_from_library_file(&file_path);
        }
    }

    // Fall back to searching all Library files
    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("Library-")
                    && name.contains("ModelInfo")
                    && name.ends_with(".json")
                {
                    if let Ok(model_info) = load_modelinfo_from_library_file(&path) {
                        return Ok(model_info);
                    }
                }
            }
        }
    }

    anyhow::bail!(
        "No ModelInfo found in package directory: {}",
        package_dir.display()
    )
}

/// Load ModelInfo from a Library JSON file.
///
/// The Library resource is expected to have a `content` array with an entry
/// containing the ModelInfo XML as base64-encoded data.
fn load_modelinfo_from_library_file(file_path: &Path) -> anyhow::Result<ModelInfo> {
    use anyhow::Context;

    let contents = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let library: serde_json::Value = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON: {}", file_path.display()))?;

    // Navigate to content[0].data (base64 encoded XML)
    let content = library
        .get("content")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .ok_or_else(|| anyhow::anyhow!("No content array in Library resource"))?;

    let base64_data = content
        .get("data")
        .and_then(|d| d.as_str())
        .ok_or_else(|| anyhow::anyhow!("No data field in content"))?;

    // Decode base64
    let xml_bytes =
        decode_base64(base64_data).with_context(|| "Failed to decode base64 content")?;

    let xml_string =
        String::from_utf8(xml_bytes).with_context(|| "ModelInfo content is not valid UTF-8")?;

    // Parse XML to ModelInfo
    ModelInfo::from_xml(&xml_string).with_context(|| "Failed to parse ModelInfo XML")
}

/// Simple base64 decoder (no external dependency).
fn decode_base64(input: &str) -> anyhow::Result<Vec<u8>> {
    fn char_to_value(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => Some(0), // Padding
            _ => None,
        }
    }

    // Remove whitespace and validate
    let clean: Vec<u8> = input
        .bytes()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    if clean.is_empty() {
        return Ok(Vec::new());
    }

    // Process in 4-byte chunks
    let mut result = Vec::with_capacity(clean.len() * 3 / 4);
    let mut chunks = clean.chunks_exact(4);

    for chunk in chunks.by_ref() {
        let a =
            char_to_value(chunk[0]).ok_or_else(|| anyhow::anyhow!("Invalid base64 character"))?;
        let b =
            char_to_value(chunk[1]).ok_or_else(|| anyhow::anyhow!("Invalid base64 character"))?;
        let c =
            char_to_value(chunk[2]).ok_or_else(|| anyhow::anyhow!("Invalid base64 character"))?;
        let d =
            char_to_value(chunk[3]).ok_or_else(|| anyhow::anyhow!("Invalid base64 character"))?;

        result.push((a << 2) | (b >> 4));
        if chunk[2] != b'=' {
            result.push((b << 4) | (c >> 2));
        }
        if chunk[3] != b'=' {
            result.push((c << 6) | d);
        }
    }

    // Handle remainder (should be empty if properly padded)
    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        anyhow::bail!("Invalid base64: length not a multiple of 4");
    }

    Ok(result)
}

/// Get the default FHIR packages directory.
///
/// Returns `~/.fhir/packages` on Unix or `%USERPROFILE%\.fhir\packages` on Windows.
pub fn get_default_packages_dir() -> anyhow::Result<std::path::PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
    Ok(std::path::PathBuf::from(home)
        .join(".fhir")
        .join("packages"))
}

/// Get the package directory path for a specific package.
///
/// # Arguments
///
/// * `package_name` - The package name (e.g., "fhir.cqf.common")
/// * `version` - The package version (e.g., "4.0.1")
///
/// # Returns
///
/// The path to the package directory (e.g., `~/.fhir/packages/fhir.cqf.common#4.0.1`)
pub fn get_package_dir(package_name: &str, version: &str) -> anyhow::Result<std::path::PathBuf> {
    let packages_dir = get_default_packages_dir()?;
    Ok(packages_dir.join(format!("{package_name}#{version}")))
}

/// Try to load FHIR R4 ModelInfo from the fhir.cqf.common package.
///
/// This looks for the ModelInfo in `~/.fhir/packages/fhir.cqf.common#4.0.1`.
/// If the package is not present, returns an error.
///
/// # Example
///
/// ```rust,no_run
/// use rh_cql::provider::load_fhir_r4_modelinfo_from_package;
///
/// // Load from ~/.fhir/packages/fhir.cqf.common#4.0.1
/// let model_info = load_fhir_r4_modelinfo_from_package().unwrap();
/// assert_eq!(model_info.name, Some("FHIR".to_string()));
/// assert!(model_info.conversion_info.len() > 200);
/// ```
pub fn load_fhir_r4_modelinfo_from_package() -> anyhow::Result<ModelInfo> {
    let package_dir = get_package_dir("fhir.cqf.common", "4.0.1")?;

    if !package_dir.exists() {
        anyhow::bail!(
            "FHIR R4 ModelInfo package not found. Download it with:\n\
             rh package download fhir.cqf.common 4.0.1\n\n\
             Expected location: {}",
            package_dir.display()
        );
    }

    load_modelinfo_from_package(&package_dir)
}

/// Create a provider pre-loaded with FHIR R4 ModelInfo from the package.
///
/// This loads the full ModelInfo from `fhir.cqf.common@4.0.1` which includes:
/// - All FHIR R4 type definitions
/// - 264 implicit conversion definitions (FHIRHelpers.*)
/// - Context and relationship information
///
/// Falls back to the built-in minimal `fhir_r4_model_info()` if the package
/// is not available.
pub fn fhir_r4_provider_from_package() -> MemoryModelInfoProvider {
    let provider = MemoryModelInfoProvider::new();

    match load_fhir_r4_modelinfo_from_package() {
        Ok(model_info) => {
            provider.register_model(model_info);
        }
        Err(_) => {
            // Fall back to built-in minimal model info
            provider.register_model(fhir_r4_model_info());
        }
    }

    provider
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider() {
        let provider = MemoryModelInfoProvider::new();
        assert_eq!(provider.model_count(), 0);
        assert!(provider.list_models().is_empty());
    }

    #[test]
    fn test_register_model() {
        let provider = MemoryModelInfoProvider::new();

        let model = ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            ..Default::default()
        };
        provider.register_model(model);

        assert_eq!(provider.model_count(), 1);
        assert!(provider.has_model("FHIR", Some("4.0.1")));
        assert!(!provider.has_model("FHIR", Some("3.0.2")));
    }

    #[test]
    fn test_get_model_exact_version() {
        let provider = MemoryModelInfoProvider::new();

        let model = ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            url: Some("http://hl7.org/fhir".to_string()),
            ..Default::default()
        };
        provider.register_model(model);

        let found = provider.get_model("FHIR", Some("4.0.1"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().url, Some("http://hl7.org/fhir".to_string()));
    }

    #[test]
    fn test_get_model_no_version() {
        let provider = MemoryModelInfoProvider::new();

        let model = ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            ..Default::default()
        };
        provider.register_model(model);

        // Without version, should find any version
        let found = provider.get_model("FHIR", None);
        assert!(found.is_some());
    }

    #[test]
    fn test_list_models() {
        let provider = MemoryModelInfoProvider::new();

        provider.register_model(ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            ..Default::default()
        });
        provider.register_model(ModelInfo {
            name: Some("QDM".to_string()),
            version: Some("5.6".to_string()),
            ..Default::default()
        });

        let names = provider.list_models();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"FHIR".to_string()));
        assert!(names.contains(&"QDM".to_string()));
    }

    #[test]
    fn test_resolve_type() {
        let provider = fhir_r4_provider();

        // Resolve by simple name
        let patient = provider.resolve_type("FHIR", Some("4.0.1"), "Patient");
        assert!(patient.is_some());
        assert!(matches!(patient.unwrap(), TypeInfo::ClassInfo(_)));

        // Resolve by qualified name
        let patient = provider.resolve_type("FHIR", Some("4.0.1"), "FHIR.Patient");
        assert!(patient.is_some());

        // Non-existent type
        let unknown = provider.resolve_type("FHIR", Some("4.0.1"), "NonExistent");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_resolve_class() {
        let provider = fhir_r4_provider();

        let patient = provider.resolve_class("FHIR", Some("4.0.1"), "Patient");
        assert!(patient.is_some());
        let patient = patient.unwrap();
        assert_eq!(patient.name, Some("Patient".to_string()));
        assert!(patient.retrievable.unwrap_or(false));
    }

    #[test]
    fn test_get_patient_class() {
        let provider = fhir_r4_provider();

        let patient = provider.get_patient_class("FHIR", Some("4.0.1"));
        assert!(patient.is_some());
        assert_eq!(patient.unwrap().name, Some("Patient".to_string()));
    }

    #[test]
    fn test_fhir_r4_model_info() {
        let model = fhir_r4_model_info();

        assert_eq!(model.name, Some("FHIR".to_string()));
        assert_eq!(model.version, Some("4.0.1".to_string()));
        assert!(!model.type_info.is_empty());

        // Check we have the expected types
        let type_names: Vec<_> = model
            .type_info
            .iter()
            .filter_map(|ti| match ti {
                TypeInfo::ClassInfo(ci) => ci.name.clone(),
                TypeInfo::SimpleTypeInfo(si) => si.name.clone(),
                _ => None,
            })
            .collect();

        assert!(type_names.contains(&"Patient".to_string()));
        assert!(type_names.contains(&"Observation".to_string()));
        assert!(type_names.contains(&"Condition".to_string()));
        assert!(type_names.contains(&"String".to_string()));
        assert!(type_names.contains(&"Boolean".to_string()));
    }

    #[test]
    fn test_remove_model() {
        let provider = MemoryModelInfoProvider::new();

        provider.register_model(ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            ..Default::default()
        });

        assert!(provider.has_model("FHIR", Some("4.0.1")));

        let removed = provider.remove_model("FHIR", Some("4.0.1"));
        assert!(removed.is_some());
        assert!(!provider.has_model("FHIR", Some("4.0.1")));
    }

    #[test]
    fn test_clear_provider() {
        let provider = fhir_r4_provider();
        assert!(provider.model_count() > 0);

        provider.clear();
        assert_eq!(provider.model_count(), 0);
    }

    #[test]
    fn test_provider_clone_shares_data() {
        let provider1 = MemoryModelInfoProvider::new();
        provider1.register_model(ModelInfo {
            name: Some("FHIR".to_string()),
            version: Some("4.0.1".to_string()),
            ..Default::default()
        });

        let provider2 = provider1.clone();

        // Changes through provider2 should be visible in provider1
        provider2.register_model(ModelInfo {
            name: Some("QDM".to_string()),
            version: Some("5.6".to_string()),
            ..Default::default()
        });

        assert!(provider1.has_model("QDM", Some("5.6")));
    }

    #[test]
    fn test_provider_with_stats() {
        let provider = MemoryModelInfoProvider::with_stats();
        provider.register_model(fhir_r4_model_info());

        // Make some lookups
        provider.get_model("FHIR", Some("4.0.1"));
        provider.get_model("FHIR", Some("4.0.1"));
        provider.get_model("NonExistent", None);

        let stats = provider.stats();
        assert!(stats.hits >= 2);
        assert!(stats.misses >= 1);
    }

    // =========================================================================
    // FHIR Package Loading Tests
    // =========================================================================

    #[test]
    fn test_decode_base64_simple() {
        let result = decode_base64("SGVsbG8=").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_base64_with_whitespace() {
        let result = decode_base64("SGVs\nbG8=").unwrap();
        assert_eq!(result, b"Hello");
    }

    #[test]
    fn test_decode_base64_padding() {
        // No padding needed
        assert_eq!(decode_base64("YWJj").unwrap(), b"abc");
        // One padding
        assert_eq!(decode_base64("YWI=").unwrap(), b"ab");
        // Two padding
        assert_eq!(decode_base64("YQ==").unwrap(), b"a");
    }

    #[test]
    fn test_get_default_packages_dir() {
        let dir = get_default_packages_dir().unwrap();
        assert!(dir.to_string_lossy().contains(".fhir"));
        assert!(dir.to_string_lossy().contains("packages"));
    }

    #[test]
    fn test_get_package_dir() {
        let dir = get_package_dir("fhir.cqf.common", "4.0.1").unwrap();
        assert!(dir.to_string_lossy().contains("fhir.cqf.common#4.0.1"));
    }

    #[test]
    fn test_load_from_package_if_exists() {
        // This test only runs if the package is downloaded
        let package_dir = get_package_dir("fhir.cqf.common", "4.0.1");
        if let Ok(dir) = package_dir {
            if dir.exists() {
                let model_info = load_modelinfo_from_package(&dir).unwrap();

                assert_eq!(model_info.name, Some("FHIR".to_string()));
                assert_eq!(model_info.version, Some("4.0.1".to_string()));

                // Should have many types
                assert!(
                    model_info.type_info.len() > 100,
                    "Expected > 100 types, got {}",
                    model_info.type_info.len()
                );

                // Should have conversion info
                assert!(
                    model_info.conversion_info.len() > 200,
                    "Expected > 200 conversions, got {}",
                    model_info.conversion_info.len()
                );

                // Verify key conversions exist
                let has_to_code = model_info
                    .conversion_info
                    .iter()
                    .any(|c| c.function_name.as_deref() == Some("FHIRHelpers.ToCode"));
                assert!(has_to_code, "Should have FHIRHelpers.ToCode conversion");

                let has_to_concept = model_info
                    .conversion_info
                    .iter()
                    .any(|c| c.function_name.as_deref() == Some("FHIRHelpers.ToConcept"));
                assert!(
                    has_to_concept,
                    "Should have FHIRHelpers.ToConcept conversion"
                );
            }
        }
    }

    #[test]
    fn test_fhir_r4_provider_from_package() {
        let provider = fhir_r4_provider_from_package();

        // Should have FHIR model regardless of whether package exists
        assert!(provider.has_model("FHIR", Some("4.0.1")));

        let model = provider.get_model("FHIR", Some("4.0.1")).unwrap();
        assert_eq!(model.name, Some("FHIR".to_string()));

        // Should have Patient type
        let patient = provider.resolve_class("FHIR", Some("4.0.1"), "Patient");
        assert!(patient.is_some());
    }
}
