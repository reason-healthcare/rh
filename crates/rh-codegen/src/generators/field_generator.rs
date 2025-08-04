//! Field generation functionality for FHIR types
//!
//! This module handles the creation of Rust fields from FHIR ElementDefinitions,
//! including field naming, type mapping, and metadata assignment.

use std::collections::HashMap;

use crate::config::CodegenConfig;
use crate::fhir_types::{ElementDefinition, ElementType};
use crate::generators::{DocumentationGenerator, NameGenerator};
use crate::rust_types::{RustField, RustStruct, RustType};
use crate::type_mapper::TypeMapper;
use crate::value_sets::ValueSetManager;
use crate::CodegenResult;

/// Generator for creating Rust fields from FHIR ElementDefinitions
pub struct FieldGenerator<'a> {
    config: &'a CodegenConfig,
    type_cache: &'a HashMap<String, RustStruct>,
    value_set_manager: &'a mut ValueSetManager,
}

impl<'a> FieldGenerator<'a> {
    /// Create a new field generator
    pub fn new(
        config: &'a CodegenConfig,
        type_cache: &'a HashMap<String, RustStruct>,
        value_set_manager: &'a mut ValueSetManager,
    ) -> Self {
        Self {
            config,
            type_cache,
            value_set_manager,
        }
    }

    /// Create a RustField from an ElementDefinition
    pub fn create_field_from_element(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        // Get the field name from the path (last segment)
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
        let rust_field_name = Self::to_rust_field_name(field_name);

        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        // Get the field type
        let field_type = if let Some(element_types) = &element.element_type {
            // Check if this should use a nested struct type
            if self.should_use_nested_struct_type(element, element_types) {
                self.build_nested_struct_type(element, is_array)
            } else {
                let mut type_mapper = TypeMapper::new(self.config, self.value_set_manager);
                type_mapper.map_fhir_type_with_binding(
                    element_types,
                    element.binding.as_ref(),
                    is_array,
                )
            }
        } else {
            // No type specified, default to String
            if is_array {
                RustType::Vec(Box::new(RustType::String))
            } else {
                RustType::String
            }
        };

        // Create the field
        let mut field = RustField::new(rust_field_name.clone(), field_type);
        field.is_optional = is_optional;

        // Add documentation if available
        field.doc_comment = DocumentationGenerator::generate_field_documentation(element);

        // Add serde rename if the field name was changed
        if rust_field_name != field_name {
            field = field.with_serde_rename(field_name.to_string());
        }

        Ok(Some(field))
    }

    /// Convert a FHIR field name to a valid Rust field name
    pub fn to_rust_field_name(name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = clean_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        // Handle Rust keywords by appending underscore
        Self::handle_rust_keywords(&snake_case)
    }

    /// Handle Rust keywords by appending underscore
    fn handle_rust_keywords(name: &str) -> String {
        match name {
            "type" => "type_".to_string(),
            "use" => "use_".to_string(),
            "ref" => "ref_".to_string(),
            "mod" => "mod_".to_string(),
            "fn" => "fn_".to_string(),
            "let" => "let_".to_string(),
            "const" => "const_".to_string(),
            "static" => "static_".to_string(),
            "struct" => "struct_".to_string(),
            "enum" => "enum_".to_string(),
            "impl" => "impl_".to_string(),
            "trait" => "trait_".to_string(),
            "for" => "for_".to_string(),
            "if" => "if_".to_string(),
            "else" => "else_".to_string(),
            "while" => "while_".to_string(),
            "loop" => "loop_".to_string(),
            "match" => "match_".to_string(),
            "return" => "return_".to_string(),
            "where" => "where_".to_string(),
            "abstract" => "abstract_".to_string(),
            _ => name.to_string(),
        }
    }

