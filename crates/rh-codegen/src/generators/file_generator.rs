//! File generation and organization functionality
//!
//! This module handles writing generated code to files and organizing the output structure.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use quote::quote;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::enum_generator::EnumGenerator;
use crate::generators::import_manager::ImportManager;
use crate::generators::primitive_generator::PrimitiveGenerator;
use crate::generators::token_generator::TokenGenerator;
use crate::generators::utils::GeneratorUtils;
use crate::rust_types::{RustStruct, RustTrait};
use crate::{CodegenError, CodegenResult};

/// Classification of FHIR types for organizing into appropriate directories
#[derive(Debug, Clone, PartialEq)]
pub enum FhirTypeCategory {
    Resource,
    DataType,
    Primitive,
}

/// File generator for organizing and writing generated code
pub struct FileGenerator<'a> {
    config: &'a CodegenConfig,
    token_generator: &'a TokenGenerator,
}

impl<'a> FileGenerator<'a> {
    /// Create a new file generator
    pub fn new(config: &'a CodegenConfig, token_generator: &'a TokenGenerator) -> Self {
        Self {
            config,
            token_generator,
        }
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
            FhirTypeCategory::Primitive => base_dir.join("src").join("primitives"),
        };

        // Ensure the target directory exists
        std::fs::create_dir_all(&target_dir).map_err(CodegenError::Io)?;

        // Generate the file in the appropriate directory
        let filename = GeneratorUtils::to_filename(structure_def);
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
        let struct_name = GeneratorUtils::generate_struct_name(structure_def);
        let snake_case_name = GeneratorUtils::to_snake_case(&struct_name);
        let filename = format!("{snake_case_name}.rs");
        let output_path = traits_dir.join(filename);

