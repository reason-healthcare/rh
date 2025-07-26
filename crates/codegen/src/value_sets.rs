//! ValueSet management and code generation utilities
//!
//! This module handles FHIR ValueSets, including generation of Rust enums
//! from ValueSet codes and management of code system mappings.

use crate::rust_types::{RustEnum, RustEnumVariant};
use std::collections::HashMap;

/// Manages FHIR ValueSets and their conversion to Rust enums
#[derive(Debug, Clone)]
pub struct ValueSetManager {
    /// Cache of ValueSet URLs to generated enum names
    value_set_cache: HashMap<String, String>,
    /// Cache of generated enums by name
    enum_cache: HashMap<String, RustEnum>,
}

impl ValueSetManager {
    pub fn new() -> Self {
        Self {
            value_set_cache: HashMap::new(),
            enum_cache: HashMap::new(),
        }
    }

    /// Generate a Rust enum name from a ValueSet URL
    pub fn generate_enum_name(&self, value_set_url: &str) -> String {
        // Extract the last part of the URL and convert to PascalCase
        let name = value_set_url
            .split('/')
            .next_back()
            .unwrap_or("UnknownValueSet")
            .split('-')
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
            format!("ValueSet{name}")
        } else {
            name
        }
    }

    /// Check if a ValueSet is already cached
    pub fn is_cached(&self, value_set_url: &str) -> bool {
        self.value_set_cache.contains_key(value_set_url)
    }

    /// Get the enum name for a cached ValueSet
    pub fn get_enum_name(&self, value_set_url: &str) -> Option<&String> {
        self.value_set_cache.get(value_set_url)
    }

    /// Cache a ValueSet with its generated enum
    pub fn cache_value_set(
        &mut self,
        value_set_url: String,
        enum_name: String,
        rust_enum: RustEnum,
    ) {
        self.value_set_cache
            .insert(value_set_url, enum_name.clone());
        self.enum_cache.insert(enum_name, rust_enum);
    }

    /// Get all cached enums
    pub fn get_cached_enums(&self) -> &HashMap<String, RustEnum> {
        &self.enum_cache
    }

    /// Generate a basic enum for unknown ValueSets
    pub fn generate_placeholder_enum(&mut self, value_set_url: &str) -> String {
        let enum_name = self.generate_enum_name(value_set_url);

        if !self.is_cached(value_set_url) {
            let mut rust_enum = RustEnum::new(enum_name.clone());
            rust_enum.doc_comment = Some(format!("Generated enum for ValueSet: {value_set_url}"));

            // Add a placeholder variant
            rust_enum.add_variant(RustEnumVariant::new("Unknown".to_string()));

            self.cache_value_set(value_set_url.to_string(), enum_name.clone(), rust_enum);
        }

        enum_name
    }
}

impl Default for ValueSetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a FHIR ValueSet concept
#[derive(Debug, Clone)]
pub struct ValueSetConcept {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
    pub system: Option<String>,
}

impl ValueSetConcept {
    pub fn new(code: String) -> Self {
        Self {
            code,
            display: None,
            definition: None,
            system: None,
        }
    }

    /// Convert the concept code to a valid Rust enum variant name
    pub fn to_variant_name(&self) -> String {
        // Convert kebab-case, snake_case, or other formats to PascalCase
        let name = self
            .code
            .split(&['-', '_', '.', ' '][..])
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        // Ensure it starts with a letter
        if name.chars().next().unwrap_or('0').is_ascii_digit() {
            format!("Code{name}")
        } else if name.is_empty() {
            "Unknown".to_string()
        } else {
            name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enum_name() {
        let manager = ValueSetManager::new();

        assert_eq!(
            manager.generate_enum_name("http://hl7.org/fhir/ValueSet/administrative-gender"),
            "AdministrativeGender"
        );

        assert_eq!(
            manager.generate_enum_name("http://hl7.org/fhir/ValueSet/123-test"),
            "ValueSet123Test"
        );
    }

    #[test]
    fn test_concept_variant_name() {
        let concept = ValueSetConcept::new("male".to_string());
        assert_eq!(concept.to_variant_name(), "Male");

        let concept = ValueSetConcept::new("unknown-gender".to_string());
        assert_eq!(concept.to_variant_name(), "UnknownGender");

        let concept = ValueSetConcept::new("123-code".to_string());
        assert_eq!(concept.to_variant_name(), "Code123Code");
    }
}
