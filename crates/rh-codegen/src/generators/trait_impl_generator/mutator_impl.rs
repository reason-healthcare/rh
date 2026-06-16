//! Mutator trait implementation generation

use super::TraitImplGenerator;
use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustMethodParam, RustTraitImpl, RustTraitImplMethod};

#[allow(dead_code)]
impl TraitImplGenerator {
    /// Generate Resource mutators trait implementation
    pub(crate) fn generate_resource_mutators_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::ResourceMutators".to_string(),
            struct_name.to_string(),
        );

        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        let resource_access = if base_access == "self" {
            String::new()
        } else {
            base_access
                .strip_prefix("self.")
                .unwrap_or(&base_access)
                .to_string()
        };

        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None);
        trait_impl.add_method(new_method);

        if use_trait_methods {
            let set_id_method = RustTraitImplMethod::new("set_id".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_id(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_id_method);

            let set_meta_method = RustTraitImplMethod::new("set_meta".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::Custom("crate::datatypes::meta::Meta".to_string()),
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_meta(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_meta_method);

            let set_implicit_rules_method = RustTraitImplMethod::new("set_implicit_rules".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_implicit_rules(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_implicit_rules_method);

            let set_language_method = RustTraitImplMethod::new("set_language".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_language(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_language_method);
        } else {
            let set_id_method = RustTraitImplMethod::new("set_id".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body(if resource_access.is_empty() {
                    "let mut resource = self.clone();\n        resource.id = Some(value);\n        resource".to_string()
                } else {
                    format!(
                        "let mut resource = self.clone();\n        resource.{resource_access}.id = Some(value);\n        resource"
                    )
                })
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_id_method);

            let set_meta_method = RustTraitImplMethod::new("set_meta".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::Custom("crate::datatypes::meta::Meta".to_string()),
                ))
                .with_return_type("Self".to_string())
                .with_body(if resource_access.is_empty() {
                    "let mut resource = self.clone();\n        resource.meta = Some(value);\n        resource".to_string()
                } else {
                    format!(
                        "let mut resource = self.clone();\n        resource.{resource_access}.meta = Some(value);\n        resource"
                    )
                })
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_meta_method);

            let set_implicit_rules_method = RustTraitImplMethod::new("set_implicit_rules".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body(if resource_access.is_empty() {
                    "let mut resource = self.clone();\n        resource.implicit_rules = Some(value);\n        resource".to_string()
                } else {
                    format!(
                        "let mut resource = self.clone();\n        resource.{resource_access}.implicit_rules = Some(value);\n        resource"
                    )
                })
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_implicit_rules_method);

            let set_language_method = RustTraitImplMethod::new("set_language".to_string())
                .with_param(RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body(if resource_access.is_empty() {
                    "let mut resource = self.clone();\n        resource.language = Some(value);\n        resource".to_string()
                } else {
                    format!(
                        "let mut resource = self.clone();\n        resource.{resource_access}.language = Some(value);\n        resource"
                    )
                })
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_language_method);
        }

        trait_impl
    }

    /// Generate DomainResource mutators trait implementation
    pub(crate) fn generate_domain_resource_mutators_trait_impl(
        &self,
        struct_name: &str,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceMutators".to_string(),
            struct_name.to_string(),
        );

        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None);
        trait_impl.add_method(new_method);

        let set_text_method = RustTraitImplMethod::new("set_text".to_string())
            .with_param(RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Custom("crate::datatypes::narrative::Narrative".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.text = Some(value);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_text_method);

        let set_contained_method = RustTraitImplMethod::new("set_contained".to_string())
            .with_param(RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::resources::resource::Resource".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.contained = value;\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_contained_method);

        let add_contained_method = RustTraitImplMethod::new("add_contained".to_string())
            .with_param(RustMethodParam::new(
                "item".to_string(),
                crate::rust_types::RustType::Custom("crate::resources::resource::Resource".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.contained.push(item);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_contained_method);

        let set_extension_method = RustTraitImplMethod::new("set_extension".to_string())
            .with_param(RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.extension = value;\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_extension_method);

        let add_extension_method = RustTraitImplMethod::new("add_extension".to_string())
            .with_param(RustMethodParam::new(
                "item".to_string(),
                crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.extension.push(item);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_extension_method);

        let set_modifier_extension_method = RustTraitImplMethod::new("set_modifier_extension".to_string())
            .with_param(RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.modifier_extension = value;\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_modifier_extension_method);

        let add_modifier_extension_method =
            RustTraitImplMethod::new("add_modifier_extension".to_string())
                .with_param(RustMethodParam::new(
                    "item".to_string(),
                    crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()),
                ))
                .with_return_type("Self".to_string())
                .with_body(
                    "let mut resource = self.clone();\n        resource.base.modifier_extension.push(item);\n        resource"
                        .to_string(),
                )
                .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_modifier_extension_method);

        trait_impl
    }

    /// Generate specific resource mutators trait implementation
    pub(crate) fn generate_specific_resource_mutators_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let trait_name = format!(
            "crate::traits::{}::{}Mutators",
            crate::naming::Naming::to_snake_case(struct_name),
            struct_name
        );

        let mut trait_impl = RustTraitImpl::new(trait_name, struct_name.to_string());

        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None);
        trait_impl.add_method(new_method);

        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl;
        };

        for element in elements {
            if self.should_generate_accessor_impl(element, structure_def) {
                if let Some(methods) = self.generate_field_mutator_methods(element) {
                    for method in methods {
                        trait_impl.add_method(method);
                    }
                }
            }
        }

        trait_impl
    }

    /// Generate field mutator methods for a trait implementation (returns both set and add for arrays)
    pub(crate) fn generate_field_mutator_methods(
        &self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> Option<Vec<RustTraitImplMethod>> {
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

        let rust_type = self.get_field_rust_type(element, &field_name).ok()?;

        let mut methods = Vec::new();

        if is_array {
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

            let set_method_name = format!("set_{rust_field_name}");
            let set_body = format!(
                "let mut resource = self.clone();\n        resource.{rust_field_name} = value;\n        resource"
            );

            methods.push(
                RustTraitImplMethod::new(set_method_name)
                    .with_param(RustMethodParam::new(
                        "value".to_string(),
                        crate::rust_types::RustType::Vec(Box::new(
                            crate::rust_types::RustType::Custom(inner_type.clone()),
                        )),
                    ))
                    .with_return_type("Self".to_string())
                    .with_body(set_body)
                    .with_self_param(Some("self".to_string())),
            );

            let add_method_name = format!("add_{rust_field_name}");
            let add_body = format!(
                "let mut resource = self.clone();\n        resource.{rust_field_name}.push(item);\n        resource"
            );

            methods.push(
                RustTraitImplMethod::new(add_method_name)
                    .with_param(RustMethodParam::new(
                        "item".to_string(),
                        crate::rust_types::RustType::Custom(inner_type),
                    ))
                    .with_return_type("Self".to_string())
                    .with_body(add_body)
                    .with_self_param(Some("self".to_string())),
            );
        } else {
            let method_name = format!("set_{rust_field_name}");
            let inner_type = match &rust_type {
                crate::rust_types::RustType::Option(inner) => inner.to_string(),
                _ => rust_type.to_string(),
            };

            let body = if is_optional {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = Some(value);\n        resource"
                )
            } else {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = value;\n        resource"
                )
            };

            methods.push(
                RustTraitImplMethod::new(method_name)
                    .with_param(RustMethodParam::new(
                        "value".to_string(),
                        crate::rust_types::RustType::Custom(inner_type),
                    ))
                    .with_return_type("Self".to_string())
                    .with_body(body)
                    .with_self_param(Some("self".to_string())),
            );
        }

        Some(methods)
    }
}
