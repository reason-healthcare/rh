//! FHIR Code Generation Library
//!
//! This library provides functionality to generate Rust code from FHIR (Fast Healthcare
//! Interoperability Resources) StructureDefinition files.

pub use rh_common::{CommonError, Config};

mod config;
pub mod crate_generator;
pub mod fhir_types;
mod generator;
mod rust_types;
mod token_generator;
mod type_mapper;
mod value_sets;

// Re-export loader types for convenience
pub use rh_loader::{
    LoaderConfig as PackageDownloadConfig, LoaderError, LoaderResult, PackageDist,
    PackageLoader as PackageDownloader, PackageManifest,
};

// Re-export modular code generation types
pub use config::CodegenConfig;
pub use crate_generator::{
    generate_crate_structure, parse_package_metadata, CrateGenerationParams, CrateStatistics,
};
pub use fhir_types::{ElementBinding, ElementDefinition, ElementType, StructureDefinition};
pub use generator::{CodeGenerator, FhirTypeCategory};
pub use rust_types::{
    RustEnum, RustField, RustMethodParam, RustModule, RustStruct, RustTrait, RustTraitMethod,
    RustType,
};
pub use token_generator::TokenGenerator;
pub use type_mapper::TypeMapper;
pub use value_sets::{ValueSetConcept, ValueSetManager};

/// Errors specific to code generation
#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
    #[error("Invalid FHIR type: {fhir_type}")]
    InvalidFhirType { fhir_type: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Code generation failed: {message}")]
    Generation { message: String },

    #[error("Loader error: {0}")]
    Loader(#[from] rh_loader::LoaderError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Common error: {0}")]
    Common(#[from] CommonError),
}

/// Result type for codegen operations
pub type CodegenResult<T> = std::result::Result<T, CodegenError>;
