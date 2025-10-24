//! Structural validation example
//!
//! This example shows how the validator catches structural errors
//! through Rust's type system and serde deserialization.

use hl7_fhir_r4_core::resources::observation::Observation;
use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Structural Validation Examples ===\n");

    let validator = FhirValidator::new()?;

    // Example 1: Missing required field
    println!("Example 1: Missing required field (status in Observation)");
    let missing_status = r#"{
        "resourceType": "Observation",
        "id": "example",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "15074-8"
            }]
        }
    }"#;

    match validator.validate_full::<Observation>(missing_status) {
        Ok(result) => {
            println!("  Valid: {}", result.is_valid());
            if !result.is_valid() {
                for issue in result.issues {
                    println!("  - {}: {}", issue.code, issue.details);
                }
            }
        }
        Err(e) => println!("  Validation error: {e}"),
    }
    println!();

    // Example 2: Wrong field type
    println!("Example 2: Wrong field type (name should be array)");
    let wrong_type = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }],
        "name": "Should be an array, not a string",
        "gender": "male"
    }"#;

    match validator.validate_full::<Patient>(wrong_type) {
        Ok(result) => {
            println!("  Valid: {}", result.is_valid());
            if !result.is_valid() {
                for issue in result.issues {
                    println!("  - {}: {}", issue.code, issue.details);
                }
            }
        }
        Err(e) => println!("  Validation error: {e}"),
    }
    println!();

    // Example 3: Invalid enum value
    println!("Example 3: Invalid enum value (gender)");
    let invalid_enum = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }],
        "name": [{
            "family": "Doe"
        }],
        "gender": "invalid-gender-value"
    }"#;

    match validator.validate_full::<Patient>(invalid_enum) {
        Ok(result) => {
            println!("  Valid: {}", result.is_valid());
            if !result.is_valid() {
                for issue in result.issues {
                    println!("  - {}: {}", issue.code, issue.details);
                }
            }
        }
        Err(e) => println!("  Validation error: {e}"),
    }
    println!();

    // Example 4: Malformed JSON
    println!("Example 4: Malformed JSON");
    let malformed = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe"
        }
        missing closing bracket
    }"#;

    match validator.validate_full::<Patient>(malformed) {
        Ok(result) => {
            println!("  Valid: {}", result.is_valid());
            if !result.is_valid() {
                for issue in result.issues {
                    println!("  - {}: {}", issue.code, issue.details);
                }
            }
        }
        Err(e) => println!("  Validation error: {e}"),
    }
    println!();

    println!("✅ Structural validation examples completed!");

    Ok(())
}
