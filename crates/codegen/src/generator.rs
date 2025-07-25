//! FHIR type generation functionality
//!
//! This module contains the core logic for generating Rust types from FHIR StructureDefinitions.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::{parse_quote, Type};

use crate::{CodegenError, CodegenResult};
use common::Config;

/// Configuration for the code generator
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodegenConfig {
    /// Output directory for generated files
    pub output_dir: String,
    /// Module name for generated types
    pub module_name: String,
    /// Whether to generate serde annotations
    pub with_serde: bool,
    /// Whether to generate documentation
    pub with_docs: bool,
    /// Custom type mappings from FHIR to Rust types
    pub type_mappings: HashMap<String, String>,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        let mut type_mappings = HashMap::new();

        // Common FHIR to Rust type mappings
        type_mappings.insert("string".to_string(), "String".to_string());
        type_mappings.insert("integer".to_string(), "i32".to_string());
        type_mappings.insert("boolean".to_string(), "bool".to_string());
        type_mappings.insert("decimal".to_string(), "f64".to_string());
        type_mappings.insert("uri".to_string(), "String".to_string());
        type_mappings.insert("url".to_string(), "String".to_string());
        type_mappings.insert("canonical".to_string(), "String".to_string());
        type_mappings.insert("code".to_string(), "String".to_string());
        type_mappings.insert("oid".to_string(), "String".to_string());
        type_mappings.insert("id".to_string(), "String".to_string());
        type_mappings.insert("markdown".to_string(), "String".to_string());
        type_mappings.insert("base64Binary".to_string(), "String".to_string());
        type_mappings.insert("instant".to_string(), "String".to_string()); // Could be chrono::DateTime
        type_mappings.insert("date".to_string(), "String".to_string()); // Could be chrono::NaiveDate
        type_mappings.insert("dateTime".to_string(), "String".to_string()); // Could be chrono::DateTime
        type_mappings.insert("time".to_string(), "String".to_string()); // Could be chrono::NaiveTime

        Self {
            output_dir: "src/generated".to_string(),
            module_name: "fhir_types".to_string(),
            with_serde: true,
            with_docs: true,
            type_mappings,
        }
    }
}

impl Config for CodegenConfig {}

/// Represents a FHIR StructureDefinition element
#[derive(Debug, Deserialize, Clone)]
pub struct ElementDefinition {
    pub id: Option<String>,
    pub path: String,
    pub short: Option<String>,
    pub definition: Option<String>,
    pub min: Option<u32>,
    pub max: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<Vec<ElementType>>,
    pub fixed: Option<serde_json::Value>,
    pub pattern: Option<serde_json::Value>,
}

/// Represents a FHIR element type
#[derive(Debug, Deserialize, Clone)]
pub struct ElementType {
    pub code: String,
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,
}

