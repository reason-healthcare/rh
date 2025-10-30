use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};
use rh_snapshot::SnapshotGenerator;

fn create_element(path: &str) -> ElementDefinition {
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

fn create_element_with_cardinality(path: &str, min: u32, max: &str) -> ElementDefinition {
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
fn test_cache_stores_generated_snapshots() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.org/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base")],
        }),
        differential: None,
    };

    generator.load_structure_definition(base);

    assert_eq!(generator.cache_size(), 0);

    let _snapshot = generator
        .generate_snapshot("http://example.org/Base")
        .unwrap();

    assert_eq!(generator.cache_size(), 1);
}

#[test]
fn test_cache_reuses_generated_snapshots() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.org/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base")],
        }),
        differential: None,
    };

    let profile = StructureDefinition {
        url: "http://example.org/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.org/Base".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![create_element_with_cardinality("Base", 1, "1")],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot1 = generator
        .generate_snapshot("http://example.org/Profile")
        .unwrap();

    assert_eq!(generator.cache_size(), 2);

    let snapshot2 = generator
        .generate_snapshot("http://example.org/Profile")
        .unwrap();

    assert_eq!(snapshot1.element.len(), snapshot2.element.len());
    assert_eq!(generator.cache_size(), 2);
}

#[test]
fn test_cache_handles_inheritance_chain() {
    let mut generator = SnapshotGenerator::new();

    let base1 = StructureDefinition {
        url: "http://example.org/Base1".to_string(),
        name: "Base1".to_string(),
        type_: "Base1".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base1")],
        }),
        differential: None,
    };

    let base2 = StructureDefinition {
        url: "http://example.org/Base2".to_string(),
        name: "Base2".to_string(),
        type_: "Base1".to_string(),
        base_definition: Some("http://example.org/Base1".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![create_element_with_cardinality("Base1", 0, "1")],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.org/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base1".to_string(),
        base_definition: Some("http://example.org/Base2".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![create_element_with_cardinality("Base1", 1, "1")],
        }),
    };

    generator.load_structure_definition(base1);
    generator.load_structure_definition(base2);
    generator.load_structure_definition(profile);

    let _snapshot = generator
        .generate_snapshot("http://example.org/Profile")
        .unwrap();

    assert_eq!(generator.cache_size(), 3);
}

#[test]
fn test_clear_cache_removes_all_entries() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.org/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base")],
        }),
        differential: None,
    };

    generator.load_structure_definition(base);

    let _snapshot = generator
        .generate_snapshot("http://example.org/Base")
        .unwrap();

    assert_eq!(generator.cache_size(), 1);

    generator.clear_cache();

    assert_eq!(generator.cache_size(), 0);
}

#[test]
fn test_cache_after_clear_regenerates() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.org/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base")],
        }),
        differential: None,
    };

    generator.load_structure_definition(base);

    let snapshot1 = generator
        .generate_snapshot("http://example.org/Base")
        .unwrap();

    generator.clear_cache();

    let snapshot2 = generator
        .generate_snapshot("http://example.org/Base")
        .unwrap();

    assert_eq!(snapshot1.element.len(), snapshot2.element.len());
    assert_eq!(generator.cache_size(), 1);
}

#[test]
fn test_cache_with_multiple_profiles() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.org/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Base")],
        }),
        differential: None,
    };

    let profile1 = StructureDefinition {
        url: "http://example.org/Profile1".to_string(),
        name: "Profile1".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.org/Base".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![create_element_with_cardinality("Base", 1, "1")],
        }),
    };

    let profile2 = StructureDefinition {
        url: "http://example.org/Profile2".to_string(),
        name: "Profile2".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.org/Base".to_string()),
        snapshot: None,
        differential: Some(Differential {
            element: vec![create_element_with_cardinality("Base", 0, "5")],
        }),
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile1);
    generator.load_structure_definition(profile2);

    let _snapshot1 = generator
        .generate_snapshot("http://example.org/Profile1")
        .unwrap();
    let _snapshot2 = generator
        .generate_snapshot("http://example.org/Profile2")
        .unwrap();

    assert_eq!(generator.cache_size(), 3);
}
