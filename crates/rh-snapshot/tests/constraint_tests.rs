use rh_snapshot::error::SnapshotError;
use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{
    Differential, ElementConstraint, ElementDefinition, Snapshot, StructureDefinition,
};

fn create_element_with_constraints(
    path: &str,
    constraints: Vec<(&str, &str, &str, Option<&str>)>,
) -> ElementDefinition {
    ElementDefinition {
        path: path.to_string(),
        id: Some(path.to_string()),
        min: Some(0),
        max: Some("*".to_string()),
        type_: None,
        binding: None,
        constraint: Some(
            constraints
                .into_iter()
                .map(|(key, severity, human, expression)| ElementConstraint {
                    key: key.to_string(),
                    severity: severity.to_string(),
                    human: human.to_string(),
                    expression: expression.map(|s| s.to_string()),
                })
                .collect(),
        ),
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

fn create_element_without_constraints(path: &str) -> ElementDefinition {
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
fn test_constraint_accumulation() {
    // Base has 1 constraint, differential adds 1 more
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![("base-1", "error", "Base constraint", Some("value.exists()"))],
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
            element: vec![create_element_with_constraints(
                "Base.value",
                vec![(
                    "diff-1",
                    "warning",
                    "Differential constraint",
                    Some("value.length() > 0"),
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
    let value_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.value")
        .unwrap();

    assert!(value_element.constraint.is_some());
    let constraints = value_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 2);

    // Check both constraints are present
    assert!(constraints.iter().any(|c| c.key == "base-1"));
    assert!(constraints.iter().any(|c| c.key == "diff-1"));
}

#[test]
fn test_constraint_duplicate_key_different_expression() {
    // Base and differential both define same key with different expressions (error)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![("con-1", "error", "Must exist", Some("value.exists()"))],
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
            element: vec![create_element_with_constraints(
                "Base.value",
                vec![(
                    "con-1",
                    "error",
                    "Different constraint",
                    Some("value.length() > 0"),
                )],
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
            assert!(msg.contains("Duplicate constraint key"));
            assert!(msg.contains("con-1"));
            assert!(msg.contains("Base.value"));
        }
        _ => panic!("Expected MergeError, got {err:?}"),
    }
}

#[test]
fn test_constraint_duplicate_key_same_expression() {
    // Base and differential define same key with same expression (allowed - skip duplicate)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![("con-1", "error", "Must exist", Some("value.exists()"))],
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
            element: vec![create_element_with_constraints(
                "Base.value",
                vec![("con-1", "error", "Must exist", Some("value.exists()"))],
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
        .find(|e| e.path == "Base.value")
        .unwrap();

    assert!(value_element.constraint.is_some());
    let constraints = value_element.constraint.as_ref().unwrap();
    // Should only have 1 constraint (duplicate skipped)
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].key, "con-1");
}

#[test]
fn test_constraint_multiple_accumulation() {
    // Base has 2 constraints, differential adds 2 more
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![
                        ("base-1", "error", "First base constraint", Some("exists()")),
                        (
                            "base-2",
                            "warning",
                            "Second base constraint",
                            Some("length() > 0"),
                        ),
                    ],
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
            element: vec![create_element_with_constraints(
                "Base.value",
                vec![
                    (
                        "diff-1",
                        "error",
                        "First diff constraint",
                        Some("matches('[A-Z]')"),
                    ),
                    (
                        "diff-2",
                        "error",
                        "Second diff constraint",
                        Some("length() <= 100"),
                    ),
                ],
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
        .find(|e| e.path == "Base.value")
        .unwrap();

    assert!(value_element.constraint.is_some());
    let constraints = value_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 4);

    // Verify all 4 constraints are present
    assert!(constraints.iter().any(|c| c.key == "base-1"));
    assert!(constraints.iter().any(|c| c.key == "base-2"));
    assert!(constraints.iter().any(|c| c.key == "diff-1"));
    assert!(constraints.iter().any(|c| c.key == "diff-2"));
}

#[test]
fn test_constraint_inherits_from_base() {
    // Base has constraints, differential doesn't add any
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![(
                        "base-1",
                        "error",
                        "Must be valid",
                        Some("matches('[A-Z]+')"),
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
            element: vec![ElementDefinition {
                path: "Base.value".to_string(),
                id: Some("Base.value".to_string()),
                min: Some(1), // Only changes cardinality
                max: Some("1".to_string()),
                type_: None,
                binding: None,
                constraint: None, // Doesn't add constraints
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

    // Constraints should be inherited from base
    assert!(value_element.constraint.is_some());
    let constraints = value_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].key, "base-1");
}

#[test]
fn test_constraint_properties() {
    // Verify all constraint properties are preserved
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.identifier",
                    vec![(
                        "id-1",
                        "error",
                        "Identifier must have a value",
                        Some("value.exists()"),
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
            element: vec![create_element_with_constraints(
                "Base.identifier",
                vec![(
                    "id-2",
                    "warning",
                    "Identifier should follow pattern",
                    Some("value.matches('[A-Z][0-9]+')"),
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
    let id_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.identifier")
        .unwrap();

    let constraints = id_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 2);

    // Check first constraint properties
    let con1 = constraints.iter().find(|c| c.key == "id-1").unwrap();
    assert_eq!(con1.severity, "error");
    assert_eq!(con1.human, "Identifier must have a value");
    assert_eq!(con1.expression.as_deref(), Some("value.exists()"));

    // Check second constraint properties
    let con2 = constraints.iter().find(|c| c.key == "id-2").unwrap();
    assert_eq!(con2.severity, "warning");
    assert_eq!(con2.human, "Identifier should follow pattern");
    assert_eq!(
        con2.expression.as_deref(),
        Some("value.matches('[A-Z][0-9]+')")
    );
}

#[test]
fn test_constraint_no_expression() {
    // Some constraints may not have expressions (legacy xpath only)
    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Base"),
                create_element_with_constraints(
                    "Base.value",
                    vec![("base-1", "error", "Legacy constraint", None)],
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
            element: vec![create_element_with_constraints(
                "Base.value",
                vec![("diff-1", "error", "Modern constraint", Some("exists()"))],
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
        .find(|e| e.path == "Base.value")
        .unwrap();

    let constraints = value_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 2);

    let legacy = constraints.iter().find(|c| c.key == "base-1").unwrap();
    assert!(legacy.expression.is_none());

    let modern = constraints.iter().find(|c| c.key == "diff-1").unwrap();
    assert_eq!(modern.expression.as_deref(), Some("exists()"));
}

#[test]
fn test_constraint_multi_level_inheritance() {
    // Three-level hierarchy: Resource → DomainResource → Patient
    // Each level adds constraints
    let resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Resource".to_string(),
        name: "Resource".to_string(),
        type_: "Resource".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![
                create_element_without_constraints("Resource"),
                create_element_with_constraints(
                    "Resource.id",
                    vec![("res-1", "error", "Resource constraint", Some("exists()"))],
                ),
            ],
        }),
    };

    let domain_resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
        name: "DomainResource".to_string(),
        type_: "DomainResource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_constraints(
                "Resource.id",
                vec![(
                    "dom-1",
                    "error",
                    "DomainResource constraint",
                    Some("length() > 0"),
                )],
            )],
        }),
        snapshot: None,
    };

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Patient".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: Some(Differential {
            element: vec![create_element_with_constraints(
                "Resource.id",
                vec![(
                    "pat-1",
                    "warning",
                    "Patient constraint",
                    Some("matches('[a-z0-9-]+')"),
                )],
            )],
        }),
        snapshot: None,
    };

    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definition(resource);
    generator.load_structure_definition(domain_resource);
    generator.load_structure_definition(patient);

    let result = generator.generate_snapshot("http://hl7.org/fhir/StructureDefinition/Patient");
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    let id_element = snapshot
        .element
        .iter()
        .find(|e| e.path == "Resource.id")
        .unwrap();

    // Should have all 3 constraints from the hierarchy
    let constraints = id_element.constraint.as_ref().unwrap();
    assert_eq!(constraints.len(), 3);
    assert!(constraints.iter().any(|c| c.key == "res-1"));
    assert!(constraints.iter().any(|c| c.key == "dom-1"));
    assert!(constraints.iter().any(|c| c.key == "pat-1"));
}
