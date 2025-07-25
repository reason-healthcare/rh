//! FHIR type generation functionality
//!
//! This module contains the core logic for generating Rust types from FHIR StructureDefinitions.

use std::collections::{HashMap, HashSet};
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

/// Represents a FHIR element definition
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
    pub binding: Option<ElementBinding>,
}

/// Represents a FHIR element binding to a value set
#[derive(Debug, Deserialize, Clone)]
pub struct ElementBinding {
    pub strength: String,
    pub description: Option<String>,
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
}/// Represents a FHIR element type
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
    pub version: Option<String>,
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
    pub serde_flatten: bool,
}

/// Represents a generated Rust struct
#[derive(Debug, Clone)]
pub struct RustStruct {
    pub name: String,
    pub fields: Vec<RustField>,
    pub documentation: Option<String>,
    pub derives: Vec<String>,
}

/// Represents a generated Rust enum variant
#[derive(Debug, Clone)]
pub struct RustEnumVariant {
    pub name: String,
    pub value: String,
    pub documentation: Option<String>,
}

/// Represents a generated Rust enum
#[derive(Debug, Clone)]
pub struct RustEnum {
    pub name: String,
    pub variants: Vec<RustEnumVariant>,
    pub documentation: Option<String>,
    pub derives: Vec<String>,
    pub value_set: Option<String>,
}

/// Represents a FHIR ValueSet concept
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetConcept {
    pub code: String,
    pub display: Option<String>,
    pub definition: Option<String>,
}

/// Represents a FHIR ValueSet include
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetInclude {
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<String>>,
    pub system: Option<String>,
    pub concept: Option<Vec<ValueSetConcept>>,
}

/// Represents a FHIR ValueSet compose
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSetCompose {
    pub include: Option<Vec<ValueSetInclude>>,
}

/// Represents a FHIR ValueSet
#[derive(Debug, Deserialize, Clone)]
pub struct ValueSet {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub compose: Option<ValueSetCompose>,
}

/// Main code generator struct
pub struct CodeGenerator {
    config: CodegenConfig,
    /// Cache of previously generated types to avoid regenerating the same struct
    type_cache: HashMap<String, RustStruct>,
    /// Cache of generated enums for value set bindings
    enum_cache: HashMap<String, RustEnum>,
    /// Cache of loaded ValueSets indexed by URL
    value_set_cache: HashMap<String, ValueSet>,
    /// Directory to scan for ValueSet JSON files
    value_set_directory: Option<String>,
}

impl CodeGenerator {
    /// Create a new code generator with the given configuration
    pub fn new(config: CodegenConfig) -> Self {
        Self {
            config,
            type_cache: HashMap::new(),
            enum_cache: HashMap::new(),
            value_set_cache: HashMap::new(),
            value_set_directory: None,
        }
    }

    /// Create a new code generator with a ValueSet directory for dynamic enum generation
    pub fn new_with_value_set_directory<P: AsRef<Path>>(config: CodegenConfig, value_set_dir: P) -> Self {
        Self {
            config,
            type_cache: HashMap::new(),
            enum_cache: HashMap::new(),
            value_set_cache: HashMap::new(),
            value_set_directory: Some(value_set_dir.as_ref().to_string_lossy().to_string()),
        }
    }

    /// Clear the type cache
    ///
    /// This is useful when generating types for different FHIR packages that might
    /// have conflicting type definitions.
    pub fn clear_cache(&mut self) {
        self.type_cache.clear();
        self.enum_cache.clear();
        self.value_set_cache.clear();
    }

    /// Set the ValueSet directory for dynamic enum generation
    pub fn set_value_set_directory<P: AsRef<Path>>(&mut self, value_set_dir: P) {
        self.value_set_directory = Some(value_set_dir.as_ref().to_string_lossy().to_string());
        // Clear the ValueSet cache when directory changes
        self.value_set_cache.clear();
    }

    /// Load all ValueSets from the configured directory
    fn load_value_sets(&mut self) -> CodegenResult<()> {
        let value_set_dir = match &self.value_set_directory {
            Some(dir) => dir,
            None => return Ok(()), // No directory configured, skip loading
        };

        let dir_path = Path::new(value_set_dir);
        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(()); // Directory doesn't exist, skip loading
        }

