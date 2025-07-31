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
        type_mappings.insert("instant".to_string(), "String".to_string());
        type_mappings.insert("date".to_string(), "String".to_string());
        type_mappings.insert("dateTime".to_string(), "String".to_string());
        type_mappings.insert("time".to_string(), "String".to_string());

        Self {
            output_dir: "generated".to_string(),
            module_name: "fhir_types".to_string(),
            with_serde: true,
            with_docs: true,
            type_mappings,
        }
    }
}

impl Config for CodegenConfig {}
