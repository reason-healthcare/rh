//! Custom validation workflow example.
//!
//! This example demonstrates building a custom validation workflow
//! with filtering, aggregation, and reporting.

use anyhow::Result;
use rh_validator::{FhirValidator, Severity};
use serde_json::{json, Value};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("=== Custom Validation Workflow Example ===\n");

    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    // Sample resources of different types
    let resources = get_sample_resources();

    // Custom workflow: validate, filter, and aggregate
    let results = validate_batch(&validator, &resources)?;

    // Report by resource type
    report_by_resource_type(&results);

    // Report by severity
    report_by_severity(&results);

    // Report worst issues
    report_worst_issues(&results, 5);

    // Export to different formats
    export_results(&results)?;

    Ok(())
}

fn get_sample_resources() -> Vec<Value> {
    vec![
        json!({"resourceType": "Patient", "id": "p1", "name": [{"family": "Doe"}], "gender": "male"}),
        json!({"resourceType": "Patient", "id": "p2", "name": [{"family": "Smith"}]}),
        json!({"resourceType": "Patient", "id": "p3"}),
        json!({"resourceType": "Observation", "id": "o1", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}),
        json!({"resourceType": "Observation", "id": "o2", "code": {"coding": [{"system": "http://loinc.org", "code": "456"}]}}),
        json!({"id": "invalid", "data": "test"}),
    ]
}

fn validate_batch(
    validator: &FhirValidator,
    resources: &[Value],
) -> Result<Vec<(String, Value, rh_validator::ValidationResult)>> {
    let mut results = Vec::new();

    for (i, resource) in resources.iter().enumerate() {
        let default_id = format!("resource-{i}");
        let id = resource
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or(&default_id);

        let result = validator.validate(resource)?;
        results.push((id.to_string(), resource.clone(), result));
    }

    Ok(results)
}

fn report_by_resource_type(results: &[(String, Value, rh_validator::ValidationResult)]) {
    println!("=== Report by Resource Type ===");

    let mut by_type: HashMap<String, (usize, usize, usize)> = HashMap::new();

    for (_, resource, result) in results {
        let resource_type = resource
            .get("resourceType")
            .and_then(|v| v.as_str())
            .unwrap_or("(unknown)");

        let entry = by_type
            .entry(resource_type.to_string())
            .or_insert((0, 0, 0));
        entry.0 += 1; // total count

        if result.valid {
            entry.1 += 1; // valid count
        } else {
            entry.2 += result.issues.len(); // issue count
        }
    }

    for (resource_type, (total, valid, issues)) in by_type {
        println!("{resource_type}: {total} resources ({valid} valid, {issues} issues)");
    }
    println!();
}

fn report_by_severity(results: &[(String, Value, rh_validator::ValidationResult)]) {
    println!("=== Report by Severity ===");

    let mut errors = 0;
    let mut warnings = 0;
    let mut info = 0;

    for (_, _, result) in results {
        for issue in &result.issues {
            match issue.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
                Severity::Information => info += 1,
            }
        }
    }

    println!("Errors: {errors}");
    println!("Warnings: {warnings}");
    println!("Information: {info}");
    println!();
}

fn report_worst_issues(results: &[(String, Value, rh_validator::ValidationResult)], limit: usize) {
    println!("=== Top {limit} Resources by Issue Count ===");

    let mut resource_issues: Vec<_> = results
        .iter()
        .map(|(id, _, result)| (id.clone(), result.issues.len()))
        .collect();

    resource_issues.sort_by(|a, b| b.1.cmp(&a.1));

    for (id, count) in resource_issues.iter().take(limit) {
        println!("{id}: {count} issues");
    }
    println!();
}

fn export_results(results: &[(String, Value, rh_validator::ValidationResult)]) -> Result<()> {
    println!("=== Export Results ===");

    // Export as JSON summary
    let summary: Vec<_> = results
        .iter()
        .map(|(id, resource, result)| {
            json!({
                "id": id,
                "resourceType": resource.get("resourceType"),
                "valid": result.valid,
                "issueCount": result.issues.len(),
                "errors": result.issues.iter().filter(|i| matches!(i.severity, Severity::Error)).count(),
                "warnings": result.issues.iter().filter(|i| matches!(i.severity, Severity::Warning)).count(),
            })
        })
        .collect();

    println!("JSON Summary:");
    println!("{}\n", serde_json::to_string_pretty(&summary)?);

    // Export as OperationOutcome (NDJSON)
    println!("OperationOutcome (NDJSON):");
    for (id, _, result) in results {
        let operation_outcome = result.to_operation_outcome();
        // Add custom extension with resource ID
        println!(
            "{{\"resourceType\":\"OperationOutcome\",\"id\":\"{}\",\"issue\":{}}}",
            id,
            serde_json::to_string(&operation_outcome.issue)?
        );
    }

    Ok(())
}
