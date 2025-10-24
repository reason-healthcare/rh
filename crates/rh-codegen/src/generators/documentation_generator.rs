//! Documentation generation utilities for FHIR types
//!
//! This module contains utilities for generating documentation comments and descriptions
//! for FHIR structs, fields, traits, and other code elements.

use crate::fhir_types::{ElementDefinition, StructureDefinition};
use crate::value_sets::ValueSetManager;

/// Generator for documentation and doc comments
pub struct DocumentationGenerator;

impl DocumentationGenerator {
    /// Create a new documentation generator
    pub fn new() -> Self {
        Self
    }

    /// Generate comprehensive documentation for a struct based on StructureDefinition metadata
    pub fn generate_struct_documentation(structure_def: &StructureDefinition) -> Option<String> {
        let mut doc_lines = Vec::new();

        // Add title if available, otherwise use name
        if let Some(title) = &structure_def.title {
            doc_lines.push(title.clone());
        } else {
            doc_lines.push(structure_def.name.clone());
        }

        // Add description if available
        if let Some(description) = &structure_def.description {
            doc_lines.push("".to_string());
            // Clean up the description by removing carriage returns and other problematic characters
            let cleaned_description = description.replace('\r', "").replace('\n', " ");
            doc_lines.push(cleaned_description);
        }

        // Add source information
        doc_lines.push("".to_string());
        doc_lines.push("**Source:**".to_string());
        doc_lines.push(format!("- URL: {url}", url = structure_def.url));

        if let Some(version) = &structure_def.version {
            doc_lines.push(format!("- Version: {version}"));
        }

        doc_lines.push(format!("- Kind: {kind}", kind = structure_def.kind));
        doc_lines.push(format!(
            "- Type: {base_type}",
            base_type = structure_def.base_type
        ));

        if let Some(base_def) = &structure_def.base_definition {
            doc_lines.push(format!("- Base Definition: {base_def}"));
        }

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }

    /// Generate documentation for a field based on ElementDefinition
    pub fn generate_field_documentation(element: &ElementDefinition) -> Option<String> {
        let mut doc_parts = Vec::new();

        // Add basic field description
        if let Some(short) = &element.short {
            doc_parts.push(short.clone());
        } else if let Some(definition) = &element.definition {
            doc_parts.push(definition.clone());
        }

        // Add binding information for non-required bindings
        if let Some(binding) = &element.binding {
            if binding.strength != "required" {
                // Add binding strength information
                doc_parts.push(format!(
                    "Binding: {} ({})",
                    binding.strength,
                    binding.description.as_deref().unwrap_or("No description")
                ));

                // Note: We'll add available values here when we have access to ValueSetManager
            }
        }

        if doc_parts.is_empty() {
            None
        } else {
            Some(doc_parts.join("\n\n"))
        }
    }

    /// Generate enhanced field documentation with ValueSet information
    pub fn generate_field_documentation_with_binding(
        element: &ElementDefinition,
        value_set_manager: &ValueSetManager,
    ) -> Option<String> {
        let mut doc_parts = Vec::new();

        // Add basic field description
        if let Some(short) = &element.short {
            doc_parts.push(short.clone());
        } else if let Some(definition) = &element.definition {
            doc_parts.push(definition.clone());
        }

        // Add binding information for non-required bindings
        if let Some(binding) = &element.binding {
            if binding.strength != "required" {
                // Add binding strength information
                doc_parts.push(format!(
                    "Binding: {} ({})",
                    binding.strength,
                    binding.description.as_deref().unwrap_or("No description")
                ));

                // Add available values from ValueSet
                if let Some(value_set_url) = &binding.value_set {
                    // Parse ValueSet URL to remove version if present
                    let url = if let Some(version_pos) = value_set_url.find('|') {
                        &value_set_url[..version_pos]
                    } else {
                        value_set_url
                    };

                    match value_set_manager.get_value_set_codes(url, None) {
                        Ok(codes) => {
                            if !codes.is_empty() {
                                let mut values_doc = String::from("Available values:");
                                for (code, display) in codes.iter().take(10) {
                                    // Limit to first 10 to avoid huge docs
                                    if let Some(display) = display {
                                        values_doc.push_str(&format!("\n- `{code}`: {display}"));
                                    } else {
                                        values_doc.push_str(&format!("\n- `{code}`"));
                                    }
                                }
                                if codes.len() > 10 {
                                    values_doc.push_str(&format!(
                                        "\n- ... and {} more values",
                                        codes.len() - 10
                                    ));
                                }
                                doc_parts.push(values_doc);
                            }
                        }
                        Err(_) => {
                            // ValueSet not found, just add the URL reference
                            doc_parts.push(format!("ValueSet: {value_set_url}"));
                        }
                    }
                }
            }
        }

        if doc_parts.is_empty() {
            None
        } else {
            Some(doc_parts.join("\n\n"))
        }
    }

