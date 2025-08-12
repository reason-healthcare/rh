//! Trait implementation generation functionality
//!
//! This module handles the generation of trait implementations for FHIR resources.

use super::utils::GeneratorUtils;
use crate::fhir_types::StructureDefinition;
use crate::generators::name_generator::NameGenerator;
use crate::rust_types::{RustTraitImpl, RustTraitImplMethod};
use crate::CodegenResult;

/// Generator for trait implementations
pub struct TraitImplGenerator;

impl TraitImplGenerator {
    /// Create a new trait implementation generator
    pub fn new() -> Self {
        Self
    }

    /// Extract the base resource type from a FHIR baseDefinition URL
    /// For example: "http://hl7.org/fhir/StructureDefinition/Group" -> "Group"
    fn extract_base_resource_type(base_definition: &str) -> Option<String> {
        // FHIR baseDefinition URLs follow the pattern:
        // http://hl7.org/fhir/StructureDefinition/{ResourceType}
        if base_definition.starts_with("http://hl7.org/fhir/StructureDefinition/") {
            if let Some(last_segment) = base_definition.split('/').last() {
                return Some(last_segment.to_string());
            }
        }
        None
    }

    /// Check if a baseDefinition indicates this is a core FHIR resource
    /// Core resources inherit directly from Resource or DomainResource
    fn is_core_resource(base_definition: &str) -> bool {
        matches!(
            base_definition,
            "http://hl7.org/fhir/StructureDefinition/Resource"
                | "http://hl7.org/fhir/StructureDefinition/DomainResource"
        )
    }

    /// Get the core resource type that a profile ultimately inherits from
    /// This handles known profile inheritance chains and can be extended with dynamic loading
    fn resolve_to_core_resource_type(
        base_resource_type: &str,
        _base_definition_url: &str,
    ) -> String {
        // For now, use hardcoded mapping for common profiles
        // TODO: Implement dynamic StructureDefinition loading
        match base_resource_type.to_lowercase().as_str() {
            // VitalSigns is a profile on Observation
            "vitalsigns" => "Observation".to_string(),
            // BodyWeight, BodyHeight, etc. are profiles on VitalSigns -> Observation
            "bodyweight" | "bodyheight" | "bmi" | "bodytemp" | "heartrate" | "resprate"
            | "oxygensat" => "Observation".to_string(),
            // Add other known profile chains here as needed
            _ => {
                // If it's already a core resource type, return it
                if GeneratorUtils::is_fhir_resource_type(base_resource_type) {
                    base_resource_type.to_string()
                } else {
                    // Unknown profile - return the original name as fallback
                    base_resource_type.to_string()
                }
            }
        }
    }

    /// Get the appropriate resource type for a structure definition
    /// For profiles, returns the base resource type; for core resources, returns the struct name
    fn get_resource_type_for_struct(
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> String {
        // Check if this has a baseDefinition
        if let Some(base_def) = &structure_def.base_definition {
            // If it's a core resource (inherits from Resource/DomainResource), use the struct name
            if Self::is_core_resource(base_def) {
                return struct_name.to_string();
            }

            // If it's a profile (inherits from another resource), extract the base resource type
            if let Some(base_resource_type) = Self::extract_base_resource_type(base_def) {
                // Resolve the base resource type to a core resource type
                let core_resource_type =
                    Self::resolve_to_core_resource_type(&base_resource_type, base_def);

                // Only return the core resource type if it's actually a known FHIR resource type
                if GeneratorUtils::is_fhir_resource_type(&core_resource_type) {
                    return core_resource_type;
                }
            }
        }

        // Fallback to the struct name if no baseDefinition or couldn't extract
        struct_name.to_string()
    }

    /// Generate trait implementations for a FHIR resource
    pub fn generate_trait_impls(
        &self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<Vec<RustTraitImpl>> {
        let mut trait_impls = Vec::new();

        // Skip non-resource types
        if structure_def.kind != "resource" {
            return Ok(trait_impls);
        }

        let struct_name = NameGenerator::generate_struct_name(structure_def);

        // Generate Resource trait implementation for all resources
        trait_impls.push(self.generate_resource_trait_impl(&struct_name, structure_def));

        // Generate DomainResource trait implementation for domain resources
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("DomainResource") {
                trait_impls.push(self.generate_domain_resource_trait_impl(&struct_name));
            }
        }

        // Generate specific resource trait implementation (e.g., PatientTrait for Patient)
        let specific_trait_impl =
            self.generate_specific_resource_trait_impl(&struct_name, structure_def);

        // Only include specific trait impl if it has methods
        if !specific_trait_impl.is_empty() {
            trait_impls.push(specific_trait_impl);
        }

        Ok(trait_impls)
    }

    /// Generate Resource trait implementation
    fn generate_resource_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::Resource".to_string(),
            struct_name.to_string(),
        );

