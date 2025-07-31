//! Type mapping utilities for FHIR to Rust type conversion
//!
//! This module handles the conversion of FHIR data types to appropriate Rust types,
//! including handling of complex types, references, and custom mappings.

use crate::config::CodegenConfig;
use crate::fhir_types::ElementType;
use crate::rust_types::RustType;
use crate::value_sets::ValueSetManager;

/// Handles mapping of FHIR types to Rust types
#[derive(Debug)]
pub struct TypeMapper<'a> {
    config: &'a CodegenConfig,
    value_set_manager: &'a mut ValueSetManager,
}

impl<'a> TypeMapper<'a> {
    pub fn new(config: &'a CodegenConfig, value_set_manager: &'a mut ValueSetManager) -> Self {
        Self {
            config,
            value_set_manager,
        }
    }

    /// Map a FHIR type to a Rust type
    pub fn map_fhir_type(&mut self, fhir_types: &[ElementType], is_array: bool) -> RustType {
        self.map_fhir_type_with_binding(fhir_types, None, is_array)
    }

    /// Map a FHIR type to a Rust type, considering binding information for enum generation
    pub fn map_fhir_type_with_binding(
        &mut self,
        fhir_types: &[ElementType],
        binding: Option<&crate::fhir_types::ElementBinding>,
        is_array: bool,
    ) -> RustType {
        if fhir_types.is_empty() {
            return RustType::String; // Default fallback
        }

        let primary_type = &fhir_types[0];
        let rust_type = self.map_single_fhir_type_with_binding(primary_type, binding);

        if is_array {
            RustType::Vec(Box::new(rust_type))
        } else {
            rust_type
        }
    }

    /// Parse ValueSet URL to extract URL and version
    fn parse_valueset_url(&self, url: &str) -> (String, Option<String>) {
        if let Some(pipe_pos) = url.find('|') {
            let base_url = url[..pipe_pos].to_string();
            let version = url[pipe_pos + 1..].to_string();
            (base_url, Some(version))
        } else {
            (url.to_string(), None)
        }
    }

    /// Generate enum for required ValueSet binding
    fn generate_enum_for_required_binding(
        &mut self,
        url: &str,
        version: Option<&str>,
    ) -> Option<String> {
        // Try to generate enum from ValueSet file
        match self
            .value_set_manager
            .generate_enum_from_value_set(url, version)
        {
            Ok(enum_name) => Some(enum_name),
            Err(_) => {
                // Fallback to placeholder enum
                Some(self.value_set_manager.generate_placeholder_enum(url))
            }
        }
    }

    /// Map a single FHIR ElementType to a Rust type
    #[allow(dead_code)]
    fn map_single_fhir_type(&mut self, element_type: &ElementType) -> RustType {
        self.map_single_fhir_type_with_binding(element_type, None)
    }

    /// Map a single FHIR ElementType to a Rust type, considering binding information
    fn map_single_fhir_type_with_binding(
        &mut self,
        element_type: &ElementType,
        binding: Option<&crate::fhir_types::ElementBinding>,
    ) -> RustType {
        // Handle cases where code is missing - default to String
        let code = match &element_type.code {
            Some(c) => c,
            None => return RustType::String,
        };

        // Check for custom type mappings first
        if let Some(rust_type) = self.config.type_mappings.get(code) {
            return self.parse_rust_type_string(rust_type);
        }

        // Handle built-in FHIR types
        match code.as_str() {
            // Primitive types
            "string" | "markdown" | "uri" | "url" | "canonical" | "oid" | "uuid" => {
                RustType::String
            }
            "integer" | "positiveInt" | "unsignedInt" => RustType::Integer,
            "boolean" => RustType::Boolean,
            "decimal" => RustType::Float,

            // Date/time types (represented as strings for now)
            "date" | "dateTime" | "instant" | "time" => RustType::String,

            // Binary data
            "base64Binary" => RustType::String,

            // Code types - check for required binding and generate enum
            "code" => {
                if let Some(binding) = binding {
                    if binding.strength == "required" {
                        if let Some(value_set_url) = &binding.value_set {
                            // Parse ValueSet URL and version
                            let (url, version) = self.parse_valueset_url(value_set_url);

                            // Generate enum for required binding
                            if let Some(enum_name) =
                                self.generate_enum_for_required_binding(&url, version.as_deref())
                            {
                                return RustType::Custom(enum_name);
                            }
                        }
                    }
                }
                // Fall back to String for non-required bindings or when enum generation fails
                RustType::String
            }

            // Complex types
            "Reference" => self.handle_reference_type(element_type),
            "CodeableConcept" => RustType::Custom("CodeableConcept".to_string()),
            "Coding" => RustType::Custom("Coding".to_string()),
            "Identifier" => RustType::Custom("Identifier".to_string()),
            "Period" => RustType::Custom("Period".to_string()),
            "Quantity" => RustType::Custom("Quantity".to_string()),
            "Range" => RustType::Custom("Range".to_string()),
            "Ratio" => RustType::Custom("Ratio".to_string()),
            "SampledData" => RustType::Custom("SampledData".to_string()),
            "Attachment" => RustType::Custom("Attachment".to_string()),
            "ContactPoint" => RustType::Custom("ContactPoint".to_string()),
            "HumanName" => RustType::Custom("HumanName".to_string()),
            "Address" => RustType::Custom("Address".to_string()),
            "Age" => RustType::Custom("Age".to_string()),
            "Count" => RustType::Custom("Count".to_string()),
            "Distance" => RustType::Custom("Distance".to_string()),
            "Duration" => RustType::Custom("Duration".to_string()),
            "Money" => RustType::Custom("Money".to_string()),

            // Extension type
            "Extension" => RustType::Custom("Extension".to_string()),

            // StructureDefinition sub-types
            "BackboneElement" => RustType::Custom("BackboneElement".to_string()),
            "ElementDefinition" => RustType::Custom("ElementDefinition".to_string()),

            // Handle FHIRPath system types
            typ if typ.starts_with("http://hl7.org/fhirpath/System.") => {
                let system_type = typ
                    .strip_prefix("http://hl7.org/fhirpath/System.")
                    .unwrap_or("String");
                match system_type {
                    "String" => RustType::String,
                    "Integer" => RustType::Integer,
                    "Boolean" => RustType::Boolean,
                    "Decimal" => RustType::Float,
                    _ => RustType::String,
                }
            }

            // Resource types - use the type name directly
            resource_type if self.is_resource_type(resource_type) => {
                RustType::Custom(resource_type.to_string())
            }

            // Unknown type - default to string
            _ => {
                eprintln!("Warning: Unknown FHIR type '{code}', defaulting to String");
                RustType::String
            }
        }
    }

