//! Enum generation functionality
//!
//! This module handles the generation of Rust enums from FHIR ValueSets.

use std::collections::HashMap;

use crate::generators::type_registry::{TypeClassification, TypeRegistry};
use crate::rust_types::RustEnum;
use crate::value_sets::ValueSetManager;
use crate::CodegenResult;

/// Enum generator for FHIR ValueSets
pub struct EnumGenerator<'a> {
    value_set_manager: &'a mut ValueSetManager,
    enum_cache: &'a mut HashMap<String, RustEnum>,
}

impl<'a> EnumGenerator<'a> {
    /// Create a new enum generator
    pub fn new(
        value_set_manager: &'a mut ValueSetManager,
        enum_cache: &'a mut HashMap<String, RustEnum>,
    ) -> Self {
        Self {
            value_set_manager,
            enum_cache,
        }
    }

    /// Generate an enum for a value set binding
    pub fn generate_enum_for_value_set(
        &mut self,
        value_set_url: &str,
    ) -> CodegenResult<Option<RustEnum>> {
        // Check if we've already generated this enum
        if let Some(cached_enum) = self.enum_cache.get(value_set_url) {
            return Ok(Some(cached_enum.clone()));
        }

        // Generate a placeholder enum using the value set manager
        let enum_name = self
            .value_set_manager
            .generate_placeholder_enum(value_set_url);

        // Register the enum as a ValueSet-based enum in the TypeRegistry
        TypeRegistry::register_type(&enum_name, TypeClassification::ValueSetEnum);

        // Get the generated enum from the value set manager's cache
        if let Some(rust_enum) = self.value_set_manager.get_cached_enums().get(&enum_name) {
            // Cache it in our own cache as well
            self.enum_cache
                .insert(value_set_url.to_string(), rust_enum.clone());
            Ok(Some(rust_enum.clone()))
        } else {
            Ok(None)
        }
    }

    /// Check if any ValueSet enums have been generated
    pub fn has_cached_enums(&self) -> bool {
        !self.value_set_manager.get_cached_enums().is_empty()
    }

    /// Convert an enum name to a filename using snake_case
    pub fn enum_name_to_filename(enum_name: &str) -> String {
        let snake_case_name = crate::naming::Naming::to_snake_case(enum_name);
        format!("{snake_case_name}.rs")
    }

    /// Convert an enum name to a module name using snake_case (matching filename)
    pub fn enum_name_to_module_name(enum_name: &str) -> String {
        crate::naming::Naming::to_snake_case(enum_name)
    }

    /// Get all cached enums from the value set manager
    pub fn get_cached_enums(&self) -> &HashMap<String, RustEnum> {
        self.value_set_manager.get_cached_enums()
    }
}
