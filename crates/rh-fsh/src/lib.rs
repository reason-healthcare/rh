//! rh-fsh: FHIR Shorthand (FSH) compiler
//!
//! Transforms FSH source into FHIR R4 JSON packages.

pub mod definition_index;
pub mod dependencies;
pub mod error;
pub mod export;
pub mod fhirdefs;
pub mod parser;
pub mod resolver;
pub mod schema;
pub mod semantic;
pub mod sushi_config;
pub mod tank;

pub use definition_index::{
    build_definition_index, build_definition_index_with_default_dependencies, DefinitionIndex,
    DefinitionSource, IndexedStructureDefinition, LocalDefinitionKind,
};
pub use dependencies::{
    load_dependency_structure_definitions, load_dependency_structure_definitions_from_dir,
    DependencyDefinitionSet, DependencyExtensionSlice, DependencyStructureDefinition,
};
pub use error::FshError;
pub use export::{FhirPackage, FshExporter};
pub use fhirdefs::FhirDefs;
pub use parser::{FshDocument, FshParser, SourceLocation};
pub use resolver::FshResolver;
pub use schema::{CompiledProfileView, CompiledSchema, ElementShape, FieldShape, SchemaView};
pub use semantic::{
    PathSelection, RepetitionSelection, SemanticAssignment, SemanticOperation, SemanticPath,
    SemanticProgram,
};
pub use sushi_config::{find_sushi_config_for_files, parse_sushi_config, read_sushi_config};
pub use tank::FshTank;

use std::path::PathBuf;

/// Project-level configuration (equivalent to sushi-config.yaml)
#[derive(Debug, Clone, Default)]
pub struct FshConfig {
    /// Canonical base URL for all generated resources (e.g. "https://example.org/fhir")
    pub canonical: Option<String>,
    /// FHIR version string (e.g. "4.0.1")
    pub fhir_version: Option<String>,
    /// Package id
    pub id: Option<String>,
    /// Human-readable name
    pub name: Option<String>,
    /// Resource status: active, draft, retired, unknown
    pub status: Option<String>,
    /// Human-readable ImplementationGuide title.
    pub title: Option<String>,
    /// ImplementationGuide description.
    pub description: Option<String>,
    /// SPDX license code.
    pub license: Option<String>,
    /// Whether the guide is experimental.
    pub experimental: Option<bool>,
    /// Root extensions copied from `sushi-config.yaml`.
    pub extensions: Vec<serde_json::Value>,
    /// Jurisdiction coding copied from `sushi-config.yaml`.
    pub jurisdiction: Option<FshCoding>,
    /// Publisher name
    pub publisher: Option<String>,
    /// Implementation guide contacts normalized from `sushi-config.yaml`.
    pub contacts: Vec<FshContact>,
    /// Package version string (e.g. "0.1.0")
    pub version: Option<String>,
    /// Dependency packages declared by the project config.
    pub dependencies: Vec<FshDependency>,
    /// Ordered guide pages.
    pub pages: Vec<FshPage>,
    /// Ordered artifact groups.
    pub groups: Vec<FshGroup>,
    /// Publisher parameters.
    pub parameters: Vec<(String, String)>,
    /// Per-resource ImplementationGuide metadata.
    pub resources: indexmap::IndexMap<String, FshResourceMetadata>,
}

/// A simple coding declared in project configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshCoding {
    pub system: String,
    pub code: String,
    pub display: Option<String>,
}

/// A page declared in project configuration.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FshPage {
    pub source: String,
    pub title: String,
    pub extensions: Vec<serde_json::Value>,
    pub pages: Vec<FshPage>,
}

/// An artifact grouping declared in project configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub resources: Vec<String>,
}

/// Metadata for a resource listed in an ImplementationGuide.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshResourceMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub example_canonical: Option<String>,
    pub example_boolean: Option<bool>,
    pub grouping_id: Option<String>,
}

/// Contact details declared by a FSH project.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshContact {
    /// Optional contact name.
    pub name: Option<String>,
    /// Contact points for this person or organization.
    pub telecom: Vec<FshTelecom>,
}

/// A contact point declared by a FSH project.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshTelecom {
    /// FHIR ContactPoint system, such as `url` or `email`.
    pub system: String,
    /// Contact point value.
    pub value: String,
}

/// Dependency package declaration from project configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FshDependency {
    /// FHIR package id, e.g. `hl7.fhir.us.core`.
    pub package_id: String,
    /// Package version, e.g. `6.1.0`.
    pub version: String,
    /// Optional short id used by SUSHI/IG Publisher.
    pub id: Option<String>,
    /// Optional ImplementationGuide URI.
    pub uri: Option<String>,
}

/// Compilation options
pub struct CompilerOptions {
    pub pretty_print: bool,
    pub config: FshConfig,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            pretty_print: true,
            config: FshConfig::default(),
        }
    }
}

