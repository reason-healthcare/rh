//! Binding constant generation
//!
//! This module generates constant declarations for FHIR element bindings that can be
//! embedded in generated resource and datatype structs.

use crate::bindings;
use crate::fhir_types::StructureDefinition;
use quote::quote;

/// Generator for binding constants
pub struct BindingGenerator;

impl BindingGenerator {
    /// Generate a BINDINGS constant for a StructureDefinition
    ///
    /// Returns Rust code as a string containing the static declaration,
    /// or an empty string if there are no required bindings.
    ///
    /// Uses `once_cell::sync::Lazy` for runtime initialization.
    ///
    /// Example output:
    /// ```rust,ignore
    /// pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    ///     once_cell::sync::Lazy::new(|| vec![
    ///         rh_foundation::ElementBinding::new(
    ///             "Patient.gender",
    ///             rh_foundation::BindingStrength::Required,
    ///             "http://hl7.org/fhir/ValueSet/administrative-gender"
    ///         ).with_description("The gender of the patient"),
    ///     ]);
    /// ```
    pub fn generate_bindings_constant(structure_def: &StructureDefinition) -> String {
        let bindings = bindings::extract_required_bindings(structure_def);

        if bindings.is_empty() {
            return String::new();
        }

        let mut code = String::new();
        code.push_str("/// FHIR required bindings for this resource/datatype\n");
        code.push_str("///\n");
        code.push_str(
            "/// These bindings define which ValueSets must be used for coded elements.\n",
        );
        code.push_str(
            "/// Only 'required' strength bindings are included (extensible/preferred are not enforced).\n",
        );
        code.push_str("pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> = once_cell::sync::Lazy::new(|| vec![\n");

        for binding in &bindings {
            // Escape strings for Rust string literals
            let path = escape_rust_string(&binding.path);
            let value_set_url = escape_rust_string(&binding.value_set_url);

            let strength = match binding.strength {
                rh_foundation::validation::BindingStrength::Required => {
                    "rh_foundation::BindingStrength::Required"
                }
                rh_foundation::validation::BindingStrength::Extensible => {
                    "rh_foundation::BindingStrength::Extensible"
                }
                rh_foundation::validation::BindingStrength::Preferred => {
                    "rh_foundation::BindingStrength::Preferred"
                }
                rh_foundation::validation::BindingStrength::Example => {
                    "rh_foundation::BindingStrength::Example"
                }
            };

            // Use the builder pattern: ElementBinding::new().with_description() if description exists
            code.push_str(&format!(
                "    rh_foundation::ElementBinding::new(\"{path}\", {strength}, \"{value_set_url}\")"
            ));

            if let Some(description) = &binding.description {
                let escaped_desc = escape_rust_string(description);
                code.push_str(&format!(".with_description(\"{escaped_desc}\")"));
            }

            code.push_str(",\n");
        }

        code.push_str("]);\n");
        code
    }

    /// Generate bindings constant using quote! macro (alternative implementation)
    #[allow(dead_code)]
    pub fn generate_bindings_tokens(
        structure_def: &StructureDefinition,
    ) -> proc_macro2::TokenStream {
        let bindings = bindings::extract_required_bindings(structure_def);

        if bindings.is_empty() {
            return quote! {};
        }

        let binding_items: Vec<_> = bindings
            .iter()
            .map(|binding| {
                let path = &binding.path;
                let value_set_url = &binding.value_set_url;

                let strength_tokens = match binding.strength {
                    rh_foundation::validation::BindingStrength::Required => {
                        quote! { rh_foundation::BindingStrength::Required }
                    }
                    rh_foundation::validation::BindingStrength::Extensible => {
                        quote! { rh_foundation::BindingStrength::Extensible }
                    }
                    rh_foundation::validation::BindingStrength::Preferred => {
                        quote! { rh_foundation::BindingStrength::Preferred }
                    }
                    rh_foundation::validation::BindingStrength::Example => {
                        quote! { rh_foundation::BindingStrength::Example }
                    }
                };

                if let Some(description) = &binding.description {
                    quote! {
                        rh_foundation::ElementBinding::new(#path, #strength_tokens, #value_set_url)
                            .with_description(#description)
                    }
                } else {
                    quote! {
                        rh_foundation::ElementBinding::new(#path, #strength_tokens, #value_set_url)
                    }
                }
            })
            .collect();

        quote! {
            /// FHIR required bindings for this resource/datatype
            ///
            /// These bindings define which ValueSets must be used for coded elements.
            /// Only 'required' strength bindings are included (extensible/preferred are not enforced).
            pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
                once_cell::sync::Lazy::new(|| vec![
                    #(#binding_items),*
                ]);
        }
    }
}

/// Escape a string for use in a Rust string literal
fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::{
        ElementBinding as FhirElementBinding, ElementDefinition, StructureDefinitionSnapshot,
    };

    fn make_test_element(
        path: &str,
        strength: &str,
        value_set: &str,
        description: Option<&str>,
    ) -> ElementDefinition {
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
                description: description.map(|s| s.to_string()),
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
    fn test_generate_single_binding() {
        let element = make_test_element(
            "Patient.gender",
            "required",
            "http://hl7.org/fhir/ValueSet/administrative-gender",
            Some("The gender of the patient"),
        );
        let sd = make_test_sd(vec![element]);

        let code = BindingGenerator::generate_bindings_constant(&sd);

        assert!(!code.is_empty());
        assert!(code.contains("BINDINGS"));
        assert!(code.contains("Patient.gender"));
        assert!(code.contains("BindingStrength::Required"));
        assert!(code.contains("administrative-gender"));
        assert!(code.contains("The gender of the patient"));
    }

    #[test]
    fn test_generate_no_bindings() {
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

        let code = BindingGenerator::generate_bindings_constant(&sd);

        assert!(code.is_empty());
    }

    #[test]
    fn test_generate_skip_non_required() {
        let element = make_test_element(
            "Patient.maritalStatus",
            "extensible",
            "http://hl7.org/fhir/ValueSet/marital-status",
            None,
        );
        let sd = make_test_sd(vec![element]);

        let code = BindingGenerator::generate_bindings_constant(&sd);

        // Should be empty because binding is not required
        assert!(code.is_empty());
    }

    #[test]
    fn test_string_escaping() {
        assert_eq!(escape_rust_string("simple"), "simple");
        assert_eq!(escape_rust_string("with \"quotes\""), "with \\\"quotes\\\"");
        assert_eq!(escape_rust_string("with \n newline"), "with \\n newline");
        assert_eq!(
            escape_rust_string("with \\ backslash"),
            "with \\\\ backslash"
        );
    }

    #[test]
    fn test_generate_tokens() {
        let element = make_test_element(
            "Patient.gender",
            "required",
            "http://hl7.org/fhir/ValueSet/administrative-gender",
            Some("The gender of the patient"),
        );
        let sd = make_test_sd(vec![element]);

        let tokens = BindingGenerator::generate_bindings_tokens(&sd);
        let code = tokens.to_string();

        assert!(!code.is_empty());
        assert!(code.contains("BINDINGS"));
        assert!(code.contains("Patient.gender"));
    }
}
