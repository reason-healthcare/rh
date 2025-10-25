//! Example demonstrating FHIR cardinality validation
//!
//! Cardinality constraints define the minimum and maximum occurrences allowed
//! for each element in a FHIR resource. This example shows how to:
//! - Access cardinality metadata from generated resources
//! - Validate resources against cardinality constraints
//! - Skip cardinality validation when needed
use hl7_fhir_r4_core::resources::patient::Patient;
use hl7_fhir_r4_core::validation::ValidatableResource;
use rh_validator::{FhirValidator, ValidatorConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIR Cardinality Validation Example ===\n");

    // Example 1: Access cardinality metadata
    println!("1. Cardinality Metadata");
    println!("   ---------------------");
    let cardinalities = Patient::cardinalities();
    println!(
        "   Patient has {} cardinality constraints",
        cardinalities.len()
    );

    // Show some examples
    for card in cardinalities.iter().take(10) {
        println!(
            "   - {}: {} (min={}, max={:?}{})",
            card.path,
            card.to_fhir_notation(),
            card.min,
            card.max,
            if card.is_required() {
                " REQUIRED"
            } else if card.is_unbounded() {
                " UNBOUNDED"
            } else {
                ""
            }
        );
    }

    // Example 2: Validate a resource with cardinality checking
    println!("\n2. Validate Resource with Cardinality");
    println!("   ----------------------------------");

    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "active": true,
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }]
    }"#;

    // Skip invariant and binding validation to focus on cardinality
    let config = ValidatorConfig::default()
        .with_skip_invariants(true)
        .with_skip_bindings(true);
    let validator = FhirValidator::with_config(config)?;

    let result = validator.validate_full::<Patient>(patient_json)?;

    println!("   Valid: {}", result.is_valid());
    println!("   Issues: {}", result.issues.len());

    for issue in &result.issues {
        println!("   - [{:?}] {}", issue.severity, issue.details);
    }

    // Example 3: Skip cardinality validation
    println!("\n3. Skip Cardinality Validation");
    println!("   ---------------------------");

    let config = ValidatorConfig::default()
        .with_skip_invariants(true)
        .with_skip_bindings(true)
        .with_skip_cardinality(true);

    let validator = FhirValidator::with_config(config)?;
    let result = validator.validate_full::<Patient>(patient_json)?;

    println!("   Valid: {}", result.is_valid());
    println!(
        "   Issues: {} (all validation skipped)",
        result.issues.len()
    );

    // Example 4: Cardinality helpers
    println!("\n4. Cardinality Helper Methods");
    println!("   --------------------------");

    // Find a specific cardinality
    if let Some(id_card) = cardinalities.iter().find(|c| c.path == "Patient.id") {
        println!("   Patient.id cardinality:");
        println!("     - Notation: {}", id_card.to_fhir_notation());
        println!("     - Required: {}", id_card.is_required());
        println!("     - Array: {}", id_card.is_array());
        println!("     - Unbounded: {}", id_card.is_unbounded());
    }

    if let Some(name_card) = cardinalities.iter().find(|c| c.path == "Patient.name") {
        println!("\n   Patient.name cardinality:");
        println!("     - Notation: {}", name_card.to_fhir_notation());
        println!("     - Required: {}", name_card.is_required());
        println!("     - Array: {}", name_card.is_array());
        println!("     - Unbounded: {}", name_card.is_unbounded());
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
