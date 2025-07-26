/// FHIR Code Generation - Basic Usage Example
///
/// This example demonstrates how to use the codegen library to generate
/// Rust types from FHIR StructureDefinitions.
use codegen::{CodeGenerator, CodegenConfig, StructureDefinition};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a basic configuration
    let config = CodegenConfig {
        output_dir: "generated".to_string(),
        module_name: "example".to_string(),
        with_serde: true,
        with_docs: true,
        type_mappings: HashMap::new(),
    };

    // Create the code generator
    let mut generator = CodeGenerator::new(config);

    // Example 1: Create a simple StructureDefinition for demonstration
    println!("Creating a simple Patient StructureDefinition...");
    let patient_structure = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        version: Some("4.0.1".to_string()),
        name: "Patient".to_string(),
        title: Some("Patient".to_string()),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: None,
        snapshot: None,
    };

    // Example 2: Generate a Rust struct from the StructureDefinition
    println!("Generating Rust struct from StructureDefinition...");
    let rust_struct = generator.generate_struct(&patient_structure)?;
    println!("Generated struct: {}", rust_struct.name);

    // Example 3: Generate to file
    println!("Writing generated struct to file...");
    std::fs::create_dir_all("generated")?;
    generator.generate_to_file(&patient_structure, "generated/patient.rs")?;

    println!("✅ Code generation completed successfully!");
    println!("📁 Check the generated/patient.rs file for the generated Rust struct");

    Ok(())
}
