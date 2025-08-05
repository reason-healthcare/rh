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
pub use generators::utils::GeneratorUtils;
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
            message: format!("Skipping retired StructureDefinition '{name}' - retired StructureDefinitions are not generated as Rust types", name = structure_def.name)
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
    structure_def: &StructureDefinition,
    base_output_dir: P,
) -> CodegenResult<()> {
    let traits_dir = base_output_dir.as_ref().join("src").join("traits");
    std::fs::create_dir_all(&traits_dir)?;

    // Generate both the generic Resource trait and the specific resource trait

    // 1. Generate the generic Resource trait
    let generic_trait_file = traits_dir.join("resource.rs");
    match generator.generate_resource_trait() {
        Ok(trait_content) => {
            std::fs::write(&generic_trait_file, trait_content)?;
        }
        Err(e) => return Err(e),
    }

    // 2. Generate the specific resource trait with choice type methods
    let trait_filename = trait_filename_from_name(&structure_def.name);
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

/// Convert a FHIR structure definition name to a proper snake_case filename for traits
fn trait_filename_from_name(name: &str) -> String {
    // First handle spaces, dashes, dots, and other separators
    let cleaned = name
        .replace([' ', '-', '.'], "_")
        .replace(['(', ')', '[', ']'], "")
        .replace(['/', '\\', ':'], "_");

    // Then apply snake_case conversion for CamelCase
    GeneratorUtils::to_snake_case(&cleaned)
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
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

/// Format the generated crate using rustfmt
pub fn format_generated_crate<P: AsRef<std::path::Path>>(crate_path: P) -> CodegenResult<()> {
    let crate_path = crate_path.as_ref();

    println!("Formatting generated crate at: {}", crate_path.display());

    // Try cargo fmt first, fallback to rustfmt directly if it fails
    let output = std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir(crate_path)
        .output()
        .map_err(|e| CodegenError::Generation {
            message: format!("Failed to run cargo fmt: {e}"),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // If cargo fmt fails due to no targets, try formatting the lib.rs directly
        if stderr.contains("Failed to find targets") || stderr.contains("no targets") {
            println!("⚠️  cargo fmt found no targets, trying direct rustfmt...");

            let lib_rs = crate_path.join("src").join("lib.rs");
            if lib_rs.exists() {
                let rustfmt_output = std::process::Command::new("rustfmt")
                    .arg("--edition")
                    .arg("2021")
                    .arg(&lib_rs)
                    .output()
                    .map_err(|e| CodegenError::Generation {
                        message: format!("Failed to run rustfmt directly: {e}"),
                    })?;

                if !rustfmt_output.status.success() {
                    let rustfmt_stderr = String::from_utf8_lossy(&rustfmt_output.stderr);
                    return Err(CodegenError::Generation {
                        message: format!("rustfmt failed: {rustfmt_stderr}"),
                    });
                }

                println!("✅ Formatting completed successfully (using rustfmt directly)");
                return Ok(());
            }
        }

        return Err(CodegenError::Generation {
            message: format!("cargo fmt failed: {stderr}"),
        });
    }

    println!("✅ Formatting completed successfully");
    Ok(())
}

/// Run clippy on the generated crate to check for warnings and errors
pub fn check_generated_crate<P: AsRef<std::path::Path>>(crate_path: P) -> CodegenResult<()> {
    let crate_path = crate_path.as_ref();

    println!(
        "Running clippy on generated crate at: {}",
        crate_path.display()
    );

    let output = std::process::Command::new("cargo")
        .arg("clippy")
        .arg("--lib") // Only check the library target
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir(crate_path)
        .output()
        .map_err(|e| CodegenError::Generation {
            message: format!("Failed to run cargo clippy: {e}"),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("⚠️  Clippy found issues:");
        println!("{stdout}");
        println!("{stderr}");
        return Err(CodegenError::Generation {
            message: "cargo clippy found issues - see output above".to_string(),
        });
    }

    println!("✅ Clippy check completed successfully");
    Ok(())
}

/// Check if the generated crate compiles successfully
pub fn compile_generated_crate<P: AsRef<std::path::Path>>(crate_path: P) -> CodegenResult<()> {
    let crate_path = crate_path.as_ref();

    println!(
        "Checking compilation of generated crate at: {}",
        crate_path.display()
    );

    let output = std::process::Command::new("cargo")
        .arg("check")
        .arg("--lib") // Only check the library target
        .current_dir(crate_path)
        .output()
        .map_err(|e| CodegenError::Generation {
            message: format!("Failed to run cargo check: {e}"),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("❌ Compilation failed:");
        println!("{stdout}");
        println!("{stderr}");
        return Err(CodegenError::Generation {
            message: "cargo check failed - see output above".to_string(),
        });
    }

    println!("✅ Compilation check completed successfully");
    Ok(())
}

/// Run a complete quality check pipeline on the generated crate
pub fn run_quality_checks<P: AsRef<std::path::Path>>(crate_path: P) -> CodegenResult<()> {
    let crate_path = crate_path.as_ref();

    println!("🔧 Running quality checks on generated crate...");

    // 1. Format the code
    format_generated_crate(crate_path)?;

    // 2. Check compilation
    compile_generated_crate(crate_path)?;

    // 3. Run clippy (but don't fail on warnings for now)
    match check_generated_crate(crate_path) {
        Ok(()) => println!("✅ All quality checks passed!"),
        Err(e) => {
            println!("⚠️  Quality checks completed with warnings: {e}");
            println!("📝 Note: Warnings don't prevent code generation but should be reviewed");
        }
    }

    Ok(())
}