    /// Check if a field should use a nested struct type instead of BackboneElement
    fn should_use_nested_struct_type(
        &self,
        element: &ElementDefinition,
        element_types: &[ElementType],
    ) -> bool {
        // Check if this element is a BackboneElement and we have nested elements for it
        if let Some(first_type) = element_types.first() {
            if let Some(code) = &first_type.code {
                if code == "BackboneElement" {
                    // Extract parent struct name and field name from the path
                    let path_parts: Vec<&str> = element.path.split('.').collect();
                    if path_parts.len() >= 2 {
                        let parent_struct_name =
                            NameGenerator::to_valid_rust_identifier(path_parts[0]);
                        let field_name = path_parts[path_parts.len() - 1];

                        // For nested structures, we need to build the correct nested struct name
                        // For example: Bundle.entry.search -> BundleEntrySearch
                        let nested_struct_name = if path_parts.len() == 2 {
                            // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                            format!(
                                "{}{}",
                                parent_struct_name,
                                NameGenerator::to_pascal_case(field_name)
                            )
                        } else {
                            // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                            let mut nested_name = parent_struct_name;
                            for part in path_parts.iter().skip(1) {
                                nested_name.push_str(&NameGenerator::to_pascal_case(part));
                            }
                            nested_name
                        };

                        // Check if we have generated this nested struct
                        return self.type_cache.contains_key(&nested_struct_name);
                    }
                }
            }
        }
        false
    }

    /// Build the correct nested struct type based on the full path
    fn build_nested_struct_type(&self, element: &ElementDefinition, is_array: bool) -> RustType {
        let path_parts: Vec<&str> = element.path.split('.').collect();
        let nested_struct_name = if path_parts.len() >= 2 {
            let parent_struct_name = NameGenerator::to_valid_rust_identifier(path_parts[0]);
            if path_parts.len() == 2 {
                // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                format!(
                    "{}{}",
                    parent_struct_name,
                    NameGenerator::to_pascal_case(path_parts[1])
                )
            } else {
                // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                let mut nested_name = parent_struct_name;
                for part in path_parts.iter().skip(1) {
                    nested_name.push_str(&NameGenerator::to_pascal_case(part));
                }
                nested_name
            }
        } else {
            format!(
                "{}Unknown",
                NameGenerator::to_valid_rust_identifier(&element.path)
            )
        };

        if is_array {
            RustType::Vec(Box::new(RustType::Custom(nested_struct_name)))
        } else {
            RustType::Custom(nested_struct_name)
        }
    }

    /// Extract field name from element path (utility method)
    pub fn extract_field_name_from_path(path: &str) -> &str {
        path.split('.').next_back().unwrap_or("unknown")
    }

    /// Check if a field name requires serde rename
    pub fn requires_serde_rename(original_name: &str, rust_field_name: &str) -> bool {
        original_name != rust_field_name
    }

    /// Determine field cardinality from ElementDefinition
    pub fn determine_field_cardinality(element: &ElementDefinition) -> (bool, bool) {
        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        (is_optional, is_array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::ElementType;

    #[test]
    fn test_to_rust_field_name() {
        // Test basic field names
        assert_eq!(FieldGenerator::to_rust_field_name("active"), "active");
        assert_eq!(FieldGenerator::to_rust_field_name("name"), "name");

        // Test PascalCase to snake_case conversion
        assert_eq!(
            FieldGenerator::to_rust_field_name("birthDate"),
            "birth_date"
        );
        assert_eq!(
            FieldGenerator::to_rust_field_name("multipleBirthBoolean"),
            "multiple_birth_boolean"
        );

        // Test choice types with [x] suffix
        assert_eq!(FieldGenerator::to_rust_field_name("value[x]"), "value");
        assert_eq!(
            FieldGenerator::to_rust_field_name("deceased[x]"),
            "deceased"
        );

        // Test Rust keywords
        assert_eq!(FieldGenerator::to_rust_field_name("type"), "type_");
        assert_eq!(FieldGenerator::to_rust_field_name("use"), "use_");
        assert_eq!(FieldGenerator::to_rust_field_name("ref"), "ref_");
        assert_eq!(FieldGenerator::to_rust_field_name("for"), "for_");
        assert_eq!(FieldGenerator::to_rust_field_name("match"), "match_");
    }

    #[test]
    fn test_handle_rust_keywords() {
        assert_eq!(FieldGenerator::handle_rust_keywords("type"), "type_");
        assert_eq!(FieldGenerator::handle_rust_keywords("struct"), "struct_");
        assert_eq!(FieldGenerator::handle_rust_keywords("impl"), "impl_");
        assert_eq!(FieldGenerator::handle_rust_keywords("normal"), "normal");
    }

    #[test]
    fn test_extract_field_name_from_path() {
        assert_eq!(
            FieldGenerator::extract_field_name_from_path("Patient.active"),
            "active"
        );
        assert_eq!(
            FieldGenerator::extract_field_name_from_path("Bundle.entry.resource"),
            "resource"
        );
        assert_eq!(
            FieldGenerator::extract_field_name_from_path("unknown"),
            "unknown"
        );
    }

    #[test]
    fn test_requires_serde_rename() {
        assert!(!FieldGenerator::requires_serde_rename("active", "active"));
        assert!(FieldGenerator::requires_serde_rename(
            "birthDate",
            "birth_date"
        ));
        assert!(FieldGenerator::requires_serde_rename("type", "type_"));
    }

    #[test]
    fn test_determine_field_cardinality() {
        use crate::fhir_types::ElementDefinition;

        // Test optional field
        let optional_element = ElementDefinition {
            id: Some("Patient.active".to_string()),
            path: "Patient.active".to_string(),
            short: Some("Whether this patient record is in active use".to_string()),
            definition: None,
            min: Some(0),
            max: Some("1".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("boolean".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let (is_optional, is_array) =
            FieldGenerator::determine_field_cardinality(&optional_element);
        assert!(is_optional);
        assert!(!is_array);

        // Test required array field
        let array_element = ElementDefinition {
            id: Some("Patient.name".to_string()),
            path: "Patient.name".to_string(),
            short: Some("A name associated with the patient".to_string()),
            definition: None,
            min: Some(1),
            max: Some("*".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("HumanName".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let (is_optional, is_array) = FieldGenerator::determine_field_cardinality(&array_element);
        assert!(!is_optional);
        assert!(is_array);

        // Test required single field
        let required_element = ElementDefinition {
            id: Some("Patient.id".to_string()),
            path: "Patient.id".to_string(),
            short: Some("Logical id of this artifact".to_string()),
            definition: None,
            min: Some(1),
            max: Some("1".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("id".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let (is_optional, is_array) =
            FieldGenerator::determine_field_cardinality(&required_element);
        assert!(!is_optional);
        assert!(!is_array);
    }

    #[test]
    fn test_create_field_from_element() {
        use crate::config::CodegenConfig;
        use std::collections::HashMap;

        let config = CodegenConfig::default();
        let type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);

        // Test simple boolean field
        let boolean_element = ElementDefinition {
            id: Some("Patient.active".to_string()),
            path: "Patient.active".to_string(),
            short: Some("Whether this patient record is in active use".to_string()),
            definition: None,
            min: Some(0),
            max: Some("1".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("boolean".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let result = field_generator.create_field_from_element(&boolean_element);
        assert!(result.is_ok());

        let field = result.unwrap();
        assert!(field.is_some());

        let field = field.unwrap();
        assert_eq!(field.name, "active");
        assert!(field.is_optional);
        assert!(matches!(field.field_type, RustType::Boolean));
    }
}
