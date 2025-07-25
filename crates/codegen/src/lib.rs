//! FHIR Code Generation Library
//!
//! This library provides functionality to generate Rust code from FHIR (Fast Healthcare
//! Interoperability Resources) StructureDefinition files.

pub use common::{CommonError, Config};

mod config;
pub mod download;
mod fhir_types;
mod generator;
mod rust_types;
mod token_generator;
mod type_mapper;
mod value_sets;

// Re-export download types for convenience
pub use download::{
    PackageDist, PackageDownloadConfig, PackageDownloader, PackageManifest, RegistryResponse,
};

// Re-export modular code generation types
pub use config::CodegenConfig;
pub use fhir_types::{ElementBinding, ElementDefinition, ElementType, StructureDefinition};
pub use generator::CodeGenerator;
pub use rust_types::{RustEnum, RustField, RustModule, RustStruct, RustType};
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

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Package not found: {package}@{version}")]
    PackageNotFound { package: String, version: String },

    #[error("Invalid package manifest: {message}")]
    InvalidManifest { message: String },

    #[error("Archive extraction failed: {message}")]
    ArchiveError { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Common error: {0}")]
    Common(#[from] CommonError),
}

/// Result type for codegen operations
pub type CodegenResult<T> = std::result::Result<T, CodegenError>;
