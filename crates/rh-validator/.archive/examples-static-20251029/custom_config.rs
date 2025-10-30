//! Custom validator configuration example
//!
//! This example shows how to configure the validator with
//! custom settings.

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, ValidatorConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Custom Configuration Examples ===\n");

    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }],
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    // Example 1: Default configuration
    println!("Example 1: Default configuration");
    let validator = FhirValidator::new()?;
    let result = validator.validate_full::<Patient>(patient_json)?;
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    // Example 2: Skip invariant validation
    println!("Example 2: Skip invariant validation");
    let config = ValidatorConfig::new().with_skip_invariants(true);
    let validator = FhirValidator::with_config(config)?;
    let result = validator.validate_full::<Patient>(patient_json)?;
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!("  (No invariants evaluated)");
    println!();

    // Example 3: Custom depth limit
    println!("Example 3: Custom depth limit (prevents deeply nested attacks)");
    let config = ValidatorConfig::new().with_max_depth(64);
    let validator = FhirValidator::with_config(config)?;

    let deeply_nested = format!(
        r#"{{
        "resourceType": "Patient",
        "id": "deep",
        "extension": [{}],
        "name": [{{"family": "Test"}}]
    }}"#,
        "{".repeat(100) + &"}".repeat(100)
    );

    match validator.validate_full::<Patient>(&deeply_nested) {
        Ok(result) => {
            println!("  Valid: {}", result.is_valid());
            if !result.is_valid() {
                println!("  First error: {}", result.issues[0].details);
            }
        }
        Err(e) => println!("  Validation error: {e}"),
    }
    println!();

    // Example 4: Chained configuration
    println!("Example 4: Chained configuration");
    let config = ValidatorConfig::new()
        .with_max_depth(128)
        .with_skip_invariants(false);

    let validator = FhirValidator::with_config(config)?;
    let result = validator.validate_full::<Patient>(patient_json)?;
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!("  Configuration: max_depth=128, invariants=enabled");
    println!();

    println!("✅ Configuration examples completed!");

    Ok(())
}
