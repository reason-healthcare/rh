//! Trait implementation generation functionality
//!
//! This module handles the generation of trait implementations for FHIR resources.

use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustTraitImpl, RustTraitImplMethod};
use crate::CodegenResult;

/// Generator for trait implementations
pub struct TraitImplGenerator;

impl TraitImplGenerator {
    /// Create a new trait implementation generator
    pub fn new() -> Self {
        Self
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

        let struct_name = structure_def.name.clone();

        // Generate Resource trait implementation for all resources
        trait_impls.push(self.generate_resource_trait_impl(&struct_name));

        // Generate DomainResource trait implementation for domain resources
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("DomainResource") {
                trait_impls.push(self.generate_domain_resource_trait_impl(&struct_name));
            }
        }

        // Generate specific resource trait implementation
        trait_impls.push(self.generate_specific_resource_trait_impl(&struct_name, structure_def));

        Ok(trait_impls)
    }

    /// Generate Resource trait implementation
    fn generate_resource_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::Resource".to_string(),
            struct_name.to_string(),
        );

        // resource_type method
        let resource_type_method = RustTraitImplMethod::new("resource_type".to_string())
            .with_return_type("&'static str".to_string())
            .with_body(format!("\"{}\"", struct_name));
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
        // Only generate methods for important fields to keep traits focused
        let important_fields = [
            "identifier",
            "status",
            "type",
            "name",
            "subject",
            "active",
            "code",
            "value",
            "category",
            "description",
            "title",
        ];

        if !important_fields.contains(&field_name) {
            return None;
        }

        let method_name = if field_name == "type" {
            "type_".to_string()
        } else {
            field_name.to_string()
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
        let field_access = if field_name == "type" {
            "self.type_".to_string()
        } else {
            format!("self.{}", field_name)
        };

        // Determine optionality and array nature using same logic as return type
        let is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        if is_array {
            // Array field - just clone
            format!("{}.clone()", field_access)
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
                            format!("{}.as_ref().map(|s| s.to_string())", field_access)
                        } else {
                            format!("{}.to_string()", field_access)
                        }
                    }
                    "boolean" => {
                        if is_optional {
                            format!("{}.map(|b| b.into())", field_access)
                        } else {
                            format!("{}.into()", field_access)
                        }
                    }
                    "integer" | "positiveInt" | "unsignedInt" => {
                        if is_optional {
                            format!("{}.map(|i| i.into())", field_access)
                        } else {
                            format!("{}.into()", field_access)
                        }
                    }
                    "decimal" => {
                        if is_optional {
                            format!("{}.map(|d| d.into())", field_access)
                        } else {
                            format!("{}.into()", field_access)
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
                            format!("{}.as_ref().map(|v| format!(\"{{:?}}\", v))", field_access)
                        } else {
                            format!("format!(\"{{:?}}\", {})", field_access)
                        }
                    }
                }
            } else if is_optional {
                format!("{}.clone()", field_access)
            } else {
                format!("{}.clone()", field_access)
            }
        } else if is_optional {
            format!("{}.clone()", field_access)
        } else {
            format!("{}.clone()", field_access)
        }
    }
}

impl Default for TraitImplGenerator {
    fn default() -> Self {
        Self::new()
    }
}
