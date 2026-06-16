//! Existence trait implementation generation

use super::TraitImplGenerator;
use crate::fhir_types::StructureDefinition;
use crate::naming::Naming;
use crate::rust_types::{RustTraitImpl, RustTraitImplMethod};

#[allow(dead_code)]
impl TraitImplGenerator {
    /// Generate Resource existence trait implementation
    pub(crate) fn generate_resource_existence_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::ResourceExistence".to_string(),
            struct_name.to_string(),
        );

        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        let has_id_method = RustTraitImplMethod::new("has_id".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_id()")
            } else {
                format!("{base_access}.id.is_some()")
            });
        trait_impl.add_method(has_id_method);

        let has_meta_method = RustTraitImplMethod::new("has_meta".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_meta()")
            } else {
                format!("{base_access}.meta.is_some()")
            });
        trait_impl.add_method(has_meta_method);

        let has_implicit_rules_method = RustTraitImplMethod::new("has_implicit_rules".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_implicit_rules()")
            } else {
                format!("{base_access}.implicit_rules.is_some()")
            });
        trait_impl.add_method(has_implicit_rules_method);

        let has_language_method = RustTraitImplMethod::new("has_language".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_language()")
            } else {
                format!("{base_access}.language.is_some()")
            });
        trait_impl.add_method(has_language_method);

        trait_impl
    }

    /// Generate DomainResource existence trait implementation
    pub(crate) fn generate_domain_resource_existence_trait_impl(
        &self,
        struct_name: &str,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceExistence".to_string(),
            struct_name.to_string(),
        );

        let has_text_method = RustTraitImplMethod::new("has_text".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.text.is_some()".to_string());
        trait_impl.add_method(has_text_method);

        let has_contained_method = RustTraitImplMethod::new("has_contained".to_string())
            .with_return_type("bool".to_string())
            .with_body("!self.base.contained.is_empty()".to_string());
        trait_impl.add_method(has_contained_method);

        let has_extension_method = RustTraitImplMethod::new("has_extension".to_string())
            .with_return_type("bool".to_string())
            .with_body("!self.base.extension.is_empty()".to_string());
        trait_impl.add_method(has_extension_method);

        let has_modifier_extension_method =
            RustTraitImplMethod::new("has_modifier_extension".to_string())
                .with_return_type("bool".to_string())
                .with_body("!self.base.modifier_extension.is_empty()".to_string());
        trait_impl.add_method(has_modifier_extension_method);

        trait_impl
    }

    /// Generate specific resource existence trait implementation
    pub(crate) fn generate_specific_resource_existence_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let trait_name = format!(
            "crate::traits::{}::{}Existence",
            crate::naming::Naming::to_snake_case(struct_name),
            struct_name
        );

        let mut trait_impl = RustTraitImpl::new(trait_name, struct_name.to_string());

        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);

        let extends_domain_resource = structure_def
            .base_definition
            .as_ref()
            .map(|base| base.ends_with("/DomainResource"))
            .unwrap_or(false);

        let extends_resource_directly = structure_def
            .base_definition
            .as_ref()
            .map(|base| base.ends_with("/Resource") && !base.ends_with("/DomainResource"))
            .unwrap_or(false);

        let _ = (
            is_profile,
            extends_domain_resource,
            extends_resource_directly,
        );

        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl;
        };

        let mut choice_fields = std::collections::BTreeSet::new();
        for element in elements {
            let path_parts: Vec<&str> = element.path.split('.').collect();
            if path_parts.len() == 2 && path_parts[0] == structure_def.name {
                let field_name = path_parts[1];
                if field_name.ends_with("[x]") {
                    choice_fields.insert(field_name.trim_end_matches("[x]").to_string());
                }
            }
        }

        for choice_field in &choice_fields {
            let choice_path = format!("{}.{}[x]", structure_def.name, choice_field);
            if let Some(choice_element) = elements.iter().find(|e| e.path == choice_path) {
                if let Some(method) =
                    self.generate_choice_type_existence_method(choice_field, choice_element)
                {
                    trait_impl.add_method(method);
                }
            }
        }

        for element in elements {
            let path_parts: Vec<&str> = element.path.split('.').collect();
            if path_parts.len() == 2 && path_parts[0] == structure_def.name {
                let field_name = path_parts[1];
                if !field_name.ends_with("[x]")
                    && self.should_generate_accessor_impl(element, structure_def)
                {
                    if let Some(method) = self.generate_field_existence_method(element) {
                        trait_impl.add_method(method);
                    }
                }
            }
        }

        trait_impl
    }

    /// Generate a field existence check method for a trait implementation
    pub(crate) fn generate_field_existence_method(
        &self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> Option<RustTraitImplMethod> {
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

        let is_optional = element.min.unwrap_or(0) == 0;

        let method_name = format!("has_{rust_field_name}");

        let body = if is_array {
            format!("!self.{rust_field_name}.is_empty()")
        } else if is_optional {
            format!("self.{rust_field_name}.is_some()")
        } else {
            "true".to_string()
        };

        Some(
            RustTraitImplMethod::new(method_name)
                .with_return_type("bool".to_string())
                .with_body(body),
        )
    }

    /// Generate existence checker method for choice-type fields.
    pub(crate) fn generate_choice_type_existence_method(
        &self,
        choice_field: &str,
        choice_element: &crate::fhir_types::ElementDefinition,
    ) -> Option<RustTraitImplMethod> {
        let types = choice_element.element_type.as_ref()?;

        if types.is_empty() {
            return None;
        }

        let is_optional = choice_element.min.unwrap_or(0) == 0;

        let mut variants = Vec::new();

        for type_def in types {
            if let Some(type_code) = &type_def.code {
                let type_suffix = Naming::type_suffix(type_code);
                let field_name = format!("{choice_field}_{type_suffix}");
                let rust_field_name = Naming::field_name(&field_name);
                variants.push(rust_field_name);
            }
        }

        if variants.is_empty() {
            return None;
        }

        let method_name = format!("has_{}", Naming::to_snake_case(choice_field));

        let body = if is_optional {
            variants
                .iter()
                .map(|v| format!("self.{v}.is_some()"))
                .collect::<Vec<_>>()
                .join(" || ")
        } else {
            "true".to_string()
        };

        Some(
            RustTraitImplMethod::new(method_name)
                .with_return_type("bool".to_string())
                .with_body(body),
        )
    }
}