    /// Handle Reference types with target profiles
    fn handle_reference_type(&mut self, _element_type: &ElementType) -> RustType {
        // For now, just use a generic Reference type
        // In the future, we could generate different Reference types for different targets
        RustType::Custom("Reference".to_string())
    }

    /// Extract resource name from a profile URL
    #[allow(dead_code)]
    fn extract_resource_name(&self, profile_url: &str) -> String {
        profile_url
            .split('/')
            .next_back()
            .unwrap_or("Resource")
            .to_string()
    }

    /// Check if a type name represents a FHIR resource
    fn is_resource_type(&self, type_name: &str) -> bool {
        // This is a simplified check - in a real implementation, you might want
        // to maintain a comprehensive list of FHIR resource types
        type_name.chars().next().is_some_and(|c| c.is_uppercase())
            && !matches!(type_name, "String" | "Boolean" | "Integer" | "Float")
    }

    /// Parse a Rust type string from configuration
    #[allow(clippy::only_used_in_recursion)]
    fn parse_rust_type_string(&self, type_str: &str) -> RustType {
        match type_str {
            "String" => RustType::String,
            "i32" => RustType::Integer,
            "bool" => RustType::Boolean,
            "f64" => RustType::Float,
            s if s.starts_with("Option<") && s.ends_with('>') => {
                let inner = &s[7..s.len() - 1];
                RustType::Option(Box::new(self.parse_rust_type_string(inner)))
            }
            s if s.starts_with("Vec<") && s.ends_with('>') => {
                let inner = &s[4..s.len() - 1];
                RustType::Vec(Box::new(self.parse_rust_type_string(inner)))
            }
            _ => RustType::Custom(type_str.to_string()),
        }
    }

    /// Get the appropriate Rust type for a ValueSet binding
    pub fn get_value_set_type(&mut self, value_set_url: &str) -> RustType {
        if self.value_set_manager.is_cached(value_set_url) {
            let enum_name = self
                .value_set_manager
                .get_enum_name(value_set_url)
                .expect("Cached ValueSet should have enum name")
                .clone();
            RustType::Custom(enum_name)
        } else {
            let enum_name = self
                .value_set_manager
                .generate_placeholder_enum(value_set_url);
            RustType::Custom(enum_name)
        }
    }

    /// Determine if a field should be optional based on FHIR cardinality
    pub fn is_optional(
        &self,
        min_cardinality: Option<u32>,
        _max_cardinality: Option<&str>,
    ) -> bool {
        match min_cardinality {
            Some(0) => true,
            Some(_) => false,
            None => true, // Default to optional if not specified
        }
    }

    /// Determine if a field represents an array based on FHIR cardinality
    pub fn is_array(&self, max_cardinality: Option<&str>) -> bool {
        match max_cardinality {
            Some("1") => false,
            Some("0") => false,
            Some(_) => true, // "*", numbers > 1
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CodegenConfig;

    #[test]
    fn test_primitive_type_mapping() {
        let config = CodegenConfig::default();
        let mut value_set_manager = ValueSetManager::new();
        let mut mapper = TypeMapper::new(&config, &mut value_set_manager);

        let string_type = ElementType {
            code: Some("string".to_string()),
            target_profile: None,
        };

        assert!(matches!(
            mapper.map_single_fhir_type(&string_type),
            RustType::String
        ));

        let boolean_type = ElementType {
            code: Some("boolean".to_string()),
            target_profile: None,
        };

        assert!(matches!(
            mapper.map_single_fhir_type(&boolean_type),
            RustType::Boolean
        ));
    }

    #[test]
    fn test_cardinality_checks() {
        let config = CodegenConfig::default();
        let mut value_set_manager = ValueSetManager::new();
        let mapper = TypeMapper::new(&config, &mut value_set_manager);

        assert!(mapper.is_optional(Some(0), Some("1")));
        assert!(!mapper.is_optional(Some(1), Some("1")));
        assert!(mapper.is_optional(None, Some("1")));

        assert!(!mapper.is_array(Some("1")));
        assert!(mapper.is_array(Some("*")));
        assert!(mapper.is_array(Some("5")));
    }
}
