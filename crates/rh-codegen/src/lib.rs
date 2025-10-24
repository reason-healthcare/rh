//! FHIR code generation library
//!
//! This library provides functionality for generating Rust types from FHIR (Fast Healthcare
//! Interoperability Resources) StructureDefinition files.
//!
//! ## Macro Call Generation
//!
//! The library can generate macro calls for FHIR primitive types instead of regular struct fields.
//! This provides better handling of FHIR's primitive extension pattern where each primitive field
//! can have an associated extension element.
//!
//! To enable macro call generation, set `use_macro_calls: true` in your `CodegenConfig`:
//!
//! ```rust
//! use rh_codegen::CodegenConfig;
//!
//! let mut config = CodegenConfig::default();
//! config.use_macro_calls = true;
//! ```
//!
/// When enabled, primitive fields like `boolean`, `string`, `integer`, etc. will be generated as
/// macro calls such as `primitive_boolean!("active", true)` instead of regular struct fields.
/// These macros automatically generate both the primitive field and its companion extension field.
pub use rh_foundation::{Config, FoundationError};

mod config;
pub mod fhir_types;
mod generator;
pub mod generators;
pub mod invariants;
pub mod macros;
pub mod metadata;
pub mod naming;
pub mod quality;
mod rust_types;
mod type_mapper;
pub mod value_sets;

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
    generate_crate_structure, generate_module_files, parse_package_metadata, CrateGenerationParams,
};
pub use generators::file_generator::{FhirTypeCategory, FileGenerator};
pub use generators::token_generator::TokenGenerator;
pub use generators::utils::GeneratorUtils;
pub use naming::Naming;
pub use quality::{format_generated_crate, QualityConfig};
pub use rust_types::{RustEnum, RustStruct, RustTrait, RustTraitMethod, RustType};
pub use type_mapper::TypeMapper;
pub use value_sets::{ValueSetConcept, ValueSetManager};

/// Errors specific to code generation.
///
/// This error type extends FoundationError with domain-specific
/// error variants for FHIR code generation.
#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
    /// Invalid FHIR type encountered
    #[error("Invalid FHIR type: {fhir_type}")]
    InvalidFhirType { fhir_type: String },

    /// Missing required field in structure definition
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// General code generation failure
    #[error("Code generation failed: {message}")]
    Generation { message: String },

    /// Package loader error
    #[error(transparent)]
    Loader(#[from] rh_loader::LoaderError),

    /// Foundation error (covers IO, JSON, etc.)
    #[error(transparent)]
    Foundation(#[from] FoundationError),
}

// Implement From for common types that should go through FoundationError
impl From<std::io::Error> for CodegenError {
    fn from(err: std::io::Error) -> Self {
        CodegenError::Foundation(FoundationError::Io(err))
    }
}

impl From<serde_json::Error> for CodegenError {
    fn from(err: serde_json::Error) -> Self {
        CodegenError::Foundation(FoundationError::Serialization(err))
    }
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
            message: format!("Skipping retired StructureDefinition '{name}' - retired StructureDefinitions are not generated as Rust types", name = structure_def.name)
        });
    }

    // First generate the main structure
    generator.generate_to_organized_directories(structure_def, &base_output_dir)?;

    // Then generate trait if this is a resource or profile
    let category = generator.classify_fhir_structure_def(structure_def);
    if category == FhirTypeCategory::Resource || category == FhirTypeCategory::Profile {
        generate_resource_trait_for_structure(generator, structure_def, &base_output_dir)?;
    }

    Ok(())
}

/// Generate Resource trait for a specific structure definition
pub fn generate_resource_trait_for_structure<P: AsRef<std::path::Path>>(
    generator: &mut CodeGenerator,
    structure_def: &StructureDefinition,
    base_output_dir: P,
) -> CodegenResult<()> {
    let traits_dir = base_output_dir.as_ref().join("src").join("traits");
    std::fs::create_dir_all(&traits_dir)?;

    // Generate both the generic Resource trait and the specific resource trait

    // 1. Generate the generic Resource trait
    // let generic_trait_file = traits_dir.join("resource.rs");
    // match generator.generate_resource_trait() {
    //     Ok(trait_content) => {
    //         std::fs::write(&generic_trait_file, trait_content)?;
    //     }
    //     Err(e) => return Err(e),
    // }

    // 2. Generate the specific resource trait with choice type methods
    // For profiles, use the struct name to ensure consistency with trait implementation references
    // For regular resources, use the name field which may have spaces/special chars
    let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);
    let trait_filename = if is_profile {
        let struct_name = crate::naming::Naming::struct_name(structure_def);
        crate::naming::Naming::to_snake_case(&struct_name)
    } else {
        crate::naming::Naming::trait_module_name(&structure_def.name)
    };

    let specific_trait_file = traits_dir.join(format!("{trait_filename}.rs"));
    match generator.generate_trait_to_file(structure_def, &specific_trait_file) {
        Ok(()) => {
            // Update traits/mod.rs to include both traits
            update_traits_mod_file(&traits_dir, &trait_filename)?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Update the traits/mod.rs file to include the resource module
fn update_traits_mod_file(traits_dir: &std::path::Path, resource_name: &str) -> CodegenResult<()> {
    let mod_file = traits_dir.join("mod.rs");
    if !mod_file.exists() {
        // Create initial mod.rs with both generic resource and specific resource modules
        let mod_content = format!(
            "//! FHIR traits for common functionality
//!
//! This module contains traits that define common interfaces for FHIR types.

pub mod resource;
pub mod {resource_name};
"
        );
        std::fs::write(&mod_file, mod_content)?;
    } else {
        // Check if mod.rs already contains the module declarations
        let existing_content = std::fs::read_to_string(&mod_file)?;
        let mut new_content = existing_content.clone();

        if !existing_content.contains("pub mod resource;") {
            new_content = new_content.trim_end().to_string() + "\npub mod resource;\n";
        }

        let specific_module_line = format!("pub mod {resource_name};");
        if !existing_content.contains(&specific_module_line) {
            new_content =
                new_content.trim_end().to_string() + &format!("\npub mod {resource_name};\n");
        }

        // Only write if content changed
        if new_content != existing_content {
            std::fs::write(&mod_file, new_content)?;
        }
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
            status: "retired".to_string(), // This is the key field
            description: Some("A retired test structure".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        // Attempt to generate - should return an error about retired status
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &retired_structure_def,
            temp_dir.path(),
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
            status: "active".to_string(), // This should allow processing
            description: Some("An active test structure".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        // Attempt to generate - should succeed (even though it might fail later due to missing data)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &active_structure_def,
            temp_dir.path(),
        );

        // We expect this to either succeed or fail for a different reason (not retired status)
        if let Err(error) = result {
            let error_message = error.to_string();
            // Should not be failing due to retired status
            assert!(!error_message.contains("retired"));
        }
    }
}
