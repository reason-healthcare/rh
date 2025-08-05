//! Test to verify the Element/ElementDefinition nested struct fix
//!
//! This test generates Element and ElementDefinition structures to ensure
//! that nested structs are correctly assigned to their parent files.

use rh_codegen::{CodeGenerator, CodegenConfig};
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Create test structures that mimic Element and ElementDefinition
    let element_structure = create_element_structure_definition();
    let element_definition_structure = create_element_definition_structure_definition();

    let output_dir = Path::new("./test-nested-struct-fix");
    let src_dir = output_dir.join("src");

    // Clean up any existing output
    if output_dir.exists() {
        std::fs::remove_dir_all(output_dir)?;
    }
    std::fs::create_dir_all(&src_dir)?;

    println!("Testing nested struct assignment fix...");

    // Generate Element - this should only contain Element and its direct children
    generator.generate_to_organized_directories(&element_structure, &src_dir)?;
    
    // Generate ElementDefinition - this should only contain ElementDefinition and its direct children
    generator.generate_to_organized_directories(&element_definition_structure, &src_dir)?;

    println!("✅ Generated test files to verify nested struct fix");
    println!("📁 Check files in: {}/datatypes/", src_dir.display());
    println!("🔍 element.rs should only contain Element and Element* structs");
    println!("🔍 element_definition.rs should only contain ElementDefinition and ElementDefinition* structs");

    Ok(())
}

fn create_element_structure_definition() -> rh_codegen::fhir_types::StructureDefinition {
    use rh_codegen::fhir_types::*;

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Element".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Element".to_string(),
        name: "Element".to_string(),
        title: Some("Element".to_string()),
        status: "active".to_string(),
        description: Some("Base definition for all elements in a resource.".to_string()),
        purpose: None,
        kind: "complex-type".to_string(),
        is_abstract: false,
        base_type: "Element".to_string(),
        base_definition: None,
        version: Some("4.0.1".to_string()),
        differential: None,
        snapshot: None,
    }
}

fn create_element_definition_structure_definition() -> rh_codegen::fhir_types::StructureDefinition {
    use rh_codegen::fhir_types::*;

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "ElementDefinition".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/ElementDefinition".to_string(),
        name: "ElementDefinition".to_string(),
        title: Some("ElementDefinition".to_string()),
        status: "active".to_string(),
        description: Some("Captures constraints on each element within the resource, profile, or extension.".to_string()),
        purpose: None,
        kind: "complex-type".to_string(),
        is_abstract: false,
        base_type: "BackboneElement".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/BackboneElement".to_string()),
        version: Some("4.0.1".to_string()),
        differential: None,
        snapshot: None,
    }
}
