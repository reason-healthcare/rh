//! Struct generation functionality
//!
//! This module handles the generation of Rust structs from FHIR StructureDefinitions.

use std::collections::HashMap;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::{utils::GeneratorUtils, DocumentationGenerator, FieldGenerator};
use crate::rust_types::{RustField, RustStruct, RustType};
use crate::value_sets::ValueSetManager;
use crate::{CodegenError, CodegenResult};

/// Struct generator for FHIR StructureDefinitions
pub struct StructGenerator<'a> {
    config: &'a CodegenConfig,
    type_cache: &'a mut HashMap<String, RustStruct>,
    value_set_manager: &'a mut ValueSetManager,
}

impl<'a> StructGenerator<'a> {
    /// Create a new struct generator
    pub fn new(
        config: &'a CodegenConfig,
        type_cache: &'a mut HashMap<String, RustStruct>,
        value_set_manager: &'a mut ValueSetManager,
    ) -> Self {
        Self {
            config,
            type_cache,
            value_set_manager,
        }
    }

    /// Generate a Rust struct from a FHIR StructureDefinition
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        // Skip LogicalModels as they are conceptual models, not implementable types
        if structure_def.kind == "logical" {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping LogicalModel '{}' - logical models are not generated as Rust types",
                    structure_def.name
                ),
            });
        }

        // Skip examples
        if structure_def.url.to_lowercase().contains("example") {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping example StructureDefinition '{}'",
                    structure_def.name
                ),
            });
        }

        // Generate struct name from URL or ID, not the name field
        let struct_name = GeneratorUtils::generate_struct_name(structure_def);

        // Check if we've already generated this type
        if let Some(cached_struct) = self.type_cache.get(&struct_name) {
            return Ok(cached_struct.clone());
        }

        // Create the struct with enhanced documentation
        let mut rust_struct = RustStruct::new(struct_name.clone());
        rust_struct.doc_comment =
            DocumentationGenerator::generate_struct_documentation(structure_def);

        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        rust_struct.derives = derives;

        // Set the base definition for inheritance if present
        if let Some(base_def) = &structure_def.base_definition {
            rust_struct.base_definition = Some(
                base_def
                    .split('/')
                    .next_back()
                    .unwrap_or(base_def)
                    .to_string(),
            );
        }

        // Handle primitive types specially
        if structure_def.kind == "primitive-type" {
            return self.generate_primitive_type_struct(structure_def, rust_struct);
        }

        // Extract element definitions from differential only (preferred FHIR approach)
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return Ok(rust_struct); // No elements to process
        };

        // Process each element definition to create struct fields and nested structs
        let mut nested_structs_info = HashMap::new();
        let mut direct_fields = Vec::new();

        for element in elements {
            // Skip the root element
            if element.path == structure_def.name || element.path == structure_def.base_type {
                continue;
            }

            // Only process elements that belong to this struct
            let base_path = &structure_def.name;
            if !element.path.starts_with(&format!("{base_path}.")) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();

            if field_path.contains('.') {
                // This is a nested field - collect it for nested struct generation
                let nested_field_name = field_path.split('.').next().unwrap();
                nested_structs_info
                    .entry(nested_field_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            } else {
                // This is a direct field of this struct
                direct_fields.push(element.clone());
            }
        }

        // First pass: Generate nested structs for BackboneElements
        for (nested_field_name, nested_elements) in &nested_structs_info {
            if let Some(nested_struct) = self.generate_nested_struct(
                &struct_name,
                nested_field_name,
                nested_elements,
                structure_def,
            )? {
                // Store the nested struct in cache for later use
                self.type_cache
                    .insert(nested_struct.name.clone(), nested_struct);
            }
        }

        // Second pass: Process direct fields (now nested structs are available)
        for element in direct_fields {
            if let Some(field) = self.create_field_from_element(&element)? {
                rust_struct.add_field(field);
            }
        }

        // Cache the generated struct for future use
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
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
        let rust_primitive_type = match structure_def.name.as_str() {
            "boolean" => RustType::Boolean,
            "integer" | "positiveInt" | "unsignedInt" => RustType::Integer,
            "decimal" => RustType::Float,
            "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" | "oid"
            | "uuid" | "base64Binary" | "xhtml" => RustType::String,
            "date" | "dateTime" | "time" | "instant" => RustType::String, // Could use chrono types later
            _ => RustType::String, // Default to String for unknown primitive types
        };

        // The primitive type is just a type alias or newtype wrapper around the Rust primitive
        // For now, we'll create a struct with a single `value` field
        let value_field = RustField::new("value".to_string(), rust_primitive_type);
        rust_struct.add_field(value_field);

        // Cache the generated struct for future use
        let struct_name = rust_struct.name.clone();
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Generate a nested struct for BackboneElements
    pub fn generate_nested_struct(
        &mut self,
        parent_struct_name: &str,
        nested_field_name: &str,
        nested_elements: &[crate::fhir_types::ElementDefinition],
        parent_structure_def: &StructureDefinition,
    ) -> CodegenResult<Option<RustStruct>> {
        // Generate the nested struct name (e.g., BundleEntry, BundleLink)
        let nested_struct_name = format!(
            "{}{}",
            parent_struct_name,
            GeneratorUtils::to_pascal_case(nested_field_name)
        );

        // Check if we've already generated this nested struct
        if self.type_cache.contains_key(&nested_struct_name) {
            return Ok(None);
        }

        // Create the nested struct
        let mut nested_struct = RustStruct::new(nested_struct_name.clone());

        // Add documentation
        nested_struct.doc_comment = Some(
            DocumentationGenerator::generate_nested_struct_documentation(
                parent_struct_name,
                nested_field_name,
            ),
        );

        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        nested_struct.derives = derives;

        // Set base as BackboneElement since these are typically BackboneElements
        nested_struct.base_definition = Some("BackboneElement".to_string());

        // Process the nested elements
        let base_path = format!("{}.{}", parent_structure_def.name, nested_field_name);
        let mut sub_nested_structs = HashMap::new();
        let mut direct_fields = Vec::new();

        for element in nested_elements {
            if !element.path.starts_with(&base_path) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();

            if field_path.contains('.') {
                // This is a sub-nested field - collect it for recursive nested struct generation
                let sub_nested_field_name = field_path.split('.').next().unwrap();
                sub_nested_structs
                    .entry(sub_nested_field_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            } else {
                // This is a direct field of this nested struct
                direct_fields.push(element.clone());
            }
        }

        // First, generate any sub-nested structs
        for (sub_nested_field_name, sub_nested_elements) in &sub_nested_structs {
            // For recursive calls, we need to create a modified context
            // The base path for sub-nested structs should be the current nested struct's path
            let sub_nested_struct_name = format!(
                "{}{}",
                nested_struct_name,
                GeneratorUtils::to_pascal_case(sub_nested_field_name)
            );

            if !self.type_cache.contains_key(&sub_nested_struct_name) {
                let mut sub_nested_struct = RustStruct::new(sub_nested_struct_name.clone());

                sub_nested_struct.doc_comment = Some(
                    DocumentationGenerator::generate_sub_nested_struct_documentation(
                        &nested_struct_name,
                        sub_nested_field_name,
                    ),
                );

                // Use config to determine derives
                let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
                if self.config.with_serde {
                    derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
                }
                sub_nested_struct.derives = derives;
                sub_nested_struct.base_definition = Some("BackboneElement".to_string());

                // Process the sub-nested elements
                let sub_base_path = format!("{base_path}.{sub_nested_field_name}");

                for element in sub_nested_elements {
                    if !element.path.starts_with(&sub_base_path) {
                        continue;
                    }

                    let sub_field_path = element
                        .path
                        .strip_prefix(&format!("{sub_base_path}."))
                        .unwrap();

                    // Only process direct fields (no further nesting for now)
                    if !sub_field_path.contains('.') {
                        if let Some(field) = self.create_field_from_element(element)? {
                            sub_nested_struct.add_field(field);
                        }
                    }
                }

                // Store the sub-nested struct in cache
                self.type_cache
                    .insert(sub_nested_struct_name, sub_nested_struct);
            }
        }

        // Then, process direct fields (now sub-nested structs are available)
        for element in direct_fields {
            if let Some(field) = self.create_field_from_element(&element)? {
                nested_struct.add_field(field);
            }
        }

        Ok(Some(nested_struct))
    }

    /// Check if a field should use a nested struct type instead of BackboneElement
    pub fn should_use_nested_struct_type(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        element_types: &[crate::fhir_types::ElementType],
    ) -> bool {
        // Check if this element is a BackboneElement and we have nested elements for it
        if let Some(first_type) = element_types.first() {
            if let Some(code) = &first_type.code {
                if code == "BackboneElement" {
                    // Extract parent struct name and field name from the path
                    let path_parts: Vec<&str> = element.path.split('.').collect();
                    if path_parts.len() >= 2 {
                        let _parent_name = path_parts[0];
                        let _field_name = path_parts[1];
                        // We would check here if we have nested elements for this field
                        // For now, always return true for BackboneElements
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Create a RustField from an ElementDefinition
    pub fn create_field_from_element(
        &mut self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> CodegenResult<Option<RustField>> {
        let mut field_generator =
            FieldGenerator::new(self.config, self.type_cache, self.value_set_manager);
        field_generator.create_field_from_element(element)
    }
}
