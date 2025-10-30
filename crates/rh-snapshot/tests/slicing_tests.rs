use rh_snapshot::{
    types::{Differential, ElementDefinition, ElementDiscriminator, ElementSlicing},
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

fn create_sliced_element(path: &str, slicing: ElementSlicing) -> ElementDefinition {
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
        slicing: Some(slicing),
        slice_name: None,
    }
}

fn create_slice(path: &str, slice_name: &str, min: u32) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: None,
        min: Some(min),
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
        slice_name: Some(slice_name.to_string()),
    }
}

#[test]
fn test_simple_slicing() {
    let mut generator = SnapshotGenerator::new();

    // Base: Patient with identifier element
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
            ],
        }),
    };

    // Profile: Add slicing to Patient.identifier
    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                create_sliced_element(
                    "Patient.identifier",
                    ElementSlicing {
                        discriminator: Some(vec![ElementDiscriminator {
                            type_: "value".to_string(),
                            path: "system".to_string(),
                        }]),
                        rules: Some("open".to_string()),
                        ordered: Some(false),
                        description: Some("Slice by identifier system".to_string()),
                    },
                ),
                create_slice("Patient.identifier", "MRN", 1),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify slicing was added to Patient.identifier
    let identifier_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name.is_none())
        .expect("Should have Patient.identifier base element");

    assert!(
        identifier_element.slicing.is_some(),
        "Patient.identifier should have slicing"
    );

    // Verify slice exists
    let mrn_slice = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name == Some("MRN".to_string()))
        .expect("Should have MRN slice");

    assert_eq!(mrn_slice.min, Some(1));
    assert_eq!(mrn_slice.max, Some("1".to_string()));
}

#[test]
fn test_multiple_slices() {
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
                create_element("Patient.identifier"),
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
                create_sliced_element(
                    "Patient.identifier",
                    ElementSlicing {
                        discriminator: Some(vec![ElementDiscriminator {
                            type_: "value".to_string(),
                            path: "system".to_string(),
                        }]),
                        rules: Some("open".to_string()),
                        ordered: Some(false),
                        description: Some("Slice by identifier system".to_string()),
                    },
                ),
                create_slice("Patient.identifier", "MRN", 1),
                create_slice("Patient.identifier", "SSN", 0),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify both slices exist
    let mrn_slice = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name == Some("MRN".to_string()))
        .expect("Should have MRN slice");
    assert_eq!(mrn_slice.min, Some(1));

    let ssn_slice = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name == Some("SSN".to_string()))
        .expect("Should have SSN slice");
    assert_eq!(ssn_slice.min, Some(0));
}

#[test]
fn test_slice_with_children() {
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
                create_element("Patient.identifier"),
                create_element("Patient.identifier.system"),
                create_element("Patient.identifier.value"),
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
                create_sliced_element(
                    "Patient.identifier",
                    ElementSlicing {
                        discriminator: Some(vec![ElementDiscriminator {
                            type_: "value".to_string(),
                            path: "system".to_string(),
                        }]),
                        rules: Some("open".to_string()),
                        ordered: Some(false),
                        description: None,
                    },
                ),
                create_slice("Patient.identifier", "MRN", 1),
                ElementDefinition {
                    path: "Patient.identifier.system".to_string(),
                    id: None,
                    min: Some(1),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: Some("MRN system".to_string()),
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: Some("MRN".to_string()),
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify slice child exists
    let slice_system = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier.system" && e.slice_name == Some("MRN".to_string()))
        .expect("Should have Patient.identifier.system for MRN slice");

    assert_eq!(slice_system.min, Some(1));
    assert_eq!(slice_system.definition, Some("MRN system".to_string()));
}

#[test]
fn test_slice_inherits_from_base() {
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
                    definition: Some("Base identifier definition".to_string()),
                    short: Some("Identifier".to_string()),
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
                create_sliced_element(
                    "Patient.identifier",
                    ElementSlicing {
                        discriminator: Some(vec![ElementDiscriminator {
                            type_: "value".to_string(),
                            path: "system".to_string(),
                        }]),
                        rules: Some("open".to_string()),
                        ordered: Some(false),
                        description: None,
                    },
                ),
                create_slice("Patient.identifier", "MRN", 1),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify slice inherits base properties
    let mrn_slice = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name == Some("MRN".to_string()))
        .expect("Should have MRN slice");

    // Slice should inherit definition and short from base element
    assert_eq!(
        mrn_slice.definition,
        Some("Base identifier definition".to_string())
    );
    assert_eq!(mrn_slice.short, Some("Identifier".to_string()));
}

#[test]
fn test_discriminator_types() {
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
                create_element("Patient.identifier"),
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
                create_sliced_element(
                    "Patient.identifier",
                    ElementSlicing {
                        discriminator: Some(vec![
                            ElementDiscriminator {
                                type_: "value".to_string(),
                                path: "system".to_string(),
                            },
                            ElementDiscriminator {
                                type_: "pattern".to_string(),
                                path: "use".to_string(),
                            },
                        ]),
                        rules: Some("closed".to_string()),
                        ordered: Some(true),
                        description: Some("Complex discriminator".to_string()),
                    },
                ),
                create_slice("Patient.identifier", "MRN", 1),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    let identifier_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier" && e.slice_name.is_none())
        .expect("Should have Patient.identifier");

    let slicing = identifier_element
        .slicing
        .as_ref()
        .expect("Should have slicing");

    let discriminators = slicing
        .discriminator
        .as_ref()
        .expect("Should have discriminators");

    assert_eq!(discriminators.len(), 2);
    assert_eq!(discriminators[0].type_, "value");
    assert_eq!(discriminators[0].path, "system");
    assert_eq!(discriminators[1].type_, "pattern");
    assert_eq!(discriminators[1].path, "use");

    assert_eq!(slicing.rules, Some("closed".to_string()));
    assert_eq!(slicing.ordered, Some(true));
}
