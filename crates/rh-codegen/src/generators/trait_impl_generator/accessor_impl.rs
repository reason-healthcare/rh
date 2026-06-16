//! Accessor trait implementation generation

use super::TraitImplGenerator;
use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustTraitImpl, RustTraitImplMethod};
use crate::type_mapper::TypeMapper;
use crate::value_sets::ValueSetManager;

#[allow(dead_code)]
impl TraitImplGenerator {
    /// Generate Resource trait implementation
    pub(crate) fn generate_resource_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::ResourceAccessors".to_string(),
            struct_name.to_string(),
        );

        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        let id_method = RustTraitImplMethod::new("id".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.id()")
            } else {
                format!("{base_access}.id.clone()")
            });
        trait_impl.add_method(id_method);

        let meta_method = RustTraitImplMethod::new("meta".to_string())
            .with_return_type("Option<crate::datatypes::meta::Meta>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.meta()")
            } else {
                format!("{base_access}.meta.clone()")
            });
        trait_impl.add_method(meta_method);

        let implicit_rules_method = RustTraitImplMethod::new("implicit_rules".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.implicit_rules()")
            } else {
                format!("{base_access}.implicit_rules.clone()")
            });
        trait_impl.add_method(implicit_rules_method);

        let language_method = RustTraitImplMethod::new("language".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.language()")
            } else {
                format!("{base_access}.language.clone()")
            });
        trait_impl.add_method(language_method);

        trait_impl
    }

    /// Generate DomainResource trait implementation
    pub(crate) fn generate_domain_resource_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceAccessors".to_string(),
            struct_name.to_string(),
        );

        let text_method = RustTraitImplMethod::new("text".to_string())
            .with_return_type("Option<crate::datatypes::narrative::Narrative>".to_string())
            .with_body("self.base.text.clone()".to_string());
        trait_impl.add_method(text_method);

        let contained_method = RustTraitImplMethod::new("contained".to_string())
            .with_return_type("&[crate::resources::resource::Resource]".to_string())
            .with_body("self.base.contained.as_slice()".to_string());
        trait_impl.add_method(contained_method);

        let extension_method = RustTraitImplMethod::new("extension".to_string())
            .with_return_type("&[crate::datatypes::extension::Extension]".to_string())
            .with_body("self.base.extension.as_slice()".to_string());
        trait_impl.add_method(extension_method);

        let modifier_extension_method = RustTraitImplMethod::new("modifier_extension".to_string())
            .with_return_type("&[crate::datatypes::extension::Extension]".to_string())
            .with_body("self.base.modifier_extension.as_slice()".to_string());
        trait_impl.add_method(modifier_extension_method);

        trait_impl
    }

    /// Generate specific resource trait implementation
    pub(crate) fn generate_specific_resource_trait_impl(
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

        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl;
        };

        for element in elements {
            if self.should_generate_accessor_impl(element, structure_def) {
                if let Some(method) = self.generate_field_accessor_method(element) {
                    trait_impl.add_method(method);
                }
            }
        }

        trait_impl
    }

    /// Generate a field accessor method for a trait implementation
    pub(crate) fn generate_field_accessor_method(
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

        let config = CodegenConfig::default();
        let mut value_set_manager = ValueSetManager::new();
        let mut type_mapper = TypeMapper::new(&config, &mut value_set_manager);

        let fhir_types = element.element_type.as_ref()?;

        let rust_type = if self.is_backbone_element(fhir_types) {
            self.get_nested_type_for_backbone_element(element, is_array)
        } else {
            let resolved = type_mapper.map_fhir_type_with_binding(
                fhir_types,
                element.binding.as_ref(),
                is_array,
            );
            let resource_name = element.path.split('.').next().unwrap_or("");
            if matches!(&resolved, crate::rust_types::RustType::Custom(n) if n == resource_name) {
                crate::rust_types::RustType::Custom("StringType".to_string())
            } else {
                resolved
            }
        };

        let (return_type, body) = if is_array {
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

            let return_type = format!("&[{inner_type}]");
            let body = if is_optional {
                format!("self.{rust_field_name}.as_slice()")
            } else {
                format!("&self.{rust_field_name}")
            };
            (return_type, body)
        } else if is_optional {
            let inner_type = match &rust_type {
                crate::rust_types::RustType::Option(inner) => inner.to_string(),
                _ => rust_type.to_string(),
            };
            let return_type = format!("Option<{inner_type}>");

            let body = if self.is_copy_type(&rust_type) {
                format!("self.{rust_field_name}")
            } else {
                format!("self.{rust_field_name}.clone()")
            };

            (return_type, body)
        } else {
            let return_type = match &rust_type {
                crate::rust_types::RustType::Option(inner) => inner.to_string(),
                _ => rust_type.to_string(),
            };

            let body = if self.is_copy_type(&rust_type) {
                format!("self.{rust_field_name}")
            } else {
                format!("self.{rust_field_name}.clone()")
            };

            (return_type, body)
        };

        Some(
            RustTraitImplMethod::new(rust_field_name)
                .with_return_type(return_type)
                .with_body(body),
        )
    }
}
