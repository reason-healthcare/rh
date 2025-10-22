//! Field generation functionality for FHIR types
//!
//! This module handles the creation of Rust fields from FHIR ElementDefinitions,
//! including field naming, type mapping, and metadata assignment.

use std::collections::HashMap;

use crate::config::CodegenConfig;
use crate::fhir_types::{ElementDefinition, ElementType};
use crate::generators::DocumentationGenerator;
use crate::naming::Naming;
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

    /// Create RustField(s) from an ElementDefinition
    /// Returns a vector because choice types generate multiple fields
    pub fn create_fields_from_element(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Vec<RustField>> {
        // Get the field name from the path (last segment)
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");

        // Check if this is a choice type (ends with [x])
        if field_name.ends_with("[x]") {
            return self.create_choice_type_fields(element);
        }

        // Check if this is a primitive type that should use explicit field generation
        if self.config.use_macro_calls {
            if let Some(primitive_field) = self.create_primitive_field_macro_call(element)? {
                // Also create the companion extension field
                let companion_field = self.create_companion_extension_field(field_name)?;
                return Ok(vec![primitive_field, companion_field]);
            }
        }

        // Regular field - create single field and possibly companion field
        let mut fields = Vec::new();
        if let Some(field) = self.create_single_field_from_element(element)? {
            fields.push(field);

            // Check if we need to create a companion field for primitive types
            if let Some(companion_field) = self.create_companion_field_if_primitive(element)? {
                fields.push(companion_field);
            }
        }
        Ok(fields)
    }

    /// Create multiple fields for FHIR choice types (e.g., value[x], effective[x])
    fn create_choice_type_fields(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Vec<RustField>> {
        let mut fields = Vec::new();
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
        let base_name = field_name.strip_suffix("[x]").unwrap_or(field_name);

        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        if let Some(element_types) = &element.element_type {
            for element_type in element_types {
                if let Some(type_code) = &element_type.code {
                    // Create field name: base_name + type_code in snake_case
                    let type_suffix = crate::naming::Naming::type_suffix(type_code);
                    let field_name = format!("{base_name}_{type_suffix}");
                    let rust_field_name = crate::naming::Naming::field_name(&field_name);

                    // Map the type
                    let mut type_mapper = TypeMapper::new(self.config, self.value_set_manager);
                    let field_type = type_mapper.map_fhir_type_with_binding(
                        std::slice::from_ref(element_type),
                        element.binding.as_ref(),
                        is_array,
                    );

                    // Create the field
                    let mut field = RustField::new(rust_field_name.clone(), field_type);
                    field.is_optional = is_optional;

                    // Add documentation
                    field.doc_comment =
                        DocumentationGenerator::generate_choice_field_documentation_with_binding(
                            element,
                            type_code,
                            self.value_set_manager,
                        );

                    // Add serde rename for the original FHIR field name with type suffix
                    let serde_name = format!(
                        "{base_name}{type_code_capitalized}",
                        type_code_capitalized = Self::capitalize_first_char(type_code)
                    );
                    field = field.with_serde_rename(serde_name);

                    fields.push(field);
                }
            }
        }

        Ok(fields)
    }

    /// Create a single RustField from an ElementDefinition
    fn create_single_field_from_element(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        // Get the field name from the path (last segment)
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
        let rust_field_name = crate::naming::Naming::field_name(field_name);

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
            // No type specified, default to StringType
            if is_array {
                RustType::Vec(Box::new(RustType::Custom("StringType".to_string())))
            } else {
                RustType::Custom("StringType".to_string())
            }
        };

        // Create the field
        let mut field = RustField::new(rust_field_name.clone(), field_type);
        field.is_optional = is_optional;

        // Add documentation if available
        field.doc_comment = DocumentationGenerator::generate_field_documentation_with_binding(
            element,
            self.value_set_manager,
        );

        // Add serde rename if the field name was changed
        if rust_field_name != field_name {
            field = field.with_serde_rename(field_name.to_string());
        }

        Ok(Some(field))
    }

    /// Legacy method for backward compatibility
    pub fn create_field_from_element(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        let fields = self.create_fields_from_element(element)?;
        Ok(fields.into_iter().next())
    }

    /// Convert a FHIR field name to a valid Rust field name
    pub fn to_rust_field_name(name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Handle field name conflicts with inherited base field
        let conflict_resolved_name = if clean_name == "base" {
            // Rename FHIR 'base' elements to avoid conflict with the inherited base field
            "base_definition"
        } else {
            clean_name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = conflict_resolved_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{c}", c = c.to_lowercase())
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

    /// Convert FHIR type code to snake_case for field suffix
    pub fn type_code_to_snake_case(type_code: &str) -> String {
        type_code
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{c}", c = c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect()
    }

    /// Capitalize the first character of a string
    fn capitalize_first_char(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
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
                        let parent_struct_name = Naming::to_rust_identifier(path_parts[0]);
                        let field_name = path_parts[path_parts.len() - 1];

                        // For nested structures, we need to build the correct nested struct name
                        // For example: Bundle.entry.search -> BundleEntrySearch
                        let nested_struct_name = if path_parts.len() == 2 {
                            // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                            format!(
                                "{parent_struct_name}{field_pascal}",
                                field_pascal = Naming::to_pascal_case(field_name)
                            )
                        } else {
                            // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                            let mut nested_name = parent_struct_name;
                            for part in path_parts.iter().skip(1) {
                                nested_name.push_str(&Naming::to_pascal_case(part));
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
            let parent_struct_name = Naming::to_rust_identifier(path_parts[0]);
            if path_parts.len() == 2 {
                // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                format!(
                    "{parent_struct_name}{part_pascal}",
                    part_pascal = Naming::to_pascal_case(path_parts[1])
                )
            } else {
                // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                let mut nested_name = parent_struct_name;
                for part in path_parts.iter().skip(1) {
                    nested_name.push_str(&Naming::to_pascal_case(part));
                }
                nested_name
            }
        } else {
            format!(
                "{path_identifier}Unknown",
                path_identifier = Naming::to_rust_identifier(&element.path)
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

    /// Extract choice type information from a StructureDefinition
    /// Returns a vector of (base_name, type_codes) tuples for each choice type found
    pub fn extract_choice_types_from_structure(
        structure_def: &crate::fhir_types::StructureDefinition,
    ) -> Vec<(String, Vec<String>)> {
        let mut choice_types = Vec::new();

        // Get elements from differential or snapshot
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return choice_types; // No elements to process
        };

        for element in elements {
            // Check if this is a choice type field (ends with [x])
            let field_name = element.path.split('.').next_back().unwrap_or("unknown");
            if field_name.ends_with("[x]") {
                let base_name = field_name.strip_suffix("[x]").unwrap_or(field_name);

                // Extract type codes from element types
                if let Some(element_types) = &element.element_type {
                    let type_codes: Vec<String> = element_types
                        .iter()
                        .filter_map(|et| et.code.clone())
                        .collect();

                    if !type_codes.is_empty() {
                        choice_types.push((base_name.to_string(), type_codes));
                    }
                }
            }
        }

        choice_types
    }

    /// Create a companion field for primitive types that handles extensions
    fn create_companion_field_if_primitive(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        // Check if this element has a primitive type
        if let Some(element_types) = &element.element_type {
            if let Some(first_type) = element_types.first() {
                if let Some(type_code) = &first_type.code {
                    // Check if this is a primitive type
                    if self.is_primitive_type(type_code) {
                        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
                        let companion_field_name = format!("_{field_name}");
                        let rust_companion_field_name =
                            crate::naming::Naming::field_name(&companion_field_name);

                        // Map primitive type to companion element type
                        let companion_element_type = self.get_companion_element_type(type_code);

                        // Companion elements are always optional in FHIR specification
                        // They contain extensions and metadata for primitive fields
                        let is_optional = true;

                        // Create the companion field
                        let mut companion_field = RustField::new(
                            rust_companion_field_name.clone(),
                            RustType::Custom(companion_element_type),
                        );
                        companion_field.is_optional = is_optional;

                        // Add documentation
                        companion_field.doc_comment = Some(format!(
                            "Extension element for the '{field_name}' primitive field. Contains metadata and extensions."
                        ));

                        // Add serde rename if needed
                        if rust_companion_field_name != companion_field_name {
                            companion_field =
                                companion_field.with_serde_rename(companion_field_name);
                        }

                        return Ok(Some(companion_field));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Check if a type code represents a FHIR primitive type
    fn is_primitive_type(&self, type_code: &str) -> bool {
        matches!(
            type_code,
            "boolean"
                | "integer"
                | "positiveInt"
                | "unsignedInt"
                | "decimal"
                | "string"
                | "code"
                | "id"
                | "markdown"
                | "uri"
                | "url"
                | "canonical"
                | "oid"
                | "uuid"
                | "base64Binary"
                | "xhtml"
                | "date"
                | "dateTime"
                | "time"
                | "instant"
        )
    }

    /// Get the companion element type for a primitive type
    /// All companion elements now use the base Element type
    fn get_companion_element_type(&self, _primitive_type: &str) -> String {
        "Element".to_string()
    }

    /// Generate explicit primitive fields with companion extension fields
    /// This creates two RustField instances: the primitive field and its companion Element field
    pub fn create_primitive_field_macro_call(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        // Get the field name from the path (last segment)
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");

        // Skip choice types for now as they need special handling
        if field_name.ends_with("[x]") {
            return Ok(None);
        }

        // Check if this element has a primitive type
        if let Some(element_types) = &element.element_type {
            if let Some(first_type) = element_types.first() {
                if let Some(type_code) = &first_type.code {
                    // Check if this is a primitive type
                    if self.is_primitive_type(type_code) {
                        // Determine if this field is optional (min = 0)
                        let is_optional = element.min.unwrap_or(0) == 0;

                        // Map FHIR primitive type to Rust type
                        let rust_type_name = match type_code.as_str() {
                            "string" => "StringType",
                            "boolean" => "BooleanType",
                            "integer" => "IntegerType",
                            "decimal" => "DecimalType",
                            "dateTime" => "DateTimeType",
                            "date" => "DateType",
                            "time" => "TimeType",
                            "uri" => "UriType",
                            "canonical" => "CanonicalType",
                            "base64Binary" => "Base64BinaryType",
                            "instant" => "InstantType",
                            "positiveInt" => "PositiveIntType",
                            "unsignedInt" => "UnsignedIntType",
                            "id" => "IdType",
                            "oid" => "OidType",
                            "uuid" => "UuidType",
                            "code" => "CodeType",
                            "markdown" => "MarkdownType",
                            "url" => "UrlType",
                            _ => return Ok(None), // Unknown primitive type
                        };

                        // Create the main primitive field with proper type
                        let rust_type =
                            RustType::Custom(format!("crate::primitives::{rust_type_name}"));
                        let mut field = RustField::new(field_name.to_string(), rust_type);

                        if is_optional {
                            field = field.optional();
                        }

                        field = field.with_doc(format!("Field: {field_name}"));

                        // Note: The companion extension field will be handled separately by the caller
                        // to maintain clean separation of concerns

                        return Ok(Some(field));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Generate the companion extension field for a primitive field
    /// Returns a RustField representing the _fieldname extension companion
    pub fn create_companion_extension_field(&self, field_name: &str) -> CodegenResult<RustField> {
        let companion_name = format!("_{field_name}");
        let element_type = RustType::Custom("crate::datatypes::element::Element".to_string());

        let mut companion_field = RustField::new(companion_name.clone(), element_type);
        companion_field = companion_field.optional(); // Companion fields are always optional
        companion_field = companion_field.with_doc(format!(
            "Extension element for the '{field_name}' primitive field. Contains metadata and extensions."
        ));
        companion_field = companion_field.with_serde_rename(companion_name);

        Ok(companion_field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::ElementType;

    #[test]
    fn test_to_rust_field_name() {
        // Test basic field names
        assert_eq!(crate::naming::Naming::field_name("active"), "active");
        assert_eq!(crate::naming::Naming::field_name("name"), "name");

        // Test PascalCase to snake_case conversion
        assert_eq!(crate::naming::Naming::field_name("birthDate"), "birth_date");
        assert_eq!(
            crate::naming::Naming::field_name("multipleBirthBoolean"),
            "multiple_birth_boolean"
        );

        // Test choice types with [x] suffix
        assert_eq!(crate::naming::Naming::field_name("value[x]"), "value");
        assert_eq!(crate::naming::Naming::field_name("deceased[x]"), "deceased");

        // Test Rust keywords
        assert_eq!(crate::naming::Naming::field_name("type"), "type_");
        assert_eq!(crate::naming::Naming::field_name("use"), "use_");
        assert_eq!(crate::naming::Naming::field_name("ref"), "ref_");
        assert_eq!(crate::naming::Naming::field_name("for"), "for_");
        assert_eq!(crate::naming::Naming::field_name("match"), "match_");
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
        // The field type should be BooleanType (custom type from the type mapping)
        assert!(
            matches!(field.field_type, RustType::Custom(ref type_name) if type_name == "BooleanType")
        );
    }

    #[test]
    fn test_macro_call_generation() {
        use crate::config::CodegenConfig;
        use std::collections::HashMap;

        let config = CodegenConfig {
            use_macro_calls: true, // Enable macro call generation
            ..CodegenConfig::default()
        };

        let type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);

        // Test boolean field with macro calls enabled
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

        let result = field_generator.create_fields_from_element(&boolean_element);
        assert!(result.is_ok());

        let fields = result.unwrap();
        assert_eq!(fields.len(), 2); // Should return primitive field + companion extension field

        // Check the main primitive field
        let primitive_field = &fields[0];
        assert_eq!(primitive_field.name, "active");
        assert!(primitive_field.is_optional);
        assert!(primitive_field
            .field_type
            .to_string()
            .contains("BooleanType"));
        assert_eq!(
            primitive_field.doc_comment,
            Some("Field: active".to_string())
        );

        // Check the companion extension field
        let companion_field = &fields[1];
        assert_eq!(companion_field.name, "_active");
        assert!(companion_field.is_optional); // Companion fields are always optional
        assert!(companion_field.field_type.to_string().contains("Element"));
    }

    #[test]
    fn test_companion_fields_always_optional() {
        use crate::config::CodegenConfig;
        use std::collections::HashMap;

        let config = CodegenConfig::default();
        let type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);

        // Test required boolean field (min = 1) - companion should still be optional
        let required_boolean_element = ElementDefinition {
            id: Some("Patient.active".to_string()),
            path: "Patient.active".to_string(),
            short: Some("Whether this patient record is in active use".to_string()),
            definition: None,
            min: Some(1), // Required field
            max: Some("1".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("boolean".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let result = field_generator.create_fields_from_element(&required_boolean_element);
        assert!(result.is_ok());

        let fields = result.unwrap();
        assert_eq!(fields.len(), 2); // Main field + companion field

        let main_field = &fields[0];
        let companion_field = &fields[1];

        // Main field should be required (not optional) since min = 1
        assert_eq!(main_field.name, "active");
        assert!(!main_field.is_optional); // Required field

        // Companion field should ALWAYS be optional regardless of main field
        assert_eq!(companion_field.name, "_active");
        assert!(companion_field.is_optional); // Should always be optional

        // Test required string field (min = 1) - companion should still be optional
        let required_string_element = ElementDefinition {
            id: Some("Patient.name".to_string()),
            path: "Patient.name".to_string(),
            short: Some("Patient name".to_string()),
            definition: None,
            min: Some(1), // Required field
            max: Some("1".to_string()),
            element_type: Some(vec![ElementType {
                code: Some("string".to_string()),
                target_profile: None,
            }]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        let result = field_generator.create_fields_from_element(&required_string_element);
        assert!(result.is_ok());

        let fields = result.unwrap();
        assert_eq!(fields.len(), 2); // Main field + companion field

        let main_field = &fields[0];
        let companion_field = &fields[1];

        // Main field should be required (not optional) since min = 1
        assert_eq!(main_field.name, "name");
        assert!(!main_field.is_optional); // Required field

        // Companion field should ALWAYS be optional regardless of main field
        assert_eq!(companion_field.name, "_name");
        assert!(companion_field.is_optional); // Should always be optional
    }

    #[test]
    fn test_choice_type_field_generation() {
        use crate::config::CodegenConfig;
        use std::collections::HashMap;

        let config = CodegenConfig::default();
        let type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();

        let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);

        // Create a test element definition for a choice type
        let element = ElementDefinition {
            id: Some("Observation.effective[x]".to_string()),
            path: "Observation.effective[x]".to_string(),
            short: Some("Clinically relevant time/time-period for observation".to_string()),
            definition: Some(
                "The time or time-period the observed value is asserted as being true.".to_string(),
            ),
            min: Some(0),
            max: Some("1".to_string()),
            element_type: Some(vec![
                ElementType {
                    code: Some("dateTime".to_string()),
                    target_profile: None,
                },
                ElementType {
                    code: Some("Period".to_string()),
                    target_profile: None,
                },
                ElementType {
                    code: Some("Timing".to_string()),
                    target_profile: None,
                },
                ElementType {
                    code: Some("instant".to_string()),
                    target_profile: None,
                },
            ]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        // Generate fields
        let fields = field_generator
            .create_fields_from_element(&element)
            .unwrap();

        // Verify that multiple fields were generated
        assert_eq!(
            fields.len(),
            4,
            "Should generate 4 fields for 4 choice types"
        );

        // Verify field names
        let field_names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
        assert!(field_names.contains(&"effective_date_time"));
        assert!(field_names.contains(&"effective_period"));
        assert!(field_names.contains(&"effective_timing"));
        assert!(field_names.contains(&"effective_instant"));

        // Verify serde rename attributes
        assert!(fields[0]
            .serde_attributes
            .contains(&"rename = \"effectiveDateTime\"".to_string()));
        assert!(fields[1]
            .serde_attributes
            .contains(&"rename = \"effectivePeriod\"".to_string()));
        assert!(fields[2]
            .serde_attributes
            .contains(&"rename = \"effectiveTiming\"".to_string()));
        assert!(fields[3]
            .serde_attributes
            .contains(&"rename = \"effectiveInstant\"".to_string()));

        // Verify all fields are optional
        for field in &fields {
            assert!(field.is_optional, "Choice type fields should be optional");
        }
    }

    #[test]
    fn test_choice_type_documentation() {
        use crate::config::CodegenConfig;
        use std::collections::HashMap;

        let config = CodegenConfig::default();
        let type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();

        let mut field_generator = FieldGenerator::new(&config, &type_cache, &mut value_set_manager);

        // Create a test element definition for a choice type
        let element = ElementDefinition {
            id: Some("Observation.value[x]".to_string()),
            path: "Observation.value[x]".to_string(),
            short: Some("Actual result".to_string()),
            definition: Some(
                "The information determined as a result of making the observation.".to_string(),
            ),
            min: Some(0),
            max: Some("1".to_string()),
            element_type: Some(vec![
                ElementType {
                    code: Some("Quantity".to_string()),
                    target_profile: None,
                },
                ElementType {
                    code: Some("string".to_string()),
                    target_profile: None,
                },
            ]),
            fixed: None,
            pattern: None,
            binding: None,
        };

        // Generate fields
        let fields = field_generator
            .create_fields_from_element(&element)
            .unwrap();

        // Verify that documentation includes type information
        assert_eq!(fields.len(), 2);

        let quantity_field = &fields[0];
        let string_field = &fields[1];

        assert!(quantity_field
            .doc_comment
            .as_ref()
            .unwrap()
            .contains("(Quantity)"));
        assert!(string_field
            .doc_comment
            .as_ref()
            .unwrap()
            .contains("(string)"));
    }
}
