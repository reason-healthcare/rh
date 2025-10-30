use rh_snapshot::error::SnapshotError;
use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};

fn create_element(path: &str, min: u32, max: &str) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(min),
        max: Some(max.to_string()),
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
fn test_cardinality_can_make_stricter() {
    // Base: 0..*, Differential: 1..1 (valid - stricter)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.field", 0, "*"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.field", 1, "1")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let field_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(field_element.min, Some(1));
    assert_eq!(field_element.max, Some("1".to_string()));
}

#[test]
fn test_cardinality_cannot_relax_min() {
    // Base: 1..1, Differential: 0..1 (invalid - relaxes min)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.field", 1, "1"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.field", 0, "1")],
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
            assert!(msg.contains("differential min"));
            assert!(msg.contains("less than base min"));
            assert!(msg.contains("Base.field"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_cardinality_cannot_relax_max() {
    // Base: 0..1, Differential: 0..* (invalid - relaxes max)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.field", 0, "1"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.field", 0, "*")],
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
            assert!(msg.contains("differential max"));
            assert!(msg.contains("greater than base max"));
            assert!(msg.contains("Base.field"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_cardinality_min_must_not_exceed_max() {
    // Differential: 2..1 (invalid - min > max)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.field", 0, "*"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.field", 2, "1")],
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
            assert!(msg.contains("min"));
            assert!(msg.contains("greater than max"));
            assert!(msg.contains("Base.field"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_cardinality_unbounded_to_bounded() {
    // Base: 0..*, Differential: 1..5 (valid)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.items", 0, "*"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.items", 1, "5")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let field_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.items")
        .unwrap();

    assert_eq!(field_element.min, Some(1));
    assert_eq!(field_element.max, Some("5".to_string()));
}

#[test]
fn test_cardinality_same_as_base() {
    // Base: 1..1, Differential: 1..1 (valid - same)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.required", 1, "1"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.required", 1, "1")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let field_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.required")
        .unwrap();

    assert_eq!(field_element.min, Some(1));
    assert_eq!(field_element.max, Some("1".to_string()));
}

#[test]
fn test_cardinality_optional_to_required() {
    // Base: 0..1, Differential: 1..1 (valid - makes required)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.optional", 0, "1"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.optional", 1, "1")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let field_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.optional")
        .unwrap();

    assert_eq!(field_element.min, Some(1));
    assert_eq!(field_element.max, Some("1".to_string()));
}

#[test]
fn test_cardinality_array_restriction() {
    // Base: 0..*, Differential: 2..10 (valid)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Base", 0, "*"),
                create_element("Base.array", 0, "*"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Base.array", 2, "10")],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.com/Profile");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let field_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.array")
        .unwrap();

    assert_eq!(field_element.min, Some(2));
    assert_eq!(field_element.max, Some("10".to_string()));
}
