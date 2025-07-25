//! Code generation library for creating Rust types from FHIR StructureDefinitions
//!
//! This crate provides functionality to parse FHIR StructureDefinition JSON files
//! and generate corresponding Rust type definitions.

pub use common::{CommonError, Config};

pub mod download;
pub mod generator;

// Re-export main types for convenience
pub use download::{
    PackageDist, PackageDownloadConfig, PackageDownloader, PackageManifest, RegistryResponse,
};
pub use generator::{
    CodeGenerator, CodegenConfig, ElementDefinition, ElementType, RustField, RustStruct,
    StructureDefinition, StructureDefinitionDifferential, StructureDefinitionSnapshot,
};

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
