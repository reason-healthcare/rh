use rh_snapshot::generator::SnapshotGenerator;
use rh_snapshot::types::{Differential, ElementDefinition, Snapshot, StructureDefinition};

fn create_test_element(path: &str) -> ElementDefinition {
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
fn test_property_definition_override() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                definition: Some("Base definition text".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                definition: Some("Profile definition text".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.definition.as_deref(),
        Some("Profile definition text"),
        "Definition should be overridden by differential"
    );
}

#[test]
fn test_property_definition_inherited() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                definition: Some("Base definition text".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                definition: None,
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.definition.as_deref(),
        Some("Base definition text"),
        "Definition should be inherited from base when not specified in differential"
    );
}

#[test]
fn test_property_short_override() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                short: Some("Base short description".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                short: Some("Profile short description".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.short.as_deref(),
        Some("Profile short description"),
        "Short description should be overridden by differential"
    );
}

#[test]
fn test_property_comment_override() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                comment: Some("Base comment".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                comment: Some("Profile comment".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.comment.as_deref(),
        Some("Profile comment"),
        "Comment should be overridden by differential"
    );
}

#[test]
fn test_property_requirements_override() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                requirements: Some("Base requirements".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                requirements: Some("Profile requirements".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.requirements.as_deref(),
        Some("Profile requirements"),
        "Requirements should be overridden by differential"
    );
}

#[test]
fn test_property_must_support_true() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                must_support: Some(false),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                must_support: Some(true),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.must_support,
        Some(true),
        "mustSupport should be overridden to true by differential"
    );
}

#[test]
fn test_property_must_support_inherited() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                must_support: Some(true),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                must_support: None,
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.must_support,
        Some(true),
        "mustSupport should be inherited from base when not specified in differential"
    );
}

#[test]
fn test_property_is_summary_true() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                is_summary: Some(false),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                is_summary: Some(true),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.is_summary,
        Some(true),
        "isSummary should be overridden to true by differential"
    );
}

#[test]
fn test_property_is_modifier_true() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                is_modifier: Some(false),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                is_modifier: Some(true),
                is_modifier_reason: Some("This field changes the meaning".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(
        field.is_modifier,
        Some(true),
        "isModifier should be overridden to true by differential"
    );
    assert_eq!(
        field.is_modifier_reason.as_deref(),
        Some("This field changes the meaning"),
        "isModifierReason should be set by differential"
    );
}

#[test]
fn test_property_multi_level_text_override() {
    let mut generator = SnapshotGenerator::new();

    let resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Resource".to_string(),
        name: "Resource".to_string(),
        type_: "Resource".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                definition: Some("Resource definition".to_string()),
                short: Some("Resource short".to_string()),
                ..create_test_element("Resource.field")
            }],
        }),
    };

    let domain_resource = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
        name: "DomainResource".to_string(),
        type_: "Resource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                definition: Some("DomainResource definition".to_string()),
                ..create_test_element("Resource.field")
            }],
        }),
        snapshot: None,
    };

    let patient = StructureDefinition {
        url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        name: "Patient".to_string(),
        type_: "Resource".to_string(),
        base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                short: Some("Patient short".to_string()),
                ..create_test_element("Resource.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(resource);
    generator.load_structure_definition(domain_resource);
    generator.load_structure_definition(patient);

    let snapshot = generator
        .generate_snapshot("http://hl7.org/fhir/StructureDefinition/Patient")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Resource.field")
        .unwrap();

    assert_eq!(
        field.definition.as_deref(),
        Some("DomainResource definition"),
        "Definition should use most recent override (from DomainResource)"
    );
    assert_eq!(
        field.short.as_deref(),
        Some("Patient short"),
        "Short should use most recent override (from Patient)"
    );
}

#[test]
fn test_property_all_text_fields() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                definition: Some("Base definition".to_string()),
                short: Some("Base short".to_string()),
                comment: Some("Base comment".to_string()),
                requirements: Some("Base requirements".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                definition: Some("Profile definition".to_string()),
                short: Some("Profile short".to_string()),
                comment: Some("Profile comment".to_string()),
                requirements: Some("Profile requirements".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(field.definition.as_deref(), Some("Profile definition"));
    assert_eq!(field.short.as_deref(), Some("Profile short"));
    assert_eq!(field.comment.as_deref(), Some("Profile comment"));
    assert_eq!(field.requirements.as_deref(), Some("Profile requirements"));
}

#[test]
fn test_property_all_boolean_flags() {
    let mut generator = SnapshotGenerator::new();

    let base = StructureDefinition {
        url: "http://example.com/Base".to_string(),
        name: "Base".to_string(),
        type_: "Base".to_string(),
        base_definition: None,
        differential: None,
        snapshot: Some(Snapshot {
            element: vec![ElementDefinition {
                must_support: Some(false),
                is_summary: Some(false),
                is_modifier: Some(false),
                ..create_test_element("Base.field")
            }],
        }),
    };

    let profile = StructureDefinition {
        url: "http://example.com/Profile".to_string(),
        name: "Profile".to_string(),
        type_: "Base".to_string(),
        base_definition: Some("http://example.com/Base".to_string()),
        differential: Some(Differential {
            element: vec![ElementDefinition {
                must_support: Some(true),
                is_summary: Some(true),
                is_modifier: Some(true),
                is_modifier_reason: Some("Changes meaning".to_string()),
                ..create_test_element("Base.field")
            }],
        }),
        snapshot: None,
    };

    generator.load_structure_definition(base);
    generator.load_structure_definition(profile);

    let snapshot = generator
        .generate_snapshot("http://example.com/Profile")
        .unwrap();

    let field = snapshot
        .element
        .iter()
        .find(|e| e.path == "Base.field")
        .unwrap();

    assert_eq!(field.must_support, Some(true));
    assert_eq!(field.is_summary, Some(true));
    assert_eq!(field.is_modifier, Some(true));
    assert_eq!(field.is_modifier_reason.as_deref(), Some("Changes meaning"));
}
