//! Multi-Resource Type Validation Example
//!
//! This example demonstrates validating different FHIR resource types
//! using automatic resource type detection.

use rh_validator::FhirValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multi-Resource Type Validation Example ===\n");

    // Create validator
    let validator = FhirValidator::new()?;

    // Example 1: Validate a Patient resource
    println!("1. Validating Patient resource:");
    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "text": {
            "status": "generated",
            "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">John Doe</div>"
        },
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    let result = validator.validate_any_resource(patient_json)?;
    print_result("Patient", &result);

    // Example 2: Validate an Observation resource
    println!("\n2. Validating Observation resource:");
    let observation_json = r#"{
        "resourceType": "Observation",
        "id": "example",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "15074-8",
                "display": "Glucose"
            }]
        },
        "subject": {
            "reference": "Patient/example"
        }
    }"#;

    let result = validator.validate_any_resource(observation_json)?;
    print_result("Observation", &result);

    // Example 3: Validate an Organization resource
    println!("\n3. Validating Organization resource:");
    let organization_json = r#"{
        "resourceType": "Organization",
        "id": "example",
        "name": "Health Level Seven International",
        "alias": ["HL7 International"]
    }"#;

    let result = validator.validate_any_resource(organization_json)?;
    print_result("Organization", &result);

    // Example 4: Validate a Practitioner resource
    println!("\n4. Validating Practitioner resource:");
    let practitioner_json = r#"{
        "resourceType": "Practitioner",
        "id": "example",
        "name": [{
            "family": "Smith",
            "given": ["Jane"],
            "prefix": ["Dr."]
        }]
    }"#;

    let result = validator.validate_any_resource(practitioner_json)?;
    print_result("Practitioner", &result);

    // Example 5: Invalid resource type
    println!("\n5. Validating with unknown resource type:");
    let invalid_type_json = r#"{
        "resourceType": "InvalidType",
        "id": "example"
    }"#;

    let result = validator.validate_any_resource(invalid_type_json)?;
    print_result("InvalidType", &result);

    // Example 6: Typo in resource type (should suggest correction)
    println!("\n6. Validating with typo in resource type:");
    let typo_json = r#"{
        "resourceType": "Patint",
        "id": "example"
    }"#;

    let result = validator.validate_any_resource(typo_json)?;
    print_result("Patint (typo)", &result);

    // Example 7: Missing resource type
    println!("\n7. Validating without resourceType field:");
    let missing_type_json = r#"{
        "id": "example",
        "name": "Test"
    }"#;

    match validator.validate_any_resource(missing_type_json) {
        Ok(result) => print_result("Missing resourceType", &result),
        Err(e) => println!("   ❌ Error: {e}"),
    }

    // Example 8: Batch validation with mixed resource types
    println!("\n8. Batch validation with mixed resource types:");
    let mixed_ndjson = r#"{"resourceType": "Patient", "id": "1", "text": {"status": "generated", "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">P1</div>"}, "gender": "male"}
{"resourceType": "Observation", "id": "2", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}
{"resourceType": "Organization", "id": "3", "name": "Acme Corp"}
{"resourceType": "Practitioner", "id": "4", "name": [{"family": "Jones"}]}"#;

    let batch_results = validator.validate_ndjson_any(mixed_ndjson)?;
    println!("   Total resources validated: {}", batch_results.len());
    println!(
        "   Valid: {}",
        batch_results.iter().filter(|(_, r)| r.is_valid()).count()
    );
    println!(
        "   Invalid: {}",
        batch_results.iter().filter(|(_, r)| !r.is_valid()).count()
    );

    for (line_num, result) in &batch_results {
        let status = if result.is_valid() { "✅" } else { "❌" };
        println!(
            "   Line {}: {} (errors: {}, warnings: {})",
            line_num,
            status,
            result.error_count(),
            result.warning_count()
        );
    }

    Ok(())
}

fn print_result(resource_type: &str, result: &rh_validator::ValidationResult) {
    if result.is_valid() {
        println!("   ✅ {resource_type} is valid");
    } else {
        println!("   ❌ {resource_type} is invalid");
    }

    let errors = result.error_count();
    let warnings = result.warning_count();

    if errors > 0 {
        println!("      Errors: {errors}");
    }
    if warnings > 0 {
        println!("      Warnings: {warnings}");
    }

    if !result.issues.is_empty() {
        for issue in result.issues.iter().take(3) {
            let icon = match issue.severity {
                rh_validator::Severity::Error => "❌",
                rh_validator::Severity::Warning => "⚠️ ",
                rh_validator::Severity::Information => "ℹ️ ",
            };
            println!("      {icon} {}", issue.details);
        }
        if result.issues.len() > 3 {
            let more = result.issues.len() - 3;
            println!("      ... and {more} more issue(s)");
        }
    }
}
