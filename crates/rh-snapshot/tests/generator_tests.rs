use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};
use rh_snapshot::{SnapshotError, SnapshotGenerator};

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
fn test_generate_snapshot_with_existing_snapshot() {
    let mut generator = SnapshotGenerator::new();

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Patient")],
        }),
    };

    generator.load_structure_definition(patient);

    let result = generator.generate_snapshot("http://hl7.org/fhir/StructureDefinition/Patient");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert_eq!(snapshot.element.len(), 1);
    assert_eq!(snapshot.element[0].path, "Patient");
}

#[test]
fn test_generate_snapshot_with_differential_only() {
    let mut generator = SnapshotGenerator::new();

    let custom = StructureDefinition {
        url: "http://example.org/StructureDefinition/custom".to_string(),
        name: "Custom".to_string(),
        type_: "Custom".to_string(),
        base_definition: None,
        differential: Some(Differential {
            element: vec![{
                let mut elem = create_element("Custom");
                elem.min = Some(0);
                elem.max = Some("1".to_string());
                elem
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(custom);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/custom");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert_eq!(snapshot.element.len(), 1);
    assert_eq!(snapshot.element[0].path, "Custom");
}

#[test]
fn test_generate_snapshot_with_base_and_differential() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
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
        differential: Some(Differential {
            element: vec![{
                let mut elem = create_element("Patient.identifier");
                elem.min = Some(1);
                elem.max = None;
                elem
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/MyPatient");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert_eq!(snapshot.element.len(), 2);

    let identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier")
        .unwrap();
    assert_eq!(identifier.min, Some(1));
    assert_eq!(identifier.max, Some("*".to_string()));
}

#[test]
fn test_circular_dependency_detection() {
    let mut generator = SnapshotGenerator::new();

    let sd1 = StructureDefinition {
        url: "http://example.org/StructureDefinition/A".to_string(),
        name: "A".to_string(),
        type_: "A".to_string(),
        base_definition: Some("http://example.org/StructureDefinition/B".to_string()),
        differential: Some(Differential { element: vec![] }),
        snapshot: None,
    };

    let sd2 = StructureDefinition {
        url: "http://example.org/StructureDefinition/B".to_string(),
        name: "B".to_string(),
        type_: "B".to_string(),
        base_definition: Some("http://example.org/StructureDefinition/A".to_string()),
        differential: Some(Differential { element: vec![] }),
        snapshot: None,
    };

    generator.load_structure_definition(sd1);
    generator.load_structure_definition(sd2);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/A");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SnapshotError::CircularDependency(_)
    ));
}

#[test]
fn test_missing_base_definition() {
    let mut generator = SnapshotGenerator::new();

    let profile = StructureDefinition {
        url: "http://example.org/StructureDefinition/MyPatient".to_string(),
        name: "MyPatient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Patient".to_string()),
        differential: Some(Differential { element: vec![] }),
        snapshot: None,
    };

    generator.load_structure_definition(profile);

    let result = generator.generate_snapshot("http://example.org/StructureDefinition/MyPatient");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SnapshotError::BaseNotFound(_)
    ));
}

#[test]
fn test_multi_level_inheritance() {
    let mut generator = SnapshotGenerator::new();

    let resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Resource".to_string(),
        name: "Resource".to_string(),
        type_: "Resource".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![create_element("Resource")],
        }),
    };

    let domain_resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
        name: "DomainResource".to_string(),
        type_: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
        differential: Some(Differential {
            element: vec![{
                let mut elem = create_element("DomainResource.text");
                elem.min = Some(0);
                elem.max = Some("1".to_string());
                elem
            }],
        }),
        snapshot: None,
    };

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: Some(Differential {
            element: vec![create_element("Patient.name")],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(resource);
    generator.load_structure_definition(domain_resource);
    generator.load_structure_definition(patient);

    let result = generator.generate_snapshot("http://hl7.org/fhir/StructureDefinition/Patient");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert_eq!(snapshot.element.len(), 3);

    let paths: Vec<String> = snapshot.element.iter().map(|e| e.path.clone()).collect();
    assert!(paths.contains(&"Resource".to_string()));
    assert!(paths.contains(&"DomainResource.text".to_string()));
    assert!(paths.contains(&"Patient.name".to_string()));
}