    /// Generate documentation for a choice type field
    pub fn generate_choice_field_documentation(
        element: &ElementDefinition,
        type_code: &str,
    ) -> Option<String> {
        // Create documentation that indicates this is a specific type variant of a choice field
        let base_doc = if let Some(short) = &element.short {
            short.clone()
        } else if let Some(definition) = &element.definition {
            definition.clone()
        } else {
            "Choice type field".to_string()
        };

        // Add type-specific suffix
        Some(format!("{base_doc} ({type_code})"))
    }

    /// Generate enhanced choice field documentation with ValueSet information
    pub fn generate_choice_field_documentation_with_binding(
        element: &ElementDefinition,
        type_code: &str,
        value_set_manager: &ValueSetManager,
    ) -> Option<String> {
        let mut doc_parts = Vec::new();

        // Create base documentation that indicates this is a specific type variant of a choice field
        let base_doc = if let Some(short) = &element.short {
            short.clone()
        } else if let Some(definition) = &element.definition {
            definition.clone()
        } else {
            "Choice type field".to_string()
        };

        // Add type-specific suffix
        doc_parts.push(format!("{base_doc} ({type_code})"));

        // Add binding information for code type choice fields with non-required bindings
        if type_code == "code" {
            if let Some(binding) = &element.binding {
                if binding.strength != "required" {
                    // Add binding strength information
                    doc_parts.push(format!(
                        "Binding: {} ({})",
                        binding.strength,
                        binding.description.as_deref().unwrap_or("No description")
                    ));

                    // Add available values from ValueSet
                    if let Some(value_set_url) = &binding.value_set {
                        // Parse ValueSet URL to remove version if present
                        let url = if let Some(version_pos) = value_set_url.find('|') {
                            &value_set_url[..version_pos]
                        } else {
                            value_set_url
                        };

                        match value_set_manager.get_value_set_codes(url, None) {
                            Ok(codes) => {
                                if !codes.is_empty() {
                                    let mut values_doc = String::from("Available values:");
                                    for (code, display) in codes.iter().take(10) {
                                        // Limit to first 10 to avoid huge docs
                                        if let Some(display) = display {
                                            values_doc
                                                .push_str(&format!("\n- `{code}`: {display}"));
                                        } else {
                                            values_doc.push_str(&format!("\n- `{code}`"));
                                        }
                                    }
                                    if codes.len() > 10 {
                                        values_doc.push_str(&format!(
                                            "\n- ... and {} more values",
                                            codes.len() - 10
                                        ));
                                    }
                                    doc_parts.push(values_doc);
                                }
                            }
                            Err(_) => {
                                // ValueSet not found, just add the URL reference
                                doc_parts.push(format!("ValueSet: {value_set_url}"));
                            }
                        }
                    }
                }
            }
        }

        if doc_parts.is_empty() {
            None
        } else {
            Some(doc_parts.join("\n\n"))
        }
    }

    /// Generate documentation for a primitive element struct
    pub fn generate_primitive_element_documentation(primitive_name: &str) -> String {
        format!(
            "Element structure for the '{primitive_name}' primitive type. Contains metadata and extensions."
        )
    }

    /// Generate documentation for a nested struct
    pub fn generate_nested_struct_documentation(
        parent_struct_name: &str,
        nested_field_name: &str,
    ) -> String {
        format!("{parent_struct_name} nested structure for the '{nested_field_name}' field")
    }

    /// Generate documentation for a sub-nested struct
    pub fn generate_sub_nested_struct_documentation(
        nested_struct_name: &str,
        sub_nested_field_name: &str,
    ) -> String {
        format!("{nested_struct_name} nested structure for the '{sub_nested_field_name}' field")
    }

    /// Generate documentation for primitive type aliases
    pub fn generate_primitive_type_alias_documentation(
        structure_def: &StructureDefinition,
    ) -> String {
        if let Some(description) = &structure_def.description {
            description.clone()
        } else {
            format!("FHIR primitive type: {name}", name = structure_def.name)
        }
    }

