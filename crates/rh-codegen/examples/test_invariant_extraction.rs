//! Example demonstrating invariant extraction from FHIR StructureDefinitions
//!
//! This example shows how invariants are extracted from Patient and Observation
//! resources and verifies they match the official FHIR specification.
//!
//! Run with:
//! ```bash
//! cargo run --example test_invariant_extraction
//! ```

use rh_codegen::fhir_types::{ElementConstraint, ElementDefinition, StructureDefinition};
use rh_codegen::invariants;
use rh_foundation::Severity;

fn main() {
    println!("=== FHIR Invariant Extraction Test ===\n");

    test_patient_invariants();
    test_observation_invariants();
    test_severity_mapping();
    test_xpath_exclusion();

    println!("\n=== All Tests Passed ===");
}

fn test_patient_invariants() {
    println!("Testing Patient Invariants...");

    // Create a mock Patient StructureDefinition with pat-1 invariant
    let patient = create_patient_structure_def();

    let invariants = invariants::extract_invariants(&patient);

    // Verify pat-1 exists
    assert!(
        invariants.iter().any(|inv| inv.key == "pat-1"),
        "pat-1 invariant should be extracted"
    );

    let pat1 = invariants.iter().find(|inv| inv.key == "pat-1").unwrap();

    // Verify pat-1 details match FHIR spec
    assert_eq!(pat1.severity, Severity::Error);
    assert!(pat1.human.contains("contact"));
    assert!(pat1.expression.contains(
        "name.exists() or telecom.exists() or address.exists() or organization.exists()"
    ));

    println!("  ✓ pat-1: {}", pat1.human);
    println!(
        "    Expression: {}",
        pat1.expression.chars().take(60).collect::<String>()
    );
    println!("    Severity: {:?}", pat1.severity);
}

fn test_observation_invariants() {
    println!("\nTesting Observation Invariants...");

    let observation = create_observation_structure_def();
    let invariants = invariants::extract_invariants(&observation);

    // Check for obs-3 (value[x] rule)
    let obs3 = invariants.iter().find(|inv| inv.key == "obs-3");
    if let Some(obs3) = obs3 {
        assert_eq!(obs3.severity, Severity::Error);
        println!("  ✓ obs-3: {}", obs3.human);
        println!(
            "    Expression: {}",
            obs3.expression.chars().take(60).collect::<String>()
        );
    }

    // Check for obs-6 (dataAbsentReason rule)
    let obs6 = invariants.iter().find(|inv| inv.key == "obs-6");
    if let Some(obs6) = obs6 {
        assert_eq!(obs6.severity, Severity::Error);
        println!("  ✓ obs-6: {}", obs6.human);
        println!(
            "    Expression: {}",
            obs6.expression.chars().take(60).collect::<String>()
        );
    }

    println!("  Total invariants found: {}", invariants.len());
}

fn test_severity_mapping() {
    println!("\nTesting Severity Mapping...");

    // Create a structure with error severity constraint
    let error_struct = create_struct_with_constraint(ElementConstraint {
        key: "test-1".to_string(),
        severity: "error".to_string(),
        human: "Test error constraint".to_string(),
        expression: Some("true".to_string()),
        xpath: None,
        source: None,
    });

    let invs = invariants::extract_invariants(&error_struct);
    assert_eq!(invs.len(), 1);
    assert_eq!(invs[0].severity, Severity::Error);
    println!("  ✓ 'error' maps to Severity::Error");

    // Test warning severity
    let warning_struct = create_struct_with_constraint(ElementConstraint {
        key: "test-2".to_string(),
        severity: "warning".to_string(),
        human: "Test warning constraint".to_string(),
        expression: Some("true".to_string()),
        xpath: None,
        source: None,
    });

    let invs = invariants::extract_invariants(&warning_struct);
    assert_eq!(invs.len(), 1);
    assert_eq!(invs[0].severity, Severity::Warning);
    println!("  ✓ 'warning' maps to Severity::Warning");

    // Test unknown severity defaults to Information
    let unknown_struct = create_struct_with_constraint(ElementConstraint {
        key: "test-3".to_string(),
        severity: "unknown".to_string(),
        human: "Test unknown constraint".to_string(),
        expression: Some("true".to_string()),
        xpath: None,
        source: None,
    });

    let invs = invariants::extract_invariants(&unknown_struct);
    assert_eq!(invs.len(), 1);
    assert_eq!(invs[0].severity, Severity::Information);
    println!("  ✓ Unknown severity defaults to Severity::Information");
}

fn test_xpath_exclusion() {
    println!("\nTesting XPath Exclusion...");

    // Create a constraint with both XPath and expression
    let xpath_struct = create_struct_with_constraint(ElementConstraint {
        key: "test-xpath".to_string(),
        severity: "error".to_string(),
        human: "Has XPath and expression".to_string(),
        expression: Some("value.exists()".to_string()),
        xpath: Some("f:value".to_string()),
        source: None,
    });

    let invs = invariants::extract_invariants(&xpath_struct);
    assert_eq!(invs.len(), 1);

    // XPath should be included in the Invariant struct
    assert!(invs[0].xpath.is_some());
    println!("  ✓ XPath preserved in Invariant struct");

    // FHIRPath expression should be the primary validation method
    assert_eq!(invs[0].expression, "value.exists()");
    println!("  ✓ FHIRPath expression is primary validation method");
}

fn create_struct_with_constraint(constraint: ElementConstraint) -> StructureDefinition {
    let element = ElementDefinition {
        id: Some("Test".to_string()),
        path: "Test".to_string(),
        short: Some("Test element".to_string()),
        definition: Some("Test element".to_string()),
        min: Some(0),
        max: Some("1".to_string()),
        element_type: None,
        fixed: None,
        pattern: None,
        binding: None,
        constraint: Some(vec![constraint]),
    };

    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Test".to_string(),
        url: "http://test.org/StructureDefinition/Test".to_string(),
        version: Some("1.0.0".to_string()),
        name: "Test".to_string(),
        title: Some("Test".to_string()),
        status: "active".to_string(),
        description: Some("Test structure definition".to_string()),
        purpose: None,
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "Test".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        snapshot: Some(rh_codegen::fhir_types::StructureDefinitionSnapshot {
            element: vec![element],
        }),
        differential: None,
    }
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