        // Scan directory for JSON files
        let entries = fs::read_dir(dir_path).map_err(|e| CodegenError::Io(e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| CodegenError::Io(e))?;
            let path = entry.path();
            
            // Only process JSON files
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    // Try to parse as a FHIR resource
                    if let Ok(resource) = serde_json::from_str::<serde_json::Value>(&content) {
                        // Check if it's a ValueSet
                        if resource.get("resourceType")
                            .and_then(|rt| rt.as_str()) == Some("ValueSet") {
                            
                            // Parse as ValueSet
                            if let Ok(value_set) = serde_json::from_str::<ValueSet>(&content) {
                                if let Some(url) = &value_set.url {
                                    // Cache by URL (without version)
                                    let base_url = url.split('|').next().unwrap_or(url);
                                    self.value_set_cache.insert(base_url.to_string(), value_set);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Find a ValueSet by URL, loading from filesystem if necessary
    fn find_value_set(&mut self, value_set_url: &str) -> CodegenResult<Option<ValueSet>> {
        // Strip version from URL if present
        let base_url = value_set_url.split('|').next().unwrap_or(value_set_url);
        
        // Check cache first
        if let Some(value_set) = self.value_set_cache.get(base_url) {
            return Ok(Some(value_set.clone()));
        }

        // Load ValueSets from filesystem if not already loaded
        self.load_value_sets()?;
        
        // Check cache again after loading
        Ok(self.value_set_cache.get(base_url).cloned())
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

        // Collect all elements from differential and inherited base definitions
        let all_elements = self.collect_all_elements(structure_def)?;

        let mut fields = Vec::new();
        let mut processed_paths = std::collections::HashSet::new();
        let mut nested_elements: HashMap<String, Vec<ElementDefinition>> = HashMap::new();

        // Add base type field if this struct inherits from another
        if let Some(base_definition_url) = &structure_def.base_definition {
            if let Some(base_type_name) = self.extract_base_type_from_url(base_definition_url) {
                let base_field = RustField {
                    name: "base".to_string(),
                    rust_type: base_type_name,
                    is_optional: false,
                    is_array: false,
                    documentation: Some("Inherited fields from base type".to_string()),
                    serde_rename: None,
                    serde_flatten: true,
                };
                fields.push(base_field);
            }
        }

        // First pass: separate direct fields from nested structure elements
        for element in &all_elements {
            // Skip the root element (e.g., "Patient")
            if element.path == structure_def.base_type {
                continue;
            }

            let field_parts: Vec<&str> = element.path.split('.').collect();
            if field_parts.len() < 2 {
                continue;
            }

            if field_parts.len() == 2 {
                // Direct field (e.g., "ValueSet.url")
                let field_name = field_parts[1];
                let field_path = format!("{}.{}", structure_def.base_type, field_name);

                // Skip if we've already processed this field
                if processed_paths.contains(&field_path) {
                    continue;
                }
                processed_paths.insert(field_path);

                // Check if this field has nested elements (skip generating it as a simple field)
                let has_nested_elements = all_elements.iter().any(|e| {
                    let parts: Vec<&str> = e.path.split('.').collect();
                    parts.len() > 2 && parts[1] == field_name
                });

                if has_nested_elements {
                    // This field will be handled as a nested structure, skip generating it as a simple field
                    continue;
                }

                // Check if this is a choice type (ends with [x])
                if field_name.ends_with("[x]") {
                    // Handle choice type - generate multiple fields for each possible type
                    if let Some(element_types) = &element.element_type {
                        for element_type in element_types {
                            let choice_field = self.element_to_choice_field(element, field_name, &element_type.code)?;
                            fields.push(choice_field);
                        }
                    }
                } else {
                    // Handle regular field
                    let rust_field = self.element_to_rust_field(element, field_name)?;
                    fields.push(rust_field);
                }
            } else if field_parts.len() > 2 {
                // Nested structure element (e.g., "ValueSet.compose.include")
                let nested_structure_name = field_parts[1];
                nested_elements.entry(nested_structure_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            }
        }

        // Second pass: generate nested structures and add them as fields
        for (nested_name, nested_element_list) in nested_elements {
            let nested_struct_name = format!("{}{}", struct_name, self.to_rust_type_name(&nested_name));
            
            // Generate the nested structure
            let nested_struct = self.generate_nested_struct(
                &nested_struct_name,
                &nested_element_list,
                &format!("{}.{}", structure_def.base_type, nested_name),
            )?;

            // Add the nested struct to our type cache
            self.type_cache.insert(nested_struct_name.clone(), nested_struct);

            // Check if there's already a field with this name (avoid duplicates)
            let rust_field_name = self.to_rust_field_name(&nested_name);
            let field_exists = fields.iter().any(|f| f.name == rust_field_name);
            
            if !field_exists {
                // Find the element definition for this nested field to get proper info
                let nested_element = all_elements.iter()
                    .find(|e| e.path == format!("{}.{}", structure_def.base_type, nested_name));
                
                let is_optional = nested_element.map(|e| e.min.unwrap_or(0) == 0).unwrap_or(true);
                let is_array = nested_element.map(|e| 
                    e.max.as_deref() == Some("*") || 
                    e.max.as_deref().unwrap_or("1").parse::<u32>().unwrap_or(1) > 1
                ).unwrap_or(false);
                
                let documentation = nested_element
                    .and_then(|e| e.short.clone().or_else(|| e.definition.clone()));

                // Add a field to the main struct that references the nested struct
                let nested_field = RustField {
                    name: rust_field_name,
                    rust_type: nested_struct_name,
                    is_optional,
                    is_array,
                    documentation,
                    serde_rename: if self.to_rust_field_name(&nested_name) != nested_name {
                        Some(nested_name)
                    } else {
                        None
                    },
                    serde_flatten: false,
                };
                fields.push(nested_field);
            }
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

    /// Generate a nested structure from elements that belong to a sub-path
    fn generate_nested_struct(
        &mut self,
        struct_name: &str,
        elements: &[ElementDefinition],
        base_path: &str,
    ) -> CodegenResult<RustStruct> {
        let mut fields = Vec::new();
        let mut processed_paths = std::collections::HashSet::new();
        let mut deeper_nested_elements: HashMap<String, Vec<ElementDefinition>> = HashMap::new();

        for element in elements {
            // Extract the sub-field name relative to the base path
            // e.g., "ValueSet.compose.include" with base_path "ValueSet.compose" -> "include"
            let relative_path = element.path.strip_prefix(&format!("{}.", base_path));
            if let Some(relative_path) = relative_path {
                let field_parts: Vec<&str> = relative_path.split('.').collect();
                if !field_parts.is_empty() {
                    let field_name = field_parts[0];
                    let field_path = format!("{}.{}", base_path, field_name);

                    if field_parts.len() == 1 {
                        // Direct field at this level
                        // Skip if we've already processed this field
                        if processed_paths.contains(&field_path) {
                            continue;
                        }
                        processed_paths.insert(field_path);

                        // Check if this field has deeper nested elements (skip generating it as a simple field)
                        let has_deeper_nested_elements = elements.iter().any(|e| {
                            if let Some(rel_path) = e.path.strip_prefix(&format!("{}.", base_path)) {
                                let parts: Vec<&str> = rel_path.split('.').collect();
                                parts.len() > 1 && parts[0] == field_name
                            } else {
                                false
                            }
                        });

                        if has_deeper_nested_elements {
                            // This field will be handled as a nested structure, skip generating it as a simple field
                            continue;
                        }

                        // Check if this is a choice type (ends with [x])
                        if field_name.ends_with("[x]") {
                            // Handle choice type - generate multiple fields for each possible type
                            if let Some(element_types) = &element.element_type {
                                for element_type in element_types {
                                    let choice_field = self.element_to_choice_field(element, field_name, &element_type.code)?;
                                    fields.push(choice_field);
                                }
                            }
                        } else {
                            // Handle regular field
                            let rust_field = self.element_to_rust_field(element, field_name)?;
                            fields.push(rust_field);
                        }
                    } else if field_parts.len() > 1 {
                        // Deeper nested structure element (e.g., "include.system" when base_path is "ValueSet.compose")
                        let nested_structure_name = field_parts[0];
                        deeper_nested_elements.entry(nested_structure_name.to_string())
                            .or_insert_with(Vec::new)
                            .push(element.clone());
                    }
                }
            }
        }

        // Handle deeper nested structures
        for (nested_name, nested_element_list) in deeper_nested_elements {
            let nested_struct_name = format!("{}{}", struct_name, self.to_rust_type_name(&nested_name));
            
            // Generate the deeper nested structure
            let nested_struct = self.generate_nested_struct(
                &nested_struct_name,
                &nested_element_list,
                &format!("{}.{}", base_path, nested_name),
            )?;

            // Add the nested struct to our type cache
            self.type_cache.insert(nested_struct_name.clone(), nested_struct);

            // Find the element definition for this nested field to get proper info
            let nested_element = elements.iter()
                .find(|e| e.path == format!("{}.{}", base_path, nested_name));
            
            let is_optional = nested_element.map(|e| e.min.unwrap_or(0) == 0).unwrap_or(true);
            let is_array = nested_element.map(|e| 
                e.max.as_deref() == Some("*") || 
                e.max.as_deref().unwrap_or("1").parse::<u32>().unwrap_or(1) > 1
            ).unwrap_or(false);
            
            let documentation = nested_element
                .and_then(|e| e.short.clone().or_else(|| e.definition.clone()));

            // Add a field to this struct that references the deeper nested struct
            let nested_field = RustField {
                name: self.to_rust_field_name(&nested_name),
                rust_type: nested_struct_name,
                is_optional,
                is_array,
                documentation,
                serde_rename: if self.to_rust_field_name(&nested_name) != nested_name {
                    Some(nested_name)
                } else {
                    None
                },
                serde_flatten: false,
            };
            fields.push(nested_field);
        }

        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend_from_slice(&["Serialize".to_string(), "Deserialize".to_string()]);
        }

        let rust_struct = RustStruct {
            name: struct_name.to_string(),
            fields,
            documentation: Some(format!("Nested structure for {}", struct_name)),
            derives,
        };

        Ok(rust_struct)
    }

    /// Collect all elements from this StructureDefinition and its base definitions
    fn collect_all_elements(&mut self, structure_def: &StructureDefinition) -> CodegenResult<Vec<ElementDefinition>> {
        // For inheritance-based generation, we only need the differential elements
        // The base definition will be handled as a separate field in the struct
        if let Some(differential) = &structure_def.differential {
            Ok(differential.element.clone())
        } else {
            return Err(CodegenError::MissingField {
                field: "differential".to_string(),
            });
        }
    }
    
    /// Extract the base type name from a base definition URL
    fn extract_base_type_from_url(&self, base_definition_url: &str) -> Option<String> {
        // Extract the last part of the URL after the last '/'
        // e.g., "http://hl7.org/fhir/StructureDefinition/DomainResource" -> "DomainResource"
        base_definition_url.split('/').last().map(|s| s.to_string())
    }

    /// Convert an ElementDefinition to a RustField
    fn element_to_rust_field(
        &mut self,
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
                let type_code = &element_types[0].code;
                
                // Check if this should be an enum
                if self.should_generate_enum(element, type_code) {
                    if let Some(enum_name) = self.generate_enum_for_binding(element) {
                        enum_name
                    } else {
                        self.fhir_type_to_rust_type(type_code)?
                    }
                } else {
                    self.fhir_type_to_rust_type(type_code)?
                }
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
            serde_flatten: false,
        })
    }

    /// Convert an ElementDefinition with a specific type to a RustField for choice types
    fn element_to_choice_field(
        &mut self,
        element: &ElementDefinition,
        field_name: &str,
        type_code: &str,
    ) -> CodegenResult<RustField> {
        // Remove [x] from field name and add the specific type
        let base_name = field_name.trim_end_matches("[x]");
        let type_suffix = self.to_rust_field_name(type_code);
        let choice_field_name = format!("{}_{}", base_name, type_suffix);
        
        let rust_name = self.to_rust_field_name(&choice_field_name);
        let is_optional = element.min.unwrap_or(0) == 0; // Choice fields are typically optional
        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap_or(1)
                > 1;

        // Check if this should be an enum
        let rust_type = if self.should_generate_enum(element, type_code) {
            if let Some(enum_name) = self.generate_enum_for_binding(element) {
                enum_name
            } else {
                self.fhir_type_to_rust_type(type_code)?
            }
        } else {
            self.fhir_type_to_rust_type(type_code)?
        };
        
        // For choice types, the serde rename should use the original field name with the type
        let serde_rename = if self.config.with_serde {
            // FHIR choice fields are named like "valueString", "valueQuantity", etc.
            let capitalized_type = type_code.chars().next().unwrap_or('a').to_uppercase().collect::<String>() + &type_code[1..];
            Some(format!("{}{}", base_name, capitalized_type))
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
            serde_flatten: false,
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

        let snake_case_name = cleaned_name.to_case(Case::Snake);
        
        // Handle Rust reserved keywords by appending underscore
        match snake_case_name.as_str() {
            "abstract" => "abstract_".to_string(),
            "type" => "type_".to_string(),
            "match" => "match_".to_string(),
            "use" => "use_".to_string(),
            "mod" => "mod_".to_string(),
            "fn" => "fn_".to_string(),
            "struct" => "struct_".to_string(),
            "enum" => "enum_".to_string(),
            "trait" => "trait_".to_string(),
            "impl" => "impl_".to_string(),
            "self" => "self_".to_string(),
            "Self" => "self_type".to_string(),
            "super" => "super_".to_string(),
            "crate" => "crate_".to_string(),
            "where" => "where_".to_string(),
            "for" => "for_".to_string(),
            "if" => "if_".to_string(),
            "else" => "else_".to_string(),
            "while" => "while_".to_string(),
            "loop" => "loop_".to_string(),
            "return" => "return_".to_string(),
            "break" => "break_".to_string(),
            "continue" => "continue_".to_string(),
            "let" => "let_".to_string(),
            "mut" => "mut_".to_string(),
            "const" => "const_".to_string(),
            "static" => "static_".to_string(),
            "ref" => "ref_".to_string(),
            "move" => "move_".to_string(),
            "box" => "box_".to_string(),
            "async" => "async_".to_string(),
            "await" => "await_".to_string(),
            _ => snake_case_name,
        }
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

                // Add serde flatten if needed
                if field.serde_flatten {
                    attrs.push(quote! { #[serde(flatten)] });
                }

                // Add serde rename if needed
                if let Some(rename) = &field.serde_rename {
                    attrs.push(quote! { #[serde(rename = #rename)] });
                }

                // Add documentation
                if let Some(doc) = &field.documentation {
                    let formatted_doc = format!(" {}", doc);
                    attrs.push(quote! { #[doc = #formatted_doc] });
                }

                quote! {
                    #(#attrs)*
                    pub #field_name: #field_type
                }
            })
            .collect();

        let doc_attr = if let Some(doc) = &rust_struct.documentation {
            let formatted_doc = format!(" {}", doc);
            quote! { #[doc = #formatted_doc] }
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

    /// Check if a code field should become an enum based on its binding
    fn should_generate_enum(&self, element: &ElementDefinition, type_code: &str) -> bool {
        type_code == "code" && 
        element.binding.as_ref()
            .map(|b| b.strength == "required")
            .unwrap_or(false)
    }

    /// Generate an enum name from a value set URL
    fn value_set_to_enum_name(&self, value_set: &str) -> String {
        // Extract the last part of the value set URL and convert to PascalCase
        // e.g., "http://hl7.org/fhir/ValueSet/administrative-gender" -> "AdministrativeGender"
        // Also handle versioned URLs like "http://hl7.org/fhir/ValueSet/publication-status|4.0.1"
        let enum_name = value_set
            .split('|')  // Remove version if present
            .next()
            .unwrap_or(value_set)
            .split('/')
            .last()
            .unwrap_or("UnknownValueSet")
            .replace('-', " ")
            .to_case(Case::Pascal);
        
        // Ensure it doesn't conflict with standard types
        if enum_name.is_empty() || enum_name == "ValueSet" {
            "UnknownValueSet".to_string()
        } else {
            enum_name
        }
    }

    /// Generate a Rust enum from a value set binding
    fn generate_enum_for_binding(&mut self, element: &ElementDefinition) -> Option<String> {
        let binding = element.binding.as_ref()?;
        let value_set = binding.value_set.as_ref()?;
        
        if binding.strength != "required" {
            return None;
        }

        // Generate enum name from value set
        let enum_name = self.value_set_to_enum_name(value_set);
        
        // Check if we already generated this enum
        if self.enum_cache.contains_key(&enum_name) {
            return Some(enum_name);
        }

        // For now, generate a placeholder enum with common values
        // In a full implementation, you would fetch the actual ValueSet definition
        let variants = self.get_value_set_variants(value_set, &enum_name);

        // If we only got a fallback "Unknown" variant, fall back to String type
        if variants.len() == 1 && variants[0].value == "unknown" {
            return None;
        }

        let rust_enum = RustEnum {
            name: enum_name.clone(),
            variants,
            documentation: binding.description.clone(),
            derives: vec![
                "Debug".to_string(),
                "Clone".to_string(),
                "PartialEq".to_string(),
                "Eq".to_string(),
                "Hash".to_string(),
            ],
            value_set: Some(value_set.clone()),
        };

        // Add serde derives if enabled
        if self.config.with_serde {
            let mut enum_with_serde = rust_enum.clone();
            enum_with_serde.derives.extend(vec![
                "Serialize".to_string(),
                "Deserialize".to_string(),
            ]);
            self.enum_cache.insert(enum_name.clone(), enum_with_serde);
        } else {
            self.enum_cache.insert(enum_name.clone(), rust_enum);
        }

        Some(enum_name)
    }

    /// Get variants for a known value set (with filesystem support)
    fn get_value_set_variants(&mut self, value_set: &str, enum_name: &str) -> Vec<RustEnumVariant> {
        // First, try to load the ValueSet from filesystem
        if let Ok(Some(loaded_value_set)) = self.find_value_set(value_set) {
            let mut variants = Vec::new();
            
            // Extract concepts from the ValueSet
            if let Some(compose) = &loaded_value_set.compose {
                if let Some(includes) = &compose.include {
                    for include in includes {
                        // Handle direct concepts
                        if let Some(concepts) = &include.concept {
                            for concept in concepts {
                                let variant_name = self.code_to_variant_name(&concept.code);
                                let documentation = concept.display.clone()
                                    .or_else(|| concept.definition.clone())
                                    .or_else(|| Some(format!("{} value", concept.code)));
                                
                                variants.push(RustEnumVariant {
                                    name: variant_name,
                                    value: concept.code.clone(),
                                    documentation,
                                });
                            }
                        }
                        
                        // Handle included value sets (recursive includes)
                        if let Some(included_value_sets) = &include.value_set {
                            for included_vs in included_value_sets {
                                if let Ok(Some(_included_vs_def)) = self.find_value_set(included_vs) {
                                    // Recursively get variants from included ValueSet
                                    let included_variants = self.get_value_set_variants(
                                        included_vs, 
                                        &format!("{}Included", enum_name)
                                    );
                                    variants.extend(included_variants);
                                }
                            }
                        }
                    }
                }
            }
            
            // If we found variants from the filesystem, return them
            if !variants.is_empty() {
                return variants;
            }
        }
        
        // Fallback to hardcoded known value sets if filesystem loading failed
        // Handle versioned value sets by stripping the version
        let base_value_set = value_set.split('|').next().unwrap_or(value_set);
        
        // For now, provide some common known value sets
        match base_value_set {
            "http://hl7.org/fhir/ValueSet/administrative-gender" => vec![
                RustEnumVariant {
                    name: "Male".to_string(),
                    value: "male".to_string(),
                    documentation: Some("Male gender".to_string()),
                },
                RustEnumVariant {
                    name: "Female".to_string(),
                    value: "female".to_string(),
                    documentation: Some("Female gender".to_string()),
                },
                RustEnumVariant {
                    name: "Other".to_string(),
                    value: "other".to_string(),
                    documentation: Some("Other gender".to_string()),
                },
                RustEnumVariant {
                    name: "Unknown".to_string(),
                    value: "unknown".to_string(),
                    documentation: Some("Unknown gender".to_string()),
                },
            ],
            "http://hl7.org/fhir/ValueSet/publication-status" => vec![
                RustEnumVariant {
                    name: "Draft".to_string(),
                    value: "draft".to_string(),
                    documentation: Some("Draft status".to_string()),
                },
                RustEnumVariant {
                    name: "Active".to_string(),
                    value: "active".to_string(),
                    documentation: Some("Active status".to_string()),
                },
                RustEnumVariant {
                    name: "Retired".to_string(),
                    value: "retired".to_string(),
                    documentation: Some("Retired status".to_string()),
                },
                RustEnumVariant {
                    name: "Unknown".to_string(),
                    value: "unknown".to_string(),
                    documentation: Some("Unknown status".to_string()),
                },
            ],
            "http://hl7.org/fhir/ValueSet/structure-definition-kind" => vec![
                RustEnumVariant {
                    name: "PrimitiveType".to_string(),
                    value: "primitive-type".to_string(),
                    documentation: Some("Primitive data type".to_string()),
                },
                RustEnumVariant {
                    name: "ComplexType".to_string(),
                    value: "complex-type".to_string(),
                    documentation: Some("Complex data type".to_string()),
                },
                RustEnumVariant {
                    name: "Resource".to_string(),
                    value: "resource".to_string(),
                    documentation: Some("Resource definition".to_string()),
                },
                RustEnumVariant {
                    name: "Logical".to_string(),
                    value: "logical".to_string(),
                    documentation: Some("Logical model".to_string()),
                },
            ],
            "http://hl7.org/fhir/ValueSet/FHIR-version" => vec![
                RustEnumVariant {
                    name: "R4".to_string(),
                    value: "4.0.1".to_string(),
                    documentation: Some("FHIR R4".to_string()),
                },
                RustEnumVariant {
                    name: "R5".to_string(),
                    value: "5.0.0".to_string(),
                    documentation: Some("FHIR R5".to_string()),
                },
            ],
            "http://hl7.org/fhir/ValueSet/type-derivation-rule" => vec![
                RustEnumVariant {
                    name: "Specialization".to_string(),
                    value: "specialization".to_string(),
                    documentation: Some("Specialization rule".to_string()),
                },
                RustEnumVariant {
                    name: "Constraint".to_string(),
                    value: "constraint".to_string(),
                    documentation: Some("Constraint rule".to_string()),
                },
            ],
            "http://hl7.org/fhir/ValueSet/extension-context-type" => vec![
                RustEnumVariant {
                    name: "Fhirpath".to_string(),
                    value: "fhirpath".to_string(),
                    documentation: Some("FHIRPath expression".to_string()),
                },
                RustEnumVariant {
                    name: "Element".to_string(),
                    value: "element".to_string(),
                    documentation: Some("Element".to_string()),
                },
                RustEnumVariant {
                    name: "Extension".to_string(),
                    value: "extension".to_string(),
                    documentation: Some("Extension".to_string()),
                },
            ],
            _ => {
                // For unknown value sets, create a minimal enum
                vec![
                    RustEnumVariant {
                        name: "Unknown".to_string(),
                        value: "unknown".to_string(),
                        documentation: Some(format!("Unknown value for {}", enum_name)),
                    },
                ]
            }
        }
    }

    /// Convert a FHIR code to a valid Rust enum variant name
    fn code_to_variant_name(&self, code: &str) -> String {
        // Handle various FHIR code formats and convert to PascalCase
        match code {
            // Handle special cases first
            "primitive-type" => "PrimitiveType".to_string(),
            "complex-type" => "ComplexType".to_string(),
            _ => {
                // Convert hyphenated codes to PascalCase
                if code.contains('-') {
                    code.split('-')
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                            }
                        })
                        .collect()
                } else {
                    // Simple case: capitalize first letter
                    let mut chars = code.chars();
                    match chars.next() {
                        None => "Unknown".to_string(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    }
                }
            }
        }
    }

    /// Generate TokenStream for a RustEnum
    fn generate_enum_tokens(&self, rust_enum: &RustEnum) -> TokenStream {
        let enum_name = format_ident!("{}", rust_enum.name);
        let derives: Vec<_> = rust_enum
            .derives
            .iter()
            .map(|d| format_ident!("{}", d))
            .collect();

        let variants: Vec<_> = rust_enum
            .variants
            .iter()
            .map(|variant| {
                let variant_name = format_ident!("{}", variant.name);
                let variant_value = &variant.value;

                let doc_attr = if let Some(doc) = &variant.documentation {
                    let formatted_doc = format!(" {}", doc);
                    quote! { #[doc = #formatted_doc] }
                } else {
                    quote! {}
                };

                let serde_attr = if self.config.with_serde {
                    quote! { #[serde(rename = #variant_value)] }
                } else {
                    quote! {}
                };

                quote! {
                    #doc_attr
                    #serde_attr
                    #variant_name
                }
            })
            .collect();

        let doc_attr = if let Some(doc) = &rust_enum.documentation {
            let formatted_doc = format!(" {}", doc);
            quote! { #[doc = #formatted_doc] }
        } else {
            quote! {}
        };

        quote! {
            #doc_attr
            #[derive(#(#derives),*)]
            pub enum #enum_name {
                #(#variants,)*
            }
        }
    }

    /// Add header comment with StructureDefinition information
    fn add_header_comment(&self, code: &str, structure_def: &StructureDefinition) -> String {
        let version_info = if let Some(version) = &structure_def.version {
            format!(" (version {})", version)
        } else {
            String::new()
        };

        let header = format!(
            "// Generated from: {}{}\n// URL: {}\n\n{}",
            structure_def.name,
            version_info,
            structure_def.url,
            code
        );
        
        header
    }

    /// Collect all enum types that are actually used in the given structs
    fn collect_used_enum_types(&self, structs: &[RustStruct]) -> HashSet<String> {
        let mut used_enums = HashSet::new();
        
        for rust_struct in structs {
            for field in &rust_struct.fields {
                // Check if the field type (without Option wrapper) is an enum we have cached
                let base_type = field.rust_type
                    .strip_prefix("Option<")
                    .and_then(|s| s.strip_suffix(">"))
                    .unwrap_or(&field.rust_type);
                
                // Remove Vec wrapper if present
                let base_type = base_type
                    .strip_prefix("Vec<")
                    .and_then(|s| s.strip_suffix(">"))
                    .unwrap_or(base_type);
                
                if self.enum_cache.contains_key(base_type) {
                    used_enums.insert(base_type.to_string());
                }
            }
        }
        
        used_enums
    }

    /// Generate code and write to file
    pub fn generate_to_file(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: &Path,
    ) -> CodegenResult<()> {
        let main_struct = self.generate_struct(structure_def)?;
        
        // Collect all structs that were generated (including nested ones)
        let mut structs_to_generate = Vec::new();
        structs_to_generate.push(main_struct);
        
        // Add any nested structs from the type cache that start with the main struct name
        let main_struct_name = self.to_rust_type_name(&structure_def.name);
        for (type_name, rust_struct) in &self.type_cache {
            if type_name.starts_with(&main_struct_name) && type_name != &main_struct_name {
                structs_to_generate.push(rust_struct.clone());
            }
        }
        
        // Collect only the enums that are actually used by these structs
        let used_enum_types = self.collect_used_enum_types(&structs_to_generate);
        
        // Generate tokens for the main struct and all nested structs
        let mut all_tokens = Vec::new();
        
        // First, generate tokens only for used enums
        for enum_name in &used_enum_types {
            if let Some(rust_enum) = self.enum_cache.get(enum_name) {
                let enum_tokens = self.generate_enum_tokens(rust_enum);
                all_tokens.push(enum_tokens);
            }
        }
        
        // Generate tokens for all structs
        for rust_struct in structs_to_generate {
            let struct_tokens = self.generate_tokens(&rust_struct);
            all_tokens.push(struct_tokens);
        }

        // Create output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Combine all tokens
        let combined_struct_tokens = quote! {
            #(#all_tokens)*
        };

        // Write the generated code
        let code = if self.config.with_serde {
            let serde_import = quote! { use serde::{Deserialize, Serialize}; };
            let combined_tokens = quote! {
                #serde_import
                
                #combined_struct_tokens
            };
            
            // Parse the tokens into a syn::File for formatting
            let file = syn::parse2::<syn::File>(combined_tokens)
                .map_err(|e| CodegenError::Generation { message: format!("Failed to parse generated tokens: {}", e) })?;
            
            // Format with prettyplease
            let formatted_code = prettyplease::unparse(&file);
            
            // Add header comment with StructureDefinition info
            self.add_header_comment(&formatted_code, structure_def)
        } else {
            // Parse the tokens into a syn::File for formatting
            let file = syn::parse2::<syn::File>(combined_struct_tokens)
                .map_err(|e| CodegenError::Generation { message: format!("Failed to parse generated tokens: {}", e) })?;
            
            // Format with prettyplease
            let formatted_code = prettyplease::unparse(&file);
            
            // Add header comment with StructureDefinition info
            self.add_header_comment(&formatted_code, structure_def)
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
            version: Some("1.0.0".to_string()),
            name: "Patient".to_string(),
            title: Some("Test Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: Some(StructureDefinitionDifferential {
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
                        binding: None,
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
                        binding: None,
                    },
                ],
            }),
            snapshot: None,
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

    #[test]
    fn test_choice_type_generation() {
        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a test StructureDefinition with a choice type
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-observation".to_string(),
            url: "http://example.com/Observation".to_string(),
            version: Some("1.0.0".to_string()),
            name: "TestObservation".to_string(),
            title: Some("Test Observation".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Observation".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: Some(StructureDefinitionDifferential {
                element: vec![
                    ElementDefinition {
                        id: Some("Observation".to_string()),
                        path: "Observation".to_string(),
                        short: None,
                        definition: None,
                        min: Some(1),
                        max: Some("1".to_string()),
                        element_type: None,
                        fixed: None,
                        pattern: None,
                        binding: None,
                    },
                    ElementDefinition {
                        id: Some("Observation.value[x]".to_string()),
                        path: "Observation.value[x]".to_string(),
                        short: Some("Actual result".to_string()),
                        definition: None,
                        min: Some(0),
                        max: Some("1".to_string()),
                        element_type: Some(vec![
                            ElementType {
                                code: "string".to_string(),
                                target_profile: None,
                            },
                            ElementType {
                                code: "Quantity".to_string(),
                                target_profile: None,
                            },
                        ]),
                        fixed: None,
                        pattern: None,
                        binding: None,
                    },
                ],
            }),
            snapshot: None,
        };

        // Generate struct with choice type
        let result = generator.generate_struct(&structure_def).unwrap();
        
        // Should have base field plus choice type fields
        // Base field: base (1 field) - inherits from DomainResource
        // Choice fields: value_string, value_quantity (2 fields)
        // Total: 3 fields
        assert_eq!(result.fields.len(), 3);
        
        // Check that we have the base field and choice fields
        let field_names: Vec<_> = result.fields.iter().map(|f| f.name.as_str()).collect();
        assert!(field_names.contains(&"base"));
        assert!(field_names.contains(&"value_string"));
        assert!(field_names.contains(&"value_quantity"));
        
        // Check that base field is flattened
        let base_field = result.fields.iter().find(|f| f.name == "base").unwrap();
        assert!(base_field.serde_flatten);
        assert_eq!(base_field.rust_type, "DomainResource");
        
        // Check serde rename attributes for choice fields
        let value_string_field = result.fields.iter().find(|f| f.name == "value_string").unwrap();
        assert_eq!(value_string_field.serde_rename, Some("valueString".to_string()));
        assert!(!value_string_field.serde_flatten);
        
        let value_quantity_field = result.fields.iter().find(|f| f.name == "value_quantity").unwrap();
        assert_eq!(value_quantity_field.serde_rename, Some("valueQuantity".to_string()));
        assert!(!value_quantity_field.serde_flatten);
        
        // Check field types
        assert_eq!(value_string_field.rust_type, "String");
        assert_eq!(value_quantity_field.rust_type, "Quantity");
        
        // Choice fields should be optional
        assert!(value_string_field.is_optional);
        assert!(value_quantity_field.is_optional);
    }

    #[test]
    fn test_enum_generation_for_required_binding() {
        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a StructureDefinition with a code field that has a required binding
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "TestPatient".to_string(),
            url: "http://example.org/StructureDefinition/TestPatient".to_string(),
            version: Some("1.0.0".to_string()),
            name: "TestPatient".to_string(),
            title: Some("Test Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: Some(StructureDefinitionDifferential {
                element: vec![
                    ElementDefinition {
                        id: Some("Patient.gender".to_string()),
                        path: "Patient.gender".to_string(),
                        short: Some("male | female | other | unknown".to_string()),
                        definition: Some("Administrative Gender".to_string()),
                        min: Some(0),
                        max: Some("1".to_string()),
                        element_type: Some(vec![ElementType {
                            code: "code".to_string(),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                        binding: Some(ElementBinding {
                            strength: "required".to_string(),
                            description: Some("Administrative gender".to_string()),
                            value_set: Some("http://hl7.org/fhir/ValueSet/administrative-gender".to_string()),
                        }),
                    },
                ],
            }),
            snapshot: None,
        };

        // Generate struct
        let result = generator.generate_struct(&structure_def).unwrap();
        
        // Should have gender field with enum type
        let gender_field = result.fields.iter().find(|f| f.name == "gender").unwrap();
        assert_eq!(gender_field.rust_type, "AdministrativeGender");
        assert!(gender_field.is_optional);
        
        // Should have generated the enum in the cache
        assert!(generator.enum_cache.contains_key("AdministrativeGender"));
        
        let enum_def = &generator.enum_cache["AdministrativeGender"];
        assert_eq!(enum_def.name, "AdministrativeGender");
        assert_eq!(enum_def.variants.len(), 4); // male, female, other, unknown
        
        // Check variant names and values
        let male_variant = enum_def.variants.iter().find(|v| v.name == "Male").unwrap();
        assert_eq!(male_variant.value, "male");
        
        let female_variant = enum_def.variants.iter().find(|v| v.name == "Female").unwrap();
        assert_eq!(female_variant.value, "female");
    }

    #[test]
    fn test_no_enum_generation_for_non_required_binding() {
        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a StructureDefinition with a code field that has a preferred binding
        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "TestPatient2".to_string(),
            url: "http://example.org/StructureDefinition/TestPatient".to_string(),
            version: Some("1.0.0".to_string()),
            name: "TestPatient".to_string(),
            title: Some("Test Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: Some(StructureDefinitionDifferential {
                element: vec![
                    ElementDefinition {
                        id: Some("Patient.language".to_string()),
                        path: "Patient.language".to_string(),
                        short: Some("Language of the patient".to_string()),
                        definition: Some("Preferred language".to_string()),
                        min: Some(0),
                        max: Some("1".to_string()),
                        element_type: Some(vec![ElementType {
                            code: "code".to_string(),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                        binding: Some(ElementBinding {
                            strength: "preferred".to_string(), // Not required
                            description: Some("Language".to_string()),
                            value_set: Some("http://hl7.org/fhir/ValueSet/languages".to_string()),
                        }),
                    },
                ],
            }),
            snapshot: None,
        };

        // Generate struct
        let result = generator.generate_struct(&structure_def).unwrap();
        
        // Should have language field with String type (not enum)
        let language_field = result.fields.iter().find(|f| f.name == "language").unwrap();
        assert_eq!(language_field.rust_type, "String");
        
        // Should not have generated any enum for this
        assert!(generator.enum_cache.is_empty());
    }
}
