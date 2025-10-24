//! Patient validation example
//!
//! This example shows comprehensive Patient resource validation
//! with real-world scenarios.

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Patient Validation Examples ===\n");

    let validator = FhirValidator::new()?;

    // Example 1: Minimal valid patient
    println!("Example 1: Minimal valid Patient");
    let minimal = r#"{
        "resourceType": "Patient",
        "id": "min",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }]
    }"#;

    let result = validator.validate_full::<Patient>(minimal)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!();

    // Example 2: Complete patient with all common fields
    println!("Example 2: Complete Patient record");
    let complete = r#"{
        "resourceType": "Patient",
        "id": "complete-example",
        "meta": {
            "versionId": "1",
            "lastUpdated": "2024-01-15T10:30:00Z"
        },
        "text": {
            "status": "generated",
            "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">John Doe</div>"
        },
        "extension": [{
            "url": "http://hl7.org/fhir/StructureDefinition/patient-birthPlace",
            "valueAddress": {
                "city": "Seattle",
                "state": "WA",
                "country": "USA"
            }
        }],
        "identifier": [{
            "use": "official",
            "type": {
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                    "code": "MR"
                }]
            },
            "system": "http://hospital.example.org",
            "value": "12345"
        }],
        "active": true,
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["John", "Q"],
            "prefix": ["Mr."],
            "suffix": ["Jr."]
        }],
        "telecom": [{
            "system": "phone",
            "value": "+1-555-123-4567",
            "use": "home"
        }, {
            "system": "email",
            "value": "john.doe@example.com",
            "use": "work"
        }],
        "gender": "male",
        "birthDate": "1974-12-25",
        "address": [{
            "use": "home",
            "type": "physical",
            "line": ["123 Main St", "Apt 4"],
            "city": "Seattle",
            "state": "WA",
            "postalCode": "98101",
            "country": "USA"
        }],
        "maritalStatus": {
            "coding": [{
                "system": "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
                "code": "M",
                "display": "Married"
            }]
        },
        "contact": [{
            "relationship": [{
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0131",
                    "code": "N"
                }]
            }],
            "name": {
                "family": "Doe",
                "given": ["Jane"]
            },
            "telecom": [{
                "system": "phone",
                "value": "+1-555-123-4568"
            }]
        }],
        "communication": [{
            "language": {
                "coding": [{
                    "system": "urn:ietf:bcp:47",
                    "code": "en-US"
                }]
            },
            "preferred": true
        }]
    }"#;

    let result = validator.validate_full::<Patient>(complete)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    // Example 3: Patient with validation errors
    println!("Example 3: Patient with common errors");
    let with_errors = r#"{
        "resourceType": "Patient",
        "id": "errors",
        "name": [{
            "family": "Smith"
        }],
        "gender": "invalid-value",
        "birthDate": "not-a-date",
        "contact": [{
            "relationship": [{
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0131",
                    "code": "N"
                }]
            }]
        }]
    }"#;

    let result = validator.validate_full::<Patient>(with_errors)?;
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());

    if result.error_count() > 0 {
        println!("  Issues found:");
        for issue in result.issues {
            if let Some(inv_key) = &issue.invariant_key {
                println!("    - Invariant {}: {}", inv_key, issue.details);
            } else {
                println!("    - {}: {}", issue.code, issue.details);
            }
        }
    }
    println!();

    // Example 4: Parsing and validating in one step
    println!("Example 4: Parse and validate together");
    let json = r#"{
        "resourceType": "Patient",
        "id": "parse-validate",
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }],
        "name": [{
            "family": "Johnson",
            "given": ["Mary"]
        }],
        "gender": "female",
        "birthDate": "1985-06-15"
    }"#;

    // First validate the JSON structure
    let result = validator.validate_full::<Patient>(json)?;

    if result.is_valid() {
        // Only parse if validation passed
        let patient: Patient = serde_json::from_str(json)?;
        println!("  ✅ Valid patient parsed successfully");
        println!(
            "  ID: {}",
            patient.base.base.id.as_deref().unwrap_or("none")
        );
        if let Some(names) = &patient.name {
            if let Some(name) = names.first() {
                let given = name
                    .given
                    .as_ref()
                    .and_then(|g| g.first())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let family = name.family.as_deref().unwrap_or("");
                println!("  Name: {given} {family}");
            }
        }
    } else {
        println!("  ❌ Validation failed - not parsing");
    }
    println!();

    println!("✅ Patient validation examples completed!");

    Ok(())
}