/// Represents a FHIR StructureDefinition
#[derive(Debug, Deserialize)]
pub struct StructureDefinition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub url: String,
    pub name: String,
    pub title: Option<String>,
    pub status: String,
    pub kind: String,
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
    #[serde(rename = "type")]
    pub base_type: String,
    #[serde(rename = "baseDefinition")]
    pub base_definition: Option<String>,
    pub differential: Option<StructureDefinitionDifferential>,
    pub snapshot: Option<StructureDefinitionSnapshot>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionDifferential {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionSnapshot {
    pub element: Vec<ElementDefinition>,
}

/// Represents a generated Rust field
#[derive(Debug, Clone)]
pub struct RustField {
    pub name: String,
    pub rust_type: String,
    pub is_optional: bool,
    pub is_array: bool,
    pub documentation: Option<String>,
    pub serde_rename: Option<String>,
}

/// Represents a generated Rust struct
#[derive(Debug, Clone)]
pub struct RustStruct {
    pub name: String,
    pub fields: Vec<RustField>,
    pub documentation: Option<String>,
    pub derives: Vec<String>,
}

/// Main code generator struct
pub struct CodeGenerator {
    config: CodegenConfig,
    /// Cache of previously generated types to avoid regenerating the same struct
    type_cache: HashMap<String, RustStruct>,
}

impl CodeGenerator {
    /// Create a new code generator with the given configuration
    pub fn new(config: CodegenConfig) -> Self {
        Self {
            config,
            type_cache: HashMap::new(),
        }
    }

    /// Clear the type cache
    ///
    /// This is useful when generating types for different FHIR packages that might
    /// have conflicting type definitions.
    pub fn clear_cache(&mut self) {
        self.type_cache.clear();
    }

    /// Load and parse a FHIR StructureDefinition from a JSON file
    pub fn load_structure_definition<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> CodegenResult<StructureDefinition> {
        let content = fs::read_to_string(&path).map_err(|e| CodegenError::Io(e))?;

        let structure_def: StructureDefinition =
            serde_json::from_str(&content).map_err(|e| CodegenError::Json(e))?;

        Ok(structure_def)
    }

    /// Generate a Rust struct from a FHIR StructureDefinition
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        let struct_name = self.to_rust_type_name(&structure_def.name);

        // Check if we've already generated this type
        if let Some(cached_struct) = self.type_cache.get(&struct_name) {
            return Ok(cached_struct.clone());
        }

        // Get elements from snapshot or differential
        let elements = if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else if let Some(differential) = &structure_def.differential {
            &differential.element
        } else {
            return Err(CodegenError::MissingField {
                field: "snapshot or differential".to_string(),
            });
        };

        let mut fields = Vec::new();
        let mut processed_paths = std::collections::HashSet::new();

        for element in elements {
            // Skip the root element (e.g., "Patient")
            if element.path == structure_def.base_type {
                continue;
            }

            // Extract field name from path (e.g., "Patient.name" -> "name")
            let field_parts: Vec<&str> = element.path.split('.').collect();
            if field_parts.len() < 2 {
                continue;
            }

            let field_name = field_parts[1];
            let field_path = format!("{}.{}", structure_def.base_type, field_name);

            // Skip if we've already processed this field
            if processed_paths.contains(&field_path) {
                continue;
            }
            processed_paths.insert(field_path);

            let rust_field = self.element_to_rust_field(element, field_name)?;
            fields.push(rust_field);
        }

        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend_from_slice(&["Serialize".to_string(), "Deserialize".to_string()]);
        }

        let rust_struct = RustStruct {
            name: struct_name.clone(),
            fields,
            documentation: structure_def
                .title
                .clone()
                .or_else(|| Some(format!("FHIR {} resource", structure_def.name))),
            derives,
        };

        // Cache the generated struct for future use
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Convert an ElementDefinition to a RustField
    fn element_to_rust_field(
        &self,
        element: &ElementDefinition,
        field_name: &str,
    ) -> CodegenResult<RustField> {
        let rust_name = self.to_rust_field_name(field_name);
        let is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap_or(1)
                > 1;

        let rust_type = if let Some(element_types) = &element.element_type {
            if element_types.len() == 1 {
                self.fhir_type_to_rust_type(&element_types[0].code)?
            } else {
                // Multiple types - use an enum or serde_json::Value for now
                "serde_json::Value".to_string()
            }
        } else {
            // No type specified, might be a complex type
            "serde_json::Value".to_string()
        };

        let serde_rename = if rust_name != field_name && self.config.with_serde {
            Some(field_name.to_string())
        } else {
            None
        };

        Ok(RustField {
            name: rust_name,
            rust_type,
            is_optional,
            is_array,
            documentation: element.short.clone().or_else(|| element.definition.clone()),
            serde_rename,
        })
    }

    /// Convert FHIR type to Rust type
    fn fhir_type_to_rust_type(&self, fhir_type: &str) -> CodegenResult<String> {
        if let Some(mapped_type) = self.config.type_mappings.get(fhir_type) {
            Ok(mapped_type.clone())
        } else if fhir_type.chars().next().unwrap_or('a').is_uppercase() {
            // Likely a complex FHIR type, convert to PascalCase
            Ok(self.to_rust_type_name(fhir_type))
        } else {
            // Unknown primitive type, default to String
            Ok("String".to_string())
        }
    }

    /// Convert a name to Rust type name (PascalCase)
    fn to_rust_type_name(&self, name: &str) -> String {
        name.to_case(Case::Pascal)
    }

    /// Convert a name to Rust field name (snake_case)
    fn to_rust_field_name(&self, name: &str) -> String {
        // Handle FHIR choice types like value[x] -> value_x
        let cleaned_name = if name.contains('[') && name.contains(']') {
            name.replace('[', "_").replace(']', "")
        } else {
            name.to_string()
        };

        cleaned_name.to_case(Case::Snake)
    }

    /// Convert a FHIR resource type name to snake_case filename
    pub fn to_filename(&self, name: &str) -> String {
        name.to_case(Case::Snake)
    }

    /// Generate TokenStream for a RustStruct
    pub fn generate_tokens(&self, rust_struct: &RustStruct) -> TokenStream {
        let struct_name = format_ident!("{}", rust_struct.name);
        let derives: Vec<_> = rust_struct
            .derives
            .iter()
            .map(|d| format_ident!("{}", d))
            .collect();

        let fields: Vec<_> = rust_struct
            .fields
            .iter()
            .map(|field| {
                let field_name = format_ident!("{}", field.name);
                let field_type = self.build_field_type(field);

                let mut attrs = Vec::new();

                // Add serde rename if needed
                if let Some(rename) = &field.serde_rename {
                    attrs.push(quote! { #[serde(rename = #rename)] });
                }

                // Add documentation
                if let Some(doc) = &field.documentation {
                    attrs.push(quote! { #[doc = #doc] });
                }

                quote! {
                    #(#attrs)*
                    pub #field_name: #field_type
                }
            })
            .collect();

        let doc_attr = if let Some(doc) = &rust_struct.documentation {
            quote! { #[doc = #doc] }
        } else {
            quote! {}
        };

        quote! {
            #doc_attr
            #[derive(#(#derives),*)]
            pub struct #struct_name {
                #(#fields,)*
            }
        }
    }

    /// Build the complete type for a field (handling Option and Vec)
    fn build_field_type(&self, field: &RustField) -> Type {
        let base_type: Type = syn::parse_str(&field.rust_type).unwrap_or_else(|_| {
            parse_quote! { String }
        });

        let wrapped_type = if field.is_array {
            parse_quote! { Vec<#base_type> }
        } else {
            base_type
        };

        if field.is_optional {
            parse_quote! { Option<#wrapped_type> }
        } else {
            wrapped_type
        }
    }

    /// Generate code and write to file
    pub fn generate_to_file(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: &Path,
    ) -> CodegenResult<()> {
        let rust_struct = self.generate_struct(structure_def)?;
        let tokens = self.generate_tokens(&rust_struct);

        // Create output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the generated code
        let code = if self.config.with_serde {
            format!(
                "use serde::{{Deserialize, Serialize}};\n\n{}",
                tokens.to_string()
            )
        } else {
            tokens.to_string()
        };

        fs::write(output_path, code)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CodegenConfig::default();
        assert_eq!(config.module_name, "fhir_types");
        assert!(config.with_serde);
        assert!(config.type_mappings.contains_key("string"));
    }

    #[test]
    fn test_rust_name_conversion() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        assert_eq!(generator.to_rust_type_name("Patient"), "Patient");
        assert_eq!(generator.to_rust_type_name("humanName"), "HumanName");
        assert_eq!(generator.to_rust_field_name("birthDate"), "birth_date");
        assert_eq!(generator.to_rust_field_name("value[x]"), "value_x");
        assert_eq!(
            generator.to_rust_field_name("resourceType"),
            "resource_type"
        );
    }

    #[test]
    fn test_fhir_type_mapping() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        assert_eq!(
            generator.fhir_type_to_rust_type("string").unwrap(),
            "String"
        );
        assert_eq!(generator.fhir_type_to_rust_type("integer").unwrap(), "i32");
        assert_eq!(generator.fhir_type_to_rust_type("boolean").unwrap(), "bool");
        assert_eq!(
            generator.fhir_type_to_rust_type("Patient").unwrap(),
            "Patient"
        );
    }

    #[test]
    fn test_type_caching() {
        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a minimal test StructureDefinition
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-patient".to_string(),
            url: "http://example.com/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Test Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: None,
            differential: None,
            snapshot: Some(StructureDefinitionSnapshot {
                element: vec![
                    ElementDefinition {
                        id: Some("Patient".to_string()),
                        path: "Patient".to_string(),
                        short: None,
                        definition: None,
                        min: Some(1),
                        max: Some("1".to_string()),
                        element_type: None,
                        fixed: None,
                        pattern: None,
                    },
                    ElementDefinition {
                        id: Some("Patient.name".to_string()),
                        path: "Patient.name".to_string(),
                        short: Some("Patient name".to_string()),
                        definition: None,
                        min: Some(0),
                        max: Some("*".to_string()),
                        element_type: Some(vec![ElementType {
                            code: "string".to_string(),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                    },
                ],
            }),
        };

        // Generate struct first time - should populate cache
        let first_result = generator.generate_struct(&structure_def).unwrap();
        assert_eq!(generator.type_cache.len(), 1);
        assert!(generator.type_cache.contains_key("Patient"));

        // Generate same struct second time - should use cache
        let second_result = generator.generate_struct(&structure_def).unwrap();
        assert_eq!(first_result.name, second_result.name);
        assert_eq!(first_result.fields.len(), second_result.fields.len());

        // Cache should still contain one entry
        assert_eq!(generator.type_cache.len(), 1);
    }
}
