//! File I/O operations for FHIR code generation
//!
//! This module provides centralized file I/O operations for loading FHIR StructureDefinitions
//! and generating code files in organized directory structures.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::{FileGenerator, TokenGenerator};
use crate::rust_types::{RustStruct, RustTrait};
use crate::{CodegenError, CodegenResult};

// Re-export for convenience
pub use crate::generators::file_generator::FhirTypeCategory;

/// Manager for file I/O operations in code generation
pub struct FileIoManager<'a> {
    config: &'a CodegenConfig,
    token_generator: &'a TokenGenerator,
}

impl<'a> FileIoManager<'a> {
    /// Create a new file I/O manager
    pub fn new(config: &'a CodegenConfig, token_generator: &'a TokenGenerator) -> Self {
        Self {
            config,
            token_generator,
        }
    }

    /// Load and parse a FHIR StructureDefinition from a JSON file
    pub fn load_structure_definition<P: AsRef<Path>>(
        path: P,
    ) -> CodegenResult<StructureDefinition> {
        let content = fs::read_to_string(&path).map_err(CodegenError::Io)?;

        let structure_def: StructureDefinition =
            serde_json::from_str(&content).map_err(CodegenError::Json)?;

        Ok(structure_def)
    }

    /// Generate a Rust struct and write it to the appropriate directory based on FHIR type classification
    pub fn generate_to_organized_directories<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
        rust_struct: &RustStruct,
        nested_structs: &[RustStruct],
    ) -> CodegenResult<()> {
        let base_dir = base_output_dir.as_ref();

        // Determine the appropriate subdirectory based on FHIR type
        let target_dir = match self.classify_fhir_structure_def(structure_def) {
            FhirTypeCategory::Resource => base_dir.join("src").join("resource"),
            FhirTypeCategory::DataType => base_dir.join("src").join("datatypes"),
            FhirTypeCategory::Extension => base_dir.join("src").join("extensions"),
            FhirTypeCategory::Primitive => base_dir.join("src").join("primitives"),
        };

        // Ensure the target directory exists
        std::fs::create_dir_all(&target_dir).map_err(CodegenError::Io)?;

        // Generate the file in the appropriate directory
        let filename = crate::naming::Naming::filename(structure_def);
        let output_path = target_dir.join(filename);

        self.generate_to_file(structure_def, output_path, rust_struct, nested_structs)
    }

    /// Generate a trait and write it to the traits directory
    pub fn generate_trait_to_organized_directory<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
        rust_trait: &RustTrait,
    ) -> CodegenResult<()> {
        let traits_dir = base_output_dir.as_ref().join("src").join("traits");

        // Ensure the traits directory exists
        std::fs::create_dir_all(&traits_dir).map_err(CodegenError::Io)?;

        // Generate the trait file
        let struct_name = crate::naming::Naming::struct_name(structure_def);
        let snake_case_name = crate::naming::Naming::to_snake_case(&struct_name);
        let filename = format!("{snake_case_name}.rs");
        let output_path = traits_dir.join(filename);

        self.generate_trait_to_file(structure_def, output_path, rust_trait)
    }

    /// Generate a Rust struct and write it to a file
    pub fn generate_to_file<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        output_path: P,
        rust_struct: &RustStruct,
        nested_structs: &[RustStruct],
    ) -> CodegenResult<()> {
        let file_generator = FileGenerator::new(self.config, self.token_generator);
        file_generator.generate_to_file(structure_def, output_path, rust_struct, nested_structs)
    }

    /// Generate a Rust trait and write it to a file
    pub fn generate_trait_to_file<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        output_path: P,
        rust_trait: &RustTrait,
    ) -> CodegenResult<()> {
        let file_generator = FileGenerator::new(self.config, self.token_generator);
        file_generator.generate_trait_to_file(structure_def, output_path, rust_trait)
    }

    /// Generate a trait file directly from a RustTrait object
    pub fn generate_trait_file_from_trait<P: AsRef<Path>>(
        &self,
        rust_trait: &RustTrait,
        output_path: P,
    ) -> CodegenResult<()> {
        let file_generator = FileGenerator::new(self.config, self.token_generator);
        file_generator.generate_trait_file_from_trait(rust_trait, output_path)
    }

    /// Check if a cached struct name is a direct nested struct of the parent struct
    /// This prevents issues like Element collecting ElementDefinition nested structs
    fn is_direct_nested_struct(parent_name: &str, cached_name: &str) -> bool {
        // Must start with parent name
        if !cached_name.starts_with(parent_name) {
            return false;
        }

        // Must have something after the parent name
        if cached_name.len() <= parent_name.len() {
            return false;
        }

        // Get the remainder after the parent name
        let remainder = &cached_name[parent_name.len()..];

        // Special cases: if the remainder is exactly one of these complete words,
        // it suggests this is a separate entity, not a nested struct
        // For example: "ElementDefinition" should not be a nested struct of "Element"
        // But "ImplementationGuideDefinition" IS a nested struct of "ImplementationGuide"
        // So we need to be more specific about when to exclude "Definition"

        // Only exclude "Definition" for known cases where it's a separate FHIR type
        if remainder == "Definition" {
            // Known cases where XyzDefinition is a separate FHIR type, not a nested struct of Xyz
            let separate_definition_parents = [
                "Element",    // ElementDefinition is separate from Element
                "Activity",   // ActivityDefinition is separate from Activity
                "Event",      // EventDefinition is separate from Event
                "Plan",       // PlanDefinition is separate from Plan
                "Message",    // MessageDefinition is separate from Message
                "Operation",  // OperationDefinition is separate from Operation
                "Search",     // SearchDefinition could be separate from Search
                "Capability", // CapabilityDefinition could be separate from Capability
            ];

            if separate_definition_parents.contains(&parent_name) {
                return false;
            }
        }

        // Similarly, only exclude "definition" (lowercase) for the same known cases
        if remainder == "definition" {
            let separate_definition_parents = [
                "Element",
                "Activity",
                "Event",
                "Plan",
                "Message",
                "Operation",
                "Search",
                "Capability",
            ];

            if separate_definition_parents
                .iter()
                .any(|&p| parent_name.eq_ignore_ascii_case(p))
            {
                return false;
            }
        }

        // Keep the check for remainders that START with Definition/definition for other cases
        // but be more careful about it
        if remainder.starts_with("Definition") || remainder.starts_with("definition") {
            // Only exclude if this looks like a compound name where Definition is the main type
            // e.g., "ElementDefinitionBinding" should not be child of "Element"
            let separate_definition_parents = [
                "Element",
                "Activity",
                "Event",
                "Plan",
                "Message",
                "Operation",
                "Search",
                "Capability",
            ];

            if separate_definition_parents.contains(&parent_name) {
                return false;
            }
        }

        true
    }

    /// Collect nested structs from the type cache that are related to a main struct
    pub fn collect_nested_structs(
        struct_name: &str,
        type_cache: &HashMap<String, RustStruct>,
    ) -> Vec<RustStruct> {
        let mut nested_structs = vec![];

        // Find all nested structs that start with the main struct name
        // but ensure they are direct children, not just sharing a prefix
        for (cached_name, cached_struct) in type_cache {
            if cached_name != struct_name && Self::is_direct_nested_struct(struct_name, cached_name)
            {
                // Debug output for ImplementationGuide
                if struct_name == "ImplementationGuide" {
                    println!(
                        "DEBUG: Found nested struct '{}' for parent '{}'",
                        cached_name, struct_name
                    );
                }
                nested_structs.push(cached_struct.clone());
            }
        }

        // Debug output for ImplementationGuide
        if struct_name == "ImplementationGuide" {
            println!(
                "DEBUG: Total nested structs for '{}': {}",
                struct_name,
                nested_structs.len()
            );
            for ns in &nested_structs {
                println!("DEBUG: - {}", ns.name);
            }
        }

        nested_structs
    }

    /// Classify a FHIR StructureDefinition into the appropriate category
    pub fn classify_fhir_structure_def(
        &self,
        structure_def: &StructureDefinition,
    ) -> FhirTypeCategory {
        let file_generator = FileGenerator::new(self.config, self.token_generator);
        file_generator.classify_fhir_structure_def(structure_def)
    }

    /// Ensure a directory exists, creating it if necessary
    pub fn ensure_directory_exists<P: AsRef<Path>>(dir_path: P) -> CodegenResult<()> {
        std::fs::create_dir_all(dir_path).map_err(CodegenError::Io)
    }

    /// Generate the appropriate file path for a structure definition
    pub fn get_output_path_for_structure<P: AsRef<Path>>(
        base_dir: P,
        structure_def: &StructureDefinition,
        file_generator: &FileGenerator,
    ) -> std::path::PathBuf {
        let base_dir = base_dir.as_ref();

        // Determine the appropriate subdirectory based on FHIR type
        let target_dir = match file_generator.classify_fhir_structure_def(structure_def) {
            FhirTypeCategory::Resource => base_dir.join("src").join("resource"),
            FhirTypeCategory::DataType => base_dir.join("src").join("datatypes"),
            FhirTypeCategory::Extension => base_dir.join("src").join("extensions"),
            FhirTypeCategory::Primitive => base_dir.join("src").join("primitives"),
        };

        let filename = crate::naming::Naming::filename(structure_def);
        target_dir.join(filename)
    }

    /// Generate the traits directory path
    pub fn get_traits_directory_path<P: AsRef<Path>>(base_dir: P) -> std::path::PathBuf {
        base_dir.as_ref().join("src").join("traits")
    }

    /// Generate the trait file path for a structure definition
    pub fn get_trait_file_path<P: AsRef<Path>>(
        base_dir: P,
        structure_def: &StructureDefinition,
    ) -> std::path::PathBuf {
        let traits_dir = Self::get_traits_directory_path(base_dir);
        let struct_name = crate::naming::Naming::struct_name(structure_def);
        let snake_case_name = crate::naming::Naming::to_snake_case(&struct_name);
        let filename = format!("{snake_case_name}.rs");
        traits_dir.join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CodegenConfig;
    use crate::generators::TokenGenerator;
    use tempfile::TempDir;

    #[test]
    fn test_load_structure_definition() {
        use std::fs;

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_structure.json");

        // Create a minimal valid StructureDefinition JSON manually
        let json_content = r#"{
            "resourceType": "StructureDefinition",
            "id": "Patient",
            "url": "http://hl7.org/fhir/StructureDefinition/Patient",
            "name": "Patient",
            "title": "Patient",
            "status": "active",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "description": "A patient resource",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource"
        }"#;

        fs::write(&file_path, json_content).unwrap();

        // Test loading
        let result = FileIoManager::load_structure_definition(&file_path);
        assert!(
            result.is_ok(),
            "Should load StructureDefinition successfully"
        );

        let loaded_structure = result.unwrap();
        assert_eq!(loaded_structure.name, "Patient");
        assert_eq!(loaded_structure.kind, "resource");
    }

    #[test]
    fn test_load_structure_definition_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        // Write invalid JSON
        fs::write(&file_path, "{ invalid json }").unwrap();

        let result = FileIoManager::load_structure_definition(&file_path);
        assert!(result.is_err(), "Should fail to load invalid JSON");
    }

    #[test]
    fn test_load_structure_definition_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("missing.json");

        let result = FileIoManager::load_structure_definition(&file_path);
        assert!(result.is_err(), "Should fail to load missing file");
    }

    #[test]
    fn test_collect_nested_structs() {
        let mut type_cache = HashMap::new();

        // Add main struct
        let patient_struct = RustStruct::new("Patient".to_string());
        type_cache.insert("Patient".to_string(), patient_struct);

        // Add nested structs
        let patient_contact_struct = RustStruct::new("PatientContact".to_string());
        type_cache.insert("PatientContact".to_string(), patient_contact_struct);

        let patient_link_struct = RustStruct::new("PatientLink".to_string());
        type_cache.insert("PatientLink".to_string(), patient_link_struct);

        // Add unrelated struct
        let observation_struct = RustStruct::new("Observation".to_string());
        type_cache.insert("Observation".to_string(), observation_struct);

        let nested_structs = FileIoManager::collect_nested_structs("Patient", &type_cache);

        assert_eq!(
            nested_structs.len(),
            2,
            "Should collect exactly 2 nested structs"
        );

        let nested_names: Vec<String> = nested_structs.iter().map(|s| s.name.clone()).collect();
        assert!(nested_names.contains(&"PatientContact".to_string()));
        assert!(nested_names.contains(&"PatientLink".to_string()));
        assert!(!nested_names.contains(&"Patient".to_string())); // Shouldn't include main struct
        assert!(!nested_names.contains(&"Observation".to_string())); // Shouldn't include unrelated struct
    }

    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let new_dir = temp_dir.path().join("nested").join("directory");

        // Directory shouldn't exist initially
        assert!(!new_dir.exists());

        let result = FileIoManager::ensure_directory_exists(&new_dir);
        assert!(result.is_ok(), "Should create directory successfully");

        // Directory should now exist
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_get_output_path_for_structure() {
        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new();
        let file_generator = FileGenerator::new(&config, &token_generator);

        let temp_dir = TempDir::new().unwrap();

        // Create test JSON content manually
        let json_content = r#"{
            "resourceType": "StructureDefinition",
            "id": "Patient",
            "url": "http://hl7.org/fhir/StructureDefinition/Patient",
            "name": "Patient",
            "title": "Patient",
            "status": "active",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "description": "A patient resource",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource"
        }"#;

        // Parse the structure definition
        let patient_structure: StructureDefinition = serde_json::from_str(json_content).unwrap();

        let output_path = FileIoManager::get_output_path_for_structure(
            temp_dir.path(),
            &patient_structure,
            &file_generator,
        );

        let expected_path = temp_dir
            .path()
            .join("src")
            .join("resource")
            .join("patient.rs");
        assert_eq!(output_path, expected_path);
    }

    #[test]
    fn test_get_trait_file_path() {
        let temp_dir = TempDir::new().unwrap();

        // Create test JSON content manually
        let json_content = r#"{
            "resourceType": "StructureDefinition",
            "id": "Patient",
            "url": "http://hl7.org/fhir/StructureDefinition/Patient",
            "name": "Patient",
            "title": "Patient",
            "status": "active",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "description": "A patient resource",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource"
        }"#;

        // Parse the structure definition
        let patient_structure: StructureDefinition = serde_json::from_str(json_content).unwrap();

        let trait_path = FileIoManager::get_trait_file_path(temp_dir.path(), &patient_structure);
        let expected_path = temp_dir
            .path()
            .join("src")
            .join("traits")
            .join("patient.rs");
        assert_eq!(trait_path, expected_path);
    }

    #[test]
    fn test_file_io_manager_creation() {
        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new();
        let file_io_manager = FileIoManager::new(&config, &token_generator);

        // Test that the manager can be created successfully
        // This is mainly to ensure the lifetime parameters work correctly
        // If this compiles and runs, the test passes
        assert_eq!(
            std::mem::size_of_val(&file_io_manager),
            std::mem::size_of::<FileIoManager>()
        );
    }

    #[test]
    fn test_nested_struct_collection_fix() {
        use crate::rust_types::RustStruct;
        use std::collections::HashMap;

        let mut type_cache = HashMap::new();

        // Create main structs
        let element_struct = RustStruct::new("Element".to_string());
        let element_definition_struct = RustStruct::new("ElementDefinition".to_string());

        // Create nested structs for Element
        let element_extension_struct = RustStruct::new("ElementExtension".to_string());
        let element_binding_struct = RustStruct::new("ElementBinding".to_string());

        // Create nested structs for ElementDefinition (these should NOT be collected for Element)
        let element_definition_binding_struct =
            RustStruct::new("ElementDefinitionBinding".to_string());
        let element_definition_constraint_struct =
            RustStruct::new("ElementDefinitionConstraint".to_string());
        let element_definition_type_struct = RustStruct::new("ElementDefinitionType".to_string());

        // Add all structs to cache
        type_cache.insert("Element".to_string(), element_struct);
        type_cache.insert("ElementDefinition".to_string(), element_definition_struct);
        type_cache.insert("ElementExtension".to_string(), element_extension_struct);
        type_cache.insert("ElementBinding".to_string(), element_binding_struct);
        type_cache.insert(
            "ElementDefinitionBinding".to_string(),
            element_definition_binding_struct,
        );
        type_cache.insert(
            "ElementDefinitionConstraint".to_string(),
            element_definition_constraint_struct,
        );
        type_cache.insert(
            "ElementDefinitionType".to_string(),
            element_definition_type_struct,
        );

        // Add lowercase version of ElementDefinition structs (as they appear in real FHIR data)
        let elementdefinition_binding_struct =
            RustStruct::new("ElementdefinitionBinding".to_string());
        type_cache.insert(
            "ElementdefinitionBinding".to_string(),
            elementdefinition_binding_struct,
        );
        let elementdefinition_constraint_struct =
            RustStruct::new("ElementdefinitionConstraint".to_string());
        type_cache.insert(
            "ElementdefinitionConstraint".to_string(),
            elementdefinition_constraint_struct,
        );
        let elementdefinition_type_struct = RustStruct::new("ElementdefinitionType".to_string());
        type_cache.insert(
            "ElementdefinitionType".to_string(),
            elementdefinition_type_struct,
        );

        // Test collecting nested structs for Element
        let element_nested = FileIoManager::collect_nested_structs("Element", &type_cache);

        // Element should only collect ElementExtension and ElementBinding,
        // NOT ElementDefinition* structs
        assert_eq!(
            element_nested.len(),
            2,
            "Element should have 2 nested structs"
        );

        let element_nested_names: Vec<String> =
            element_nested.iter().map(|s| s.name.clone()).collect();
        assert!(element_nested_names.contains(&"ElementExtension".to_string()));
        assert!(element_nested_names.contains(&"ElementBinding".to_string()));
        assert!(!element_nested_names.contains(&"ElementDefinitionBinding".to_string()));
        assert!(!element_nested_names.contains(&"ElementDefinitionConstraint".to_string()));
        assert!(!element_nested_names.contains(&"ElementDefinitionType".to_string()));
        assert!(!element_nested_names.contains(&"ElementDefinition".to_string()));
        // Test that lowercase versions are also excluded
        assert!(!element_nested_names.contains(&"ElementdefinitionBinding".to_string()));
        assert!(!element_nested_names.contains(&"ElementdefinitionConstraint".to_string()));
        assert!(!element_nested_names.contains(&"ElementdefinitionType".to_string()));

        // Test collecting nested structs for ElementDefinition
        let element_definition_nested =
            FileIoManager::collect_nested_structs("ElementDefinition", &type_cache);

        // ElementDefinition should collect all ElementDefinition* structs (uppercase),
        // but NOT Elementdefinition* structs (lowercase) because they are separate entities
        assert_eq!(
            element_definition_nested.len(),
            3,
            "ElementDefinition should have 3 nested structs (only uppercase)"
        );

        let element_definition_nested_names: Vec<String> = element_definition_nested
            .iter()
            .map(|s| s.name.clone())
            .collect();
        assert!(element_definition_nested_names.contains(&"ElementDefinitionBinding".to_string()));
        assert!(
            element_definition_nested_names.contains(&"ElementDefinitionConstraint".to_string())
        );
        assert!(element_definition_nested_names.contains(&"ElementDefinitionType".to_string()));
        // Test that lowercase versions are NOT collected by ElementDefinition because they are separate entities
        assert!(!element_definition_nested_names.contains(&"ElementdefinitionBinding".to_string()));
        assert!(
            !element_definition_nested_names.contains(&"ElementdefinitionConstraint".to_string())
        );
        assert!(!element_definition_nested_names.contains(&"ElementdefinitionType".to_string()));
        assert!(!element_definition_nested_names.contains(&"ElementExtension".to_string()));
        assert!(!element_definition_nested_names.contains(&"ElementBinding".to_string()));

        // Test some edge cases to ensure the logic is robust

        // Test that Bundle correctly collects BundleEntry but not BundleDefinition (if it existed)
        type_cache.insert("Bundle".to_string(), RustStruct::new("Bundle".to_string()));
        type_cache.insert(
            "BundleEntry".to_string(),
            RustStruct::new("BundleEntry".to_string()),
        );
        type_cache.insert(
            "BundleLink".to_string(),
            RustStruct::new("BundleLink".to_string()),
        );

        let bundle_nested = FileIoManager::collect_nested_structs("Bundle", &type_cache);
        assert_eq!(
            bundle_nested.len(),
            2,
            "Bundle should have 2 nested structs"
        );

        let bundle_nested_names: Vec<String> =
            bundle_nested.iter().map(|s| s.name.clone()).collect();
        assert!(bundle_nested_names.contains(&"BundleEntry".to_string()));
        assert!(bundle_nested_names.contains(&"BundleLink".to_string()));
    }
}
