use rh_snapshot::{
    types::{Differential, ElementBinding, ElementConstraint, ElementDefinition, ElementType},
    SnapshotGenerator, StructureDefinition,
};

fn create_element(path: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: None,
        min: Some(0),
        max: Some("*".to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: None,
        short: None,
        comment: None,
        requirements: None,
        must_support: None,
        is_summary: None,
        is_modifier: None,
        is_modifier_reason: None,
        slicing: None,
        slice_name: None,
    }
}

#[test]
fn test_empty_differential_returns_base_snapshot() {
    let mut generator = SnapshotGenerator::new();

    // Base with snapshot
    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.identifier"),
                create_element("Patient.name"),
            ],
        }),
    };

    // Profile with empty differential
    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential { element: vec![] }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot with empty differential");

    // Should return base snapshot unchanged
    assert_eq!(snapshot.element.len(), 3);
    assert!(snapshot
        .element
        .iter()
        .any(|e| e.path == "Patient.identifier"));
}

#[test]
fn test_invalid_cardinality_min_greater_than_base() {
    let mut generator = SnapshotGenerator::new();

    // Base with required identifier (min=1)
    let base_required = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(1),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(0), // Try to relax from 1 to 0 - INVALID
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base_required);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/MyPatient");
    assert!(result.is_err(), "Should fail when relaxing min cardinality");
    assert!(result.unwrap_err().to_string().contains("cardinality"));
}

#[test]
fn test_invalid_binding_weakening() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.gender".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: Some(ElementBinding {
                        strength: "required".to_string(),
                        value_set: Some(
                            "http://hl7.org/fhir/ValueSet/administrative-gender".to_string(),
                        ),
                    }),
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.gender".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: Some(ElementBinding {
                        strength: "preferred".to_string(), // Try to weaken from required - INVALID
                        value_set: Some(
                            "http://hl7.org/fhir/ValueSet/administrative-gender".to_string(),
                        ),
                    }),
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/MyPatient");
    assert!(
        result.is_err(),
        "Should fail when weakening binding strength"
    );
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("binding")
            || error_msg.contains("strength")
            || error_msg.contains("weaken"),
        "Error should mention binding strength issue, got: {error_msg}"
    );
}

#[test]
fn test_duplicate_constraint_keys_different_expressions() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.name".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("*".to_string()),
                    type_: None,
                    binding: None,
                    constraint: Some(vec![ElementConstraint {
                        key: "pat-1".to_string(),
                        severity: "error".to_string(),
                        human: "Original constraint".to_string(),
                        expression: Some("family.exists() or given.exists()".to_string()),
                    }]),
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.name".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("*".to_string()),
                    type_: None,
                    binding: None,
                    constraint: Some(vec![ElementConstraint {
                        key: "pat-1".to_string(), // Same key
                        severity: "error".to_string(),
                        human: "Different constraint".to_string(),
                        expression: Some("family.exists()".to_string()), // Different expression - INVALID
                    }]),
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/MyPatient");
    assert!(
        result.is_err(),
        "Should fail with duplicate constraint key and different expression"
    );
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Duplicate constraint key"));
}

#[test]
fn test_invalid_type_restriction() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        name: "Observation".to_string(),
        type_: "Observation".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Observation"),
                ElementDefinition {
                    path: "Observation.value[x]".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: Some(vec![
                        ElementType {
                            code: "string".to_string(),
                            profile: None,
                        },
                        ElementType {
                            code: "integer".to_string(),
                            profile: None,
                        },
                    ]),
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyObservation".to_string(),
        name: "MyObservation".to_string(),
        type_: "Observation".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Observation".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Observation"),
                ElementDefinition {
                    path: "Observation.value[x]".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: Some(vec![ElementType {
                        code: "boolean".to_string(), // Not in base types - INVALID
                        profile: None,
                    }]),
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result =
        generator.generate_snapshot("http://example.org/StructureDefinition/MyObservation");
    assert!(
        result.is_err(),
        "Should fail when restricting to type not in base"
    );
    assert!(result.unwrap_err().to_string().contains("not in base"));
}

#[test]
fn test_error_messages_include_context() {
    let mut generator = SnapshotGenerator::new();

    // Test that error messages include the element path and profile URL
    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(1),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/BadProfile".to_string(),
        name: "BadProfile".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/BadProfile");
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    // Error should mention the element path
    assert!(
        error_msg.contains("Patient.identifier"),
        "Error should include element path, got: {error_msg}"
    );
}

#[test]
fn test_no_differential_no_base_returns_empty() {
    let mut generator = SnapshotGenerator::new();

    // Profile with no base and no differential - edge case
    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/EmptyProfile".to_string(),
        name: "EmptyProfile".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        snapshot: None,
        differential: None,
    };

    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/EmptyProfile");

    // This should either error or return empty snapshot
    // Let's check what happens
    match result {
        Ok(snapshot) => {
            // If it succeeds, snapshot should be empty or minimal
            assert!(
                snapshot.element.is_empty(),
                "Snapshot should be empty when no differential and no base"
            );
        }
        Err(_) => {
            // Or it could error, which is also reasonable
            // Either behavior is acceptable
        }
    }
}

#[test]
fn test_valid_cardinality_strengthening() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("*".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.identifier".to_string(),
                    id: None,
                    min: Some(1),               // Make required - VALID
                    max: Some("5".to_string()), // Restrict max - VALID
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should succeed with valid strengthening");

    let identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier")
        .expect("Should have identifier element");

    assert_eq!(identifier.min, Some(1));
    assert_eq!(identifier.max, Some("5".to_string()));
}

#[test]
fn test_valid_binding_strengthening() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.maritalStatus".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: Some(ElementBinding {
                        strength: "extensible".to_string(),
                        value_set: Some("http://hl7.org/fhir/ValueSet/marital-status".to_string()),
                    }),
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.maritalStatus".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: Some(ElementBinding {
                        strength: "required".to_string(), // Strengthen from extensible - VALID
                        value_set: Some("http://hl7.org/fhir/ValueSet/marital-status".to_string()),
                    }),
                    constraint: None,
                    definition: None,
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: None,
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should succeed with valid binding strengthening");

    let marital_status = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.maritalStatus")
        .expect("Should have maritalStatus element");

    assert_eq!(
        marital_status.binding.as_ref().unwrap().strength,
        "required"
    );
}
