use anyhow::Result;
use rh_codegen::{CodeGenerator, CodegenConfig};
use rh_codegen::fhir_types::{ElementDefinition, StructureDefinition, ElementType, StructureDefinitionDifferential};
use std::path::Path;

/// Create a test structure definition with array fields to test array handling
fn create_test_structure_with_arrays() -> StructureDefinition {
    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "TestArrayResource".to_string(),
        url: "http://example.org/fhir/StructureDefinition/TestArrayResource".to_string(),
        name: "TestArrayResource".to_string(),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "TestArrayResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: Some(StructureDefinitionDifferential {
            element: vec![
                ElementDefinition {
                    id: Some("TestArrayResource".to_string()),
                    path: "TestArrayResource".to_string(),
                    short: None,
                    definition: None,
                    min: Some(0),
                    max: Some("*".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("TestArrayResource".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
                ElementDefinition {
                    id: Some("TestArrayResource.stringArray".to_string()),
                    path: "TestArrayResource.stringArray".to_string(),
                    short: None,
                    definition: None,
                    min: Some(0),
                    max: Some("*".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("string".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
                ElementDefinition {
                    id: Some("TestArrayResource.requiredArray".to_string()),
                    path: "TestArrayResource.requiredArray".to_string(),
                    short: None,
                    definition: None,
                    min: Some(1),
                    max: Some("*".to_string()),
                    element_type: Some(vec![ElementType {
                        code: Some("CodeableConcept".to_string()),
                        target_profile: None,
                    }]),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
            ],
        }),
        snapshot: None,
        version: None,
        title: None,
        description: None,
        purpose: None,
    }
}

fn main() -> Result<()> {
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    let output_dir = Path::new("./test_array_fix");
    let src_dir = output_dir.join("src");

    // Clean up any existing output
    if output_dir.exists() {
        std::fs::remove_dir_all(output_dir)?;
    }
    std::fs::create_dir_all(&src_dir)?;

    println!("Testing array handling fix...");

    // Create a test structure with arrays
    let test_structure = create_test_structure_with_arrays();
    
    // Generate the struct to test array handling
    generator.generate_to_organized_directories(&test_structure, &src_dir)?;

    println!("✅ Generated test files for array handling verification");
    println!("📁 Files generated in: ./test_array_fix/");

    Ok(())
}