        // Get the appropriate resource type (base resource type for profiles)
        let resource_type = Self::get_resource_type_for_struct(struct_name, structure_def);

        // resource_type method
        let resource_type_method = RustTraitImplMethod::new("resource_type".to_string())
            .with_return_type("&'static str".to_string())
            .with_body(format!("\"{}\"", resource_type));
        trait_impl.add_method(resource_type_method);

        // id method
        let id_method = RustTraitImplMethod::new("id".to_string())
            .with_return_type("Option<&str>".to_string())
            .with_body("self.base.base.id.as_ref().map(|s| s.as_str())".to_string());
        trait_impl.add_method(id_method);

        // meta method
        let meta_method = RustTraitImplMethod::new("meta".to_string())
            .with_return_type("Option<&crate::datatypes::meta::Meta>".to_string())
            .with_body("self.base.base.meta.as_ref()".to_string());
        trait_impl.add_method(meta_method);

        // extensions method
        let extensions_method = RustTraitImplMethod::new("extensions".to_string())
            .with_return_type("&[crate::datatypes::extension::Extension]".to_string())
            .with_body("self.base.extension.as_deref().unwrap_or(&[])".to_string());
        trait_impl.add_method(extensions_method);

        // implicit_rules method
        let implicit_rules_method = RustTraitImplMethod::new("implicit_rules".to_string())
            .with_return_type("Option<&str>".to_string())
            .with_body("self.base.base.implicit_rules.as_ref().map(|s| s.as_str())".to_string());
        trait_impl.add_method(implicit_rules_method);

        // language method
        let language_method = RustTraitImplMethod::new("language".to_string())
            .with_return_type("Option<&str>".to_string())
            .with_body("self.base.base.language.as_ref().map(|s| s.as_str())".to_string());
        trait_impl.add_method(language_method);

