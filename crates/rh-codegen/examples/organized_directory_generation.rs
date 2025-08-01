//! Example showing how to use the organized directory generation methods directly
//!
//! This example demonstrates using the new methods:
//! - generate_to_organized_directories() for structs
//! - generate_trait_to_organized_directory() for traits

use std::path::Path;

use anyhow::Result;
use rh_codegen::{CodeGenerator, CodegenConfig};

fn main() -> Result<()> {
    // Create a basic codegen configuration
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Create test structure definitions
    let patient_structure = create_patient_structure_definition();
    let extension_structure = create_extension_structure_definition();
    let boolean_structure = create_boolean_structure_definition();

    // Set up output directory
    let output_dir = Path::new("./organized_output");
    let src_dir = output_dir.join("src");

    // Clean up any existing output
    if output_dir.exists() {
        std::fs::remove_dir_all(output_dir)?;
    }
    std::fs::create_dir_all(&src_dir)?;

    println!("Generating FHIR types to organized directories...");

    // Generate each type - they will be automatically placed in the correct directory
    generator.generate_to_organized_directories(&patient_structure, &src_dir)?;
    generator.generate_to_organized_directories(&extension_structure, &src_dir)?;
    generator.generate_to_organized_directories(&boolean_structure, &src_dir)?;

    // Generate traits for resources
    generator.generate_trait_to_organized_directory(&patient_structure, &src_dir)?;

    println!("✅ Generated files to organized directories:");
    println!("📁 {}", src_dir.display());

    // Show the organization
    display_directory_structure(&src_dir)?;

    Ok(())
}

fn create_patient_structure_definition() -> rh_codegen::fhir_types::StructureDefinition {
    use rh_codegen::fhir_types::*;

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        title: Some("Patient Resource".to_string()),
        status: "active".to_string(),
        description: Some(
            "Demographics and other administrative information about an individual.".to_string(),
        ),
        purpose: None,
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        version: Some("4.0.1".to_string()),
        differential: None,
        snapshot: None,
    }
}

fn create_extension_structure_definition() -> rh_codegen::fhir_types::StructureDefinition {
    use rh_codegen::fhir_types::*;

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Extension".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Extension".to_string(),
        name: "Extension".to_string(),
        title: Some("Extension Data Type".to_string()),
        status: "active".to_string(),
        description: Some("Optional Extension Element - found in all resources.".to_string()),
        purpose: None,
        kind: "complex-type".to_string(),
        is_abstract: false,
        base_type: "Element".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
        version: Some("4.0.1".to_string()),
        differential: None,
        snapshot: None,
    }
}

fn create_boolean_structure_definition() -> rh_codegen::fhir_types::StructureDefinition {
    use rh_codegen::fhir_types::*;

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "boolean".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/boolean".to_string(),
        name: "boolean".to_string(),
        title: Some("Boolean Primitive Type".to_string()),
        status: "active".to_string(),
        description: Some("Value of 'true' or 'false'".to_string()),
        purpose: None,
        kind: "primitive-type".to_string(),
        is_abstract: false,
        base_type: "boolean".to_string(),
        base_definition: None,
        version: Some("4.0.1".to_string()),
        differential: None,
        snapshot: None,
    }
}

fn display_directory_structure(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    fn display_recursive(dir: &Path, depth: usize) -> Result<()> {
        let indent = "  ".repeat(depth);

        let entries = std::fs::read_dir(dir)?;
        let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();

            if path.is_dir() {
                println!("{indent}📁 {name}/");
                display_recursive(&path, depth + 1)?;
            } else {
                println!("{indent}📄 {name}");
            }
        }

        Ok(())
    }

    display_recursive(dir, 0)
}
