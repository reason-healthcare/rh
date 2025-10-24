//! Example demonstrating the ValidatableResource trait generation
//!
//! This example shows how the ValidatableResource trait is generated for FHIR resources
//! and how invariants are embedded as constants.
//!
//! Run with:
//! ```bash
//! cargo run --example test_validation_trait
//! ```

use rh_codegen::fhir_types::{ElementConstraint, ElementDefinition, StructureDefinition};
use rh_codegen::generators::{InvariantGenerator, ValidationTraitGenerator};
use rh_codegen::invariants;

fn main() {
    println!("=== ValidatableResource Trait Generation Test ===\n");

    test_patient_generation();
    test_observation_generation();
    test_profile_generation();

    println!("\n=== All Tests Passed ===");
}

fn test_patient_generation() {
    println!("Testing Patient Resource Generation...");

    let patient = create_patient_structure_def();
    let invariants = invariants::extract_invariants(&patient);

    println!("  Extracted {} invariant(s) from Patient", invariants.len());

    let constant_code = InvariantGenerator::generate_invariants_constant(&patient);

    println!("\nGenerated INVARIANTS constant:");
    println!("{}", "=".repeat(60));
    println!("{constant_code}");
    println!("{}", "=".repeat(60));

    assert!(constant_code.contains("pub const INVARIANTS"));
    assert!(constant_code.contains("rh_foundation::Invariant"));
    assert!(constant_code.contains("pat-1"));
    println!("\n  ✓ INVARIANTS constant generated correctly");

    let trait_impl = ValidationTraitGenerator::generate_trait_impl(&patient);

    println!("\nGenerated ValidatableResource trait implementation:");
    println!("{}", "=".repeat(60));
    println!("{trait_impl}");
    println!("{}", "=".repeat(60));

    assert!(trait_impl.contains("ValidatableResource for Patient"));
    assert!(trait_impl.contains("fn resource_type"));
    assert!(trait_impl.contains("fn invariants"));
    assert!(trait_impl.contains("INVARIANTS"));
    println!("\n  ✓ ValidatableResource trait impl generated correctly");
}

fn test_observation_generation() {
    println!("\n\nTesting Observation Resource Generation...");

    let observation = create_observation_structure_def();
    let invariants = invariants::extract_invariants(&observation);

    println!(
        "  Extracted {} invariant(s) from Observation",
        invariants.len()
    );

    let constant_code = InvariantGenerator::generate_invariants_constant(&observation);

    assert!(constant_code.contains("obs-3"));
    assert!(constant_code.contains("obs-6"));
    println!("  ✓ Found obs-3 and obs-6 invariants");

    let trait_impl = ValidationTraitGenerator::generate_trait_impl(&observation);

    assert!(trait_impl.contains("ValidatableResource for Observation"));
    println!("  ✓ Trait impl generated for Observation");
}

fn test_profile_generation() {
    println!("\n\nTesting Profile Generation...");

    let profile = create_us_core_patient_profile();
    let invariants = invariants::extract_invariants(&profile);

    println!(
        "  Extracted {} invariant(s) from USCorePatient profile",
        invariants.len()
    );

    let trait_impl = ValidationTraitGenerator::generate_trait_impl(&profile);

    println!("\nGenerated trait impl for profile:");
    println!("{}", "=".repeat(60));
    println!("{trait_impl}");
    println!("{}", "=".repeat(60));

    assert!(
        trait_impl.contains("ValidatableResource for UsCorePatient")
            || trait_impl.contains("ValidatableResource for USCorePatient")
    );
    assert!(trait_impl.contains("fn profile_url()"));
    assert!(trait_impl.contains("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"));
    println!("\n  ✓ Profile URL included in trait impl");
}

