//! Validation trait generation
//!
//! This module generates the ValidatableResource trait and implementations
//! for FHIR resources and complex types that have invariants.

use crate::fhir_types::StructureDefinition;
use crate::invariants;

/// Generator for validation traits and implementations
pub struct ValidationTraitGenerator;

impl ValidationTraitGenerator {
    /// Generate the ValidatableResource trait definition
    ///
    /// This trait provides access to invariants for validation purposes.
    ///
    /// Example output:
    /// ```rust,ignore
    /// pub trait ValidatableResource {
    ///     fn resource_type(&self) -> &'static str;
    ///     fn invariants() -> &'static [rh_foundation::Invariant];
    ///     fn bindings() -> &'static [rh_foundation::ElementBinding] {
    ///         &[]
    ///     }
    ///     fn profile_url() -> Option<&'static str> {
    ///         None
    ///     }
    /// }
    /// ```
    pub fn generate_trait_definition() -> String {
        let mut code = String::new();

        code.push_str("/// Trait for FHIR types that can be validated using invariants\n");
        code.push_str("///\n");
        code.push_str("/// This trait provides access to the invariants (constraints) defined\n");
        code.push_str("/// in the FHIR specification for this resource or datatype.\n");
        code.push_str("pub trait ValidatableResource {\n");
        code.push_str("    /// Returns the FHIR resource type name\n");
        code.push_str("    fn resource_type(&self) -> &'static str;\n");
        code.push('\n');
        code.push_str("    /// Returns the invariants (constraints) for this resource/datatype\n");
        code.push_str("    ///\n");
        code.push_str("    /// These are the FHIRPath expressions that must evaluate to true\n");
        code.push_str("    /// for the resource to be considered valid.\n");
        code.push_str("    fn invariants() -> &'static [rh_foundation::Invariant];\n");
        code.push('\n');
        code.push_str("    /// Returns the required bindings for this resource/datatype\n");
        code.push_str("    ///\n");
        code.push_str("    /// These are the ValueSet bindings with \"required\" strength that\n");
        code.push_str("    /// must be validated at runtime.\n");
        code.push_str("    fn bindings() -> &'static [rh_foundation::ElementBinding] {\n");
        code.push_str("        &[]\n");
        code.push_str("    }\n");
        code.push('\n');
        code.push_str("    /// Returns the cardinality constraints for this resource/datatype\n");
        code.push_str("    ///\n");
        code.push_str(
            "    /// These define the minimum and maximum occurrences allowed for each element.\n",
        );
        code.push_str("    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {\n");
        code.push_str("        &[]\n");
        code.push_str("    }\n");
        code.push('\n');
        code.push_str("    /// Returns the profile URL if this is a profile, None otherwise\n");
        code.push_str("    fn profile_url() -> Option<&'static str> {\n");
        code.push_str("        None\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        code
    }

    /// Generate ValidatableResource trait implementation for a StructureDefinition
    ///
    /// Only generates an implementation if the type has invariants or bindings.
    ///
    /// Example output:
    /// ```rust,ignore
    /// impl ValidatableResource for Patient {
    ///     fn resource_type(&self) -> &'static str {
    ///         "Patient"
    ///     }
    ///     
    ///     fn invariants() -> &'static [rh_foundation::Invariant] {
    ///         &INVARIANTS
    ///     }
    ///     
    ///     fn bindings() -> &'static [rh_foundation::ElementBinding] {
    ///         &BINDINGS
    ///     }
    ///     
    ///     fn profile_url() -> Option<&'static str> {
    ///         Some("http://hl7.org/fhir/StructureDefinition/Patient")
    ///     }
    /// }
    /// ```
    pub fn generate_trait_impl(structure_def: &StructureDefinition) -> String {
        let invariants = invariants::extract_invariants(structure_def);
        let bindings = crate::bindings::extract_required_bindings(structure_def);

        // Only generate implementation if there are invariants or bindings
        if invariants.is_empty() && bindings.is_empty() {
            return String::new();
        }

        let struct_name = crate::naming::Naming::struct_name(structure_def);
        let resource_type = &structure_def.base_type;

        let mut code = String::new();

        code.push_str(&format!(
            "impl crate::validation::ValidatableResource for {struct_name} {{\n"
        ));
        code.push_str("    fn resource_type(&self) -> &'static str {\n");
        code.push_str(&format!("        \"{resource_type}\"\n"));
        code.push_str("    }\n");
        code.push('\n');
        code.push_str("    fn invariants() -> &'static [rh_foundation::Invariant] {\n");
        if invariants.is_empty() {
            code.push_str("        &[]\n");
        } else {
            code.push_str("        &INVARIANTS\n");
        }
        code.push_str("    }\n");

        // Add bindings() method if there are bindings
        if !bindings.is_empty() {
            code.push('\n');
            code.push_str("    fn bindings() -> &'static [rh_foundation::ElementBinding] {\n");
            code.push_str("        &BINDINGS\n");
            code.push_str("    }\n");
        }

        // Add cardinalities() method - always include for resources/complex types
        // We check if CARDINALITIES constant exists by looking at kind
        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            code.push('\n');
            code.push_str(
                "    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {\n",
            );
            code.push_str("        &CARDINALITIES\n");
            code.push_str("    }\n");
        }

        // Add profile_url if this is a constraint (profile)
        // Profiles are identified by having kind="resource" and derivation="constraint"
        // We can check if base_definition is present and is_abstract is false
        if structure_def.base_definition.is_some() && !structure_def.is_abstract {
            let url = &structure_def.url;
            code.push('\n');
            code.push_str("    fn profile_url() -> Option<&'static str> {\n");
            code.push_str(&format!("        Some(\"{url}\")\n"));
            code.push_str("    }\n");
        }

        code.push_str("}\n");

        code
    }

