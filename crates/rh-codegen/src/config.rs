//! Configuration types and utilities for code generation
//!
//! This module contains configuration structures and related functionality.

use rh_common::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Whether to emit macro calls for primitive types instead of regular fields
    pub use_macro_calls: bool,
    /// Custom type mappings from FHIR to Rust types
    pub type_mappings: HashMap<String, String>,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        let mut type_mappings = HashMap::new();

        // Common FHIR to Rust type mappings using new primitive type aliases
        type_mappings.insert("string".to_string(), "StringType".to_string());
        type_mappings.insert("integer".to_string(), "IntegerType".to_string());
        type_mappings.insert("boolean".to_string(), "BooleanType".to_string());
        type_mappings.insert("decimal".to_string(), "DecimalType".to_string());
        type_mappings.insert("positiveInt".to_string(), "PositiveIntType".to_string());
        type_mappings.insert("unsignedInt".to_string(), "UnsignedIntType".to_string());
        type_mappings.insert("uri".to_string(), "StringType".to_string());
        type_mappings.insert("url".to_string(), "StringType".to_string());
        type_mappings.insert("canonical".to_string(), "StringType".to_string());
        type_mappings.insert("oid".to_string(), "StringType".to_string());
        type_mappings.insert("id".to_string(), "StringType".to_string());
        type_mappings.insert("markdown".to_string(), "StringType".to_string());
        type_mappings.insert("base64Binary".to_string(), "Base64BinaryType".to_string());
        type_mappings.insert("instant".to_string(), "InstantType".to_string());
        type_mappings.insert("date".to_string(), "StringType".to_string()); // Will be DateType when implemented
        type_mappings.insert("dateTime".to_string(), "DateTimeType".to_string());
        type_mappings.insert("time".to_string(), "TimeType".to_string());

        Self {
            output_dir: "generated".to_string(),
            module_name: "fhir_types".to_string(),
            with_serde: true,
            with_docs: true,
            use_macro_calls: false, // Disabled by default for backward compatibility
            type_mappings,
        }
    }
}

impl Config for CodegenConfig {}