/// High-level FSH compiler
pub struct FshCompiler {
    pub options: CompilerOptions,
}

impl FshCompiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    /// Compile a single FSH source string
    pub fn compile(&self, input: &str, source_name: &str) -> Result<FhirPackage, FshError> {
        let doc = FshParser::parse(input, source_name)?;
        let mut tank = FshTank::new();
        tank.add_document(doc).map_err(first_error)?;
        run_pipeline(tank, &self.options.config)
    }
}

/// Compile a single FSH string using default options
pub fn compile_fsh(input: &str, source_name: &str) -> Result<FhirPackage, FshError> {
    FshCompiler::new(CompilerOptions::default()).compile(input, source_name)
}

/// Compile multiple FSH files into one package
pub fn compile_fsh_files(paths: &[PathBuf]) -> Result<FhirPackage, FshError> {
    compile_fsh_files_with_config(paths, FshConfig::default())
}

/// Compile multiple FSH files using an explicit project configuration.
pub fn compile_fsh_files_with_config(
    paths: &[PathBuf],
    config: FshConfig,
) -> Result<FhirPackage, FshError> {
    use std::fs;
    let mut tank = FshTank::new();

    for path in paths {
        let content = fs::read_to_string(path).map_err(|e| FshError::Export {
            message: e.to_string(),
        })?;
        let source_name = path.to_string_lossy().into_owned();
        let doc = FshParser::parse(&content, source_name)?;
        tank.add_document(doc).map_err(first_error)?;
    }
    run_pipeline(tank, &config)
}

/// Compile multiple FSH files, loading the nearest `sushi-config.yaml` if one exists.
pub fn compile_fsh_files_with_project_config(paths: &[PathBuf]) -> Result<FhirPackage, FshError> {
    let config = match find_sushi_config_for_files(paths) {
        Some(path) => read_sushi_config(&path)?,
        None => FshConfig::default(),
    };
    compile_fsh_files_with_config(paths, config)
}

/// Parse → resolve → export, returning the FHIR package.
fn run_pipeline(mut tank: FshTank, config: &FshConfig) -> Result<FhirPackage, FshError> {
    FshResolver::resolve(&mut tank).map_err(first_error)?;
    let defs = FhirDefs::r4();
    Ok(FshExporter::export(&tank, defs, config))
}

/// Return the first error from a non-empty error list.
fn first_error(errs: Vec<FshError>) -> FshError {
    errs.into_iter().next().unwrap()
}

#[cfg(test)]
mod debug_tests {
    use super::*;
    #[test]
    fn test_loinc_filter_vs() {
        let fsh = r#"Alias:  LNC = http://loinc.org

ValueSet: PSATestVS
Id: psa-test-vs
* include codes from system LNC where ancestor = "LP186220-2"
* include codes from system LNC where ancestor = "LP270129-2"
"#;
        let pkg = FshCompiler::new(CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(fsh, "test.fsh")
        .unwrap();
        let vs = pkg
            .resources
            .iter()
            .find(|r| r.get("resourceType").and_then(|v| v.as_str()) == Some("ValueSet"))
            .unwrap();
        println!("VS: {}", serde_json::to_string_pretty(vs).unwrap());
        assert!(vs.get("compose").is_some(), "Expected compose field");
    }

    #[test]
    fn test_snomed_filter_value() {
        let fsh = r#"Alias: SCT = http://snomed.info/sct
ValueSet: HandednessVS
Id: handedness-vs
* include codes from system SCT where concept is-a #64940007 "Handedness finding (finding)"
"#;
        let pkg = FshCompiler::new(CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(fsh, "test.fsh")
        .unwrap();
        let vs = pkg
            .resources
            .iter()
            .find(|r| r.get("resourceType").and_then(|v| v.as_str()) == Some("ValueSet"))
            .unwrap();
        println!("VS: {}", serde_json::to_string_pretty(vs).unwrap());
        assert!(vs.get("compose").is_some(), "Expected compose field");
    }

    #[test]
    fn test_loinc_with_copyright() {
        let fsh = r#"Alias:  LNC = http://loinc.org

ValueSet: PSATestVS
Id: psa-test-vs
* ^copyright = "This material contains content from LOINC (http://loinc.org). LOINC is copyright 1995-2020."
* include codes from system LNC where ancestor = "LP186220-2"
"#;
        let pkg = FshCompiler::new(CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(fsh, "test.fsh")
        .unwrap();
        let vs = pkg
            .resources
            .iter()
            .find(|r| r.get("resourceType").and_then(|v| v.as_str()) == Some("ValueSet"))
            .unwrap();
        println!(
            "VS with copyright: {}",
            serde_json::to_string_pretty(vs).unwrap()
        );
        assert!(vs.get("compose").is_some(), "Expected compose");
        assert!(vs.get("copyright").is_some(), "Expected copyright");
    }
}
