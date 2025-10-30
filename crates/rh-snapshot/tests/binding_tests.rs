use rh_snapshot::error::SnapshotError;
use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{
    Differential, ElementBinding, ElementDefinition, Snapshot, StructureDefinition,
};

fn create_element_with_binding(
    path: &str,
    strength: &str,
    value_set: Option<&str>,
) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(0),
        max: Some("*".to_string()),
        type_: None,
        binding: Some(ElementBinding {
            strength: strength.to_string(),
            value_set: value_set.map(|s| s.to_string()),
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
    }
}

fn create_element_without_binding(path: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
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
fn test_binding_strengthening_preferred_to_required() {
    // Base: preferred, Differential: required (valid - stricter)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.code",
                    "preferred",
                    Some("http://example.com/vs/codes"),
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
            element: vec![create_element_with_binding(
                "Base.code",
                "required",
                Some("http://example.com/vs/codes"),
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
    let code_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.code")
        .unwrap();

    assert!(code_element.binding.is_some());
    assert_eq!(code_element.binding.as_ref().unwrap().strength, "required");
}

#[test]
fn test_binding_strengthening_example_to_extensible() {
    // Base: example, Differential: extensible (valid - stricter)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.status",
                    "example",
                    Some("http://example.com/vs/status"),
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
            element: vec![create_element_with_binding(
                "Base.status",
                "extensible",
                Some("http://example.com/vs/status"),
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
    let status_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.status")
        .unwrap();

    assert_eq!(
        status_element.binding.as_ref().unwrap().strength,
        "extensible"
    );
}

#[test]
fn test_binding_cannot_weaken_required_to_preferred() {
    // Base: required, Differential: preferred (invalid - weaker)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.code",
                    "required",
                    Some("http://example.com/vs/codes"),
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
            element: vec![create_element_with_binding(
                "Base.code",
                "preferred",
                Some("http://example.com/vs/codes"),
            )],
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
            assert!(msg.contains("Invalid binding"));
            assert!(msg.contains("Base.code"));
            assert!(msg.contains("weaker"));
            assert!(msg.contains("preferred"));
            assert!(msg.contains("required"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_binding_cannot_weaken_extensible_to_example() {
    // Base: extensible, Differential: example (invalid - weaker)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.category",
                    "extensible",
                    Some("http://example.com/vs/category"),
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
            element: vec![create_element_with_binding(
                "Base.category",
                "example",
                Some("http://example.com/vs/category"),
            )],
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
            assert!(msg.contains("Invalid binding"));
            assert!(msg.contains("Base.category"));
            assert!(msg.contains("weaker"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_binding_same_strength() {
    // Base: required, Differential: required (valid - same)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.code",
                    "required",
                    Some("http://example.com/vs/codes"),
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
            element: vec![create_element_with_binding(
                "Base.code",
                "required",
                Some("http://example.com/vs/specific-codes"),
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
    let code_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.code")
        .unwrap();

    assert_eq!(code_element.binding.as_ref().unwrap().strength, "required");
    assert_eq!(
        code_element
            .binding
            .as_ref()
            .unwrap()
            .value_set
            .as_ref()
            .unwrap(),
        "http://example.com/vs/specific-codes"
    );
}

#[test]
fn test_binding_valueset_change() {
    // Base has valueset, differential changes to more specific valueset (same strength)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.code",
                    "extensible",
                    Some("http://hl7.org/fhir/ValueSet/all-codes"),
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
            element: vec![create_element_with_binding(
                "Base.code",
                "extensible",
                Some("http://example.com/vs/specific-codes"),
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
    let code_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.code")
        .unwrap();

    assert_eq!(
        code_element.binding.as_ref().unwrap().strength,
        "extensible"
    );
    assert_eq!(
        code_element
            .binding
            .as_ref()
            .unwrap()
            .value_set
            .as_ref()
            .unwrap(),
        "http://example.com/vs/specific-codes"
    );
}

#[test]
fn test_binding_inherits_from_base() {
    // Base has binding, differential doesn't specify: should inherit
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.status",
                    "required",
                    Some("http://example.com/vs/status"),
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
            element: vec![ElementDefinition {
                path: "Base.status".to_string(),
                id: Some("Base.status".to_string()),
                min: Some(1), // Only changes cardinality
                max: Some("1".to_string()),
                type_: None,
                binding: None, // Doesn't specify binding
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
    let status_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.status")
        .unwrap();

    // Binding should be inherited from base
    assert!(status_element.binding.is_some());
    assert_eq!(
        status_element.binding.as_ref().unwrap().strength,
        "required"
    );
    assert_eq!(
        status_element
            .binding
            .as_ref()
            .unwrap()
            .value_set
            .as_ref()
            .unwrap(),
        "http://example.com/vs/status"
    );
}

#[test]
fn test_binding_example_to_required() {
    // Base: example (weakest), Differential: required (strongest) - valid
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.code",
                    "example",
                    Some("http://example.com/vs/codes"),
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
            element: vec![create_element_with_binding(
                "Base.code",
                "required",
                Some("http://example.com/vs/codes"),
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
    let code_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.code")
        .unwrap();

    assert_eq!(code_element.binding.as_ref().unwrap().strength, "required");
}

#[test]
fn test_binding_preferred_to_extensible() {
    // Base: preferred, Differential: extensible (valid - stricter)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_binding("Base"),
                create_element_with_binding(
                    "Base.category",
                    "preferred",
                    Some("http://example.com/vs/category"),
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
            element: vec![create_element_with_binding(
                "Base.category",
                "extensible",
                Some("http://example.com/vs/category"),
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
    let category_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.category")
        .unwrap();

    assert_eq!(
        category_element.binding.as_ref().unwrap().strength,
        "extensible"
    );
}
