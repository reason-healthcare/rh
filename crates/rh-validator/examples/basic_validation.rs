//! Basic FHIR validation example.
//!
//! This example demonstrates simple validation of a FHIR Patient resource
//! against the base FHIR R4 specification.

use anyhow::Result;
use rh_validator::FhirValidator;
use serde_json::json;

fn main() -> Result<()> {
    println!("=== Basic FHIR Validation Example ===\n");

    // Create a validator without additional packages (uses base FHIR R4 only)
    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    // Example 1: Valid minimal Patient resource
    println!("1. Valid minimal Patient:");
    let valid_patient = json!({
        "resourceType": "Patient",
        "id": "example",
    });

    let result = validator.validate(&valid_patient)?;
    println!("   Valid: {}", result.valid);
    println!("   Issues: {}\n", result.issues.len());

    // Example 2: Missing resourceType
    println!("2. Missing resourceType:");
    let invalid_resource = json!({
        "id": "example",
        "name": [{"family": "Doe"}]
    });

    let result = validator.validate(&invalid_resource)?;
    println!("   Valid: {}", result.valid);
    for issue in result.issues {
        println!("   - {}: {}", issue.severity, issue.message);
    }
    println!();

    // Example 3: Not a JSON object
    println!("3. Invalid structure (not an object):");
    let not_object = json!("just a string");

    let result = validator.validate(&not_object)?;
    println!("   Valid: {}", result.valid);
    for issue in result.issues {
        println!("   - {}: {}", issue.severity, issue.message);
    }
    println!();

    // Example 4: Patient with complete data
    println!("4. Complete Patient resource:");
    let complete_patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "active": true,
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "Robert"]
            }
        ],
        "gender": "male",
        "birthDate": "1974-12-25",
        "address": [
            {
                "use": "home",
                "line": ["123 Main St"],
                "city": "Boston",
                "state": "MA",
                "postalCode": "02101",
                "country": "US"
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john.doe@example.com"
            }
        ]
    });

    let result = validator.validate(&complete_patient)?;
    println!("   Valid: {}", result.valid);
    println!("   Issues: {}", result.issues.len());

    // Show cache metrics
    let (prof_hits, prof_misses, prof_rate, rule_hits, rule_misses, rule_rate) =
        validator.cache_metrics();
    println!("\n=== Cache Metrics ===");
    println!(
        "Profile cache: {:.1}% hit rate ({} hits, {} misses)",
        prof_rate * 100.0,
        prof_hits,
        prof_misses
    );
    println!(
        "Rule cache: {:.1}% hit rate ({} hits, {} misses)",
        rule_rate * 100.0,
        rule_hits,
        rule_misses
    );

    Ok(())
}
