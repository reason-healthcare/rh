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

#[allow(dead_code)]
impl TraitImplGenerator {
    /// Create a new trait implementation generator
    pub fn new() -> Self {
        Self
    }

    /// Extract the base resource type from a FHIR baseDefinition URL
    /// For example: "http://hl7.org/fhir/StructureDefinition/Group" -> "Group"
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

        // Generate Resource trait implementations (Accessors, Mutators, Existence)
        trait_impls.push(self.generate_resource_trait_impl(&struct_name, structure_def));
        trait_impls.push(self.generate_resource_mutators_trait_impl(&struct_name, structure_def));
        trait_impls.push(self.generate_resource_existence_trait_impl(&struct_name, structure_def));

        // Generate DomainResource trait implementations for domain resources
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("DomainResource") {
                trait_impls.push(self.generate_domain_resource_trait_impl(&struct_name));
                trait_impls.push(self.generate_domain_resource_mutators_trait_impl(&struct_name));
                trait_impls.push(self.generate_domain_resource_existence_trait_impl(&struct_name));
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

            // Generate specific mutators trait implementation
            let specific_mutators_trait_impl =
                self.generate_specific_resource_mutators_trait_impl(&struct_name, structure_def);

            if !specific_mutators_trait_impl.is_empty() {
                trait_impls.push(specific_mutators_trait_impl);
            }

            // Generate specific existence trait implementation
            let specific_existence_trait_impl =
                self.generate_specific_resource_existence_trait_impl(&struct_name, structure_def);

            if !specific_existence_trait_impl.is_empty() {
                trait_impls.push(specific_existence_trait_impl);
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
                format!("{base_access}.id()")
            } else {
                format!("{base_access}.id.clone()")
            });
        trait_impl.add_method(id_method);

