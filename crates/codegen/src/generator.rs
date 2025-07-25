//! FHIR type generation functionality
//!
//! This module contains the core logic for generating Rust types from FHIR StructureDefinitions.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustEnum, RustStruct};
use crate::token_generator::TokenGenerator;
use crate::value_sets::ValueSetManager;
use crate::{CodegenError, CodegenResult};

/// Main code generator struct
pub struct CodeGenerator {
    config: CodegenConfig,
    /// Cache of previously generated types to avoid regenerating the same struct
    type_cache: HashMap<String, RustStruct>,
    /// Cache of generated enums for value set bindings
    enum_cache: HashMap<String, RustEnum>,
    /// ValueSet manager for handling ValueSet operations
    value_set_manager: ValueSetManager,
    /// Token generator for generating Rust code
    token_generator: TokenGenerator,
}

impl CodeGenerator {
    /// Create a new code generator with the given configuration
    pub fn new(config: CodegenConfig) -> Self {
        let value_set_manager = ValueSetManager::new();
        let token_generator = TokenGenerator::new();

        Self {
            config,
            type_cache: HashMap::new(),
            enum_cache: HashMap::new(),
            value_set_manager,
            token_generator,
        }
    }

    /// Create a new code generator with a ValueSet directory
    pub fn new_with_value_set_directory<P: AsRef<Path>>(config: CodegenConfig, _value_set_dir: P) -> Self {
        // For now, we'll just create a normal generator
        // In a full implementation, this would load ValueSets from the directory
        let value_set_manager = ValueSetManager::new();
        let token_generator = TokenGenerator::new();

        Self {
            config,
            type_cache: HashMap::new(),
            enum_cache: HashMap::new(),
            value_set_manager,
            token_generator,
        }
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

    /// Generate a Rust struct from a FHIR StructureDefinition (placeholder)
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        let struct_name = self.to_rust_name(&structure_def.name);

        // Check if we've already generated this type
        if let Some(cached_struct) = self.type_cache.get(&struct_name) {
            return Ok(cached_struct.clone());
        }

        // For now, create a simple struct with basic fields
        let mut rust_struct = RustStruct::new(struct_name.clone());
        rust_struct.doc_comment = structure_def.title.clone();
        
        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        rust_struct.derives = derives;

        // Cache the generated struct for future use
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Generate a Rust struct and write it to a file
    pub fn generate_to_file<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: P,
    ) -> CodegenResult<()> {
        // Generate the struct
        let rust_struct = self.generate_struct(structure_def)?;
        
        // Generate tokens using the token generator
        let tokens = self.token_generator.generate_struct(&rust_struct);
        
        // Write to file
        let code = tokens.to_string();
        fs::write(output_path.as_ref(), code).map_err(CodegenError::Io)?;
        
        Ok(())
    }

    /// Generate an enum for a value set binding
    pub fn generate_enum_for_value_set(&mut self, value_set_url: &str) -> CodegenResult<Option<RustEnum>> {
        // Check if we've already generated this enum
        if let Some(cached_enum) = self.enum_cache.get(value_set_url) {
            return Ok(Some(cached_enum.clone()));
        }

        // Generate a placeholder enum using the value set manager
        let enum_name = self.value_set_manager.generate_placeholder_enum(value_set_url);
        
        // Get the generated enum from the value set manager's cache
        if let Some(rust_enum) = self.value_set_manager.get_cached_enums().get(&enum_name) {
            // Cache it in our own cache as well
            self.enum_cache.insert(value_set_url.to_string(), rust_enum.clone());
            Ok(Some(rust_enum.clone()))
        } else {
            Ok(None)
        }
    }

    /// Convert a FHIR name to a valid Rust type name
    fn to_rust_name(&self, name: &str) -> String {
        // Convert to PascalCase and handle special characters
        name.split_whitespace() // Split on whitespace
            .flat_map(|word| word.split(&['-', '_', '.'][..])) // Split on common separators
            .filter(|word| !word.is_empty()) // Remove empty strings
            .map(|word| {
                // Convert each word to PascalCase
                let mut chars = word.chars();
                #[allow(clippy::match_single_binding)]
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + 
                        &chars.collect::<String>().to_lowercase()
                    }
                }
            })
            .collect::<String>()
    }

    /// Convert a FHIR resource type name to snake_case filename
    pub fn to_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rust_name_conversion() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        // Test normal names
        assert_eq!(generator.to_rust_name("Patient"), "Patient");
        assert_eq!(generator.to_rust_name("patient"), "Patient");
        
        // Test names with spaces
        assert_eq!(generator.to_rust_name("Relative Date Criteria"), "RelativeDateCriteria");
        assert_eq!(generator.to_rust_name("Care Plan"), "CarePlan");
        
        // Test names with dashes and underscores
        assert_eq!(generator.to_rust_name("patient-name"), "PatientName");
        assert_eq!(generator.to_rust_name("patient_name"), "PatientName");
        
        // Test mixed separators
        assert_eq!(generator.to_rust_name("some-complex_name with.spaces"), "SomeComplexNameWithSpaces");
        
        // Test empty and edge cases
        assert_eq!(generator.to_rust_name(""), "");
        assert_eq!(generator.to_rust_name("   "), "");
        assert_eq!(generator.to_rust_name("a"), "A");
    }
}
