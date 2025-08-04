//! FHIR code generation library
//!
//! This library provides functionality for generating Rust types from FHIR (Fast Healthcare
//! Interoperability Resources) StructureDefinition files.

pub use rh_common::{CommonError, Config};

mod config;
pub mod fhir_types;
mod generator;
pub mod generators;
mod rust_types;
mod type_mapper;
mod value_sets;

// Re-export loader types for convenience
pub use rh_loader::{
    LoaderConfig as PackageDownloadConfig, LoaderError, LoaderResult, PackageDist,
    PackageLoader as PackageDownloader, PackageManifest,
};

// Re-export modular code generation types
pub use config::CodegenConfig;
pub use fhir_types::StructureDefinition;
pub use generator::CodeGenerator;
pub use generators::crate_generator::{
    generate_crate_structure, parse_package_metadata, CrateGenerationParams,
};
pub use generators::file_generator::FhirTypeCategory;
pub use generators::token_generator::TokenGenerator;
pub use rust_types::{RustEnum, RustStruct, RustTrait, RustTraitMethod, RustType};
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

/// Generate organized directory structure with traits for resources
pub fn generate_organized_directories_with_traits<P: AsRef<std::path::Path>>(
    generator: &mut CodeGenerator,
    structure_def: &StructureDefinition,
    base_output_dir: P,
) -> CodegenResult<()> {
    // First generate the main structure
    generator.generate_to_organized_directories(structure_def, &base_output_dir)?;

    // Then generate trait if this is a resource
    let category = generator.classify_fhir_structure_def(structure_def);
    if category == FhirTypeCategory::Resource {
        generate_resource_trait_for_structure(generator, structure_def, &base_output_dir)?;
    }

    Ok(())
}

/// Generate Resource trait for a specific structure definition
pub fn generate_resource_trait_for_structure<P: AsRef<std::path::Path>>(
    generator: &mut CodeGenerator,
    _structure_def: &StructureDefinition,
    base_output_dir: P,
) -> CodegenResult<()> {
    let traits_dir = base_output_dir.as_ref().join("src").join("traits");
    std::fs::create_dir_all(&traits_dir)?;

    let trait_file = traits_dir.join("resource.rs");
    match generator.generate_resource_trait() {
        Ok(trait_content) => {
            std::fs::write(&trait_file, trait_content)?;

            // Update traits/mod.rs to include the resource trait
            update_traits_mod_file(&traits_dir)?;

            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Update the traits/mod.rs file to include the resource module
fn update_traits_mod_file(traits_dir: &std::path::Path) -> CodegenResult<()> {
    let mod_file = traits_dir.join("mod.rs");
    if !mod_file.exists() {
        // Create initial mod.rs
        let mod_content = r#"//! FHIR traits for common functionality
//!
//! This module contains traits that define common interfaces for FHIR types.

pub mod resource;
"#;
        std::fs::write(&mod_file, mod_content)?;
    } else {
        // Check if mod.rs already contains the resource module declaration
        let existing_content = std::fs::read_to_string(&mod_file)?;
        if !existing_content.contains("pub mod resource;") {
            // If it doesn't have the module declaration, add it
            let new_content = existing_content.trim_end().to_string() + "\npub mod resource;\n";
            std::fs::write(&mod_file, new_content)?;
        }
        // If it already has the module declaration, we don't need to do anything
    }

    Ok(())
}
