//! Error handling example
//!
//! This example demonstrates different types of validation errors
//! and how to handle them appropriately.

use hl7_fhir_r4_core::resources::observation::Observation;
use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, IssueCode, Severity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Error Handling Examples ===\n");

    let validator = FhirValidator::new()?;

    // Example 1: Inspect all validation issues
    println!("Example 1: Inspecting validation issues");
    let invalid_patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": "wrong type",
        "gender": 123
    }"#;

    let result = validator.validate_full::<Patient>(invalid_patient)?;
    println!("  Total issues: {}", result.issues.len());

    for (i, issue) in result.issues.iter().enumerate() {
        println!("  Issue {}:", i + 1);
        println!("    Severity: {}", issue.severity);
        println!("    Code: {}", issue.code);
        println!("    Details: {}", issue.details);
        if let Some(location) = &issue.location {
            println!("    Location: {location}");
        }
        if let Some(expr) = &issue.expression {
            println!("    Expression: {expr}");
        }
    }
    println!();

    // Example 2: Filter by severity
    println!("Example 2: Filtering issues by severity");
    let result = validator.validate_full::<Patient>(invalid_patient)?;

    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .collect();
    let warnings: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .collect();

    println!("  Errors: {}", errors.len());
    for error in errors {
        println!("    - {}", error.details);
    }

    println!("  Warnings: {}", warnings.len());
    for warning in warnings {
        println!("    - {}", warning.details);
    }
    println!();

    // Example 3: Filter by issue code
    println!("Example 3: Filtering by issue code");
    let result = validator.validate_full::<Patient>(invalid_patient)?;

    let structural_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == IssueCode::Structure)
        .collect();
    let invariant_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == IssueCode::Invariant)
        .collect();

    println!("  Structural errors: {}", structural_errors.len());
    println!("  Invariant errors: {}", invariant_errors.len());
    println!();

    // Example 4: Handling invariant-specific errors
    println!("Example 4: Identifying which invariants failed");
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

    let result = validator.validate_full::<Observation>(missing_status)?;

    for issue in result.issues {
        if let Some(inv_key) = &issue.invariant_key {
            println!("  Invariant {inv_key} failed:");
            println!("    {}", issue.details);
        }
    }
    println!();

    // Example 5: Converting to JSON for external tools
    println!("Example 5: Serializing validation results to JSON");
    let result = validator.validate_full::<Patient>(invalid_patient)?;
    let json = serde_json::to_string_pretty(&result)?;
    println!("  JSON output:");
    println!("{json}");
    println!();

    // Example 6: Custom error handling logic
    println!("Example 6: Custom error handling");
    let result = validator.validate_full::<Patient>(invalid_patient)?;

    if result.is_valid() {
        println!("  ✅ Resource is valid");
    } else if result.error_count() > 0 {
        println!(
            "  ❌ Resource has {} errors - must fix",
            result.error_count()
        );
        // In a real app, you might return an error here
    } else if result.warning_count() > 0 {
        println!(
            "  ⚠️  Resource has {} warnings - should review",
            result.warning_count()
        );
        // In a real app, you might log warnings but continue
    }
    println!();

    println!("✅ Error handling examples completed!");

    Ok(())
}
