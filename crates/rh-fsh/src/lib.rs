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

/// Compilation options
pub struct CompilerOptions {
    pub pretty_print: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self { pretty_print: true }
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
        tank.add_document(doc)
            .map_err(|errs| errs.into_iter().next().unwrap())?;
        FshResolver::resolve(&mut tank).map_err(|errs| errs.into_iter().next().unwrap())?;
        let defs = FhirDefs::r4();
        Ok(FshExporter::export(&tank, defs))
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
    let defs = FhirDefs::r4();

    for path in paths {
        let content = fs::read_to_string(path)
            .map_err(|e| FshError::Export { message: e.to_string() })?;
        let source_name = path.to_string_lossy().into_owned();
        let doc = FshParser::parse(&content, source_name)?;
        tank.add_document(doc)
            .map_err(|errs| errs.into_iter().next().unwrap())?;
    }
    FshResolver::resolve(&mut tank).map_err(|errs| errs.into_iter().next().unwrap())?;
    Ok(FshExporter::export(&tank, defs))
}