        self.generate_trait_to_file(structure_def, output_path, rust_trait)
    }

    /// Classify a FHIR StructureDefinition into the appropriate category
    pub fn classify_fhir_structure_def(
        &self,
        structure_def: &StructureDefinition,
    ) -> FhirTypeCategory {
        // Check if it's a primitive type
        if structure_def.kind == "primitive-type" {
            return FhirTypeCategory::Primitive;
        }

        // Check for known data types
        if GeneratorUtils::is_fhir_datatype(&structure_def.name)
            || structure_def.base_type == "Element"
            || structure_def.base_type == "BackboneElement"
            || structure_def.base_type == "DataType"
        {
            return FhirTypeCategory::DataType;
        }

        // Check for resources
        if structure_def.kind == "resource"
            || structure_def.base_type == "Resource"
            || structure_def.base_type == "DomainResource"
        {
            return FhirTypeCategory::Resource;
        }

        // Default to data type for complex types
        if structure_def.kind == "complex-type" {
            return FhirTypeCategory::DataType;
        }

        // Default to resource for unknown types
        FhirTypeCategory::Resource
    }

    /// Generate a Rust struct and write it to a file
    pub fn generate_to_file<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        output_path: P,
        rust_struct: &RustStruct,
        nested_structs: &[RustStruct],
    ) -> CodegenResult<()> {
        // Collect all imports needed for this file
        let mut imports = HashSet::new();

        // Always include serde if enabled
        if self.config.with_serde {
            imports.insert("serde::{Deserialize, Serialize}".to_string());
        }

        // Check if any struct contains macro calls and add necessary imports
        let has_macro_calls = rust_struct
            .fields
            .iter()
            .any(|field| field.macro_call.is_some())
            || nested_structs
                .iter()
                .any(|s| s.fields.iter().any(|field| field.macro_call.is_some()));

        if has_macro_calls {
            // Add the macro imports from the current crate
            imports.insert("crate::{primitive_string, primitive_boolean, primitive_integer, primitive_decimal, primitive_datetime, primitive_date, primitive_time, primitive_uri, primitive_canonical, primitive_base64binary, primitive_instant, primitive_positiveint, primitive_unsignedint, primitive_id, primitive_oid, primitive_uuid, primitive_code, primitive_markdown, primitive_url}".to_string());
        }

        let mut all_tokens = proc_macro2::TokenStream::new();

        if structure_def.kind == "primitive-type" {
            // Generate type alias for primitive types
            let mut type_cache = std::collections::HashMap::new();
            let primitive_generator = PrimitiveGenerator::new(self.config, &mut type_cache);
            let type_alias = primitive_generator.generate_primitive_type_alias(structure_def)?;
            let type_alias_tokens = self.token_generator.generate_type_alias(&type_alias);
            all_tokens.extend(type_alias_tokens);

            // Generate companion Element struct
            let mut type_cache = std::collections::HashMap::new();
            let mut primitive_generator = PrimitiveGenerator::new(self.config, &mut type_cache);
            let element_struct =
                primitive_generator.generate_primitive_element_struct(structure_def)?;
            let element_struct_tokens = self.token_generator.generate_struct(&element_struct);
            all_tokens.extend(element_struct_tokens);
        } else {
            // Generate the main struct for non-primitive types
            let mut all_structs = vec![rust_struct.clone()];
            all_structs.extend(nested_structs.iter().cloned());

            // Collect all struct names that will be in this file
            let structs_in_file: HashSet<String> =
                all_structs.iter().map(|s| s.name.clone()).collect();

            // Collect custom types from all structs, excluding types that are in the same file
            for struct_def in &all_structs {
                ImportManager::collect_custom_types_from_struct(
                    struct_def,
                    &mut imports,
                    &structs_in_file,
                );
            }

            for struct_def in all_structs {
                let struct_tokens = self.token_generator.generate_struct(&struct_def);
                all_tokens.extend(struct_tokens);
            }
        }

        // Generate import tokens AFTER collecting all custom types
        let mut import_tokens = proc_macro2::TokenStream::new();
        for import in &imports {
            let import_token: proc_macro2::TokenStream = format!("use {import};")
                .parse()
                .expect("Invalid import statement");
            import_tokens.extend(import_token);
        }

        // Prepend imports to the tokens
        let mut final_tokens = proc_macro2::TokenStream::new();
        final_tokens.extend(import_tokens);
        final_tokens.extend(all_tokens);

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(final_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated tokens: {e}"),
        })?;

        let mut formatted_code = prettyplease::unparse(&syntax_tree);

        // Add Resource trait impl if this is the Resource struct
        if structure_def.name == "Resource" {
            formatted_code.push_str("\n\n");
            formatted_code.push_str(&self.generate_resource_impl());
        }

        // Check for file collision and warn if overwriting
        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: File '{}' already exists and will be overwritten. This may indicate a naming collision between FHIR StructureDefinitions.",
                output_path.as_ref().display()
            );
        }

        // Write to file
        fs::write(output_path.as_ref(), formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate a Rust trait and write it to a file
    pub fn generate_trait_to_file<P: AsRef<Path>>(
        &self,
        _structure_def: &StructureDefinition,
        output_path: P,
        rust_trait: &RustTrait,
    ) -> CodegenResult<()> {
        // Generate import tokens
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Add common imports for traits
        let import_tokens: proc_macro2::TokenStream = "use std::collections::HashMap;"
            .parse()
            .expect("Invalid import statement");
        all_tokens.extend(import_tokens);

        // Generate the trait
        let trait_tokens = self.token_generator.generate_trait(rust_trait);
        all_tokens.extend(trait_tokens);

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated trait tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Check for file collision and warn if overwriting
        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: Trait file '{}' already exists and will be overwritten.",
                output_path.as_ref().display()
            );
        }

        // Write to file
        fs::write(output_path.as_ref(), formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate all ValueSet enums to separate files in the specified directory
    pub fn generate_enum_files<P: AsRef<Path>>(
        &self,
        enums_dir: P,
        enum_generator: &EnumGenerator,
    ) -> CodegenResult<()> {
        let enums_dir = enums_dir.as_ref();

        // Create the enums directory if it doesn't exist
        if !enums_dir.exists() {
            fs::create_dir_all(enums_dir).map_err(CodegenError::Io)?;
        }

        // Generate a file for each cached enum
        for (enum_name, rust_enum) in enum_generator.get_cached_enums() {
            let enum_filename = EnumGenerator::enum_name_to_filename(enum_name);
            let enum_file_path = enums_dir.join(enum_filename);

            // Generate tokens for this enum with imports
            let import_tokens = quote! {
                use serde::{Deserialize, Serialize};
            };
            let enum_tokens = self.token_generator.generate_enum(rust_enum);
            let combined_tokens = quote! {
                #import_tokens

                #enum_tokens
            };

            // Parse the tokens into a syntax tree and format it
            let syntax_tree =
                syn::parse2(combined_tokens).map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse generated enum tokens for {enum_name}: {e}"),
                })?;

            let formatted_code = prettyplease::unparse(&syntax_tree);

            // Check for file collision and warn if overwriting
            if enum_file_path.exists() {
                eprintln!(
                    "Warning: Enum file '{}' already exists and will be overwritten.",
                    enum_file_path.display()
                );
            }

            // Write enum to its own file
            fs::write(&enum_file_path, formatted_code).map_err(CodegenError::Io)?;
        }

        Ok(())
    }

    /// Generate a mod.rs file that re-exports all the enum modules
    pub fn generate_enums_mod_file<P: AsRef<Path>>(
        &self,
        enums_dir: P,
        enum_generator: &EnumGenerator,
    ) -> CodegenResult<()> {
        let enums_dir = enums_dir.as_ref();
        let mod_file_path = enums_dir.join("mod.rs");

        let mut mod_content = vec![
            "//! FHIR ValueSet enums".to_string(),
            "//!".to_string(),
            "//! This module contains all the generated enums from FHIR ValueSets.".to_string(),
            "//! Each enum represents a specific ValueSet and its allowed codes.".to_string(),
            "".to_string(),
        ];

        // Sort enum names for consistent output
        let mut enum_names: Vec<_> = enum_generator.get_cached_enums().keys().collect();
        enum_names.sort();

        // Generate module declarations and re-exports
        for enum_name in enum_names {
            let module_name = EnumGenerator::enum_name_to_module_name(enum_name);
            mod_content.push(format!("pub mod {module_name};"));
        }

        let final_content = mod_content.join("\n") + "\n";

        // Check for file collision and warn if overwriting
        if mod_file_path.exists() {
            eprintln!(
                "Warning: Enum mod file '{}' already exists and will be overwritten.",
                mod_file_path.display()
            );
        }

        fs::write(&mod_file_path, final_content).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate a trait file directly from a RustTrait object
    pub fn generate_trait_file_from_trait<P: AsRef<Path>>(
        &self,
        rust_trait: &RustTrait,
        output_path: P,
    ) -> CodegenResult<()> {
        // Generate import tokens
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Add common imports for traits
        let import_tokens: proc_macro2::TokenStream = "use std::collections::HashMap;"
            .parse()
            .expect("Invalid import statement");
        all_tokens.extend(import_tokens);

        // Generate the trait tokens
        let trait_tokens = self.token_generator.generate_trait(rust_trait);
        all_tokens.extend(trait_tokens);

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated trait tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Check for file collision and warn if overwriting
        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: Trait file '{}' already exists and will be overwritten.",
                output_path.as_ref().display()
            );
        }

        // Write to file
        fs::write(output_path.as_ref(), formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate a Resource trait implementation for the Resource struct
    fn generate_resource_impl(&self) -> String {
        r#"impl Resource for Resource {
    fn resource_type(&self) -> &'static str {
        "Resource"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn has_id(&self) -> bool {
        self.id.is_some()
    }

    fn meta(&self) -> Option<&crate::datatypes::Meta> {
        self.meta.as_ref()
    }

    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    fn extensions(&self) -> &[crate::datatypes::Extension] {
        &self.extension.as_deref().unwrap_or(&[])
    }

    fn implicit_rules(&self) -> Option<&str> {
        self.implicit_rules.as_deref()
    }

    fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}"#
        .to_string()
    }
}
