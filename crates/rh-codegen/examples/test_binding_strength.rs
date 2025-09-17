use anyhow::Result;
use rh_codegen::fhir_types::{ElementBinding, ElementDefinition, ElementType};
use rh_codegen::generators::DocumentationGenerator;
use rh_codegen::value_sets::ValueSetManager;

fn main() -> Result<()> {
    println!("Testing binding strength documentation generation...");

    let value_set_manager = ValueSetManager::new();

    // Create a test element with extensible binding
    let element_with_extensible_binding = ElementDefinition {
        id: None,
        path: "Test.statusCode".to_string(),
        short: Some("Status of the test".to_string()),
        definition: Some("The status code for this test resource".to_string()),
        min: Some(0),
        max: Some("1".to_string()),
        element_type: Some(vec![ElementType {
            code: Some("code".to_string()),
            target_profile: None,
        }]),
        fixed: None,
        pattern: None,
        binding: Some(ElementBinding {
            strength: "extensible".to_string(),
            description: Some("Codes representing test status".to_string()),
            value_set: Some("http://hl7.org/fhir/ValueSet/test-status".to_string()),
        }),
    };

    // Create a test element with required binding
    let element_with_required_binding = ElementDefinition {
        id: None,
        path: "Test.requiredCode".to_string(),
        short: Some("Required code field".to_string()),
        definition: Some("A required code field that should generate an enum".to_string()),
        min: Some(1),
        max: Some("1".to_string()),
        element_type: Some(vec![ElementType {
            code: Some("code".to_string()),
            target_profile: None,
        }]),
        fixed: None,
        pattern: None,
        binding: Some(ElementBinding {
            strength: "required".to_string(),
            description: Some("Required codes".to_string()),
            value_set: Some("http://hl7.org/fhir/ValueSet/required-status".to_string()),
        }),
    };

    // Test extensible binding documentation
    println!("\n=== Testing Extensible Binding ===");
    if let Some(doc) = DocumentationGenerator::generate_field_documentation_with_binding(
        &element_with_extensible_binding,
        &value_set_manager,
    ) {
        println!("Documentation for extensible binding:");
        println!("{}", doc);
    } else {
        println!("No documentation generated for extensible binding");
    }

    // Test required binding documentation
    println!("\n=== Testing Required Binding ===");
    if let Some(doc) = DocumentationGenerator::generate_field_documentation_with_binding(
        &element_with_required_binding,
        &value_set_manager,
    ) {
        println!("Documentation for required binding:");
        println!("{}", doc);
    } else {
        println!("No documentation generated for required binding");
    }

    // Test choice field with extensible binding
    println!("\n=== Testing Choice Field with Extensible Binding ===");
    if let Some(doc) = DocumentationGenerator::generate_choice_field_documentation_with_binding(
        &element_with_extensible_binding,
        "code",
        &value_set_manager,
    ) {
        println!("Documentation for choice field with extensible binding:");
        println!("{}", doc);
    } else {
        println!("No documentation generated for choice field with extensible binding");
    }

    println!("\n✅ Binding strength documentation test completed!");
    Ok(())
}
