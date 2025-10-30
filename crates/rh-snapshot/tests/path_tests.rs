use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};

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

#[test]
fn test_adding_new_child_element() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.name"),
                create_element("Patient.name.family"),
                create_element("Patient.name.given"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Patient.name.suffix")],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let suffix = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.name.suffix");
    assert!(
        suffix.is_some(),
        "New child element 'Patient.name.suffix' should be added"
    );

    assert_eq!(
        snapshot.element.len(),
        5,
        "Should have 4 base elements + 1 new child"
    );
}

#[test]
fn test_nested_element_merging() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Patient"), create_element("Patient.name"), {
                let mut elem = create_element("Patient.name.given");
                elem.min = Some(0);
                elem.max = Some("*".to_string());
                elem
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![{
                let mut elem = create_element("Patient.name.given");
                elem.min = Some(1);
                elem.max = Some("1".to_string());
                elem
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let given = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.name.given")
        .unwrap();

    assert_eq!(
        given.min,
        Some(1),
        "Nested element cardinality should be overridden"
    );
    assert_eq!(given.max.as_deref(), Some("1"));
}

#[test]
fn test_choice_type_matching() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Observation"), {
                let mut elem = create_element("Observation.value[x]");
                elem.min = Some(0);
                elem.max = Some("1".to_string());
                elem
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![{
                let mut elem = create_element("Observation.valueString");
                elem.min = Some(1);
                elem.max = Some("1".to_string());
                elem
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let value_string = snapshot
        .element
        .iter()
        .find(|e| e.path == "Observation.valueString");

    assert!(
        value_string.is_some(),
        "Choice type variant valueString should be present"
    );
    assert_eq!(value_string.unwrap().min, Some(1));
}

#[test]
fn test_multiple_choice_type_variants() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Observation"),
                create_element("Observation.value[x]"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![
                create_element("Observation.valueString"),
                create_element("Observation.valueQuantity"),
                create_element("Observation.valueCodeableConcept"),
            ],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let value_string = snapshot
        .element
        .iter()
        .find(|e| e.path == "Observation.valueString");
    let value_quantity = snapshot
        .element
        .iter()
        .find(|e| e.path == "Observation.valueQuantity");
    let value_codeable = snapshot
        .element
        .iter()
        .find(|e| e.path == "Observation.valueCodeableConcept");

    assert!(value_string.is_some(), "valueString variant should exist");
    assert!(
        value_quantity.is_some(),
        "valueQuantity variant should exist"
    );
    assert!(
        value_codeable.is_some(),
        "valueCodeableConcept variant should exist"
    );
}

#[test]
fn test_deep_nesting() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.name"),
                create_element("Patient.name.given"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Patient.name.given.extension")],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let extension = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.name.given.extension");
    assert!(extension.is_some(), "Deeply nested element should be added");
}

#[test]
fn test_element_ordering_preserved() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element("Patient"),
                create_element("Patient.identifier"),
                create_element("Patient.name"),
                create_element("Patient.telecom"),
            ],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![
                create_element("Patient.identifier.system"),
                create_element("Patient.name.family"),
            ],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let paths: Vec<&str> = snapshot.element.iter().map(|e| e.path.as_str()).collect();

    assert_eq!(paths[0], "Patient");
    assert_eq!(paths[1], "Patient.identifier");
    assert_eq!(paths[2], "Patient.identifier.system");
    assert_eq!(paths[3], "Patient.name");
    assert_eq!(paths[4], "Patient.name.family");
    assert_eq!(paths[5], "Patient.telecom");
}

#[test]
fn test_multiple_children_addition() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Patient"), create_element("Patient.name")],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![
                create_element("Patient.name.family"),
                create_element("Patient.name.given"),
                create_element("Patient.name.prefix"),
                create_element("Patient.name.suffix"),
            ],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    assert_eq!(
        snapshot.element.len(),
        6,
        "Should have 2 base + 4 new children"
    );

    let has_family = snapshot
        .element
        .iter()
        .any(|e| e.path == "Patient.name.family");
    let has_given = snapshot
        .element
        .iter()
        .any(|e| e.path == "Patient.name.given");
    let has_prefix = snapshot
        .element
        .iter()
        .any(|e| e.path == "Patient.name.prefix");
    let has_suffix = snapshot
        .element
        .iter()
        .any(|e| e.path == "Patient.name.suffix");

    assert!(has_family);
    assert!(has_given);
    assert!(has_prefix);
    assert!(has_suffix);
}
