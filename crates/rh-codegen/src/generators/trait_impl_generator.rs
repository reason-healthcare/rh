//! Trait implementation generation functionality
//!
//! This module handles the generation of trait implementations for FHIR resources.

use super::utils::GeneratorUtils;
use crate::fhir_types::StructureDefinition;
use crate::naming::Naming;
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
            if let Some(last_segment) = base_definition.split('/').next_back() {
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

        let struct_name = Naming::struct_name(structure_def);

        // Generate Resource trait implementation for all resources
        trait_impls.push(self.generate_resource_trait_impl(&struct_name, structure_def));

        // Generate DomainResource trait implementation for domain resources
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("DomainResource") {
                trait_impls.push(self.generate_domain_resource_trait_impl(&struct_name));
            }
        }

        // Generate specific resource trait implementation (e.g., PatientTrait for Patient)
        // Skip this for Resource itself to avoid conflicting implementations
        if struct_name != "Resource" {
            let specific_trait_impl =
                self.generate_specific_resource_trait_impl(&struct_name, structure_def);

            // Only include specific trait impl if it has methods
            if !specific_trait_impl.is_empty() {
                trait_impls.push(specific_trait_impl);
            }
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
            "crate::traits::resource::ResourceAccessors".to_string(),
            struct_name.to_string(),
        );

        // Determine the base access pattern based on the inheritance chain
        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        // id method
        let id_method = RustTraitImplMethod::new("id".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{}.id()", base_access)
            } else {
                format!("{}.id.clone()", base_access)
            });
        trait_impl.add_method(id_method);

        // meta method
        let meta_method = RustTraitImplMethod::new("meta".to_string())
            .with_return_type("Option<crate::datatypes::meta::Meta>".to_string())
            .with_body(if use_trait_methods {
                format!("{}.meta()", base_access)
            } else {
                format!("{}.meta.clone()", base_access)
            });
        trait_impl.add_method(meta_method);

        // implicit_rules method
        let implicit_rules_method = RustTraitImplMethod::new("implicit_rules".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{}.implicit_rules()", base_access)
            } else {
                format!("{}.implicit_rules.clone()", base_access)
            });
        trait_impl.add_method(implicit_rules_method);

        // language method
        let language_method = RustTraitImplMethod::new("language".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{}.language()", base_access)
            } else {
                format!("{}.language.clone()", base_access)
            });
        trait_impl.add_method(language_method);

        trait_impl
    }

    /// Get the base access pattern for resource fields
    fn get_resource_base_access(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> (String, bool) {
        if struct_name == "Resource" {
            // For Resource itself, access fields directly
            ("self".to_string(), false)
        } else if struct_name == "DomainResource" {
            // For DomainResource, access Resource fields directly
            ("self.base".to_string(), false)
        } else if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("DomainResource") {
                // For core resources that inherit from DomainResource - access fields directly
                ("self.base.base".to_string(), false)
            } else if base_def.contains("Resource") && struct_name != "DomainResource" {
                // For resources that inherit directly from Resource - access fields directly
                ("self.base".to_string(), false)
            } else if base_def.starts_with("http://hl7.org/fhir/StructureDefinition/") {
                // This is a profile of another resource - delegate to trait method
                ("self.base".to_string(), true)
            } else {
                // Default case - treat as core resource
                ("self.base.base".to_string(), false)
            }
        } else {
            // Default case when no baseDefinition - treat as core resource
            ("self.base.base".to_string(), false)
        }
    }

    /// Generate DomainResource trait implementation
    fn generate_domain_resource_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceAccessors".to_string(),
            struct_name.to_string(),
        );

        // text method
        let text_method = RustTraitImplMethod::new("text".to_string())
            .with_return_type("Option<crate::datatypes::narrative::Narrative>".to_string())
            .with_body("self.base.text.clone()".to_string());
        trait_impl.add_method(text_method);

        // contained method
        let contained_method = RustTraitImplMethod::new("contained".to_string())
            .with_return_type("&[crate::resources::resource::Resource]".to_string())
            .with_body("self.base.contained.as_deref().unwrap_or(&[])".to_string());
        trait_impl.add_method(contained_method);

        // extension method
        let extension_method = RustTraitImplMethod::new("extension".to_string())
            .with_return_type("&[crate::datatypes::extension::Extension]".to_string())
            .with_body("self.base.extension.as_deref().unwrap_or(&[])".to_string());
        trait_impl.add_method(extension_method);

        // modifier_extension method
        let modifier_extension_method = RustTraitImplMethod::new("modifier_extension".to_string())
            .with_return_type("&[crate::datatypes::extension::Extension]".to_string())
            .with_body("self.base.modifier_extension.as_deref().unwrap_or(&[])".to_string());
        trait_impl.add_method(modifier_extension_method);

        trait_impl
    }

    /// Generate specific resource trait implementation
    fn generate_specific_resource_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let trait_name = format!(
            "crate::traits::{}::{}Accessors",
            crate::naming::Naming::to_snake_case(struct_name),
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

        // Generate methods for all direct fields (matching accessor trait generator logic)
        for element in elements {
            if self.should_generate_accessor_impl(element, structure_def) {
                if let Some(method) = self.generate_field_accessor_method(element) {
                    trait_impl.add_method(method);
                }
            }
        }

        trait_impl
    }

    /// Check if we should generate an accessor implementation for this element
    /// This mirrors the logic in AccessorTraitGenerator::should_generate_accessor
    fn should_generate_accessor_impl(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        structure_def: &StructureDefinition,
    ) -> bool {
        let field_path = &element.path;
        let base_name = &structure_def.name;

        // The path must start with the base name of the structure.
        if !field_path.starts_with(base_name) {
            return false;
        }

        // We are interested in direct fields of the resource, which have paths like "Patient.active".
        // Splitting by '.' should result in exactly two parts.
        let path_parts: Vec<&str> = field_path.split('.').collect();
        if path_parts.len() != 2 {
            return false;
        }

        // The first part must match the base name.
        if path_parts[0] != base_name {
            return false;
        }

        // We don't generate accessors for choice types here, they are handled separately.
        let field_name = path_parts[1];
        !field_name.ends_with("[x]")
    }

    /// Generate a field accessor method for a trait implementation
    fn generate_field_accessor_method(
        &self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> Option<RustTraitImplMethod> {
        use crate::config::CodegenConfig;
        use crate::type_mapper::TypeMapper;
        use crate::value_sets::ValueSetManager;

        let path_parts: Vec<&str> = element.path.split('.').collect();
        let field_name = path_parts.last()?.to_string();
        let rust_field_name = crate::naming::Naming::field_name(&field_name);

        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<i32>()
                .unwrap_or(1)
                > 1;

        // Check if field is optional based on minimum cardinality
        let is_optional = element.min.unwrap_or(0) == 0;

        // Create TypeMapper for consistent type resolution
        let config = CodegenConfig::default();
        let mut value_set_manager = ValueSetManager::new();
        let mut type_mapper = TypeMapper::new(&config, &mut value_set_manager);

        // Get the FHIR types for this element
        let fhir_types = element.element_type.as_ref()?;

        // Map FHIR type to Rust type using TypeMapper with binding information
        let rust_type =
            type_mapper.map_fhir_type_with_binding(fhir_types, element.binding.as_ref(), is_array);

        // Generate return type and body based on the mapped type
        let (return_type, body) = if is_array {
            // For arrays, return slice references
            let inner_type = match &rust_type {
                crate::rust_types::RustType::Vec(inner) => inner.to_string(),
                crate::rust_types::RustType::Option(inner) => {
                    if let crate::rust_types::RustType::Vec(vec_inner) = inner.as_ref() {
                        vec_inner.to_string()
                    } else {
                        inner.to_string()
                    }
                }
                _ => rust_type.to_string(),
            };

            let return_type = format!("&[{}]", inner_type);
            let body = format!("self.{}.as_deref().unwrap_or(&[])", rust_field_name);
            (return_type, body)
        } else {
            // For non-arrays, consider optionality based on cardinality
            if is_optional {
                // Field is optional (min cardinality is 0), return Option<T>
                let inner_type = match &rust_type {
                    crate::rust_types::RustType::Option(inner) => inner.to_string(),
                    _ => rust_type.to_string(),
                };
                let return_type = format!("Option<{}>", inner_type);
                let body = format!("self.{}.clone()", rust_field_name);
                (return_type, body)
            } else {
                // Field is required (min cardinality is 1+), return T directly
                let return_type = match &rust_type {
                    crate::rust_types::RustType::Option(inner) => inner.to_string(),
                    _ => rust_type.to_string(),
                };
                let body = format!("self.{}.clone()", rust_field_name);
                (return_type, body)
            }
        };

        Some(
            RustTraitImplMethod::new(rust_field_name)
                .with_return_type(return_type)
                .with_body(body),
        )
    }

    /// Get the inner type for slice return type
    fn get_inner_type_for_slice(&self, rust_type: &crate::rust_types::RustType) -> String {
        match rust_type {
            crate::rust_types::RustType::Vec(inner) => inner.to_string(),
            crate::rust_types::RustType::Option(inner) => {
                if let crate::rust_types::RustType::Vec(vec_inner) = inner.as_ref() {
                    vec_inner.to_string()
                } else {
                    inner.to_string()
                }
            }
            _ => rust_type.to_string(),
        }
    }

    /// Get the type for option return type
    fn get_type_for_option(&self, rust_type: &crate::rust_types::RustType) -> String {
        match rust_type {
            crate::rust_types::RustType::Option(inner) => inner.to_string(),
            _ => rust_type.to_string(),
        }
    }

    /// Check if a rust type represents an enum
    fn is_enum_type(&self, rust_type: &crate::rust_types::RustType) -> bool {
        match rust_type {
            crate::rust_types::RustType::Custom(type_name) => self.is_enum_type_name(type_name),
            _ => false,
        }
    }

    /// Check if a type name represents a FHIR enum type
    fn is_enum_type_name(&self, type_name: &str) -> bool {
        // Common FHIR enum type patterns
        type_name.ends_with("Status")
            || type_name.ends_with("Kind")
            || type_name.ends_with("Code")
            || type_name.ends_with("Codes")
            || type_name.ends_with("Priority")
            || type_name.ends_with("Intent")
            || matches!(
                type_name,
                "PublicationStatus"
                    | "CapabilityStatementKind"
                    | "CodeSearchSupport"
                    | "FmStatus"
                    | "ReportStatusCodes"
                    | "ReportResultCodes"
                    | "VerificationresultStatus"
                    | "TaskStatus"
                    | "TaskIntent"
                    | "RequestPriority"
                    | "SupplydeliveryStatus"
                    | "SupplyrequestStatus"
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
            crate::naming::Naming::field_name(field_name)
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
    use crate::fhir_types::StructureDefinitionDifferential;

    fn create_test_structure_definition(
        name: &str,
        base_definition: Option<&str>,
    ) -> StructureDefinition {
        StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: name.to_lowercase(),
            url: format!("http://test.com/{name}"),
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
        let mut structure_def = create_test_structure_definition("EmptyProfile", None);
        structure_def.differential = Some(StructureDefinitionDifferential { element: vec![] });

        // Generate trait implementations
        let trait_impls = generator.generate_trait_impls(&structure_def).unwrap();

        // Should have Resource trait impl but not specific trait impl (since it would be empty)
        assert!(
            !trait_impls.is_empty(),
            "Should have at least Resource trait impl"
        );

        // Check that there's no empty specific trait implementation
        let specific_trait_name = format!(
            "crate::traits::{}::{}Accessors",
            crate::naming::Naming::to_snake_case("EmptyProfile"),
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
