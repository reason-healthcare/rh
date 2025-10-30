use rh_snapshot::error::SnapshotError;
use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{
    Differential, ElementDefinition, ElementType, Snapshot, StructureDefinition,
};

fn create_element_with_type(path: &str, type_code: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(0),
        max: Some("*".to_string()),
        type_: Some(vec![ElementType {
            code: type_code.to_string(),
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
    }
}

fn create_element_with_types(
    path: &str,
    types: Vec<(&str, Option<Vec<&str>>)>,
) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(0),
        max: Some("*".to_string()),
        type_: Some(
            types
                .into_iter()
                .map(|(code, profiles)| ElementType {
                    code: code.to_string(),
                    profile: profiles.map(|p| p.iter().map(|s| s.to_string()).collect()),
                })
                .collect(),
        ),
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
fn test_type_restriction_reference_with_profile() {
    // Base: Reference(Resource), Differential: Reference(Patient) via profile
    // This is how FHIR actually restricts resource references - same type code, different profile
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_types(
                    "Base.subject",
                    vec![(
                        "Reference",
                        Some(vec!["http://hl7.org/fhir/StructureDefinition/Resource"]),
                    )],
                ),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_types(
                "Base.subject",
                vec![(
                    "Reference",
                    Some(vec!["http://hl7.org/fhir/StructureDefinition/Patient"]),
                )],
            )],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let subject_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.subject")
        .unwrap();

    assert_eq!(subject_element.type_.as_ref().unwrap().len(), 1);
    assert_eq!(subject_element.type_.as_ref().unwrap()[0].code, "Reference");
    assert_eq!(
        subject_element.type_.as_ref().unwrap()[0]
            .profile
            .as_ref()
            .unwrap()[0],
        "http://hl7.org/fhir/StructureDefinition/Patient"
    );
}

#[test]
fn test_type_restriction_invalid_new_type() {
    // Base: string, Differential: integer (invalid - not a subset)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_type("Base.value", "string"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_type("Base.value", "integer")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_err());

    let err = result.unwrap_err();
    match err {
        SnapshotError::MergeError(msg) => {
            assert!(msg.contains("Invalid type restriction"));
            assert!(msg.contains("Base.value"));
            assert!(msg.contains("integer"));
            assert!(msg.contains("not in base types"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_type_multiple_to_single() {
    // Base: [Reference, CodeableConcept], Differential: Reference (valid - subset)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_types(
                    "Base.value[x]",
                    vec![("Reference", None), ("CodeableConcept", None)],
                ),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_type("Base.value[x]", "Reference")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let value_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.value[x]")
        .unwrap();

    assert_eq!(value_element.type_.as_ref().unwrap().len(), 1);
    assert_eq!(value_element.type_.as_ref().unwrap()[0].code, "Reference");
}

#[test]
fn test_type_with_profile() {
    // Base: Reference, Differential: Reference with profile
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_type("Base.subject", "Reference"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_types(
                "Base.subject",
                vec![(
                    "Reference",
                    Some(vec!["http://hl7.org/fhir/StructureDefinition/Patient"]),
                )],
            )],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let subject_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.subject")
        .unwrap();

    assert_eq!(subject_element.type_.as_ref().unwrap().len(), 1);
    assert_eq!(subject_element.type_.as_ref().unwrap()[0].code, "Reference");
    assert!(subject_element.type_.as_ref().unwrap()[0].profile.is_some());
    assert_eq!(
        subject_element.type_.as_ref().unwrap()[0]
            .profile
            .as_ref()
            .unwrap()[0],
        "http://hl7.org/fhir/StructureDefinition/Patient"
    );
}

#[test]
fn test_type_inherits_from_base() {
    // Base has type, differential doesn't specify: should inherit
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_type("Base.value", "string"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                path: "Base.value".to_string(),
                id: Some("Base.value".to_string()),
                min: Some(1), // Only changes cardinality
                max: Some("1".to_string()),
                type_: None, // Doesn't specify type
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
            }],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let value_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.value")
        .unwrap();

    // Type should be inherited from base
    assert!(value_element.type_.is_some());
    assert_eq!(value_element.type_.as_ref().unwrap()[0].code, "string");
    // But cardinality should be from differential
    assert_eq!(value_element.min, Some(1));
    assert_eq!(value_element.max, Some("1".to_string()));
}

#[test]
fn test_type_multiple_restrictions() {
    // Base: [string, integer, boolean], Differential: [string, integer] (valid subset)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_types(
                    "Base.value[x]",
                    vec![("string", None), ("integer", None), ("boolean", None)],
                ),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_types(
                "Base.value[x]",
                vec![("string", None), ("integer", None)],
            )],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let value_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.value[x]")
        .unwrap();

    assert_eq!(value_element.type_.as_ref().unwrap().len(), 2);
    let codes: Vec<&str> = value_element
        .type_
        .as_ref()
        .unwrap()
        .iter()
        .map(|t| t.code.as_str())
        .collect();
    assert!(codes.contains(&"string"));
    assert!(codes.contains(&"integer"));
    assert!(!codes.contains(&"boolean"));
}

#[test]
fn test_type_same_as_base() {
    // Base: string, Differential: string (valid - same)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_with_type("Base", "Base"),
                create_element_with_type("Base.value", "string"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_type("Base.value", "string")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let value_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.value")
        .unwrap();

    assert_eq!(value_element.type_.as_ref().unwrap().len(), 1);
    assert_eq!(value_element.type_.as_ref().unwrap()[0].code, "string");
}
