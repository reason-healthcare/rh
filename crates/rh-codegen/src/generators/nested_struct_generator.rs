//! Nested struct generation functionality for FHIR BackboneElements
//!
//! This module provides functionality for generating nested structs from FHIR BackboneElements,
//! which are complex structures that appear within other FHIR structures.

use std::collections::HashMap;

use crate::config::CodegenConfig;
use crate::fhir_types::{ElementDefinition, StructureDefinition};
use crate::generators::{DocumentationGenerator, FieldGenerator};
use crate::naming::Naming;
use crate::rust_types::RustStruct;
use crate::CodegenResult;

/// Generator for nested struct types from FHIR BackboneElements
pub struct NestedStructGenerator<'a> {
    config: &'a CodegenConfig,
    type_cache: &'a mut HashMap<String, RustStruct>,
    value_set_manager: &'a mut crate::value_sets::ValueSetManager,
}

impl<'a> NestedStructGenerator<'a> {
    /// Create a new nested struct generator
    pub fn new(
        config: &'a CodegenConfig,
        type_cache: &'a mut HashMap<String, RustStruct>,
        value_set_manager: &'a mut crate::value_sets::ValueSetManager,
    ) -> Self {
        Self {
            config,
            type_cache,
            value_set_manager,
        }
    }

    /// Generate a nested struct for BackboneElements
    pub fn generate_nested_struct(
        &mut self,
        parent_struct_name: &str,
        nested_field_name: &str,
        nested_elements: &[ElementDefinition],
        parent_structure_def: &StructureDefinition,
    ) -> CodegenResult<Option<RustStruct>> {
        // Generate the nested struct name (e.g., BundleEntry, BundleLink)
        let nested_struct_name = format!(
            "{}{}",
            parent_struct_name,
            Naming::to_pascal_case(nested_field_name)
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
            self.generate_sub_nested_struct(
                &nested_struct_name,
                sub_nested_field_name,
                sub_nested_elements,
                &base_path,
            )?;
        }

        // Then, process direct fields (now sub-nested structs are available)
        for element in direct_fields {
            if let Some(field) = self.create_field_from_element(&element)? {
                nested_struct.add_field(field);
            }
        }

        // Store the nested struct in cache before returning
        self.type_cache
            .insert(nested_struct_name.clone(), nested_struct.clone());

        Ok(Some(nested_struct))
    }