    /// Clean description text by removing problematic characters
    pub fn clean_description(description: &str) -> String {
        description.replace('\r', "").replace('\n', " ")
    }

    /// Generate standard FHIR source information block
    pub fn generate_source_info_block(structure_def: &StructureDefinition) -> Vec<String> {
        let mut lines = vec![
            "".to_string(),
            "**Source:**".to_string(),
            format!("- URL: {url}", url = structure_def.url),
        ];

        if let Some(version) = &structure_def.version {
            lines.push(format!("- Version: {version}"));
        }

        lines.push(format!("- Kind: {kind}", kind = structure_def.kind));
        lines.push(format!(
            "- Type: {base_type}",
            base_type = structure_def.base_type
        ));

        if let Some(base_def) = &structure_def.base_definition {
            lines.push(format!("- Base Definition: {base_def}"));
        }

        lines
    }

    /// Generate documentation for a trait based on StructureDefinition metadata
    pub fn generate_trait_documentation(structure_def: &StructureDefinition) -> Option<String> {
        let mut doc_lines = Vec::new();

        // Add title if available, otherwise use name
        let title = if let Some(title) = &structure_def.title {
            format!("{title} Trait")
        } else {
            format!("{name} Trait", name = structure_def.name)
        };
        doc_lines.push(title);

        // Add description if available
        if let Some(description) = &structure_def.description {
            doc_lines.push("".to_string());
            doc_lines.push(
                "This trait provides common functionality and access patterns for this FHIR resource type."
                    .to_string(),
            );
            doc_lines.push("".to_string());
            // Clean up the description by removing carriage returns and other problematic characters
            let cleaned_description = Self::clean_description(description);
            doc_lines.push(cleaned_description);
        } else {
            doc_lines.push("".to_string());
            doc_lines.push(
                "This trait provides common functionality and access patterns for this FHIR resource type."
                    .to_string(),
            );
        }

        // Add source information
        let source_info = Self::generate_source_info_block(structure_def);
        doc_lines.extend(source_info);

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }
}

impl Default for DocumentationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_documentation_generation() {
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient Resource".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("Demographics and other administrative information about an individual receiving care.".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            version: Some("4.0.1".to_string()),
            differential: None,
            snapshot: None,
        };

        let doc = DocumentationGenerator::generate_struct_documentation(&structure_def);
        assert!(doc.is_some());

        let doc_text = doc.unwrap();
        assert!(doc_text.contains("Patient Resource"));
        assert!(doc_text.contains("Demographics and other administrative information"));
        assert!(doc_text.contains("**Source:**"));
        assert!(doc_text.contains("http://hl7.org/fhir/StructureDefinition/Patient"));
        assert!(doc_text.contains("Version: 4.0.1"));
        assert!(doc_text.contains("Kind: resource"));
    }

    #[test]
    fn test_field_documentation_generation() {
        use crate::fhir_types::ElementDefinition;

        let element = ElementDefinition {
            id: Some("Patient.active".to_string()),
            path: "Patient.active".to_string(),
            short: Some("Whether this patient record is in active use".to_string()),
            definition: Some(
                "Whether this patient record is in active use. Many systems...".to_string(),
            ),
            min: Some(0),
            max: Some("1".to_string()),
            element_type: None,
            fixed: None,
            pattern: None,
            binding: None,
            constraint: None,
        };

        let doc = DocumentationGenerator::generate_field_documentation(&element);
        assert!(doc.is_some());
        assert_eq!(doc.unwrap(), "Whether this patient record is in active use");
    }

    #[test]
    fn test_primitive_element_documentation() {
        let doc = DocumentationGenerator::generate_primitive_element_documentation("uri");
        assert_eq!(
            doc,
            "Element structure for the 'uri' primitive type. Contains metadata and extensions."
        );
    }

    #[test]
    fn test_nested_struct_documentation() {
        let doc = DocumentationGenerator::generate_nested_struct_documentation("Bundle", "entry");
        assert_eq!(doc, "Bundle nested structure for the 'entry' field");
    }

    #[test]
    fn test_clean_description() {
        let dirty_description = "This is a test\r\nwith carriage returns\rand newlines\n.";
        let clean = DocumentationGenerator::clean_description(dirty_description);
        assert_eq!(clean, "This is a test with carriage returnsand newlines .");
    }
}