        // meta method
        let meta_method = RustTraitImplMethod::new("meta".to_string())
            .with_return_type("Option<crate::datatypes::meta::Meta>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.meta()")
            } else {
                format!("{base_access}.meta.clone()")
            });
        trait_impl.add_method(meta_method);

        // implicit_rules method
        let implicit_rules_method = RustTraitImplMethod::new("implicit_rules".to_string())
            .with_return_type("Option<String>".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.implicit_rules()")
            } else {
                format!("{base_access}.implicit_rules.clone()")
            });
        trait_impl.add_method(implicit_rules_method);

        // language method
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

    /// Generate Resource mutators trait implementation
    fn generate_resource_mutators_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::ResourceMutators".to_string(),
            struct_name.to_string(),
        );

        // Determine the base access pattern based on the inheritance chain
        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        // Transform base_access for use after cloning self into resource
        // "self" -> "" (direct field access)
        // "self.base" -> "base"
        // "self.base.base" -> "base.base"
        let resource_access = if base_access == "self" {
            String::new()
        } else {
            base_access
                .strip_prefix("self.")
                .unwrap_or(&base_access)
                .to_string()
        };

        // new method
        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None); // No self parameter for constructor
        trait_impl.add_method(new_method);

        // For profiles that extend other profiles, delegate through trait methods
        // For core resources, access fields directly
        if use_trait_methods {
            // set_id method - delegate to base's set_id
            let set_id_method = RustTraitImplMethod::new("set_id".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_id(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_id_method);

            // set_meta method - delegate to base's set_meta
            let set_meta_method = RustTraitImplMethod::new("set_meta".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::Custom("crate::datatypes::meta::Meta".to_string()),
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_meta(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_meta_method);

            // set_implicit_rules method - delegate to base's set_implicit_rules
            let set_implicit_rules_method = RustTraitImplMethod::new("set_implicit_rules".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_implicit_rules(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_implicit_rules_method);

            // set_language method - delegate to base's set_language
            let set_language_method = RustTraitImplMethod::new("set_language".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
                    "value".to_string(),
                    crate::rust_types::RustType::String,
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base = resource.base.set_language(value);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
            trait_impl.add_method(set_language_method);
        } else {
            // set_id method - direct field access
            let set_id_method = RustTraitImplMethod::new("set_id".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
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
                .with_self_param(Some("self".to_string())); // Take self by value
            trait_impl.add_method(set_id_method);

            // set_meta method - direct field access
            let set_meta_method = RustTraitImplMethod::new("set_meta".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
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

            // set_implicit_rules method - direct field access
            let set_implicit_rules_method = RustTraitImplMethod::new("set_implicit_rules".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
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

            // set_language method - direct field access
            let set_language_method = RustTraitImplMethod::new("set_language".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
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

    /// Generate Resource existence trait implementation
    fn generate_resource_existence_trait_impl(
        &self,
        struct_name: &str,
        structure_def: &StructureDefinition,
    ) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::resource::ResourceExistence".to_string(),
            struct_name.to_string(),
        );

        // Determine the base access pattern based on the inheritance chain
        let (base_access, use_trait_methods) =
            self.get_resource_base_access(struct_name, structure_def);

        // has_id method
        let has_id_method = RustTraitImplMethod::new("has_id".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_id()")
            } else {
                format!("{base_access}.id.is_some()")
            });
        trait_impl.add_method(has_id_method);

        // has_meta method
        let has_meta_method = RustTraitImplMethod::new("has_meta".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_meta()")
            } else {
                format!("{base_access}.meta.is_some()")
            });
        trait_impl.add_method(has_meta_method);

        // has_implicit_rules method
        let has_implicit_rules_method = RustTraitImplMethod::new("has_implicit_rules".to_string())
            .with_return_type("bool".to_string())
            .with_body(if use_trait_methods {
                format!("{base_access}.has_implicit_rules()")
            } else {
                format!("{base_access}.implicit_rules.is_some()")
            });
        trait_impl.add_method(has_implicit_rules_method);

        // has_language method
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

    /// Generate DomainResource mutators trait implementation
    fn generate_domain_resource_mutators_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceMutators".to_string(),
            struct_name.to_string(),
        );

        // new method
        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None); // No self for constructor
        trait_impl.add_method(new_method);

        // set_text method
        let set_text_method = RustTraitImplMethod::new("set_text".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Custom("crate::datatypes::narrative::Narrative".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.text = Some(value);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_text_method);

        // set_contained method
        let set_contained_method = RustTraitImplMethod::new("set_contained".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::resources::resource::Resource".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.contained = Some(value);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_contained_method);

        // add_contained method
        let add_contained_method = RustTraitImplMethod::new("add_contained".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "item".to_string(),
                crate::rust_types::RustType::Custom("crate::resources::resource::Resource".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.contained.get_or_insert_with(Vec::new).push(item);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_contained_method);

        // set_extension method
        let set_extension_method = RustTraitImplMethod::new("set_extension".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.extension = Some(value);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_extension_method);

        // add_extension method
        let add_extension_method = RustTraitImplMethod::new("add_extension".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "item".to_string(),
                crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.extension.get_or_insert_with(Vec::new).push(item);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_extension_method);

        // set_modifier_extension method
        let set_modifier_extension_method = RustTraitImplMethod::new("set_modifier_extension".to_string())
            .with_param(crate::rust_types::RustMethodParam::new(
                "value".to_string(),
                crate::rust_types::RustType::Vec(Box::new(crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()))),
            ))
            .with_return_type("Self".to_string())
            .with_body("let mut resource = self.clone();\n        resource.base.modifier_extension = Some(value);\n        resource".to_string())
            .with_self_param(Some("self".to_string()));
        trait_impl.add_method(set_modifier_extension_method);

        // add_modifier_extension method
        let add_modifier_extension_method =
            RustTraitImplMethod::new("add_modifier_extension".to_string())
                .with_param(crate::rust_types::RustMethodParam::new(
                    "item".to_string(),
                    crate::rust_types::RustType::Custom("crate::datatypes::extension::Extension".to_string()),
                ))
                .with_return_type("Self".to_string())
                .with_body("let mut resource = self.clone();\n        resource.base.modifier_extension.get_or_insert_with(Vec::new).push(item);\n        resource".to_string())
                .with_self_param(Some("self".to_string()));
        trait_impl.add_method(add_modifier_extension_method);

        trait_impl
    }

    /// Generate DomainResource existence trait implementation
    fn generate_domain_resource_existence_trait_impl(&self, struct_name: &str) -> RustTraitImpl {
        let mut trait_impl = RustTraitImpl::new(
            "crate::traits::domain_resource::DomainResourceExistence".to_string(),
            struct_name.to_string(),
        );

        // Duplicate methods from ResourceExistence (required by trait inheritance)
        // has_id method
        let has_id_method = RustTraitImplMethod::new("has_id".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.base.id.is_some()".to_string());
        trait_impl.add_method(has_id_method);

        // has_meta method
        let has_meta_method = RustTraitImplMethod::new("has_meta".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.base.meta.is_some()".to_string());
        trait_impl.add_method(has_meta_method);

        // has_implicit_rules method
        let has_implicit_rules_method = RustTraitImplMethod::new("has_implicit_rules".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.base.implicit_rules.is_some()".to_string());
        trait_impl.add_method(has_implicit_rules_method);

        // has_language method
        let has_language_method = RustTraitImplMethod::new("has_language".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.base.language.is_some()".to_string());
        trait_impl.add_method(has_language_method);

        // has_text method
        let has_text_method = RustTraitImplMethod::new("has_text".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.text.is_some()".to_string());
        trait_impl.add_method(has_text_method);

        // has_contained method
        let has_contained_method = RustTraitImplMethod::new("has_contained".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.contained.as_ref().is_some_and(|c| !c.is_empty())".to_string());
        trait_impl.add_method(has_contained_method);

        // has_extension method
        let has_extension_method = RustTraitImplMethod::new("has_extension".to_string())
            .with_return_type("bool".to_string())
            .with_body("self.base.extension.as_ref().is_some_and(|e| !e.is_empty())".to_string());
        trait_impl.add_method(has_extension_method);

        // has_modifier_extension method
        let has_modifier_extension_method =
            RustTraitImplMethod::new("has_modifier_extension".to_string())
                .with_return_type("bool".to_string())
                .with_body(
                    "self.base.modifier_extension.as_ref().is_some_and(|m| !m.is_empty())"
                        .to_string(),
                );
        trait_impl.add_method(has_modifier_extension_method);

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

    /// Generate specific resource mutators trait implementation
    fn generate_specific_resource_mutators_trait_impl(
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

        // new method
        let new_method = RustTraitImplMethod::new("new".to_string())
            .with_return_type("Self".to_string())
            .with_body("Self::default()".to_string())
            .with_self_param(None); // No self for constructor
        trait_impl.add_method(new_method);

        // Extract element definitions to generate trait methods
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl; // No elements to process
        };

        // Generate mutator methods for all direct fields
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

    /// Generate specific resource existence trait implementation
    fn generate_specific_resource_existence_trait_impl(
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

        // Check if this is a profile - profiles extend ResourceExistence and shouldn't redefine inherited methods
        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);

        // Check if this resource extends DomainResource (vs extending Resource directly)
        let extends_domain_resource = structure_def
            .base_definition
            .as_ref()
            .map(|base| base.ends_with("/DomainResource"))
            .unwrap_or(false);

        // Check if this resource extends Resource directly (not DomainResource)
        let extends_resource_directly = structure_def
            .base_definition
            .as_ref()
            .map(|base| base.ends_with("/Resource") && !base.ends_with("/DomainResource"))
            .unwrap_or(false);

        // Only add inherited methods for non-profiles
        // Profiles extend ResourceExistence or DomainResourceExistence, so they inherit these methods automatically
        if !is_profile {
            // Determine the correct base access path based on inheritance
            let (id_access, meta_access, implicit_rules_access, language_access) =
                if extends_resource_directly {
                    // Binary, Bundle, Parameters extend Resource directly -> self.base.id
                    (
                        "self.base.id.is_some()".to_string(),
                        "self.base.meta.is_some()".to_string(),
                        "self.base.implicit_rules.is_some()".to_string(),
                        "self.base.language.is_some()".to_string(),
                    )
                } else {
                    // Most resources extend DomainResource -> self.base.base.id
                    (
                        "self.base.base.id.is_some()".to_string(),
                        "self.base.base.meta.is_some()".to_string(),
                        "self.base.base.implicit_rules.is_some()".to_string(),
                        "self.base.base.language.is_some()".to_string(),
                    )
                };

            // Add inherited methods from ResourceExistence
            // has_id method
            let has_id_method = RustTraitImplMethod::new("has_id".to_string())
                .with_return_type("bool".to_string())
                .with_body(id_access);
            trait_impl.add_method(has_id_method);

            // has_meta method
            let has_meta_method = RustTraitImplMethod::new("has_meta".to_string())
                .with_return_type("bool".to_string())
                .with_body(meta_access);
            trait_impl.add_method(has_meta_method);

            // has_implicit_rules method
            let has_implicit_rules_method =
                RustTraitImplMethod::new("has_implicit_rules".to_string())
                    .with_return_type("bool".to_string())
                    .with_body(implicit_rules_access);
            trait_impl.add_method(has_implicit_rules_method);

            // has_language method
            let has_language_method = RustTraitImplMethod::new("has_language".to_string())
                .with_return_type("bool".to_string())
                .with_body(language_access);
            trait_impl.add_method(has_language_method);

            // Only add DomainResource-specific methods if this resource extends DomainResource
            // Resources like Binary, Bundle, Parameters extend Resource directly and don't have these fields
            if extends_domain_resource {
                // Add inherited methods from DomainResourceExistence
                // has_text method
                let has_text_method = RustTraitImplMethod::new("has_text".to_string())
                    .with_return_type("bool".to_string())
                    .with_body("self.base.text.is_some()".to_string());
                trait_impl.add_method(has_text_method);

                // has_contained method
                let has_contained_method = RustTraitImplMethod::new("has_contained".to_string())
                    .with_return_type("bool".to_string())
                    .with_body(
                        "self.base.contained.as_ref().is_some_and(|c| !c.is_empty())".to_string(),
                    );
                trait_impl.add_method(has_contained_method);

                // has_extension method
                let has_extension_method = RustTraitImplMethod::new("has_extension".to_string())
                    .with_return_type("bool".to_string())
                    .with_body(
                        "self.base.extension.as_ref().is_some_and(|e| !e.is_empty())".to_string(),
                    );
                trait_impl.add_method(has_extension_method);

                // has_modifier_extension method
                let has_modifier_extension_method =
                    RustTraitImplMethod::new("has_modifier_extension".to_string())
                        .with_return_type("bool".to_string())
                        .with_body(
                            "self.base.modifier_extension.as_ref().is_some_and(|m| !m.is_empty())"
                                .to_string(),
                        );
                trait_impl.add_method(has_modifier_extension_method);
            }
        }

        // Extract element definitions to generate trait methods for specific fields
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return trait_impl; // No elements to process
        };

        // First, collect choice-type fields (those ending with [x])
        let mut choice_fields = std::collections::HashSet::new();
        for element in elements {
            let path_parts: Vec<&str> = element.path.split('.').collect();
            if path_parts.len() == 2 && path_parts[0] == structure_def.name {
                let field_name = path_parts[1];
                if field_name.ends_with("[x]") {
                    choice_fields.insert(field_name.trim_end_matches("[x]").to_string());
                }
            }
        }

        // Generate existence check methods for choice types
        for choice_field in &choice_fields {
            // Find the element with this choice field name + [x]
            let choice_path = format!("{}.{}[x]", structure_def.name, choice_field);
            if let Some(choice_element) = elements.iter().find(|e| e.path == choice_path) {
                if let Some(method) =
                    self.generate_choice_type_existence_method(choice_field, choice_element)
                {
                    trait_impl.add_method(method);
                }
            }
        }

        // Generate existence check methods for all direct non-choice fields
        for element in elements {
            let path_parts: Vec<&str> = element.path.split('.').collect();
            if path_parts.len() == 2 && path_parts[0] == structure_def.name {
                let field_name = path_parts[1];
                // Skip choice type fields (they were handled above)
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

        // Check if this is a BackboneElement that should use a specific nested type
        let rust_type = if self.is_backbone_element(fhir_types) {
            self.get_nested_type_for_backbone_element(element, is_array)
        } else {
            // Map FHIR type to Rust type using TypeMapper with binding information
            type_mapper.map_fhir_type_with_binding(fhir_types, element.binding.as_ref(), is_array)
        };

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

            let return_type = format!("&[{inner_type}]");
            let body = if is_optional {
                // Optional array: Option<Vec<T>> -> use as_deref()
                format!("self.{rust_field_name}.as_deref().unwrap_or(&[])")
            } else {
                // Required array: Vec<T> -> use as_ref() or direct reference
                format!("&self.{rust_field_name}")
            };
            (return_type, body)
        } else {
            // For non-arrays, consider optionality based on cardinality
            if is_optional {
                // Field is optional (min cardinality is 0), return Option<T>
                let inner_type = match &rust_type {
                    crate::rust_types::RustType::Option(inner) => inner.to_string(),
                    _ => rust_type.to_string(),
                };
                let return_type = format!("Option<{inner_type}>");
                let body = format!("self.{rust_field_name}.clone()");
                (return_type, body)
            } else {
                // Field is required (min cardinality is 1+), return T directly
                let return_type = match &rust_type {
                    crate::rust_types::RustType::Option(inner) => inner.to_string(),
                    _ => rust_type.to_string(),
                };
                let body = format!("self.{rust_field_name}.clone()");
                (return_type, body)
            }
        };

        Some(
            RustTraitImplMethod::new(rust_field_name)
                .with_return_type(return_type)
                .with_body(body),
        )
    }

    /// Generate field mutator methods for a trait implementation (returns both set and add for arrays)
    fn generate_field_mutator_methods(
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

        // Check if field is optional based on minimum cardinality
        let is_optional = element.min.unwrap_or(0) == 0;

        // Use binding-aware type mapping to get the correct Rust type
        let rust_type = self.get_field_rust_type(element, &field_name).ok()?;

        let mut methods = Vec::new();

        // Generate methods based on array vs single value
        if is_array {
            // For arrays, generate both set_xxx and add_xxx methods
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

            // set_xxx method
            let set_method_name = format!("set_{rust_field_name}");
            let set_body = if is_optional {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = Some(value);\n        resource"
                )
            } else {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = value;\n        resource"
                )
            };

            methods.push(
                RustTraitImplMethod::new(set_method_name)
                    .with_param(crate::rust_types::RustMethodParam::new(
                        "value".to_string(),
                        crate::rust_types::RustType::Vec(Box::new(
                            crate::rust_types::RustType::Custom(inner_type.clone()),
                        )),
                    ))
                    .with_return_type("Self".to_string())
                    .with_body(set_body)
                    .with_self_param(Some("self".to_string())),
            );

            // add_xxx method
            let add_method_name = format!("add_{rust_field_name}");
            let add_body = if is_optional {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name}.get_or_insert_with(Vec::new).push(item);\n        resource"
                )
            } else {
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name}.push(item);\n        resource"
                )
            };

            methods.push(
                RustTraitImplMethod::new(add_method_name)
                    .with_param(crate::rust_types::RustMethodParam::new(
                        "item".to_string(),
                        crate::rust_types::RustType::Custom(inner_type),
                    ))
                    .with_return_type("Self".to_string())
                    .with_body(add_body)
                    .with_self_param(Some("self".to_string())),
            );
        } else {
            // For single values, generate set_xxx method only
            let method_name = format!("set_{rust_field_name}");
            let inner_type = match &rust_type {
                crate::rust_types::RustType::Option(inner) => inner.to_string(),
                _ => rust_type.to_string(),
            };

            // Generate the setter body based on field optionality
            let body = if is_optional {
                // Optional field: wrap value in Some()
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = Some(value);\n        resource"
                )
            } else {
                // Required field: assign value directly without Some()
                format!(
                    "let mut resource = self.clone();\n        resource.{rust_field_name} = value;\n        resource"
                )
            };

            methods.push(
                RustTraitImplMethod::new(method_name)
                    .with_param(crate::rust_types::RustMethodParam::new(
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

    /// Generate a field existence check method for a trait implementation
    fn generate_field_existence_method(
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

        // Check if field is optional based on minimum cardinality
        let is_optional = element.min.unwrap_or(0) == 0;

        let method_name = format!("has_{rust_field_name}");

        let body = if is_array {
            if is_optional {
                // Optional array: Option<Vec<T>> -> check if Some and not empty
                format!("self.{rust_field_name}.as_ref().is_some_and(|v| !v.is_empty())")
            } else {
                // Required array: Vec<T> -> check if not empty
                format!("!self.{rust_field_name}.is_empty()")
            }
        } else if is_optional {
            // Optional single value: Option<T> -> check if Some
            format!("self.{rust_field_name}.is_some()")
        } else {
            // Required single value: T -> always true (field always exists)
            "true".to_string()
        };

        Some(
            RustTraitImplMethod::new(method_name)
                .with_return_type("bool".to_string())
                .with_body(body),
        )
    }

    /// Generate existence checker method for choice-type fields.
    /// Choice types like subject[x] expand to multiple fields (subject_codeable_concept, subject_reference).
    /// The has_subject() method should return true if ANY variant is present.
    fn generate_choice_type_existence_method(
        &self,
        choice_field: &str,
        choice_element: &crate::fhir_types::ElementDefinition,
    ) -> Option<RustTraitImplMethod> {
        // Get the types from the element - choice types have multiple type entries
        let types = choice_element.element_type.as_ref()?;

        if types.is_empty() {
            return None;
        }

        // Check if the choice type itself is optional or required
        let is_optional = choice_element.min.unwrap_or(0) == 0;

        // Generate field names for each type variant
        // Use the same naming approach as field_generator.rs to ensure consistency
        // e.g., for "occurrence" with type "DateTime" -> "occurrence_date_time"
        let mut variants = Vec::new();

        for type_def in types {
            if let Some(type_code) = &type_def.code {
                // Use type_suffix to convert type code to snake_case with underscores
                // e.g., "DateTime" -> "date_time"
                let type_suffix = Naming::type_suffix(type_code);
                // Combine base field name with type suffix
                // e.g., "occurrence" + "_" + "date_time" -> "occurrence_date_time"
                let field_name = format!("{choice_field}_{type_suffix}");
                let rust_field_name = Naming::field_name(&field_name);
                variants.push(rust_field_name);
            }
        }

        if variants.is_empty() {
            return None;
        }

        let method_name = format!("has_{}", Naming::to_snake_case(choice_field));

        // Generate body based on whether the choice type is optional or required
        let body = if is_optional {
            // Optional choice type: each variant is Option<T> -> check with .is_some()
            variants
                .iter()
                .map(|v| format!("self.{v}.is_some()"))
                .collect::<Vec<_>>()
                .join(" || ")
        } else {
            // Required choice type: at least one variant must be present
            // For required choice types, variants are not Option<T>, so existence is always true
            // But we still need to check which variant is being used
            // Since Rust doesn't allow multiple non-Option fields for a choice, this case shouldn't happen
            // in well-formed generated code. However, if it does, we return true.
            "true".to_string()
        };

        Some(
            RustTraitImplMethod::new(method_name)
                .with_return_type("bool".to_string())
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

    /// Check if the element types contain BackboneElement
    fn is_backbone_element(&self, element_types: &[crate::fhir_types::ElementType]) -> bool {
        element_types
            .iter()
            .any(|et| et.code.as_deref() == Some("BackboneElement"))
    }

    /// Get the specific nested type for a BackboneElement field
    fn get_nested_type_for_backbone_element(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        is_array: bool,
    ) -> crate::rust_types::RustType {
        let path_parts: Vec<&str> = element.path.split('.').collect();

        if path_parts.len() == 2 {
            let resource_name = path_parts[0];
            let field_name = path_parts[1];

            // Generate the expected nested type name: ResourceFieldName (e.g., AccountCoverage)
            let field_name_pascal = crate::naming::Naming::to_pascal_case(field_name);
            let nested_type_name = format!("{resource_name}{field_name_pascal}");

            let rust_type = crate::rust_types::RustType::Custom(nested_type_name);

            if is_array {
                crate::rust_types::RustType::Vec(Box::new(rust_type))
            } else {
                rust_type
            }
        } else {
            // Fallback to BackboneElement if we can't determine the specific type
            let rust_type = crate::rust_types::RustType::Custom("BackboneElement".to_string());
            if is_array {
                crate::rust_types::RustType::Vec(Box::new(rust_type))
            } else {
                rust_type
            }
        }
    }

    /// Get the Rust type for a field element, considering ValueSet bindings.
    /// For code fields with required bindings, returns the enum type name.
    /// Otherwise, delegates to TypeUtilities for standard type mapping.
    fn get_field_rust_type(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        field_name: &str,
    ) -> CodegenResult<crate::rust_types::RustType> {
        use crate::rust_types::RustType;

        let Some(element_type) = element.element_type.as_ref().and_then(|t| t.first()) else {
            return Ok(RustType::String);
        };

        let Some(code) = &element_type.code else {
            return Ok(RustType::String);
        };

        // Check if this is a code type with a required binding - if so, use enum type
        if code == "code" {
            if let Some(binding) = &element.binding {
                if binding.strength == "required" {
                    if let Some(value_set_url) = &binding.value_set {
                        // Extract enum name from ValueSet URL
                        if let Some(enum_name) =
                            self.extract_enum_name_from_value_set(value_set_url)
                        {
                            return Ok(RustType::Custom(enum_name));
                        }
                    }
                }
            }
        }

        // Otherwise, use the standard type mapping
        use crate::generators::TypeUtilities;
        TypeUtilities::map_fhir_type_to_rust(element_type, field_name, &element.path)
    }

    /// Extract enum type name from a ValueSet URL
    /// E.g., "http://hl7.org/fhir/ValueSet/account-status" -> "AccountStatus"
    fn extract_enum_name_from_value_set(&self, url: &str) -> Option<String> {
        // Remove version suffix if present (e.g., |4.0.1)
        let url_without_version = url.split('|').next().unwrap_or(url);

        // Extract the last part after the last /
        let value_set_name = url_without_version.split('/').next_back()?;

        // Use the same logic as ValueSetManager::generate_enum_name for consistency
        // Split on hyphens and capitalize each part to get PascalCase
        let name = value_set_name
            .split(&['-', '.'][..])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        // Ensure it's a valid Rust identifier
        if name.chars().next().unwrap_or('0').is_ascii_digit() {
            Some(format!("ValueSet{name}"))
        } else {
            Some(name)
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
