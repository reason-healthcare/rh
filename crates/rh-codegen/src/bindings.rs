//! Binding extraction from FHIR StructureDefinitions
//!
//! This module extracts element bindings (especially required bindings) from FHIR
//! StructureDefinitions and prepares them for code generation.

use crate::fhir_types::{ElementDefinition, StructureDefinition};
use rh_foundation::validation::{BindingStrength, ElementBinding};
use std::collections::HashMap;

/// Extract required bindings from a StructureDefinition
///
/// This function extracts all element bindings with "required" strength from
/// the StructureDefinition's snapshot. Only required bindings are extracted
/// because they must be validated at runtime.
///
/// # Arguments
///
/// * `sd` - The StructureDefinition to extract bindings from
///
/// # Returns
///
/// A vector of ElementBinding structs, sorted by element path
pub fn extract_required_bindings(sd: &StructureDefinition) -> Vec<ElementBinding> {
    let mut bindings = Vec::new();
    let mut seen = HashMap::new();

    // Process snapshot (canonical representation)
    if let Some(snapshot) = &sd.snapshot {
        for element in &snapshot.element {
            if let Some(binding) = extract_binding_from_element(element, &sd.base_type) {
                // Only include required bindings
                if binding.strength == BindingStrength::Required {
                    // Deduplicate by path (snapshot can have duplicates)
                    if !seen.contains_key(&binding.path) {
                        seen.insert(binding.path.clone(), binding.clone());
                        bindings.push(binding);
                    }
                }
            }
        }
    }

    // Sort by path for deterministic output
    bindings.sort_by(|a, b| a.path.cmp(&b.path));
    bindings
}

/// Extract binding from a single element definition
fn extract_binding_from_element(
    element: &ElementDefinition,
    _resource_type: &str,
) -> Option<ElementBinding> {
    let binding = element.binding.as_ref()?;
    let value_set_url = binding.value_set.as_ref()?;

    // Parse binding strength
    let strength = BindingStrength::from_fhir_str(&binding.strength)?;

    // Get element path
    let path = element.path.clone();

    // Create binding with optional description
    let mut elem_binding = ElementBinding::new(path, strength, value_set_url);
    if let Some(desc) = &binding.description {
        elem_binding = elem_binding.with_description(desc);
    }

    Some(elem_binding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::{ElementBinding as FhirElementBinding, StructureDefinitionSnapshot};

    fn make_test_element(path: &str, strength: &str, value_set: &str) -> ElementDefinition {
        ElementDefinition {
            id: Some(path.to_string()),
            path: path.to_string(),
            short: None,
            definition: None,
            min: None,
            max: None,
            element_type: None,
            fixed: None,
            pattern: None,
            binding: Some(FhirElementBinding {
                strength: strength.to_string(),
                description: Some("Test binding".to_string()),
                value_set: Some(value_set.to_string()),
            }),
            constraint: None,
        }
    }

    fn make_test_sd(elements: Vec<ElementDefinition>) -> StructureDefinition {
        StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test".to_string(),
            url: "http://test.com/test".to_string(),
            version: None,
            name: "Test".to_string(),
            title: None,
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: Some(StructureDefinitionSnapshot { element: elements }),
        }
    }

    #[test]
    fn test_extract_required_binding() {
        let element = make_test_element(
            "Patient.gender",
            "required",
            "http://hl7.org/fhir/ValueSet/administrative-gender",
        );
        let sd = make_test_sd(vec![element]);

        let bindings = extract_required_bindings(&sd);

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].path, "Patient.gender");
        assert_eq!(bindings[0].strength, BindingStrength::Required);
        assert_eq!(
            bindings[0].value_set_url,
            "http://hl7.org/fhir/ValueSet/administrative-gender"
        );
        assert!(bindings[0].description.is_some());
    }

    #[test]
    fn test_extract_multiple_required_bindings() {
        let elements = vec![
            make_test_element(
                "Patient.gender",
                "required",
                "http://hl7.org/fhir/ValueSet/administrative-gender",
            ),
            make_test_element(
                "Patient.contact.gender",
                "required",
                "http://hl7.org/fhir/ValueSet/administrative-gender",
            ),
        ];
        let sd = make_test_sd(elements);

        let bindings = extract_required_bindings(&sd);

        assert_eq!(bindings.len(), 2);
        // Should be sorted by path
        assert_eq!(bindings[0].path, "Patient.contact.gender");
        assert_eq!(bindings[1].path, "Patient.gender");
    }

    #[test]
    fn test_skip_non_required_bindings() {
        let elements = vec![
            make_test_element(
                "Patient.gender",
                "required",
                "http://hl7.org/fhir/ValueSet/administrative-gender",
            ),
            make_test_element(
                "Patient.maritalStatus",
                "extensible",
                "http://hl7.org/fhir/ValueSet/marital-status",
            ),
            make_test_element(
                "Patient.communication.language",
                "preferred",
                "http://hl7.org/fhir/ValueSet/languages",
            ),
        ];
        let sd = make_test_sd(elements);

        let bindings = extract_required_bindings(&sd);

        // Only required bindings should be extracted
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].path, "Patient.gender");
    }

    #[test]
    fn test_deduplicate_bindings() {
        // Same binding appears twice (can happen in snapshot)
        let element = make_test_element(
            "Patient.gender",
            "required",
            "http://hl7.org/fhir/ValueSet/administrative-gender",
        );
        let sd = make_test_sd(vec![element.clone(), element]);

        let bindings = extract_required_bindings(&sd);

        // Should only appear once
        assert_eq!(bindings.len(), 1);
    }

    #[test]
    fn test_no_bindings() {
        let element = ElementDefinition {
            id: Some("Patient.id".to_string()),
            path: "Patient.id".to_string(),
            short: None,
            definition: None,
            min: None,
            max: None,
            element_type: None,
            fixed: None,
            pattern: None,
            binding: None,
            constraint: None,
        };
        let sd = make_test_sd(vec![element]);

        let bindings = extract_required_bindings(&sd);

        assert_eq!(bindings.len(), 0);
    }
}
