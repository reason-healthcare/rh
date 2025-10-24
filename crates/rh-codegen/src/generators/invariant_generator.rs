//! Invariant constant generation
//!
//! This module generates constant declarations for FHIR invariants that can be
//! embedded in generated resource and datatype structs.

use crate::fhir_types::StructureDefinition;
use crate::invariants;
use quote::quote;

/// Generator for invariant constants
pub struct InvariantGenerator;

impl InvariantGenerator {
    /// Generate a INVARIANTS constant for a StructureDefinition
    ///
    /// Returns Rust code as a string containing the lazy static declaration,
    /// or an empty string if there are no invariants.
    ///
    /// Uses `once_cell::sync::Lazy` for runtime initialization since `Invariant::new()`
    /// is not a const fn (it uses `.into()` for String conversions).
    ///
    /// Example output:
    /// ```rust,ignore
    /// pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    ///     once_cell::sync::Lazy::new(|| vec![
    ///         rh_foundation::Invariant::new(
    ///             "pat-1",
    ///             rh_foundation::Severity::Error,
    ///             "SHALL at least contain a contact's details or a reference to an organization",
    ///             "name.exists() or telecom.exists() or address.exists() or organization.exists()"
    ///         ).with_xpath(""),
    ///     ]);
    /// ```
    pub fn generate_invariants_constant(structure_def: &StructureDefinition) -> String {
        let invariants = invariants::extract_invariants(structure_def);

        if invariants.is_empty() {
            return String::new();
        }

        let mut code = String::new();
        code.push_str("/// FHIR invariants for this resource/datatype\n");
        code.push_str("///\n");
        code.push_str(
            "/// These constraints are defined in the FHIR specification and must be validated\n",
        );
        code.push_str("/// when creating or modifying instances of this type.\n");
        code.push_str("pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> = once_cell::sync::Lazy::new(|| vec![\n");

        for invariant in &invariants {
            // Escape strings for Rust string literals
            let key = escape_rust_string(&invariant.key);
            let human = escape_rust_string(&invariant.human);
            let expression = escape_rust_string(&invariant.expression);

            let severity = match invariant.severity {
                rh_foundation::Severity::Error => "rh_foundation::Severity::Error",
                rh_foundation::Severity::Warning => "rh_foundation::Severity::Warning",
                rh_foundation::Severity::Information => "rh_foundation::Severity::Information",
            };

            // Use the builder pattern: Invariant::new().with_xpath() if xpath exists
            code.push_str(&format!(
                "    rh_foundation::Invariant::new(\"{key}\", {severity}, \"{human}\", \"{expression}\")"
            ));

            if let Some(xpath) = &invariant.xpath {
                let escaped_xpath = escape_rust_string(xpath);
                code.push_str(&format!(".with_xpath(\"{escaped_xpath}\")"));
            }

            code.push_str(",\n");
        }

        code.push_str("]);\n");
        code
    }

    /// Generate invariants constant using quote! macro (alternative implementation)
    ///
    /// This generates the same output but uses proc_macro2::TokenStream,
    /// which can be useful for integration with other token-based code generation.
    #[allow(dead_code)]
    pub fn generate_invariants_tokens(
        structure_def: &StructureDefinition,
    ) -> proc_macro2::TokenStream {
        let invariants = invariants::extract_invariants(structure_def);

        if invariants.is_empty() {
            return quote! {};
        }

        let invariant_items: Vec<_> = invariants
            .iter()
            .map(|inv| {
                let key = &inv.key;
                let human = &inv.human;
                let expression = &inv.expression;

                let severity = match inv.severity {
                    rh_foundation::Severity::Error => {
                        quote! { rh_foundation::Severity::Error }
                    }
                    rh_foundation::Severity::Warning => {
                        quote! { rh_foundation::Severity::Warning }
                    }
                    rh_foundation::Severity::Information => {
                        quote! { rh_foundation::Severity::Information }
                    }
                };

                quote! {
                    rh_foundation::Invariant {
                        key: #key,
                        severity: #severity,
                        human: #human,
                        expression: #expression,
                    }
                }
            })
            .collect();

        quote! {
            /// FHIR invariants for this resource/datatype
            ///
            /// These constraints are defined in the FHIR specification and must be validated
            /// when creating or modifying instances of this type.
            pub const INVARIANTS: &[rh_foundation::Invariant] = &[
                #(#invariant_items),*
            ];
        }
    }
}

