//! FHIR type generation functionality
//!
//! This module contains the main code generator that orchestrates the generation of Rust types
//! from FHIR StructureDefinitions using specialized sub-generators.

#![allow(dead_code)] // Allow unused code during refactoring transition

use std::collections::HashMap;
use std::path::Path;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::token_generator::TokenGenerator;
#[cfg(test)]
use crate::generators::ImportManager;
use crate::generators::{
    EnumGenerator, FieldGenerator, FileGenerator, FileIoManager, NestedStructGenerator,
    PrimitiveGenerator, StructGenerator, TraitGenerator, TypeUtilities,
};
use crate::rust_types::{RustEnum, RustStruct, RustTrait};
use crate::value_sets::ValueSetManager;
use crate::CodegenResult;

// Re-export from file_generator for backward compatibility
pub use crate::generators::file_generator::FhirTypeCategory;

/// Main code generator struct that orchestrates specialized generators
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
    #[allow(dead_code)] // Methods below are preserved during refactoring transition
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
    pub fn new_with_value_set_directory<P: AsRef<Path>>(
        config: CodegenConfig,
        value_set_dir: P,
    ) -> Self {
        let value_set_manager = ValueSetManager::new_with_directory(value_set_dir);
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
        FileIoManager::load_structure_definition(path)
    }

    /// Generate a Rust struct from a FHIR StructureDefinition
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        let mut struct_generator = StructGenerator::new(
            &self.config,
            &mut self.type_cache,
            &mut self.value_set_manager,
        );
        let rust_struct = struct_generator.generate_struct(structure_def)?;

        Ok(rust_struct)
    }

    /// Generate traits for a structure definition
    pub fn generate_trait(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<Vec<RustTrait>> {
        let mut trait_generator = TraitGenerator::new();
        let mut traits = Vec::new();
        let categories = ["Accessors", "Mutators"]; // Add "Existence" later

        for category in &categories {
            let rust_trait = trait_generator.generate_trait(structure_def, category)?;
            traits.push(rust_trait);
        }

        Ok(traits)
    }

    /// Generate a primitive type struct with special FHIR primitive type semantics
    fn generate_primitive_type_struct(
        &mut self,
        structure_def: &StructureDefinition,
        rust_struct: RustStruct,
    ) -> CodegenResult<RustStruct> {
        let mut primitive_generator = PrimitiveGenerator::new(&self.config, &mut self.type_cache);
        primitive_generator.generate_primitive_type_struct(structure_def, rust_struct)
    }

    /// Generate a type alias for primitive types
    fn generate_primitive_type_alias(
        &self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<crate::rust_types::RustTypeAlias> {
        let mut temp_cache = HashMap::new();
        let primitive_generator = PrimitiveGenerator::new(&self.config, &mut temp_cache);
        primitive_generator.generate_primitive_type_alias(structure_def)
    }

    /// Generate the companion Element struct for a primitive type
    fn generate_primitive_element_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        let mut primitive_generator = PrimitiveGenerator::new(&self.config, &mut self.type_cache);
        primitive_generator.generate_primitive_element_struct(structure_def)
    }

    /// Generate a nested struct for BackboneElements
    fn generate_nested_struct(
        &mut self,
        parent_struct_name: &str,
        nested_field_name: &str,
        nested_elements: &[crate::fhir_types::ElementDefinition],
        parent_structure_def: &StructureDefinition,
    ) -> CodegenResult<Option<crate::rust_types::RustStruct>> {
        let mut nested_struct_generator = NestedStructGenerator::new(
            &self.config,
            &mut self.type_cache,
            &mut self.value_set_manager,
        );
        nested_struct_generator.generate_nested_struct(
            parent_struct_name,
            nested_field_name,
            nested_elements,
            parent_structure_def,
        )
    }

    /// Create a RustField from an ElementDefinition
    fn create_field_from_element(
        &mut self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> CodegenResult<Option<crate::rust_types::RustField>> {
        let mut field_generator =
            FieldGenerator::new(&self.config, &self.type_cache, &mut self.value_set_manager);
        field_generator.create_field_from_element(element)
    }

    /// Convert a FHIR field name to a valid Rust field name
    fn to_rust_field_name(&self, name: &str) -> String {
        crate::naming::Naming::field_name(name)
    }

    /// Generate a Rust struct and write it to the appropriate directory based on FHIR type classification
    pub fn generate_to_organized_directories<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
    ) -> CodegenResult<()> {
        let rust_struct = self.generate_struct(structure_def)?;
        let nested_structs =
            FileIoManager::collect_nested_structs(&rust_struct.name, &self.type_cache);

        let file_io_manager = FileIoManager::new(&self.config, &self.token_generator);
        file_io_manager.generate_to_organized_directories(
            structure_def,
            base_output_dir,
            &rust_struct,
            &nested_structs,
        )
    }

    /// Generate traits and write them to the traits directory
    pub fn generate_trait_to_organized_directory<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
    ) -> CodegenResult<()> {
        let rust_traits = self.generate_trait(structure_def)?;

        let file_io_manager = FileIoManager::new(&self.config, &self.token_generator);
        file_io_manager.generate_traits_to_organized_directory(
            structure_def,
            base_output_dir.as_ref(),
            &rust_traits,
        )
    }

    /// Classify a FHIR StructureDefinition into the appropriate category
    pub fn classify_fhir_structure_def(
        &self,
        structure_def: &StructureDefinition,
    ) -> FhirTypeCategory {
        let file_generator = FileGenerator::new(&self.config, &self.token_generator);
        file_generator.classify_fhir_structure_def(structure_def)
    }

    /// Check if a type name represents a known FHIR data type
    fn is_fhir_datatype(&self, name: &str) -> bool {
        TypeUtilities::is_fhir_datatype(name)
    }

    /// Generate a Rust struct and write it to a file
    pub fn generate_to_file<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: P,
    ) -> CodegenResult<()> {
        if structure_def.kind == "primitive-type" {
            // For primitive types, use empty placeholder values
            let empty_struct = RustStruct::new("".to_string());
            let nested_structs = vec![];
            let file_io_manager = FileIoManager::new(&self.config, &self.token_generator);
            file_io_manager.generate_to_file(
                structure_def,
                output_path,
                &empty_struct,
                &nested_structs,
            )
        } else {
            // Generate the main struct for non-primitive types
            let rust_struct = self.generate_struct(structure_def)?;
            let nested_structs =
                FileIoManager::collect_nested_structs(&rust_struct.name, &self.type_cache);
            let file_io_manager = FileIoManager::new(&self.config, &self.token_generator);

            file_io_manager.generate_to_file(
                structure_def,
                output_path,
                &rust_struct,
                &nested_structs,
            )
        }
    }

    /// Generate a Rust trait and write it to a file
    pub fn generate_trait_to_file<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: P,
    ) -> CodegenResult<()> {
        // Generate the trait first
        let rust_traits = self.generate_trait(structure_def)?;

        // Create FileIoManager and delegate
        let file_io_manager = FileIoManager::new(&self.config, &self.token_generator);

        // Use generate_traits_to_file to write all traits to the same file
        file_io_manager.generate_traits_to_file(
            structure_def,
            output_path.as_ref(),
            &rust_traits,
        )?;

        Ok(())
    }

    /// Generate all ValueSet enums to separate files in the specified directory
    pub fn generate_enum_files<P: AsRef<Path>>(&mut self, enums_dir: P) -> CodegenResult<()> {
        // Create an EnumGenerator with access to the cached enums
        let enum_generator = EnumGenerator::new(&mut self.value_set_manager, &mut self.enum_cache);

        // Create FileGenerator and delegate
        let file_generator = FileGenerator::new(&self.config, &self.token_generator);
        file_generator.generate_enum_files(enums_dir, &enum_generator)
    }

    /// Generate a mod.rs file that re-exports all the enum modules
    pub fn generate_enums_mod_file<P: AsRef<Path>>(&self, enums_dir: P) -> CodegenResult<()> {
        // Create an EnumGenerator with access to the cached enums
        let mut value_set_manager = self.value_set_manager.clone(); // Need to clone for borrow checker
        let mut enum_cache = self.enum_cache.clone();
        let enum_generator = EnumGenerator::new(&mut value_set_manager, &mut enum_cache);

        // Create FileGenerator and delegate
        let file_generator = FileGenerator::new(&self.config, &self.token_generator);
        file_generator.generate_enums_mod_file(enums_dir, &enum_generator)
    }

    /// Generate an enum for a value set binding
    pub fn generate_enum_for_value_set(
        &mut self,
        value_set_url: &str,
    ) -> CodegenResult<Option<RustEnum>> {
        let mut enum_generator =
            EnumGenerator::new(&mut self.value_set_manager, &mut self.enum_cache);
        let result = enum_generator.generate_enum_for_value_set(value_set_url)?;

        Ok(result)
    }

    /// Check if any ValueSet enums have been generated
    pub fn has_cached_enums(&self) -> bool {
        TypeUtilities::has_cached_enums(&self.value_set_manager)
    }

    /// Convert a FHIR resource type name to filename using snake_case
    pub fn to_filename(&self, structure_def: &StructureDefinition) -> String {
        crate::naming::Naming::filename(structure_def)
    }

    /// Generate a comprehensive Resource trait with common FHIR resource methods
    // pub fn generate_resource_trait(&mut self) -> CodegenResult<String> {
    //     let mut trait_generator = TraitGenerator::new();
    //     trait_generator.generate_resource_trait()
    // }

    /// Generate a trait file directly from a RustTrait object
    pub fn generate_trait_file_from_trait<P: AsRef<Path>>(
        &self,
        rust_trait: &RustTrait,
        output_path: P,
    ) -> CodegenResult<()> {
        // Create FileGenerator and delegate
        let file_generator = FileGenerator::new(&self.config, &self.token_generator);
        file_generator.generate_trait_file_from_trait(rust_trait, output_path)
    }

    // Generate an impl block for the Resource trait
    // pub fn generate_resource_impl(&self) -> String {
    //     let trait_generator = TraitGenerator::new();
    //     trait_generator.generate_resource_impl()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_valid_rust_identifier_conversion() {
        // Test FHIR resource names that should preserve original case
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("StructureDefinition"),
            "StructureDefinition"
        );
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("Patient"),
            "Patient"
        );
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("Observation"),
            "Observation"
        );
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("CodeSystem"),
            "CodeSystem"
        );

        // Test names that need conversion due to special characters
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("patient"),
            "patient"
        );

        // Test names with spaces
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("Relative Date Criteria"),
            "RelativeDateCriteria"
        );
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("Care Plan"),
            "CarePlan"
        );

        // Test names with dashes and underscores
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("patient-name"),
            "PatientName"
        );
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("patient_name"),
            "patient_name"
        );

        // Test mixed separators
        assert_eq!(
            crate::naming::Naming::to_rust_identifier("some-complex_name with.spaces"),
            "SomeComplexNameWithSpaces"
        );

        // Test empty and edge cases
        assert_eq!(crate::naming::Naming::to_rust_identifier(""), "_");
        assert_eq!(crate::naming::Naming::to_rust_identifier("   "), "_");
        assert_eq!(crate::naming::Naming::to_rust_identifier("a"), "a");
    }

    #[test]
    fn test_logical_model_skipping() {
        use crate::fhir_types::StructureDefinition;

        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a test LogicalModel StructureDefinition
        let logical_model = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-logical-model".to_string(),
            url: "http://example.org/fhir/StructureDefinition/test-logical-model".to_string(),
            name: "test-logical-model".to_string(),
            title: Some("Test Logical Model".to_string()),
            status: "active".to_string(),
            kind: "logical".to_string(),
            is_abstract: false,
            description: Some("A test logical model".to_string()),
            purpose: None,
            base_type: "Base".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Base".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test that LogicalModels are rejected
        let result = generator.generate_struct(&logical_model);
        assert!(result.is_err());

        if let Err(crate::CodegenError::Generation { message }) = result {
            assert!(message.contains("Skipping LogicalModel"));
            assert!(message.contains("test-logical-model"));
        } else {
            panic!("Expected CodegenError::Generation for LogicalModel");
        }
    }

    #[test]
    fn test_nested_struct_generation() {
        use crate::fhir_types::{
            ElementDefinition, ElementType, StructureDefinition, StructureDefinitionDifferential,
        };

        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

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

        // Generate the struct
        let result = generator.generate_struct(&bundle_structure);
        assert!(result.is_ok());

        let bundle_struct = result.unwrap();

        // Check that the main Bundle struct was generated
        assert_eq!(bundle_struct.name, "Bundle");

        // Check that an entry field exists
        let entry_field = bundle_struct.fields.iter().find(|f| f.name == "entry");
        assert!(entry_field.is_some(), "Bundle should have an entry field");

        // Check that the nested BundleEntry struct was generated and cached
        assert!(
            generator.type_cache.contains_key("BundleEntry"),
            "BundleEntry struct should be generated"
        );

        let bundle_entry_struct = generator.type_cache.get("BundleEntry").unwrap();
        assert_eq!(bundle_entry_struct.name, "BundleEntry");

        // Check that BundleEntry has the expected fields
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
    fn test_primitive_type_naming() {
        use crate::fhir_types::StructureDefinition;

        // Test primitive type - should not be capitalized
        let primitive_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "string".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/string".to_string(),
            name: "string".to_string(),
            title: Some("string".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("A sequence of Unicode characters".to_string()),
            purpose: None,
            base_type: "string".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test that primitive types are not capitalized
        let struct_name = crate::naming::Naming::struct_name(&primitive_structure);
        assert_eq!(
            struct_name, "string",
            "Primitive type 'string' should not be capitalized"
        );

        let filename = crate::naming::Naming::filename(&primitive_structure);
        assert_eq!(
            filename, "string.rs",
            "Primitive type filename should not be capitalized"
        );

        // Test another primitive type
        let boolean_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "boolean".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/boolean".to_string(),
            name: "boolean".to_string(),
            title: Some("boolean".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("Value of 'true' or 'false'".to_string()),
            purpose: None,
            base_type: "boolean".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let struct_name = crate::naming::Naming::struct_name(&boolean_structure);
        assert_eq!(
            struct_name, "boolean",
            "Primitive type 'boolean' should not be capitalized"
        );

        // Test complex type - should be capitalized
        let complex_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Period".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Period".to_string(),
            name: "Period".to_string(),
            title: Some("Period".to_string()),
            status: "active".to_string(),
            kind: "complex-type".to_string(),
            is_abstract: false,
            description: Some("A time period defined by a start and end date".to_string()),
            purpose: None,
            base_type: "Period".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let struct_name = crate::naming::Naming::struct_name(&complex_structure);
        assert_eq!(
            struct_name, "Period",
            "Complex type 'Period' should be capitalized"
        );
    }

    #[test]
    fn test_primitive_type_generation() {
        use crate::fhir_types::StructureDefinition;
        use crate::rust_types::RustType;

        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Test primitive type generation
        let uri_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "uri".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/uri".to_string(),
            name: "uri".to_string(),
            title: Some("uri".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some(
                "String of characters used to identify a name or a resource".to_string(),
            ),
            purpose: None,
            base_type: "uri".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test that primitive type alias is generated correctly
        let type_alias_result = generator.generate_primitive_type_alias(&uri_structure);
        assert!(
            type_alias_result.is_ok(),
            "Should generate primitive type alias successfully"
        );

        let uri_type_alias = type_alias_result.unwrap();

        // Check that the type alias name follows the new PascalCase Type suffix convention
        assert_eq!(
            uri_type_alias.name, "UriType",
            "Primitive type alias should use PascalCase with Type suffix"
        );

        // Check that the type alias target is String for uri
        if let RustType::String = uri_type_alias.target_type {
            // Expected
        } else {
            panic!(
                "URI primitive type alias should target String, got: {:?}",
                uri_type_alias.target_type
            );
        }

        // Test boolean primitive type
        let boolean_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "boolean".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/boolean".to_string(),
            name: "boolean".to_string(),
            title: Some("boolean".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("Value of 'true' or 'false'".to_string()),
            purpose: None,
            base_type: "boolean".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        let type_alias_result = generator.generate_primitive_type_alias(&boolean_structure);
        assert!(
            type_alias_result.is_ok(),
            "Should generate boolean primitive type alias successfully"
        );

        let boolean_type_alias = type_alias_result.unwrap();

        // Check that the boolean type alias targets bool
        if let RustType::Boolean = boolean_type_alias.target_type {
            // Expected
        } else {
            panic!(
                "Boolean primitive type alias should target bool, got: {:?}",
                boolean_type_alias.target_type
            );
        }

        // Test that the companion Element struct is generated
        let element_struct = generator.generate_primitive_element_struct(&uri_structure);
        assert!(
            element_struct.is_ok(),
            "Should generate companion Element struct successfully"
        );

        let element_struct = element_struct.unwrap();
        assert_eq!(
            element_struct.name, "_uri",
            "Companion Element struct should be named '_uri'"
        );
        assert_eq!(
            element_struct.base_definition,
            Some("Element".to_string()),
            "Companion Element struct should inherit from Element"
        );
    }

    #[test]
    fn test_trait_generation() {
        use crate::fhir_types::{
            ElementDefinition, ElementType, StructureDefinition, StructureDefinitionDifferential,
        };

        let config = CodegenConfig::default();
        let mut generator = CodeGenerator::new(config);

        // Create a simplified Patient StructureDefinition for trait generation
        let patient_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("Demographics and other administrative information about an individual receiving care.".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            version: None,
            differential: Some(StructureDefinitionDifferential {
                element: vec![
                    ElementDefinition {
                        id: Some("Patient.active".to_string()),
                        path: "Patient.active".to_string(),
                        short: Some("Whether this patient record is in active use".to_string()),
                        definition: Some("Whether this patient record is in active use".to_string()),
                        min: Some(0),
                        max: Some("1".to_string()),
                        element_type: Some(vec![ElementType {
                            code: Some("boolean".to_string()),
                            target_profile: None,
                        }]),
                        fixed: None,
                        pattern: None,
                        binding: None,
                    },
                    ElementDefinition {
                        id: Some("Patient.name".to_string()),
                        path: "Patient.name".to_string(),
                        short: Some("A name associated with the patient".to_string()),
                        definition: Some("A name associated with the patient".to_string()),
                        min: Some(0),
                        max: Some("*".to_string()),
                        element_type: Some(vec![ElementType {
                            code: Some("HumanName".to_string()),
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

        // Generate the trait
        let result = generator.generate_trait(&patient_structure);
        assert!(result.is_ok(), "Should generate Patient trait successfully");

        let patient_traits = result.unwrap();
        let patient_trait = patient_traits
            .iter()
            .find(|t| t.name == "PatientAccessors")
            .expect("PatientAccessors trait not found");

        assert_eq!(
            patient_trait.name, "PatientAccessors",
            "Trait should be named 'PatientAccessors'"
        );

        // Check that the Patient trait properly inherits from DomainResource
        assert!(
            patient_trait
                .super_traits
                .contains(&"DomainResourceAccessors".to_string()),
            "Patient trait should inherit from DomainResourceAccessors"
        );

        // The Patient trait should NOT have methods that are inherited from parent traits
        let has_extensions = patient_trait.methods.iter().any(|m| m.name == "extensions");
        assert!(
            !has_extensions,
            "Patient trait should NOT have extensions method - it should be inherited from Resource"
        );

        let has_narrative = patient_trait.methods.iter().any(|m| m.name == "narrative");
        assert!(
            !has_narrative,
            "Patient trait should NOT have narrative method - it should be inherited from DomainResource"
        );

        let has_id = patient_trait.methods.iter().any(|m| m.name == "id");
        assert!(
            !has_id,
            "Patient trait should NOT have id method - it should be inherited from Resource"
        );

        // Note: The new trait generator focuses on resource-level methods rather than
        // field-specific methods like 'active' and 'name', which are handled by struct implementations
    }

    #[test]
    fn test_filename_generation() {
        // Test PascalCase struct names generate snake_case filenames
        let patient_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A patient resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        let observation_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Observation".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
            name: "Observation".to_string(),
            title: Some("Observation".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("An observation resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test that struct names remain PascalCase
        let patient_struct_name = crate::naming::Naming::struct_name(&patient_structure);
        assert_eq!(patient_struct_name, "Patient");

        let observation_struct_name = crate::naming::Naming::struct_name(&observation_structure);
        assert_eq!(observation_struct_name, "Observation");

        // Test that filenames are snake_case
        let patient_filename = crate::naming::Naming::filename(&patient_structure);
        assert_eq!(patient_filename, "patient.rs");

        let observation_filename = crate::naming::Naming::filename(&observation_structure);
        assert_eq!(observation_filename, "observation.rs");

        // Test more complex PascalCase names
        let structure_definition = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "StructureDefinition".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/StructureDefinition".to_string(),
            name: "StructureDefinition".to_string(),
            title: Some("StructureDefinition".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A structure definition".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        let struct_def_struct_name = crate::naming::Naming::struct_name(&structure_definition);
        assert_eq!(struct_def_struct_name, "StructureDefinition");

        let struct_def_filename = crate::naming::Naming::filename(&structure_definition);
        assert_eq!(struct_def_filename, "structure_definition.rs");

        // Test enum filename generation
        let enum_filename = crate::naming::Naming::enum_filename("AdministrativeGender");
        assert_eq!(enum_filename, "administrative_gender.rs");

        let enum_module_name = crate::naming::Naming::module_name("AdministrativeGender");
        assert_eq!(enum_module_name, "administrative_gender");
    }

    #[test]
    fn test_import_classification() {
        // Test resource classification
        assert!(ImportManager::is_fhir_resource_type("DomainResource"));
        assert!(ImportManager::is_fhir_resource_type("Patient"));
        assert!(ImportManager::is_fhir_resource_type("ActivityDefinition"));
        assert!(!ImportManager::is_fhir_resource_type("Identifier"));

        // Test datatype classification
        assert!(ImportManager::is_fhir_datatype("Identifier"));
        assert!(ImportManager::is_fhir_datatype("CodeableConcept"));
        assert!(ImportManager::is_fhir_datatype("Reference"));
        assert!(!ImportManager::is_fhir_datatype("DomainResource"));

        // Test import path generation
        assert_eq!(
            ImportManager::get_import_path_for_type("DomainResource"),
            "crate::resources::domain_resource::DomainResource"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("Identifier"),
            "crate::datatypes::identifier::Identifier"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("PublicationStatus"),
            "crate::bindings::publication_status::PublicationStatus"
        );
    }
}
