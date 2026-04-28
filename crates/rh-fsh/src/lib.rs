//! rh-fsh: FHIR Shorthand (FSH) compiler
//!
//! Transforms FSH source into FHIR R4 JSON packages.

pub mod error;
pub mod export;
pub mod fhirdefs;
pub mod parser;
pub mod resolver;
pub mod tank;

pub use error::FshError;
pub use export::{FhirPackage, FshExporter};
pub use fhirdefs::FhirDefs;
pub use parser::{FshDocument, FshParser, SourceLocation};
pub use resolver::FshResolver;
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
    /// Publisher name
    pub publisher: Option<String>,
    /// Package version string (e.g. "0.1.0")
    pub version: Option<String>,
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
    run_pipeline(tank, &FshConfig::default())
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
