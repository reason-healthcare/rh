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
    /// Source directory containing `packager.toml`, the `ImplementationGuide`, and
    /// any top-level metadata files.
    pub source_dir: PathBuf,

    /// Resolved input directory — the root from which resources, narrative, and source
    /// files are loaded.
    ///
    /// - **Structured layout**: `source_dir/<config.input.dir>` (e.g. `source_dir/input/`).
    ///   Active when that directory exists on disk.
    /// - **Legacy layout**: same as `source_dir` (backward-compatible fallback).
    pub input_dir: PathBuf,

    /// Directory where the assembled `package/` tree will be written.
    pub output_dir: PathBuf,

    /// Package manifest derived from `packager.toml`. Written to the output directory
    /// as `package.json` during the build — not read from the source directory.
    pub package_json: PackageJson,

    /// Mutable map of FHIR resources: filename stem → JSON value.
    pub resources: HashMap<String, Value>,

    /// Mutable map of example FHIR resources: filename stem → JSON value.
    ///
    /// These are written to `package/examples/` in the output with their own `.index.json`,
    /// separate from definitional resources in `package/`. Sources placed under the
    /// configured `examples/` subdirectory are automatically routed here.
    pub examples: HashMap<String, Value>,

    /// Packager configuration loaded from `packager.toml` (defaults if absent).
    pub config: PublisherConfig,

    /// Markdown files that have no matching FHIR resource; written to `package/other/`.
    pub standalone_markdown: Vec<PathBuf>,
}

impl PublishContext {
    /// Create a new context with an empty resource map.
    pub fn new(
        source_dir: PathBuf,
        input_dir: PathBuf,
        output_dir: PathBuf,
        package_json: PackageJson,
        config: PublisherConfig,
    ) -> Self {
        Self {
            source_dir,
            input_dir,
            output_dir,
            package_json,
            resources: HashMap::new(),
            examples: HashMap::new(),
            config,
            standalone_markdown: Vec::new(),
        }
    }

    /// Returns `true` when the structured `input/` layout is active.
    ///
    /// This is the case when `input_dir != source_dir`.
    pub fn uses_structured_input(&self) -> bool {
        self.input_dir != self.source_dir
    }

    /// Directory for FHIR Shorthand (`*.fsh`) source files.
    ///
    /// - Structured: `input_dir/<config.input.fsh_dir>`
    /// - Legacy: `source_dir` (fsh files anywhere under the root)
    pub fn fsh_dir(&self) -> PathBuf {
        if self.uses_structured_input() {
            self.input_dir.join(&self.config.input.fsh_dir)
        } else {
            self.source_dir.clone()
        }
    }

    /// Directory for CQL (`*.cql`) source files.
    ///
    /// - Structured: `input_dir/<config.input.cql_dir>`
    /// - Legacy: `source_dir`
    pub fn cql_dir(&self) -> PathBuf {
        if self.uses_structured_input() {
            self.input_dir.join(&self.config.input.cql_dir)
        } else {
            self.source_dir.clone()
        }
    }

    /// Directory for resource-matched narrative markdown files.
    ///
    /// - Structured: `input_dir/<config.input.narrative_dir>`
    /// - Legacy: `source_dir`
    pub fn narrative_dir(&self) -> PathBuf {
        if self.uses_structured_input() {
            self.input_dir.join(&self.config.input.narrative_dir)
        } else {
            self.source_dir.clone()
        }
    }

    /// Directory for standalone markdown pages (written to `package/other/`).
    ///
    /// Always `input_dir/<config.input.docs_dir>` — in legacy mode `input_dir == source_dir`.
    pub fn docs_dir(&self) -> PathBuf {
        self.input_dir.join(&self.config.input.docs_dir)
    }

    /// Directory for example FHIR resources (written to `package/examples/`).
    ///
    /// Always `input_dir/<config.input.examples_dir>` — in legacy mode `input_dir == source_dir`.
    pub fn examples_dir(&self) -> PathBuf {
        self.input_dir.join(&self.config.input.examples_dir)
    }

    /// Create a minimal context for use in tests.
    ///
    /// Sets `source_dir` and `input_dir` both to `path` (legacy layout) and
    /// `output_dir` to `path/output` with a default package manifest
    /// (`test.fhir.pkg@1.0.0`) and empty config.
    #[cfg(test)]
    pub fn for_testing(source_dir: impl Into<PathBuf>) -> Self {
        let source_dir = source_dir.into();
        let input_dir = source_dir.clone();
        let output_dir = source_dir.join("output");
        Self::new(
            source_dir,
            input_dir,
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
