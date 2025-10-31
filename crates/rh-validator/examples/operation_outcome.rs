//! OperationOutcome output example.
//!
//! This example shows how to convert validation results to FHIR OperationOutcome
//! resources for standards-compliant error reporting.

use anyhow::Result;
use rh_validator::FhirValidator;
use serde_json::json;

fn main() -> Result<()> {
    println!("=== OperationOutcome Example ===\n");

    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    // Example 1: Single validation error
    println!("1. Single validation error:");
    let invalid_resource = json!({
        "id": "example",
        "name": "Test"
    });

    let result = validator.validate(&invalid_resource)?;
    let operation_outcome = result.to_operation_outcome();

    println!("{}\n", serde_json::to_string_pretty(&operation_outcome)?);

    // Example 2: Multiple validation issues
    println!("2. Multiple validation issues:");
    let patient_with_issues = json!({
        "resourceType": "Patient",
        "id": "example",
        // Various issues will be detected
    });

    let result = validator.validate(&patient_with_issues)?;
    let operation_outcome = result.to_operation_outcome();

    println!("{}\n", serde_json::to_string_pretty(&operation_outcome)?);

    // Example 3: Valid resource
    println!("3. Valid resource (no issues):");
    let valid_patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{"family": "Doe"}],
        "gender": "male"
    });

    let result = validator.validate(&valid_patient)?;
    let operation_outcome = result.to_operation_outcome();

    println!("{}\n", serde_json::to_string_pretty(&operation_outcome)?);

    // Example 4: Accessing individual issues
    println!("4. Programmatic access to issues:");
    let result = validator.validate(&invalid_resource)?;
    let operation_outcome = result.to_operation_outcome();

    println!("Resource type: {}", operation_outcome.resource_type);
    println!("Number of issues: {}", operation_outcome.issue.len());

    for (i, issue) in operation_outcome.issue.iter().enumerate() {
        println!("\nIssue {}:", i + 1);
        println!("  Severity: {}", issue.severity);
        println!("  Code: {}", issue.code);
        if let Some(diagnostics) = &issue.diagnostics {
            println!("  Diagnostics: {diagnostics}");
        }
        if let Some(location) = &issue.location {
            println!("  Location: {location:?}");
        }
        if let Some(expression) = &issue.expression {
            println!("  Expression: {expression:?}");
        }
    }

    // Example 5: NDJSON format for batch processing
    println!("\n5. NDJSON format for batch:");
    let resources = vec![invalid_resource, valid_patient, patient_with_issues];

    for resource in resources {
        let result = validator.validate(&resource)?;
        let operation_outcome = result.to_operation_outcome();
        // Each OperationOutcome on one line (NDJSON)
        println!("{}", serde_json::to_string(&operation_outcome)?);
    }

    Ok(())
}
