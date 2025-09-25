//! Example demonstrating trait generation from FHIR StructureDefinitions
//!
//! This example shows how to generate Rust traits from FHIR StructureDefinitions,
//! which define interfaces for accessing common FHIR resource fields.

use rh_codegen::fhir_types::{
    ElementDefinition, ElementType, StructureDefinition, StructureDefinitionDifferential,
};
use rh_codegen::{CodeGenerator, CodegenConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a code generator with default configuration
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Create a simple Patient StructureDefinition
    let patient_structure = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        title: Some("Patient Resource".to_string()),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        description: Some(
            "Demographics and other administrative information about an individual receiving care."
                .to_string(),
        ),
        purpose: None,
        base_type: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        version: Some("4.0.1".to_string()),
        differential: Some(StructureDefinitionDifferential {
            element: vec![
                ElementDefinition {
                    id: Some("Patient.active".to_string()),
                    path: "Patient.active".to_string(),
                    short: Some("Whether this patient record is in active use".to_string()),
                    definition: Some("Whether this patient record is in active use".to_string()),
                    min: Some(0),
                    max: Some("1".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("boolean".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
                ElementDefinition {
                    id: Some("Patient.name".to_string()),
                    path: "Patient.name".to_string(),
                    short: Some("A name associated with the patient".to_string()),
                    definition: Some("A name associated with the patient".to_string()),
                    min: Some(0),
                    max: Some("*".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("HumanName".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
                ElementDefinition {
                    id: Some("Patient.identifier".to_string()),
                    path: "Patient.identifier".to_string(),
                    short: Some("An identifier for this patient".to_string()),
                    definition: Some("An identifier for this patient".to_string()),
                    min: Some(0),
                    max: Some("*".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("Identifier".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
            ],
        }),
        snapshot: None,
    };

    // Generate the trait
    println!("Generating Patient trait...");
    let patient_traits = generator.generate_trait(&patient_structure)?;

    // Display information about the generated traits
    for (i, patient_trait) in patient_traits.iter().enumerate() {
        println!("Generated trait {}: {}", i + 1, patient_trait.name);
        println!("Documentation: {:?}", patient_trait.doc_comment);
        println!("Number of methods: {}", patient_trait.methods.len());
        println!();

        // List all the methods in the trait
        println!("Trait methods:");
        for method in &patient_trait.methods {
            println!("- {}", method.name);
            if let Some(doc) = &method.doc_comment {
                println!("  Documentation: {doc}");
            }
            if let Some(return_type) = &method.return_type {
                println!("  Return type: {return_type:?}");
            }
            println!();
        }
        println!();
    }

    // Generate the trait to a file
    let output_path = "target/generated_patient_trait.rs";
    generator.generate_trait_to_file(&patient_structure, output_path)?;
    println!("Generated trait written to: {output_path}");

    // Read and display the generated code
    let generated_code = std::fs::read_to_string(output_path)?;
    println!("\nGenerated trait code:");
    println!("{}", "=".repeat(80));
    println!("{generated_code}");
    println!("{}", "=".repeat(80));

    Ok(())
}