        trait_impl
    }

    /// Generate DomainResource trait implementation
    fn generate_domain_resource_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResource".to_string(),
            struct_name.to_string(),
        );

        // narrative method
        let narrative_method = RustTraitImplMethod::new("narrative".to_string())
            .with_return_type("Option<crate::datatypes::narrative::Narrative>".to_string())
            .with_body("self.base.text.clone()".to_string());
        trait_impl.add_method(narrative_method);

        trait_impl
    }

    /// Generate specific resource trait implementation
    fn generate_specific_resource_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let trait_name = format!(
            "crate::traits::{}::{}",
            crate::generators::utils::GeneratorUtils::to_snake_case(struct_name),
            struct_name
        );

        let mut trait_impl = RustTraitImpl::new(trait_name, struct_name.to_string());

        // Extract element definitions to generate trait methods
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl; // No elements to process
        };

        // Generate methods for key fields
        for element in elements {
            // Skip the root element
            if element.path == structure_def.name || element.path == structure_def.base_type {
                continue;
            }

            // Only process direct fields
            let base_path = &structure_def.name;
            if !element.path.starts_with(&format!("{base_path}.")) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();
            if field_path.contains('.') {
                continue; // Skip nested fields
            }

            // Generate method for this field
            if let Some(method) = self.generate_field_accessor_method(field_path, element) {
                trait_impl.add_method(method);
            }
        }

        trait_impl
    }

    /// Generate a field accessor method for a trait
    fn generate_field_accessor_method(
        &self,
        field_name: &str,
        element: &crate::fhir_types::ElementDefinition,
    ) -> Option<RustTraitImplMethod> {
        // Generate methods for all fields that the trait generator creates
        // This matches the logic in trait_generator.rs should_create_trait_method_for_element
        let trait_method_fields = [
            "status",
            "identifier",
            "active",
            "name",
            "telecom",
            "address",
            "gender",
            "birthDate",
            "photo",
            "qualification",
            "communication",
            "endpoint",
            "type",
            "category",
            "subject",
            "code",
            "value",
            "component",
            "interpretation",
            "note",
            "bodySite",
            "method",
            "specimen",
            "device",
            "referenceRange",
            "hasMember",
            "derivedFrom",
            "description",
            "title",
        ];

        if !trait_method_fields.contains(&field_name) {
            return None;
        }

        let method_name = if field_name == "type" {
            "type_".to_string()
        } else {
            // Convert field name to proper Rust method name (e.g., bodySite -> body_site)
            crate::generators::utils::GeneratorUtils::to_rust_field_name(field_name)
        };

        let return_type = self.determine_method_return_type(element);
        let body = self.generate_method_body(field_name, element);

        Some(
            RustTraitImplMethod::new(method_name)
                .with_return_type(return_type)
                .with_body(body),
        )
    }

    /// Determine the return type for a field accessor method
    fn determine_method_return_type(
        &self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> String {
        // Use the same logic as trait generator to ensure consistency

        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        // Get the base type
        let base_type = if let Some(element_types) = &element.element_type {
            if let Some(first_type) = element_types.first() {
                if let Some(code) = &first_type.code {
                    match code.as_str() {
                        "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical"
                        | "dateTime" | "date" | "time" | "instant" | "base64Binary" | "oid"
                        | "uuid" => "String".to_string(),
                        "boolean" => "bool".to_string(),
                        "integer" | "positiveInt" | "unsignedInt" => "i32".to_string(),
                        "decimal" => "f64".to_string(),
                        "Reference" => "crate::datatypes::reference::Reference".to_string(),
                        "Identifier" => "crate::datatypes::identifier::Identifier".to_string(),
                        "CodeableConcept" => {
                            "crate::datatypes::codeable_concept::CodeableConcept".to_string()
                        }
                        "Coding" => "crate::datatypes::coding::Coding".to_string(),
                        "Address" => "crate::datatypes::address::Address".to_string(),
                        "HumanName" => "crate::datatypes::human_name::HumanName".to_string(),
                        "ContactPoint" => {
                            "crate::datatypes::contact_point::ContactPoint".to_string()
                        }
                        "Attachment" => "crate::datatypes::attachment::Attachment".to_string(),
                        "Annotation" => "crate::datatypes::annotation::Annotation".to_string(),
                        "BackboneElement" => {
                            "crate::datatypes::backbone_element::BackboneElement".to_string()
                        }
                        _ => "String".to_string(), // For enums and other types, return String representation
                    }
                } else {
                    "String".to_string()
                }
            } else {
                "String".to_string()
            }
        } else {
            "String".to_string()
        };

        // Build the final return type with the same logic as trait generator
        if is_array {
            if is_optional {
                format!("Option<Vec<{base_type}>>")
            } else {
                format!("Vec<{base_type}>")
            }
        } else if is_optional {
            format!("Option<{base_type}>")
        } else {
            base_type
        }
    }

    /// Generate the method body for a field accessor
    fn generate_method_body(
        &self,
        field_name: &str,
        element: &crate::fhir_types::ElementDefinition,
    ) -> String {
        // Convert FHIR field name to Rust field name
        let rust_field_name = if field_name == "type" {
            "type_".to_string()
        } else {
            crate::generators::utils::GeneratorUtils::to_rust_field_name(field_name)
        };

        let field_access = format!("self.{rust_field_name}");

        // Determine optionality and array nature using same logic as return type
        let is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        if is_array {
            // Array field - just clone
            format!("{field_access}.clone()")
        } else if let Some(type_def) = element
            .element_type
            .as_ref()
            .and_then(|types| types.first())
        {
            if let Some(code) = &type_def.code {
                match code.as_str() {
                    "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical"
                    | "dateTime" | "date" | "time" | "instant" | "base64Binary" | "oid"
                    | "uuid" => {
                        if is_optional {
                            format!("{field_access}.as_ref().map(|s| s.to_string())")
                        } else {
                            format!("{field_access}.to_string()")
                        }
                    }
                    "boolean" => {
                        if is_optional {
                            format!("{field_access}.map(|b| b.into())")
                        } else {
                            format!("{field_access}.into()")
                        }
                    }
                    "integer" | "positiveInt" | "unsignedInt" => {
                        if is_optional {
                            format!("{field_access}.map(|i| i.into())")
                        } else {
                            format!("{field_access}.into()")
                        }
                    }
                    "decimal" => {
                        if is_optional {
                            format!("{field_access}.map(|d| d.into())")
                        } else {
                            format!("{field_access}.into()")
                        }
                    }
                    "CodeableConcept" | "Reference" | "Identifier" | "Coding" | "Address"
                    | "HumanName" | "ContactPoint" | "Attachment" | "Annotation"
                    | "BackboneElement" => {
                        // Complex types - just clone
                        format!("{field_access}.clone()")
                    }
                    _ => {
                        // For enums and other types
                        if is_optional {
                            format!("{field_access}.as_ref().map(|v| format!(\"{{:?}}\", v))")
                        } else {
                            format!("format!(\"{{:?}}\", {field_access})")
                        }
                    }
                }
            } else {
                format!("{field_access}.clone()")
            }
        } else {
            format!("{field_access}.clone()")
        }
    }
}

