//! Batch validation example
//!
//! This example demonstrates high-performance parallel batch validation
//! using Rayon for concurrent processing of multiple FHIR resources.

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Batch Validation Example ===\n");

    let validator = FhirValidator::new()?;

    // Example 1: Batch validation of multiple resources
    println!("Example 1: Batch validation (parallel processing)");
    let resources = vec![
        r#"{"resourceType": "Patient", "id": "patient1", "active": true}"#,
        r#"{"resourceType": "Patient", "id": "patient2", "gender": "male"}"#,
        r#"{"resourceType": "Patient", "id": "patient3", "birthDate": "1990-01-01"}"#,
        r#"{"resourceType": "Patient", "id": "patient4", "active": false}"#,
        r#"{"resourceType": "Patient", "id": "patient5", "gender": "female"}"#,
    ];

    let start = Instant::now();
    let results = validator.validate_batch::<Patient>(&resources)?;
    let elapsed = start.elapsed();

    println!("  Validated {} resources in {:?}", results.len(), elapsed);
    println!("  Results:");
    for (i, result) in results.iter().enumerate() {
        println!(
            "    Resource {}: {} (errors: {}, warnings: {})",
            i + 1,
            if result.is_valid() {
                "✅ Valid"
            } else {
                "❌ Invalid"
            },
            result.error_count(),
            result.warning_count()
        );
    }
    println!();

    // Example 2: Large batch validation
    println!("Example 2: Large batch validation (100 resources)");
    let large_batch: Vec<String> = (1..=100)
        .map(|i| {
            format!(
                r#"{{"resourceType": "Patient", "id": "patient{i}", "active": {}}}"#,
                i % 2 == 0
            )
        })
        .collect();

    let resource_refs: Vec<&str> = large_batch.iter().map(|s| s.as_str()).collect();

    let start = Instant::now();
    let results = validator.validate_batch::<Patient>(&resource_refs)?;
    let elapsed = start.elapsed();

    let valid_count = results.iter().filter(|r| r.is_valid()).count();
    let len = results.len();
    println!("  Validated {len} resources in {elapsed:?}");
    println!("  Valid: {valid_count}");
    println!(
        "  Average: {:.2}ms per resource",
        elapsed.as_secs_f64() * 1000.0 / len as f64
    );
    println!();

    // Example 3: NDJSON validation
    println!("Example 3: NDJSON validation (newline-delimited JSON)");
    let ndjson = r#"{"resourceType": "Patient", "id": "patient1", "active": true}
{"resourceType": "Patient", "id": "patient2", "gender": "male"}
# This is a comment - will be ignored
{"resourceType": "Patient", "id": "patient3", "birthDate": "1990-01-01"}

{"resourceType": "Patient", "id": "patient4", "active": false}
{"resourceType": "Patient", "name": "invalid-type"}"#;

    let start = Instant::now();
    let results = validator.validate_ndjson::<Patient>(ndjson)?;
    let elapsed = start.elapsed();

    println!("  Validated {} resources in {:?}", results.len(), elapsed);
    println!("  Results:");
    for (line_num, result) in &results {
        println!(
            "    Line {}: {} (errors: {}, warnings: {})",
            line_num,
            if result.is_valid() {
                "✅ Valid"
            } else {
                "❌ Invalid"
            },
            result.error_count(),
            result.warning_count()
        );
        if result.has_errors() {
            for issue in &result.issues {
                if issue.severity == rh_validator::Severity::Error {
                    println!("      Error: {}", issue.details);
                }
            }
        }
    }
    println!();

    // Example 4: Performance comparison
    println!("Example 4: Parallel vs Sequential comparison");

    let test_resources: Vec<String> = (1..=50)
        .map(|i| format!(r#"{{"resourceType": "Patient", "id": "test{i}"  }}"#))
        .collect();
    let test_refs: Vec<&str> = test_resources.iter().map(|s| s.as_str()).collect();

    // Sequential
    let start = Instant::now();
    for json in &test_refs {
        let _ = validator.validate_full::<Patient>(json)?;
    }
    let sequential_time = start.elapsed();

    // Parallel
    let start = Instant::now();
    let _ = validator.validate_batch::<Patient>(&test_refs)?;
    let parallel_time = start.elapsed();

    println!("  50 resources:");
    println!("    Sequential: {sequential_time:?}");
    println!("    Parallel:   {parallel_time:?}");
    println!(
        "    Speedup:    {:.2}x",
        sequential_time.as_secs_f64() / parallel_time.as_secs_f64()
    );
    println!();

    // Example 5: Error handling in batch
    println!("Example 5: Handling errors in batch validation");
    let mixed_resources = vec![
        r#"{"resourceType": "Patient", "id": "valid1"}"#,
        r#"{"resourceType": "Patient", "name": "should-be-array"}"#,
        r#"{"resourceType": "Patient", "id": "valid2"}"#,
        r#"{"invalid json"#,
    ];

    let results = validator.validate_batch::<Patient>(&mixed_resources)?;
    println!("  Validated {} resources", results.len());
    println!("  Summary:");
    let valid = results.iter().filter(|r| !r.has_errors()).count();
    let errors = results.iter().filter(|r| r.has_errors()).count();
    println!("    ✅ Valid: {valid}");
    println!("    ❌ Errors: {errors}");
    println!();

    println!("🎉 Batch validation enables high-performance validation of large datasets!");
    println!("   Use validate_batch() for arrays of JSON strings");
    println!("   Use validate_ndjson() for newline-delimited JSON files");

    Ok(())
}
