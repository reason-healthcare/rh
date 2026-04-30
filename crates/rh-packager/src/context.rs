use crate::{manifest::PackageJson, PublisherConfig};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

/// Runtime context shared across all publisher stages and hook processors.
///
/// The resource map is keyed by filename stem (e.g. `"StructureDefinition-foo"` for
/// `StructureDefinition-foo.json`). Processors may read and mutate entries or insert
/// new ones. A single write pass at the end materialises the map to disk.
#[derive(Debug)]
pub struct PublishContext {
    /// Source directory containing `package.json`, resources, and markdown files.
    pub source_dir: PathBuf,

    /// Directory where the assembled `package/` tree will be written.
    pub output_dir: PathBuf,

    /// Parsed `package.json` manifest from the source directory.
    pub package_json: PackageJson,

    /// Mutable map of FHIR resources: filename stem → JSON value.
    pub resources: HashMap<String, Value>,

    /// Packager configuration loaded from `packager.toml` (defaults if absent).
    pub config: PublisherConfig,

    /// Markdown files that have no matching FHIR resource; written to `package/other/`.
    pub standalone_markdown: Vec<PathBuf>,
}

impl PublishContext {
    /// Create a new context with an empty resource map.
    pub fn new(
        source_dir: PathBuf,
        output_dir: PathBuf,
        package_json: PackageJson,
        config: PublisherConfig,
    ) -> Self {
        Self {
            source_dir,
            output_dir,
            package_json,
            resources: HashMap::new(),
            config,
            standalone_markdown: Vec::new(),
        }
    }

    /// Create a minimal context for use in tests.
    ///
    /// Sets `source_dir` to `path` and `output_dir` to `path/output` with a
    /// default package manifest (`test.fhir.pkg@1.0.0`) and empty config.
    #[cfg(test)]
    pub fn for_testing(source_dir: impl Into<PathBuf>) -> Self {
        let source_dir = source_dir.into();
        let output_dir = source_dir.join("output");
        Self::new(
            source_dir,
            output_dir,
            PackageJson {
                name: "test.fhir.pkg".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: HashMap::new(),
                url: None,
                description: None,
                author: None,
                license: None,
                extra: HashMap::new(),
            },
            PublisherConfig::default(),
        )
    }
}
