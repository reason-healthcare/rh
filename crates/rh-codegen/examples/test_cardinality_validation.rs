use rh_codegen::{CodeGenerator, CodegenConfig, StructureDefinition};
use serde_json::json;

/// Test to validate cardinality-based return type handling
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing cardinality-based return type handling...");

    // Create a test structure definition with various cardinalities
    let test_structure = json!({
        "resourceType": "StructureDefinition",
        "id": "test-cardinality-resource",
        "url": "http://example.org/StructureDefinition/TestCardinality",
        "version": "1.0.0",
        "name": "TestCardinality",
        "title": "Test Cardinality Resource",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "TestCardinality",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "differential": {
            "element": [
                {
                    "id": "TestCardinality",
                    "path": "TestCardinality",
                    "definition": "Test resource for cardinality handling",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "TestCardinality.requiredField",
                    "path": "TestCardinality.requiredField",
                    "definition": "A required field (min=1)",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "string"}]
                },
                {
                    "id": "TestCardinality.optionalField",
                    "path": "TestCardinality.optionalField",
                    "definition": "An optional field (min=0)",
                    "min": 0,
                    "max": "1",
                    "type": [{"code": "string"}]
                },
                {
                    "id": "TestCardinality.requiredArray",
                    "path": "TestCardinality.requiredArray",
                    "definition": "A required array (min=1, max=*)",
                    "min": 1,
                    "max": "*",
                    "type": [{"code": "string"}]
                },
                {
                    "id": "TestCardinality.optionalArray",
                    "path": "TestCardinality.optionalArray",
                    "definition": "An optional array (min=0, max=*)",
                    "min": 0,
                    "max": "*",
                    "type": [{"code": "string"}]
                },
                {
                    "id": "TestCardinality.requiredCodeableConcept",
                    "path": "TestCardinality.requiredCodeableConcept",
                    "definition": "A required CodeableConcept (min=1)",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}]
                },
                {
                    "id": "TestCardinality.optionalCodeableConcept",
                    "path": "TestCardinality.optionalCodeableConcept",
                    "definition": "An optional CodeableConcept (min=0)",
                    "min": 0,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}]
                }
            ]
        }
    });

    // Parse the structure definition
    let structure_def: StructureDefinition = serde_json::from_value(test_structure)?;

    // Create generator
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Generate trait implementation
    let traits = generator.generate_trait(&structure_def)?;

    // Display information about the generated traits
    println!(
        "Generated {} trait(s) for cardinality validation",
        traits.len()
    );
    for (i, trait_def) in traits.iter().enumerate() {
        println!(
            "Trait {}: {} with {} methods",
            i + 1,
            trait_def.name,
            trait_def.methods.len()
        );
    }

    // Check that the generated code has the correct return types
    assert!(
        trait_impl.contains("fn required_field(&self) -> String"),
        "Required field should return String directly (not Option)"
    );

    assert!(
        trait_impl.contains("fn optional_field(&self) -> Option<String>"),
        "Optional field should return Option<String>"
    );

    assert!(
        trait_impl.contains("fn required_array(&self) -> Vec<String>"),
        "Required array should return Vec<String> directly"
    );

    assert!(
        trait_impl.contains("fn optional_array(&self) -> Vec<String>"),
        "Optional array should return Vec<String> (arrays are never Option<Vec<T>>)"
    );

    assert!(
        trait_impl.contains("fn required_codeable_concept(&self) -> CodeableConcept"),
        "Required CodeableConcept should return CodeableConcept directly"
    );

    assert!(
        trait_impl.contains("fn optional_codeable_concept(&self) -> Option<CodeableConcept>"),
        "Optional CodeableConcept should return Option<CodeableConcept>"
    );

    println!("✅ All cardinality return type checks passed!");

    // Now let's test the fix you mentioned about Account.type_
    // Let's simulate an Account resource with a type field that has min=0
    let account_structure = json!({
        "resourceType": "StructureDefinition",
        "id": "Account",
        "url": "http://hl7.org/fhir/StructureDefinition/Account",
        "version": "4.0.1",
        "name": "Account",
        "title": "Account",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "Account",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "differential": {
            "element": [
                {
                    "id": "Account",
                    "path": "Account",
                    "definition": "A financial tool for tracking value",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "Account.type",
                    "path": "Account.type",
                    "definition": "E.g. patient, expense, depreciation",
                    "min": 0,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}]
                }
            ]
        }
    });

    let account_def: StructureDefinition = serde_json::from_value(account_structure)?;
    let account_trait_impl = generator
        .trait_impl_generator
        .generate_trait_implementation(&account_def)?;

    println!("\nAccount trait implementation:\n{}", account_trait_impl);

    // This should now return Option<CodeableConcept> since min=0
    assert!(
        account_trait_impl.contains("fn type_(&self) -> Option<CodeableConcept>"),
        "Account.type_ should return Option<CodeableConcept> since min=0"
    );

    println!("✅ Account.type_ cardinality fix verified!");

    Ok(())
}
