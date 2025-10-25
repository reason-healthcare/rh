//! Invariant validation example
//!
//! This example demonstrates FHIRPath invariant evaluation.
//! Invariants are business rules defined in FHIR that go beyond
//! structural validation.

use hl7_fhir_r4_core::prelude::*;
use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, Severity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Invariant Validation Examples ===\n");

    let validator = FhirValidator::new()?;

    // Show what invariants exist for Patient
    println!(
        "Patient resource has {} invariants:",
        Patient::invariants().len()
    );
    for inv in Patient::invariants().iter().take(5) {
        println!("  - {}: {} ({})", inv.key, inv.human, inv.severity);
    }
    println!();

    // Example 1: Valid Patient (passes all invariants)
    println!("Example 1: Valid Patient (should pass all invariants)");
    let valid_patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/fhir/StructureDefinition/ethnicity",
            "valueString": "Hispanic"
        }],
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["Jane"]
        }],
        "gender": "female",
        "birthDate": "1990-01-01"
    }"#;

    let patient: Patient = serde_json::from_str(valid_patient)?;
    let issues = validator.validate_invariants(&patient)?;

    let errors: Vec<_> = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .collect();
    let warnings: Vec<_> = issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .collect();

    println!("  Total issues: {}", issues.len());
    println!("  Errors: {}", errors.len());
    println!("  Warnings: {}", warnings.len());

    if !errors.is_empty() {
        println!("  Error details:");
        for issue in errors {
            println!(
                "    - {}: {}",
                issue.invariant_key.as_deref().unwrap_or("unknown"),
                issue.details
            );
        }
    }
    println!();

    // Example 2: Patient with contact that violates pat-1
    println!("Example 2: Patient with contact lacking details (violates pat-1)");
    let invalid_contact = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }],
        "name": [{
            "family": "Smith"
        }],
        "contact": [{
            "relationship": [{
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0131",
                    "code": "N"
                }]
            }]
        }]
    }"#;

    let patient: Patient = serde_json::from_str(invalid_contact)?;
    let issues = validator.validate_invariants(&patient)?;

    let errors: Vec<_> = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .collect();

    println!("  Total issues: {}", issues.len());
    println!("  Errors: {}", errors.len());

    if !errors.is_empty() {
        println!("  Error details:");
        for issue in &errors {
            if let Some(key) = &issue.invariant_key {
                println!("    - {}: {}", key, issue.details);
                if let Some(expr) = &issue.expression {
                    println!("      Expression: {expr}");
                }
            }
        }
    }
    println!();

    // Example 3: Combined structural + invariant validation
    println!("Example 3: Full validation (structural + invariants)");
    let result = validator.validate_full::<Patient>(valid_patient)?;

    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());

    if result.warning_count() > 0 {
        println!("  Note: Warnings may include FHIRPath expressions not yet supported");
    }
    println!();

    println!("✅ Invariant validation examples completed!");

    Ok(())
}