    /// Generate the validation module file content
    ///
    /// This creates the validation.rs file that contains the ValidatableResource trait.
    pub fn generate_validation_module() -> String {
        let mut code = String::new();

        code.push_str("//! Validation support for FHIR resources\n");
        code.push_str("//!\n");
        code.push_str(
            "//! This module provides traits for validating FHIR resources using invariants.\n",
        );
        code.push('\n');
        // Note: We use fully qualified `rh_foundation::Invariant` in the trait signature,
        // so no import statement is needed.
        code.push_str(&Self::generate_trait_definition());

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::{ElementConstraint, ElementDefinition, StructureDefinitionSnapshot};

    fn create_test_structure_def(
        name: &str,
        base_type: &str,
        constraints: Vec<ElementConstraint>,
        url: String,
        is_profile: bool,
    ) -> StructureDefinition {
        let element = ElementDefinition {
            id: Some(base_type.to_string()),
            path: base_type.to_string(),
            element_type: None,
            min: Some(0),
            max: Some("*".to_string()),
            short: None,
            definition: None,
            fixed: None,
            pattern: None,
            binding: None,
            constraint: Some(constraints),
        };

        StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: name.to_string(),
            url,
            version: None,
            name: name.to_string(),
            title: None,
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: base_type.to_string(),
            base_definition: if is_profile {
                Some(format!(
                    "http://hl7.org/fhir/StructureDefinition/{base_type}"
                ))
            } else {
                None
            },
            snapshot: Some(StructureDefinitionSnapshot {
                element: vec![element],
            }),
            differential: None,
        }
    }

    #[test]
    fn test_generate_trait_definition() {
        let trait_def = ValidationTraitGenerator::generate_trait_definition();

        assert!(trait_def.contains("pub trait ValidatableResource"));
        assert!(trait_def.contains("fn resource_type(&self) -> &'static str"));
        assert!(trait_def.contains("fn invariants() -> &'static [rh_foundation::Invariant]"));
        assert!(trait_def.contains("fn profile_url() -> Option<&'static str>"));
        assert!(trait_def.contains("None"));
    }

    #[test]
    fn test_generate_trait_impl_with_invariants() {
        let constraints = vec![ElementConstraint {
            key: "pat-1".to_string(),
            severity: "error".to_string(),
            human: "Test constraint".to_string(),
            expression: Some("name.exists()".to_string()),
            xpath: None,
            source: None,
        }];

        let structure_def = create_test_structure_def(
            "Patient",
            "Patient",
            constraints,
            "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            false,
        );

        let impl_code = ValidationTraitGenerator::generate_trait_impl(&structure_def);

        assert!(impl_code.contains("impl crate::validation::ValidatableResource for Patient"));
        assert!(impl_code.contains("fn resource_type(&self) -> &'static str"));
        assert!(impl_code.contains("\"Patient\""));
        assert!(impl_code.contains("fn invariants() -> &'static [rh_foundation::Invariant]"));
        assert!(impl_code.contains("&INVARIANTS"));
    }

    #[test]
    fn test_generate_trait_impl_with_profile() {
        let constraints = vec![ElementConstraint {
            key: "prof-1".to_string(),
            severity: "warning".to_string(),
            human: "Profile constraint".to_string(),
            expression: Some("identifier.exists()".to_string()),
            xpath: None,
            source: None,
        }];

        let structure_def = create_test_structure_def(
            "USCorePatient",
            "Patient",
            constraints,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
            true,
        );

        let impl_code = ValidationTraitGenerator::generate_trait_impl(&structure_def);

        assert!(impl_code.contains("fn profile_url() -> Option<&'static str>"));
        assert!(impl_code
            .contains("Some(\"http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient\")"));
    }

    #[test]
    fn test_generate_trait_impl_no_invariants() {
        let structure_def = create_test_structure_def(
            "Simple",
            "Simple",
            vec![],
            "http://hl7.org/fhir/StructureDefinition/Simple".to_string(),
            false,
        );

        let impl_code = ValidationTraitGenerator::generate_trait_impl(&structure_def);

        assert_eq!(impl_code, "");
    }

    #[test]
    fn test_generate_validation_module() {
        let module_code = ValidationTraitGenerator::generate_validation_module();

        assert!(module_code.contains("//! Validation support for FHIR resources"));
        assert!(module_code.contains("pub trait ValidatableResource"));
        assert!(module_code.contains("rh_foundation::Invariant")); // Used in trait signature
        assert!(!module_code.contains("use rh_foundation::Invariant")); // No import needed
    }

    #[test]
    fn test_trait_impl_base_resource() {
        let constraints = vec![ElementConstraint {
            key: "res-1".to_string(),
            severity: "error".to_string(),
            human: "Resource constraint".to_string(),
            expression: Some("id.exists()".to_string()),
            xpath: None,
            source: None,
        }];

        let structure_def = create_test_structure_def(
            "Observation",
            "Observation",
            constraints,
            "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
            false,
        );

        let impl_code = ValidationTraitGenerator::generate_trait_impl(&structure_def);

        // Should not have profile_url for base resources (not a profile)
        assert!(!impl_code.contains("fn profile_url()"));
        assert!(impl_code.contains("impl crate::validation::ValidatableResource for Observation"));
    }
}