    /// Generate a sub-nested struct (nested within a nested struct)
    fn generate_sub_nested_struct(
        &mut self,
        nested_struct_name: &str,
        sub_nested_field_name: &str,
        sub_nested_elements: &[ElementDefinition],
        base_path: &str,
    ) -> CodegenResult<()> {
        // For recursive calls, we need to create a modified context
        // The base path for sub-nested structs should be the current nested struct's path
        let sub_nested_struct_name = format!(
            "{}{}",
            nested_struct_name,
            Naming::to_pascal_case(sub_nested_field_name)
        );

        if !self.type_cache.contains_key(&sub_nested_struct_name) {
            let mut sub_nested_struct = RustStruct::new(sub_nested_struct_name.clone());

            sub_nested_struct.doc_comment = Some(
                DocumentationGenerator::generate_sub_nested_struct_documentation(
                    nested_struct_name,
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

                let field_path = element
                    .path
                    .strip_prefix(&format!("{sub_base_path}."))
                    .unwrap();

                // Only process direct fields (no further recursion for now - can be extended)
                if !field_path.contains('.') {
                    if let Some(field) = self.create_field_from_element(element)? {
                        sub_nested_struct.add_field(field);
                    }
                }
            }

            // Store the sub-nested struct in cache
            self.type_cache
                .insert(sub_nested_struct_name, sub_nested_struct);
        }

        Ok(())
    }

    /// Create a RustField from an ElementDefinition
    fn create_field_from_element(
        &mut self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<crate::rust_types::RustField>> {
        let mut field_generator =
            FieldGenerator::new(self.config, self.type_cache, self.value_set_manager);
        field_generator.create_field_from_element(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CodegenConfig;
    use crate::fhir_types::{
        ElementDefinition, ElementType, StructureDefinition, StructureDefinitionDifferential,
    };
    use crate::value_sets::ValueSetManager;

    #[test]
    fn test_nested_struct_generation() {
        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut generator =
            NestedStructGenerator::new(&config, &mut type_cache, &mut value_set_manager);

        // Create a simplified Bundle StructureDefinition with nested entry
        let bundle_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Bundle".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Bundle".to_string(),
            name: "Bundle".to_string(),
            title: Some("Bundle".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A container for a collection of resources".to_string()),
            purpose: None,
            base_type: "Bundle".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
            version: None,
            differential: Some(StructureDefinitionDifferential {
                element: vec![
                    ElementDefinition {
                        id: Some("Bundle.entry".to_string()),
                        path: "Bundle.entry".to_string(),
                        short: Some("Entry in the bundle".to_string()),
                        definition: None,
                        min: Some(0),
                        max: Some("*".to_string()),
                        element_type: Some(vec![ElementType {
                            code: Some("BackboneElement".to_string()),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                        binding: None,
                    },
                    ElementDefinition {
                        id: Some("Bundle.entry.resource".to_string()),
                        path: "Bundle.entry.resource".to_string(),
                        short: Some("A resource in the bundle".to_string()),
                        definition: None,
                        min: Some(0),
                        max: Some("1".to_string()),
                        element_type: Some(vec![ElementType {
                            code: Some("Resource".to_string()),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                        binding: None,
                    },
                ],
            }),
            snapshot: None,
        };

        let nested_elements = bundle_structure
            .differential
            .as_ref()
            .unwrap()
            .element
            .clone();

        // Filter to only include the sub-elements of Bundle.entry (not Bundle.entry itself)
        let filtered_elements: Vec<_> = nested_elements
            .into_iter()
            .filter(|e| e.path.starts_with("Bundle.entry."))
            .collect();

        // Generate the nested struct
        let result = generator.generate_nested_struct(
            "Bundle",
            "entry",
            &filtered_elements,
            &bundle_structure,
        );
        assert!(result.is_ok(), "Should generate nested struct successfully");

        let bundle_entry_struct = result.unwrap();
        assert!(
            bundle_entry_struct.is_some(),
            "Should return a nested struct"
        );

        let bundle_entry_struct = bundle_entry_struct.unwrap();
        assert_eq!(bundle_entry_struct.name, "BundleEntry");
        assert_eq!(
            bundle_entry_struct.base_definition,
            Some("BackboneElement".to_string())
        );

        // Check that the struct was added to the type cache
        assert!(
            type_cache.contains_key("BundleEntry"),
            "BundleEntry should be cached"
        );

        // Check that the nested struct has the expected fields
        let resource_field = bundle_entry_struct
            .fields
            .iter()
            .find(|f| f.name == "resource");
        assert!(
            resource_field.is_some(),
            "BundleEntry should have a resource field"
        );
    }

    #[test]
    fn test_nested_struct_caching() {
        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();

        // Create a dummy nested struct and add it to cache
        let existing_struct = RustStruct::new("BundleEntry".to_string());
        type_cache.insert("BundleEntry".to_string(), existing_struct);

        let mut generator =
            NestedStructGenerator::new(&config, &mut type_cache, &mut value_set_manager);

        let bundle_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Bundle".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Bundle".to_string(),
            name: "Bundle".to_string(),
            title: Some("Bundle".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A container for a collection of resources".to_string()),
            purpose: None,
            base_type: "Bundle".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let nested_elements = vec![];

        // Generate the nested struct - should return None due to caching
        let result = generator.generate_nested_struct(
            "Bundle",
            "entry",
            &nested_elements,
            &bundle_structure,
        );
        assert!(result.is_ok(), "Should handle cached struct successfully");

        let bundle_entry_struct = result.unwrap();
        assert!(
            bundle_entry_struct.is_none(),
            "Should return None for cached struct"
        );
    }

    #[test]
    fn test_nested_struct_documentation() {
        let config = CodegenConfig::default();
        let mut type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut generator =
            NestedStructGenerator::new(&config, &mut type_cache, &mut value_set_manager);

        let bundle_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Bundle".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Bundle".to_string(),
            name: "Bundle".to_string(),
            title: Some("Bundle".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A container for a collection of resources".to_string()),
            purpose: None,
            base_type: "Bundle".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let nested_elements = vec![];

        // Generate the nested struct
        let result = generator.generate_nested_struct(
            "Bundle",
            "entry",
            &nested_elements,
            &bundle_structure,
        );
        assert!(result.is_ok(), "Should generate nested struct successfully");

        let bundle_entry_struct = result.unwrap().unwrap();

        // Check documentation was generated
        assert!(
            bundle_entry_struct.doc_comment.is_some(),
            "Should have documentation"
        );
        let doc = bundle_entry_struct.doc_comment.unwrap();
        assert!(
            doc.contains("Bundle"),
            "Documentation should mention parent struct"
        );
        assert!(
            doc.contains("entry"),
            "Documentation should mention field name"
        );
    }

    #[test]
    fn test_nested_struct_derives() {
        let config = CodegenConfig {
            with_serde: true,
            ..Default::default()
        };
        let mut type_cache = HashMap::new();
        let mut value_set_manager = ValueSetManager::new();
        let mut generator =
            NestedStructGenerator::new(&config, &mut type_cache, &mut value_set_manager);

        let bundle_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Bundle".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Bundle".to_string(),
            name: "Bundle".to_string(),
            title: Some("Bundle".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A container for a collection of resources".to_string()),
            purpose: None,
            base_type: "Bundle".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let nested_elements = vec![];

        // Generate the nested struct
        let result = generator.generate_nested_struct(
            "Bundle",
            "entry",
            &nested_elements,
            &bundle_structure,
        );
        assert!(result.is_ok(), "Should generate nested struct successfully");

        let bundle_entry_struct = result.unwrap().unwrap();

        // Check derives include serde when enabled
        assert!(bundle_entry_struct.derives.contains(&"Debug".to_string()));
        assert!(bundle_entry_struct.derives.contains(&"Clone".to_string()));
        assert!(bundle_entry_struct
            .derives
            .contains(&"Serialize".to_string()));
        assert!(bundle_entry_struct
            .derives
            .contains(&"Deserialize".to_string()));
    }
}
