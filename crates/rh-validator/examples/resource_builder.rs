//! Resource builder example
//!
//! This example demonstrates how to create FHIR resource instances
//! using the rh-hl7_fhir_r4_core generated types and validate them.
//!
//! This is useful when you're building resources programmatically
//! rather than parsing them from JSON.
//!
//! NEW in Phase 4: Direct struct validation with validate_resource()!

use hl7_fhir_r4_core::bindings::administrative_gender::AdministrativeGender;
use hl7_fhir_r4_core::bindings::contact_point_system::ContactPointSystem;
use hl7_fhir_r4_core::bindings::contact_point_use::ContactPointUse;
use hl7_fhir_r4_core::bindings::identifier_use::IdentifierUse;
use hl7_fhir_r4_core::bindings::name_use::NameUse;
use hl7_fhir_r4_core::bindings::narrative_status::NarrativeStatus;
use hl7_fhir_r4_core::datatypes::codeable_concept::CodeableConcept;
use hl7_fhir_r4_core::datatypes::coding::Coding;
use hl7_fhir_r4_core::datatypes::contact_point::ContactPoint;
use hl7_fhir_r4_core::datatypes::extension::Extension;
use hl7_fhir_r4_core::datatypes::human_name::HumanName;
use hl7_fhir_r4_core::datatypes::identifier::Identifier;
use hl7_fhir_r4_core::datatypes::narrative::Narrative;
use hl7_fhir_r4_core::resources::patient::Patient;
use hl7_fhir_r4_core::traits::domain_resource::DomainResourceMutators;
use hl7_fhir_r4_core::traits::patient::PatientMutators;
use hl7_fhir_r4_core::traits::resource::ResourceMutators;
use rh_validator::FhirValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIR Resource Builder and Validation Example ===\n");

    let validator = FhirValidator::new()?;

    // Example 1: Build and validate a Patient directly (RECOMMENDED - Phase 4 feature!)
    println!("Example 1: Direct validation of instantiated Patient (RECOMMENDED)");
    let minimal_patient = <Patient as PatientMutators>::new()
        .set_id("example-1".to_string())
        .add_extension(Extension {
            url: "http://example.org/test".to_string(),
            value_string: Some("test".to_string()),
            ..Default::default()
        });

    // Validate the instance directly - NO JSON serialization needed!
    // This is the most efficient way for programmatically-built resources
    let result = validator.validate_resource(&minimal_patient)?;
    println!("  Direct validation (efficient for programmatic resources):");
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    // Example 2: Traditional JSON validation (for comparison)
    println!("Example 2: JSON validation (traditional approach)");
    let json = serde_json::to_string_pretty(&minimal_patient)?;
    println!("  Generated JSON (first 200 chars):");
    println!("{}...", &json[..200.min(json.len())]);
    println!();

    let json_result = validator.validate_full::<Patient>(&json)?;
    println!("  JSON validation (requires serialization round-trip):");
    println!("  Valid: {}", json_result.is_valid());
    println!("  Errors: {}", json_result.error_count());
    println!("  Warnings: {}", json_result.warning_count());
    println!();

    // Example 3: Build a complete Patient with many fields
    println!("Example 3: Building a complete Patient with many fields");
    let patient = <Patient as PatientMutators>::new()
        .set_id("example-complete".to_string())
        .set_language("en-US".to_string())
        .set_text(Narrative {
            base: Default::default(),
            status: NarrativeStatus::Generated,
            _status: None,
            div: r#"<div xmlns="http://www.w3.org/1999/xhtml">John Doe</div>"#.to_string(),
            _div: None,
        })
        .add_extension(Extension {
            url: "http://hl7.org/fhir/StructureDefinition/patient-birthPlace".to_string(),
            value_string: Some("Seattle, WA".to_string()),
            ..Default::default()
        })
        .add_identifier(Identifier {
            use_: Some(IdentifierUse::Official),
            type_: Some(CodeableConcept {
                coding: Some(vec![Coding {
                    system: Some("http://terminology.hl7.org/CodeSystem/v2-0203".to_string()),
                    code: Some("MR".to_string()),
                    display: Some("Medical Record Number".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            system: Some("http://hospital.example.org".to_string()),
            value: Some("12345".to_string()),
            ..Default::default()
        })
        .set_active(true)
        .add_name(HumanName {
            use_: Some(NameUse::Official),
            family: Some("Doe".to_string()),
            given: Some(vec!["John".to_string(), "Q".to_string()]),
            prefix: Some(vec!["Mr.".to_string()]),
            suffix: Some(vec!["Jr.".to_string()]),
            ..Default::default()
        })
        .add_telecom(ContactPoint {
            system: Some(ContactPointSystem::Phone),
            value: Some("+1-555-123-4567".to_string()),
            use_: Some(ContactPointUse::Home),
            ..Default::default()
        })
        .add_telecom(ContactPoint {
            system: Some(ContactPointSystem::Email),
            value: Some("john.doe@example.com".to_string()),
            use_: Some(ContactPointUse::Work),
            ..Default::default()
        })
        .set_gender(AdministrativeGender::Male)
        .set_birth_date("1974-12-25".to_string());

    // Validate directly - most efficient for programmatic resources
    let result = validator.validate_resource(&patient)?;
    println!("  Direct validation result:");
    println!("  Valid: {}", result.is_valid());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!();

    // Example 4: Performance comparison - direct vs JSON validation
    println!("Example 4: Performance comparison (direct vs JSON)");
    use std::time::Instant;

    let start = Instant::now();
    let _ = validator.validate_resource(&patient)?;
    let direct_time = start.elapsed();

    let start = Instant::now();
    let json = serde_json::to_string(&patient)?;
    let _ = validator.validate_full::<Patient>(&json)?;
    let json_time = start.elapsed();

    println!("  Direct validation: {direct_time:?}");
    println!("  JSON validation: {json_time:?}");
    if direct_time.as_micros() > 0 {
        println!(
            "  Speedup: {:.1}x faster with direct validation",
            json_time.as_micros() as f64 / direct_time.as_micros() as f64
        );
    }
    println!();

    // Example 5: Builder pattern demonstration
    println!("Example 5: Using builder-style construction");

    fn build_simple_patient(id: &str, family_name: &str, given_name: &str) -> Patient {
        <Patient as PatientMutators>::new()
            .set_id(id.to_string())
            .add_extension(Extension {
                url: "http://example.org/test".to_string(),
                value_string: Some("test".to_string()),
                ..Default::default()
            })
            .add_name(HumanName {
                family: Some(family_name.to_string()),
                given: Some(vec![given_name.to_string()]),
                ..Default::default()
            })
    }

    let simple_patient = build_simple_patient("simple-1", "Smith", "Jane");

    // Use direct validation - no JSON needed!
    let result = validator.validate_resource(&simple_patient)?;

    println!(
        "  Created patient: {}",
        simple_patient.base.base.id.as_deref().unwrap()
    );
    println!("  Valid: {}", result.is_valid());
    println!();

    println!("✅ Resource builder examples completed!");
    println!("\nKey takeaway: Use validate_resource() for programmatic resources!");
    println!("It's faster and more ergonomic than JSON round-trip validation.");

    Ok(())
}
