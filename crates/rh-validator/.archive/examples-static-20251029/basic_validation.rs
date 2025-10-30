//! Basic FHIR resource validation example
//!
//! This example demonstrates the simplest use case:
//! validating a FHIR resource from a JSON string.

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic FHIR Validation Example ===\n");

    // Create a validator with default configuration
    let validator = FhirValidator::new()?;

    // Example 1: Valid Patient resource
    println!("Example 1: Validating a valid Patient resource");
    let valid_patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/fhir/StructureDefinition/patient-importance",
            "valueString": "VIP"
        }],
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["John", "Q"]
        }],
        "gender": "male",
        "birthDate": "1974-12-25"
    }"#;

    let result = validator.validate_full::<Patient>(valid_patient)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    // Example 2: Invalid Patient (structural error)
    println!("Example 2: Validating an invalid Patient (wrong type)");
    let invalid_patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": "Not an array",
        "gender": "male"
    }"#;

    let result = validator.validate_full::<Patient>(invalid_patient)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    if result.error_count() > 0 {
        println!("  First error: {}", result.issues[0].details);
    }
    println!();

    // Example 3: Patient with warnings
    println!("Example 3: Validating a Patient with unknown fields");
    let patient_with_unknown = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/fhir/StructureDefinition/test",
            "valueString": "test"
        }],
        "name": [{
            "family": "Smith"
        }],
        "unknownField": "This will be ignored by serde",
        "gender": "female"
    }"#;

    let result = validator.validate_full::<Patient>(patient_with_unknown)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    println!("✅ Basic validation examples completed successfully!");

    Ok(())
}