/// Escape a string for use in a Rust string literal
fn escape_rust_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            c => vec![c],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::{ElementConstraint, ElementDefinition, StructureDefinitionSnapshot};

    fn create_test_structure_def(constraints: Vec<ElementConstraint>) -> StructureDefinition {
        let element = ElementDefinition {
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
        };

        StructureDefinition {
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
        }
    }

    #[test]
    fn test_generate_empty_invariants() {
        let structure_def = create_test_structure_def(vec![]);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);
        assert_eq!(code, "");
    }

    #[test]
    fn test_generate_single_invariant() {
        let constraint = ElementConstraint {
            key: "pat-1".to_string(),
            severity: "error".to_string(),
            human: "Name is required".to_string(),
            expression: Some("name.exists()".to_string()),
            xpath: None,
            source: None,
        };

        let structure_def = create_test_structure_def(vec![constraint]);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);

        assert!(code.contains("pub static INVARIANTS"));
        assert!(code.contains("once_cell::sync::Lazy"));
        assert!(code.contains("\"pat-1\""));
        assert!(code.contains("rh_foundation::Severity::Error"));
        assert!(code.contains("\"Name is required\""));
        assert!(code.contains("\"name.exists()\""));
    }

    #[test]
    fn test_generate_multiple_invariants() {
        let constraints = vec![
            ElementConstraint {
                key: "pat-1".to_string(),
                severity: "error".to_string(),
                human: "Name required".to_string(),
                expression: Some("name.exists()".to_string()),
                xpath: None,
                source: None,
            },
            ElementConstraint {
                key: "pat-2".to_string(),
                severity: "warning".to_string(),
                human: "Telecom recommended".to_string(),
                expression: Some("telecom.exists()".to_string()),
                xpath: None,
                source: None,
            },
        ];

        let structure_def = create_test_structure_def(constraints);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);

        assert!(code.contains("pat-1"));
        assert!(code.contains("pat-2"));
        assert!(code.contains("rh_foundation::Severity::Error"));
        assert!(code.contains("rh_foundation::Severity::Warning"));
    }

    #[test]
    fn test_escape_strings_with_quotes() {
        let constraint = ElementConstraint {
            key: "test-1".to_string(),
            severity: "error".to_string(),
            human: "Must have \"value\"".to_string(),
            expression: Some("value.exists()".to_string()),
            xpath: None,
            source: None,
        };

        let structure_def = create_test_structure_def(vec![constraint]);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);

        assert!(code.contains("Must have \\\"value\\\""));
    }

    #[test]
    fn test_escape_strings_with_backslashes() {
        let constraint = ElementConstraint {
            key: "test-1".to_string(),
            severity: "error".to_string(),
            human: "Path: C:\\temp\\file".to_string(),
            expression: Some("true".to_string()),
            xpath: None,
            source: None,
        };

        let structure_def = create_test_structure_def(vec![constraint]);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);

        assert!(code.contains("C:\\\\temp\\\\file"));
    }

    #[test]
    fn test_tokens_generation() {
        let constraint = ElementConstraint {
            key: "pat-1".to_string(),
            severity: "error".to_string(),
            human: "Name required".to_string(),
            expression: Some("name.exists()".to_string()),
            xpath: None,
            source: None,
        };

        let structure_def = create_test_structure_def(vec![constraint]);
        let tokens = InvariantGenerator::generate_invariants_tokens(&structure_def);

        let code = tokens.to_string();
        assert!(code.contains("INVARIANTS"));
        assert!(code.contains("pat-1"));
    }

    #[test]
    fn test_severity_mapping() {
        let constraints = vec![
            ElementConstraint {
                key: "err-1".to_string(),
                severity: "error".to_string(),
                human: "Error test".to_string(),
                expression: Some("true".to_string()),
                xpath: None,
                source: None,
            },
            ElementConstraint {
                key: "warn-1".to_string(),
                severity: "warning".to_string(),
                human: "Warning test".to_string(),
                expression: Some("true".to_string()),
                xpath: None,
                source: None,
            },
            ElementConstraint {
                key: "info-1".to_string(),
                severity: "information".to_string(),
                human: "Info test".to_string(),
                expression: Some("true".to_string()),
                xpath: None,
                source: None,
            },
        ];

        let structure_def = create_test_structure_def(constraints);
        let code = InvariantGenerator::generate_invariants_constant(&structure_def);

        assert!(code.contains("rh_foundation::Severity::Error"));
        assert!(code.contains("rh_foundation::Severity::Warning"));
        assert!(code.contains("rh_foundation::Severity::Information"));
    }
}
