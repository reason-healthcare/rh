//! Batch validation example.
//!
//! This example demonstrates validating multiple FHIR resources efficiently,
//! leveraging the validator's caching for optimal performance.

use anyhow::Result;
use rh_validator::FhirValidator;
use serde_json::json;

fn main() -> Result<()> {
    println!("=== Batch Validation Example ===\n");

    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    // Create multiple resources to validate
    let resources = vec![
        json!({
            "resourceType": "Patient",
            "id": "patient1",
            "name": [{"family": "Doe", "given": ["John"]}],
            "gender": "male"
        }),
        json!({
            "resourceType": "Patient",
            "id": "patient2",
            "name": [{"family": "Smith", "given": ["Jane"]}],
            "gender": "female"
        }),
        json!({
            "resourceType": "Observation",
            "id": "obs1",
            "status": "final",
            "code": {
                "coding": [{
                    "system": "http://loinc.org",
                    "code": "29463-7"
                }]
            }
        }),
        json!({
            "resourceType": "Observation",
            "id": "obs2",
            // Missing required 'status' field
            "code": {
                "coding": [{
                    "system": "http://loinc.org",
                    "code": "8480-6"
                }]
            }
        }),
        json!({
            // Missing resourceType
            "id": "invalid1",
            "name": "Invalid"
        }),
    ];

    println!("Validating {} resources...\n", resources.len());

    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut total_issues = 0;

    for (i, resource) in resources.iter().enumerate() {
        println!("Resource {}:", i + 1);
        let result = validator.validate(resource)?;

        if result.valid {
            println!("  ✓ Valid");
            valid_count += 1;
        } else {
            println!("  ✗ Invalid ({} issues)", result.issues.len());
            invalid_count += 1;
            total_issues += result.issues.len();

            for issue in result.issues {
                println!("    - {}: {}", issue.severity, issue.message);
                if let Some(path) = &issue.path {
                    println!("      Path: {path}");
                }
            }
        }
        println!();
    }

    // Summary
    println!("=== Summary ===");
    println!("Total resources: {}", resources.len());
    println!(
        "Valid: {} ({:.1}%)",
        valid_count,
        (valid_count as f64 / resources.len() as f64) * 100.0
    );
    println!(
        "Invalid: {} ({:.1}%)",
        invalid_count,
        (invalid_count as f64 / resources.len() as f64) * 100.0
    );
    println!("Total issues: {total_issues}");

    // Cache performance
    let (prof_hits, prof_misses, prof_rate, rule_hits, rule_misses, rule_rate) =
        validator.cache_metrics();

    println!("\n=== Cache Performance ===");
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

    // Demonstrate cache benefits by validating the same resources again
    println!("\n=== Re-validating same resources (cached) ===");
    validator.reset_cache_metrics();

    for resource in &resources {
        let _ = validator.validate(resource)?;
    }

    let (prof_hits, prof_misses, prof_rate, rule_hits, rule_misses, rule_rate) =
        validator.cache_metrics();

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
    println!("(Notice the much higher cache hit rate!)");

    Ok(())
}