fn create_patient_structure_def() -> StructureDefinition {
    let constraint = ElementConstraint {
        key: "pat-1".to_string(),
        severity: "error".to_string(),
        human: "SHALL at least contain a contact's details or a reference to an organization"
            .to_string(),
        expression: Some(
            "name.exists() or telecom.exists() or address.exists() or organization.exists()"
                .to_string(),
        ),
        xpath: Some("f:name or f:telecom or f:address or f:organization".to_string()),
        source: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
    };

    let element = ElementDefinition {
        id: Some("Patient.contact".to_string()),
        path: "Patient.contact".to_string(),
        short: Some("A contact party (e.g. guardian, partner, friend) for the patient".to_string()),
        definition: Some(
            "A contact party (e.g. guardian, partner, friend) for the patient.".to_string(),
        ),
        min: Some(0),
        max: Some("*".to_string()),
        element_type: None,
        fixed: None,
        pattern: None,
        binding: None,
        constraint: Some(vec![constraint]),
    };

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Patient".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        version: Some("4.0.1".to_string()),
        name: "Patient".to_string(),
        title: Some("Patient".to_string()),
        status: "active".to_string(),
        description: Some("Demographics and other administrative information about an individual or animal receiving care or other health-related services.".to_string()),
        purpose: None,
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        snapshot: Some(rh_codegen::fhir_types::StructureDefinitionSnapshot {
            element: vec![element],
        }),
        differential: None,
    }
}

fn create_observation_structure_def() -> StructureDefinition {
    let obs3 = ElementConstraint {
        key: "obs-3".to_string(),
        severity: "error".to_string(),
        human: "Must have at least a value or a dataAbsentReason".to_string(),
        expression: Some("value.exists() or dataAbsentReason.exists()".to_string()),
        xpath: Some("f:value or f:dataAbsentReason".to_string()),
        source: Some("http://hl7.org/fhir/StructureDefinition/Observation".to_string()),
    };

    let obs6 = ElementConstraint {
        key: "obs-6".to_string(),
        severity: "error".to_string(),
        human: "dataAbsentReason SHALL only be present if Observation.value[x] is not present"
            .to_string(),
        expression: Some("dataAbsentReason.empty() or value.empty()".to_string()),
        xpath: Some("not(exists(f:dataAbsentReason)) or not(exists(f:*[starts-with(local-name(.), 'value')]))".to_string()),
        source: Some("http://hl7.org/fhir/StructureDefinition/Observation".to_string()),
    };

    let element = ElementDefinition {
        id: Some("Observation".to_string()),
        path: "Observation".to_string(),
        short: Some("Measurements and simple assertions".to_string()),
        definition: Some(
            "Measurements and simple assertions made about a patient, device or other subject."
                .to_string(),
        ),
        min: Some(0),
        max: Some("*".to_string()),
        element_type: None,
        fixed: None,
        pattern: None,
        binding: None,
        constraint: Some(vec![obs3, obs6]),
    };

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Observation".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        version: Some("4.0.1".to_string()),
        name: "Observation".to_string(),
        title: Some("Observation".to_string()),
        status: "active".to_string(),
        description: Some(
            "Measurements and simple assertions made about a patient, device or other subject."
                .to_string(),
        ),
        purpose: None,
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "Observation".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        snapshot: Some(rh_codegen::fhir_types::StructureDefinitionSnapshot {
            element: vec![element],
        }),
        differential: None,
    }
}

fn create_us_core_patient_profile() -> StructureDefinition {
    let constraint = ElementConstraint {
        key: "us-core-1".to_string(),
        severity: "error".to_string(),
        human: "Either Patient.name.given and/or Patient.name.family SHALL be present".to_string(),
        expression: Some("name.family.exists() or name.given.exists()".to_string()),
        xpath: None,
        source: Some("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string()),
    };

    let element = ElementDefinition {
        id: Some("Patient.name".to_string()),
        path: "Patient.name".to_string(),
        short: Some("A name associated with the patient".to_string()),
        definition: Some("A name associated with the patient.".to_string()),
        min: Some(1),
        max: Some("*".to_string()),
        element_type: None,
        fixed: None,
        pattern: None,
        binding: None,
        constraint: Some(vec![constraint]),
    };

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "us-core-patient".to_string(),
        url: "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        version: Some("3.1.1".to_string()),
        name: "USCorePatient".to_string(),
        title: Some("US Core Patient Profile".to_string()),
        status: "active".to_string(),
        description: Some("US Core Patient Profile".to_string()),
        purpose: None,
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: Some(rh_codegen::fhir_types::StructureDefinitionSnapshot {
            element: vec![element],
        }),
        differential: None,
    }
}
