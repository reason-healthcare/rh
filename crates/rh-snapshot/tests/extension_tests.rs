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

fn create_extension_slicing() -> ElementSlicing {
    ElementSlicing {
        discriminator: Some(vec![ElementDiscriminator {
            type_: "value".to_string(),
            path: "url".to_string(),
        }]),
        rules: Some("open".to_string()),
        ordered: Some(false),
        description: Some("Extensions are always sliced by URL".to_string()),
    }
}

fn create_extension_slice(path: &str, slice_name: &str, url: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: None,
        min: Some(0),
        max: Some("1".to_string()),
        type_: None,
        binding: None,
        constraint: None,
        definition: Some(format!("Extension: {url}")),
        short: Some(slice_name.to_string()),
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
fn test_extension_slicing() {
    let mut generator = SnapshotGenerator::new();

    // Base: Patient with extension element
    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.extension"),
            ],
        }),
    };

    // Profile: Slice extensions by URL
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
                    path: "Patient.extension".to_string(),
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
                    slicing: Some(create_extension_slicing()),
                    slice_name: None,
                },
                create_extension_slice(
                    "Patient.extension",
                    "nickname",
                    "http://example.org/fhir/StructureDefinition/patient-nickname",
                ),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify extension slicing
    let extension_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name.is_none())
        .expect("Should have Patient.extension base element");

    assert!(
        extension_element.slicing.is_some(),
        "Patient.extension should have slicing"
    );

    let slicing = extension_element.slicing.as_ref().unwrap();
    let discriminators = slicing.discriminator.as_ref().unwrap();
    assert_eq!(discriminators.len(), 1);
    assert_eq!(discriminators[0].type_, "value");
    assert_eq!(discriminators[0].path, "url");

    // Verify extension slice
    let nickname_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("nickname".to_string()))
        .expect("Should have nickname extension slice");

    assert_eq!(nickname_extension.short, Some("nickname".to_string()));
    assert!(nickname_extension
        .definition
        .as_ref()
        .unwrap()
        .contains("patient-nickname"));
}

#[test]
fn test_multiple_extension_slices() {
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
                create_element("Patient.extension"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
        name: "USCorePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.extension".to_string(),
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
                    slicing: Some(create_extension_slicing()),
                    slice_name: None,
                },
                create_extension_slice(
                    "Patient.extension",
                    "race",
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                ),
                create_extension_slice(
                    "Patient.extension",
                    "ethnicity",
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                ),
                create_extension_slice(
                    "Patient.extension",
                    "birthsex",
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                ),
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient")
        .expect("Should generate snapshot");

    // Verify all three extension slices exist
    let race_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("race".to_string()));
    assert!(race_extension.is_some(), "Should have race extension");

    let ethnicity_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("ethnicity".to_string()));
    assert!(
        ethnicity_extension.is_some(),
        "Should have ethnicity extension"
    );

    let birthsex_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("birthsex".to_string()));
    assert!(
        birthsex_extension.is_some(),
        "Should have birthsex extension"
    );
}

#[test]
fn test_modifier_extension_slicing() {
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
                create_element("Patient.modifierExtension"),
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
                    path: "Patient.modifierExtension".to_string(),
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
                    is_modifier: Some(true),
                    is_modifier_reason: Some("Modifier extensions change the meaning".to_string()),
                    slicing: Some(create_extension_slicing()),
                    slice_name: None,
                },
                ElementDefinition {
                    path: "Patient.modifierExtension".to_string(),
                    id: None,
                    min: Some(0),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: Some("Special modifier extension".to_string()),
                    short: Some("special".to_string()),
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: Some(true),
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: Some("special".to_string()),
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify modifierExtension slicing
    let modifier_extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.modifierExtension" && e.slice_name.is_none())
        .expect("Should have Patient.modifierExtension base element");

    assert!(modifier_extension.is_modifier == Some(true));
    assert!(modifier_extension.slicing.is_some());

    // Verify modifier extension slice
    let special_modifier = snapshot
        .element
        .iter()
        .find(|e| {
            e.path == "Patient.modifierExtension" && e.slice_name == Some("special".to_string())
        })
        .expect("Should have special modifierExtension slice");

    assert_eq!(special_modifier.is_modifier, Some(true));
    assert_eq!(special_modifier.short, Some("special".to_string()));
}

