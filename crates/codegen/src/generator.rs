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
        rust_struct.derives = vec!["Debug".to_string(), "Clone".to_string()];

        // Cache the generated struct for future use
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Convert a FHIR name to a valid Rust type name
    fn to_rust_name(&self, name: &str) -> String {
        // Simple conversion - in a real implementation this would be more sophisticated
        name.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().collect::<String>()
                } else if c == '-' || c == '_' {
                    String::new()
                } else {
                    c.to_string()
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