impl Default for TraitImplGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_structure_definition(
        name: &str,
        base_definition: Option<&str>,
    ) -> StructureDefinition {
        StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: name.to_lowercase(),
            url: format!("http://test.com/{}", name),
            version: Some("1.0.0".to_string()),
            name: name.to_string(),
            title: Some(name.to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Resource".to_string(),
            base_definition: base_definition.map(|s| s.to_string()),
            differential: None,
            snapshot: None,
        }
    }

    #[test]
    fn test_resource_type_for_core_resource() {
        let patient = create_test_structure_definition(
            "Patient",
            Some("http://hl7.org/fhir/StructureDefinition/DomainResource"),
        );

        let result = TraitImplGenerator::get_resource_type_for_struct("Patient", &patient);
        assert_eq!(
            result, "Patient",
            "Core resource should return its own name"
        );
    }

    #[test]
    fn test_resource_type_for_group_profile() {
        let group_definition = create_test_structure_definition(
            "GroupDefinition",
            Some("http://hl7.org/fhir/StructureDefinition/Group"),
        );

        let result =
            TraitImplGenerator::get_resource_type_for_struct("GroupDefinition", &group_definition);
        assert_eq!(result, "Group", "Group profile should return 'Group'");
    }

    #[test]
    fn test_resource_type_for_observation_profile() {
        let vital_signs = create_test_structure_definition(
            "VitalSigns",
            Some("http://hl7.org/fhir/StructureDefinition/Observation"),
        );

        let result = TraitImplGenerator::get_resource_type_for_struct("VitalSigns", &vital_signs);
        assert_eq!(
            result, "Observation",
            "Observation profile should return 'Observation'"
        );
    }

    #[test]
    fn test_resource_type_for_profile_on_profile() {
        let bmi = create_test_structure_definition(
            "BMI",
            Some("http://hl7.org/fhir/StructureDefinition/vitalsigns"),
        );

        let result = TraitImplGenerator::get_resource_type_for_struct("BMI", &bmi);
        // BMI inherits from vitalsigns, which should resolve to "Observation"
        assert_eq!(
            result, "Observation",
            "BMI profile should resolve to 'Observation' via vitalsigns"
        );
    }

    #[test]
    fn test_resource_type_without_base_definition() {
        let custom_resource = create_test_structure_definition("CustomResource", None);

        let result =
            TraitImplGenerator::get_resource_type_for_struct("CustomResource", &custom_resource);
        assert_eq!(
            result, "CustomResource",
            "Resource without baseDefinition should return struct name"
        );
    }

    #[test]
    fn test_is_core_resource() {
        assert!(TraitImplGenerator::is_core_resource(
            "http://hl7.org/fhir/StructureDefinition/Resource"
        ));
        assert!(TraitImplGenerator::is_core_resource(
            "http://hl7.org/fhir/StructureDefinition/DomainResource"
        ));
        assert!(!TraitImplGenerator::is_core_resource(
            "http://hl7.org/fhir/StructureDefinition/Patient"
        ));
        assert!(!TraitImplGenerator::is_core_resource(
            "http://hl7.org/fhir/StructureDefinition/Group"
        ));
    }

    #[test]
    fn test_extract_base_resource_type() {
        assert_eq!(
            TraitImplGenerator::extract_base_resource_type(
                "http://hl7.org/fhir/StructureDefinition/Group"
            ),
            Some("Group".to_string())
        );
        assert_eq!(
            TraitImplGenerator::extract_base_resource_type(
                "http://hl7.org/fhir/StructureDefinition/Observation"
            ),
            Some("Observation".to_string())
        );
        assert_eq!(
            TraitImplGenerator::extract_base_resource_type(
                "http://hl7.org/fhir/StructureDefinition/vitalsigns"
            ),
            Some("vitalsigns".to_string())
        );
        assert_eq!(
            TraitImplGenerator::extract_base_resource_type("invalid-url"),
            None
        );
    }

    #[test]
    fn test_resolve_to_core_resource_type() {
        // Test known profile resolution
        assert_eq!(
            TraitImplGenerator::resolve_to_core_resource_type(
                "vitalsigns",
                "http://hl7.org/fhir/StructureDefinition/vitalsigns"
            ),
            "Observation"
        );

        // Test core resource types remain unchanged
        assert_eq!(
            TraitImplGenerator::resolve_to_core_resource_type(
                "Patient",
                "http://hl7.org/fhir/StructureDefinition/Patient"
            ),
            "Patient"
        );
        assert_eq!(
            TraitImplGenerator::resolve_to_core_resource_type(
                "Group",
                "http://hl7.org/fhir/StructureDefinition/Group"
            ),
            "Group"
        );

        // Test BMI and other vital sign profiles
        assert_eq!(
            TraitImplGenerator::resolve_to_core_resource_type(
                "bmi",
                "http://hl7.org/fhir/StructureDefinition/bmi"
            ),
            "Observation"
        );

        // Test unknown profiles fallback to themselves
        assert_eq!(
            TraitImplGenerator::resolve_to_core_resource_type(
                "UnknownProfile",
                "http://hl7.org/fhir/StructureDefinition/UnknownProfile"
            ),
            "UnknownProfile"
        );
    }

    #[test]
    fn test_empty_trait_implementations_are_filtered() {
        let generator = TraitImplGenerator::new();

        // Create a structure definition with no elements (which would result in empty trait impl)
        let structure_def = create_test_structure_definition("EmptyProfile", None);

        // Generate trait implementations
        let trait_impls = generator.generate_trait_impls(&structure_def).unwrap();

        // Should have Resource trait impl but not specific trait impl (since it would be empty)
        assert!(
            !trait_impls.is_empty(),
            "Should have at least Resource trait impl"
        );

        // Check that there's no empty specific trait implementation
        let specific_trait_name = format!(
            "crate::traits::{}::{}",
            crate::generators::utils::GeneratorUtils::to_snake_case("EmptyProfile"),
            "EmptyProfile"
        );

        let has_empty_specific_impl = trait_impls
            .iter()
            .any(|impl_| impl_.trait_name == specific_trait_name && impl_.is_empty());

        assert!(
            !has_empty_specific_impl,
            "Should not include empty specific trait implementations"
        );
    }
}