#[test]
fn test_extension_inheritance() {
    let mut generator = SnapshotGenerator::new();

    // Base profile with one extension
    let base_profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/BasePatient".to_string(),
        name: "BasePatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(rh_snapshot::Snapshot {
            element: vec![
                create_element("Patient"),
                ElementDefinition {
                    path: "Patient.extension".to_string(),
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
                    slicing: Some(create_extension_slicing()),
                    slice_name: None,
                },
                create_extension_slice(
                    "Patient.extension",
                    "base-ext",
                    "http://example.org/base-extension",
                ),
            ],
        }),
    };

    // Derived profile adds another extension
    let derived_profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/DerivedPatient".to_string(),
        name: "DerivedPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://example.org/StructureDefinition/BasePatient".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![
                create_element("Patient"),
                create_extension_slice(
                    "Patient.extension",
                    "derived-ext",
                    "http://example.org/derived-extension",
                ),
            ],
        }),
    };

    generator.load_structure_definition(base_profile);
    generator.load_structure_definition(derived_profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/DerivedPatient")
        .expect("Should generate snapshot");

    // Should have both base and derived extension slices
    let base_ext = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("base-ext".to_string()));
    assert!(base_ext.is_some(), "Should inherit base extension");

    let derived_ext = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("derived-ext".to_string()));
    assert!(derived_ext.is_some(), "Should have derived extension");

    // Verify slicing is present
    let extension_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name.is_none())
        .expect("Should have Patient.extension base element");
    assert!(extension_element.slicing.is_some());
}

#[test]
fn test_nested_extension_elements() {
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
                create_element("Patient.extension"),
                create_element("Patient.extension.url"),
                create_element("Patient.extension.value[x]"),
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
                    path: "Patient.extension".to_string(),
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
                    slicing: Some(create_extension_slicing()),
                    slice_name: None,
                },
                create_extension_slice(
                    "Patient.extension",
                    "complex-ext",
                    "http://example.org/complex-extension",
                ),
                // Add a constraint on the URL for this slice
                ElementDefinition {
                    path: "Patient.extension.url".to_string(),
                    id: None,
                    min: Some(1),
                    max: Some("1".to_string()),
                    type_: None,
                    binding: None,
                    constraint: None,
                    definition: Some("URL must be the extension URL".to_string()),
                    short: None,
                    comment: None,
                    requirements: None,
                    must_support: None,
                    is_summary: None,
                    is_modifier: None,
                    is_modifier_reason: None,
                    slicing: None,
                    slice_name: Some("complex-ext".to_string()),
                },
            ],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.org/StructureDefinition/MyPatient")
        .expect("Should generate snapshot");

    // Verify extension slice and its children
    let complex_ext = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.extension" && e.slice_name == Some("complex-ext".to_string()))
        .expect("Should have complex-ext slice");
    assert!(complex_ext.slice_name.is_some());

    // Verify the child element was created for the slice
    let url_element = snapshot
        .element
        .iter()
        .find(|e| {
            e.path == "Patient.extension.url" && e.slice_name == Some("complex-ext".to_string())
        })
        .expect("Should have Patient.extension.url:complex-ext");

    assert_eq!(url_element.min, Some(1));
    assert_eq!(
        url_element.definition,
        Some("URL must be the extension URL".to_string())
    );

    // The value[x] child should also be auto-created for the slice
    let value_element = snapshot.element.iter().find(|e| {
        e.path == "Patient.extension.value[x]" && e.slice_name == Some("complex-ext".to_string())
    });
    assert!(
        value_element.is_some(),
        "Should auto-create Patient.extension.value[x]:complex-ext"
    );
}
