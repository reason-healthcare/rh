//! Primitive type generation functionality
//!
//! This module handles the generation of FHIR primitive types and their companion Element structs.

use std::collections::HashMap;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::DocumentationGenerator;
use crate::rust_types::{RustField, RustStruct, RustType, RustTypeAlias};
use crate::CodegenResult;

/// Primitive type generator for FHIR primitive types
pub struct PrimitiveGenerator<'a> {
    config: &'a CodegenConfig,
    type_cache: &'a mut HashMap<String, RustStruct>,
}

impl<'a> PrimitiveGenerator<'a> {
    /// Create a new primitive generator
    pub fn new(config: &'a CodegenConfig, type_cache: &'a mut HashMap<String, RustStruct>) -> Self {
        Self { config, type_cache }
    }

    /// Generate all primitive type aliases for a combined primitives.rs file
    pub fn generate_all_primitive_type_aliases(
        &self,
        primitive_structure_defs: &[StructureDefinition],
    ) -> CodegenResult<Vec<RustTypeAlias>> {
        let mut type_aliases = Vec::new();

        for structure_def in primitive_structure_defs {
            let type_alias = self.generate_primitive_type_alias(structure_def)?;
            type_aliases.push(type_alias);
        }

        Ok(type_aliases)
    }

    /// Generate a type alias for primitive types
    pub fn generate_primitive_type_alias(
        &self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustTypeAlias> {
        // Map FHIR primitive types to Rust types
        let rust_primitive_type = Self::map_fhir_primitive_to_rust_type(&structure_def.name);

        // Convert to PascalCase and add "Type" suffix
        let type_alias_name = format!(
            "{}Type",
            Self::fhir_primitive_to_pascal_case(&structure_def.name)
        );

        // Create type alias with documentation
        let mut type_alias = RustTypeAlias::new(type_alias_name, rust_primitive_type);
        let doc =
            DocumentationGenerator::generate_primitive_type_alias_documentation(structure_def);
        type_alias = type_alias.with_doc(doc);

        Ok(type_alias)
    }

    /// Convert FHIR primitive name to PascalCase, handling camelCase inputs
    fn fhir_primitive_to_pascal_case(name: &str) -> String {
        match name {
            // Handle special cases that need custom conversion
            "dateTime" => "DateTime".to_string(),
            "positiveInt" => "PositiveInt".to_string(),
            "unsignedInt" => "UnsignedInt".to_string(),
            "base64Binary" => "Base64Binary".to_string(),
            // For simple lowercase names, just capitalize
            _ => {
                let mut chars = name.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        }
    }

    /// Generate a primitive type struct with special FHIR primitive type semantics
    pub fn generate_primitive_type_struct(
        &mut self,
        structure_def: &StructureDefinition,
        mut rust_struct: RustStruct,
    ) -> CodegenResult<RustStruct> {
        // For primitive types, don't inherit from Element
        rust_struct.base_definition = None;

        // Map FHIR primitive types to Rust types
        let rust_primitive_type = Self::map_fhir_primitive_to_rust_type(&structure_def.name);

        // The primitive type is just a type alias or newtype wrapper around the Rust primitive
        // For now, we'll create a struct with a single `value` field
        let value_field = RustField::new("value".to_string(), rust_primitive_type);
        rust_struct.add_field(value_field);

        // Cache the generated struct for future use
        let struct_name = rust_struct.name.clone();
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Generate the companion Element struct for a primitive type
    pub fn generate_primitive_element_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        // Generate the Element struct name (e.g., _uri for uri primitive)
        let element_struct_name = format!("_{}", structure_def.name);

        // Check if we've already generated this Element struct
        if let Some(cached_struct) = self.type_cache.get(&element_struct_name) {
            return Ok(cached_struct.clone());
        }

        // Create the Element struct
        let mut element_struct = RustStruct::new(element_struct_name.clone());

        // Add documentation
        element_struct.doc_comment = Some(
            DocumentationGenerator::generate_primitive_element_documentation(&structure_def.name),
        );

        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        element_struct.derives = derives;

        // Set base as Element since these extend Element
        element_struct.base_definition = Some("Element".to_string());

        // Cache the generated Element struct for future use
        self.type_cache
            .insert(element_struct_name, element_struct.clone());

        Ok(element_struct)
    }

    /// Map FHIR primitive type names to Rust types
    pub fn map_fhir_primitive_to_rust_type(primitive_name: &str) -> RustType {
        match primitive_name {
            "boolean" => RustType::Boolean,
            "integer" | "positiveInt" | "unsignedInt" => RustType::Integer,
            "decimal" => RustType::Float,
            "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" | "oid"
            | "uuid" | "base64Binary" | "xhtml" => RustType::String,
            "date" | "dateTime" | "time" | "instant" => RustType::String, // Could use chrono types later
            _ => RustType::String, // Default to String for unknown primitive types
        }
    }

    /// Check if a given name represents a FHIR primitive type
    pub fn is_fhir_primitive_type(name: &str) -> bool {
        matches!(
            name,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_fhir_primitive_to_rust_type() {
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("boolean"),
            RustType::Boolean
        ));
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("integer"),
            RustType::Integer
        ));
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("decimal"),
            RustType::Float
        ));
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("string"),
            RustType::String
        ));
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("uri"),
            RustType::String
        ));
        assert!(matches!(
            PrimitiveGenerator::map_fhir_primitive_to_rust_type("unknown"),
            RustType::String
        ));
    }

    #[test]
    fn test_is_fhir_primitive_type() {
        assert!(PrimitiveGenerator::is_fhir_primitive_type("boolean"));
        assert!(PrimitiveGenerator::is_fhir_primitive_type("string"));
        assert!(PrimitiveGenerator::is_fhir_primitive_type("integer"));
        assert!(PrimitiveGenerator::is_fhir_primitive_type("decimal"));
        assert!(PrimitiveGenerator::is_fhir_primitive_type("uri"));
        assert!(PrimitiveGenerator::is_fhir_primitive_type("dateTime"));

        assert!(!PrimitiveGenerator::is_fhir_primitive_type("Patient"));
        assert!(!PrimitiveGenerator::is_fhir_primitive_type("Identifier"));
        assert!(!PrimitiveGenerator::is_fhir_primitive_type("unknown"));
    }

    #[test]
    fn test_generate_primitive_type_alias() {
        use crate::config::CodegenConfig;

        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let primitive_generator = PrimitiveGenerator::new(&config, &mut type_cache);

        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "boolean".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/boolean".to_string(),
            name: "boolean".to_string(),
            title: Some("boolean".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("Value of 'true' or 'false'".to_string()),
            purpose: None,
            base_type: "boolean".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let result = primitive_generator.generate_primitive_type_alias(&structure_def);
        assert!(result.is_ok());

        let type_alias = result.unwrap();
        assert_eq!(type_alias.name, "BooleanType");
        assert!(matches!(type_alias.target_type, RustType::Boolean));
    }

    #[test]
    fn test_primitive_type_naming_convention() {
        use crate::config::CodegenConfig;

        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let primitive_generator = PrimitiveGenerator::new(&config, &mut type_cache);

        // Test various primitive type naming conventions
        let test_cases = vec![
            ("string", "StringType"),
            ("integer", "IntegerType"),
            ("boolean", "BooleanType"),
            ("decimal", "DecimalType"),
            ("uri", "UriType"),
            ("dateTime", "DateTimeType"),
            ("time", "TimeType"),
            ("instant", "InstantType"),
            ("positiveInt", "PositiveIntType"),
        ];

        for (input_name, expected_name) in test_cases {
            let structure_def = StructureDefinition {
                resource_type: "StructureDefinition".to_string(),
                id: input_name.to_string(),
                url: format!("http://hl7.org/fhir/StructureDefinition/{input_name}"),
                name: input_name.to_string(),
                title: Some(input_name.to_string()),
                status: "active".to_string(),
                kind: "primitive-type".to_string(),
                is_abstract: false,
                description: Some(format!("FHIR {input_name} primitive type")),
                purpose: None,
                base_type: input_name.to_string(),
                base_definition: Some(
                    "http://hl7.org/fhir/StructureDefinition/Element".to_string(),
                ),
                version: None,
                differential: None,
                snapshot: None,
            };

            let result = primitive_generator.generate_primitive_type_alias(&structure_def);
            assert!(
                result.is_ok(),
                "Failed to generate type alias for {input_name}"
            );

            let type_alias = result.unwrap();
            assert_eq!(
                type_alias.name, expected_name,
                "Expected {expected_name} for {input_name}, got {}",
                type_alias.name
            );
        }
    }

    #[test]
    fn test_generate_primitive_element_struct() {
        use crate::config::CodegenConfig;

        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let mut primitive_generator = PrimitiveGenerator::new(&config, &mut type_cache);

        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "string".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/string".to_string(),
            name: "string".to_string(),
            title: Some("string".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("A sequence of Unicode characters".to_string()),
            purpose: None,
            base_type: "string".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let result = primitive_generator.generate_primitive_element_struct(&structure_def);
        assert!(result.is_ok());

        let element_struct = result.unwrap();
        assert_eq!(element_struct.name, "_string");
        assert_eq!(element_struct.base_definition, Some("Element".to_string()));
        assert!(element_struct.derives.contains(&"Debug".to_string()));
        assert!(element_struct.derives.contains(&"Clone".to_string()));
    }
}
