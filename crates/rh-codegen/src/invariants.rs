//! Invariant extraction from FHIR StructureDefinitions
//!
//! This module extracts invariants (constraints) from FHIR StructureDefinitions
//! and converts them to Rust Invariant structs for use in validation.

use crate::fhir_types::{ElementConstraint, StructureDefinition};
use rh_foundation::{Invariant, Severity};

/// Extract all invariants from a StructureDefinition
///
/// This function processes both the snapshot and differential elements,
/// extracting constraints and converting them to Invariant structs.
/// Only FHIRPath expressions are included (xpath is ignored as legacy).
pub fn extract_invariants(structure_def: &StructureDefinition) -> Vec<Invariant> {
    let mut invariants = Vec::new();

    if let Some(ref snapshot) = structure_def.snapshot {
        for element in &snapshot.element {
            if let Some(ref constraints) = element.constraint {
                for constraint in constraints {
                    if let Some(invariant) = convert_constraint(constraint) {
                        if !invariants
                            .iter()
                            .any(|i: &Invariant| i.key == invariant.key)
                        {
                            invariants.push(invariant);
                        }
                    }
                }
            }
        }
    }

    if let Some(ref differential) = structure_def.differential {
        for element in &differential.element {
            if let Some(ref constraints) = element.constraint {
                for constraint in constraints {
                    if let Some(invariant) = convert_constraint(constraint) {
                        if !invariants
                            .iter()
                            .any(|i: &Invariant| i.key == invariant.key)
                        {
                            invariants.push(invariant);
                        }
                    }
                }
            }
        }
    }
    invariants.sort_by(|a, b| a.key.cmp(&b.key));
    invariants
}

/// Convert a FHIR ElementConstraint to an Invariant
///
/// Returns None if the constraint has no FHIRPath expression (xpath-only constraints
/// are ignored as legacy).
fn convert_constraint(constraint: &ElementConstraint) -> Option<Invariant> {
    let expression = constraint.expression.as_ref()?;

    let severity = match constraint.severity.as_str() {
        "error" => Severity::Error,
        "warning" => Severity::Warning,
        _ => Severity::Information,
    };

    let mut invariant = Invariant::new(&constraint.key, severity, &constraint.human, expression);

    if let Some(xpath) = &constraint.xpath {
        invariant = invariant.with_xpath(xpath);
    }

    Some(invariant)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::{
        ElementDefinition, StructureDefinitionDifferential, StructureDefinitionSnapshot,
    };

    fn create_test_constraint(
        key: &str,
        severity: &str,
        human: &str,
        expression: Option<&str>,
    ) -> ElementConstraint {
        ElementConstraint {
            key: key.to_string(),
            severity: severity.to_string(),
            human: human.to_string(),
            expression: expression.map(|s| s.to_string()),
            xpath: None,
            source: None,
        }
    }

    fn create_test_element(constraints: Vec<ElementConstraint>) -> ElementDefinition {
        ElementDefinition {
            id: Some("Patient".to_string()),
            path: "Patient".to_string(),
            short: None,
            definition: None,
            min: None,
            max: None,
            element_type: None,
            fixed: None,
            pattern: None,
            binding: None,
            constraint: Some(constraints),
        }
    }

    #[test]
    fn test_convert_constraint_error() {
        let constraint =
            create_test_constraint("pat-1", "error", "Name is required", Some("name.exists()"));

        let invariant = convert_constraint(&constraint).unwrap();
        assert_eq!(invariant.key, "pat-1");
        assert_eq!(invariant.severity, Severity::Error);
        assert_eq!(invariant.human, "Name is required");
        assert_eq!(invariant.expression, "name.exists()");
    }

    #[test]
    fn test_convert_constraint_warning() {
        let constraint = create_test_constraint(
            "pat-2",
            "warning",
            "Telecom recommended",
            Some("telecom.exists()"),
        );

        let invariant = convert_constraint(&constraint).unwrap();
        assert_eq!(invariant.severity, Severity::Warning);
    }

    #[test]
    fn test_convert_constraint_no_expression() {
        let constraint = ElementConstraint {
            key: "pat-1".to_string(),
            severity: "error".to_string(),
            human: "Name is required".to_string(),
            expression: None,
            xpath: Some("f:name".to_string()),
            source: None,
        };

        let invariant = convert_constraint(&constraint);
        assert!(invariant.is_none());
    }

    #[test]
    fn test_extract_invariants_from_snapshot() {
        let constraints = vec![
            create_test_constraint("pat-1", "error", "Name required", Some("name.exists()")),
            create_test_constraint(
                "pat-2",
                "warning",
                "Telecom recommended",
                Some("telecom.exists()"),
            ),
        ];

        let element = create_test_element(constraints);
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            version: Some("4.0.1".to_string()),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: Some(StructureDefinitionSnapshot {
                element: vec![element],
            }),
        };

        let invariants = extract_invariants(&structure_def);
        assert_eq!(invariants.len(), 2);
        assert_eq!(invariants[0].key, "pat-1");
        assert_eq!(invariants[1].key, "pat-2");
    }

    #[test]
    fn test_extract_invariants_deduplication() {
        let constraints = vec![create_test_constraint(
            "pat-1",
            "error",
            "Name required",
            Some("name.exists()"),
        )];

        let element1 = create_test_element(constraints.clone());
        let element2 = create_test_element(constraints);

        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            version: Some("4.0.1".to_string()),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: None,
            differential: Some(StructureDefinitionDifferential {
                element: vec![element1],
            }),
            snapshot: Some(StructureDefinitionSnapshot {
                element: vec![element2],
            }),
        };

        let invariants = extract_invariants(&structure_def);
        assert_eq!(invariants.len(), 1);
        assert_eq!(invariants[0].key, "pat-1");
    }

    #[test]
    fn test_extract_invariants_sorted() {
        let constraints = vec![
            create_test_constraint("pat-3", "error", "Test 3", Some("true")),
            create_test_constraint("pat-1", "error", "Test 1", Some("true")),
            create_test_constraint("pat-2", "error", "Test 2", Some("true")),
        ];

        let element = create_test_element(constraints);
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Test".to_string(),
            url: "http://example.com/Test".to_string(),
            version: None,
            name: "Test".to_string(),
            title: None,
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Test".to_string(),
            base_definition: None,
            differential: None,
            snapshot: Some(StructureDefinitionSnapshot {
                element: vec![element],
            }),
        };

        let invariants = extract_invariants(&structure_def);
        assert_eq!(invariants.len(), 3);
        assert_eq!(invariants[0].key, "pat-1");
        assert_eq!(invariants[1].key, "pat-2");
        assert_eq!(invariants[2].key, "pat-3");
    }

    #[test]
    fn test_extract_invariants_empty() {
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Test".to_string(),
            url: "http://example.com/Test".to_string(),
            version: None,
            name: "Test".to_string(),
            title: None,
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Test".to_string(),
            base_definition: None,
            differential: None,
            snapshot: None,
        };

        let invariants = extract_invariants(&structure_def);
        assert_eq!(invariants.len(), 0);
    }
}
