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
    // Skip if status is retired
    if structure_def.status == "retired" {
        return Err(CodegenError::Generation {
            message: format!("Skipping retired StructureDefinition '{}' - retired StructureDefinitions are not generated as Rust types", structure_def.name)
        });
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CodeGenerator, CodegenConfig};
    use tempfile::TempDir;

    #[test]
    fn test_skip_retired_structure_definition() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();
        
        // Create a mock retired StructureDefinition
        let retired_structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-retired".to_string(),
            url: "http://example.org/StructureDefinition/TestRetired".to_string(),
            version: Some("1.0.0".to_string()),
            name: "TestRetired".to_string(),
            title: Some("Test Retired Structure".to_string()),
            status: "retired".to_string(),  // This is the key field
            description: Some("A retired test structure".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: None,
            snapshot: None,
        };

        // Attempt to generate - should return an error about retired status
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &retired_structure_def,
            temp_dir.path()
        );

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("retired"));
        assert!(error_message.contains("TestRetired"));
    }

    #[test]
    fn test_process_active_structure_definition() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();
        
        // Create a mock active StructureDefinition
        let active_structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-active".to_string(),
            url: "http://example.org/StructureDefinition/TestActive".to_string(),
            version: Some("1.0.0".to_string()),
            name: "TestActive".to_string(),
            title: Some("Test Active Structure".to_string()),
            status: "active".to_string(),  // This should allow processing
            description: Some("An active test structure".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: None,
            snapshot: None,
        };

        // Attempt to generate - should succeed (even though it might fail later due to missing data)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &active_structure_def,
            temp_dir.path()
        );

        // We expect this to either succeed or fail for a different reason (not retired status)
        if let Err(error) = result {
            let error_message = error.to_string();
            // Should not be failing due to retired status
            assert!(!error_message.contains("retired"));
        }
    }
}
