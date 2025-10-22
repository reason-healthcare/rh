//! File generation and organization functionality
//!
//! This module handles writing generated code to files and organizing the output structure.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use quote::{format_ident, quote};

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::enum_generator::EnumGenerator;
use crate::generators::import_manager::ImportManager;
use crate::generators::primitive_generator::PrimitiveGenerator;
use crate::generators::token_generator::TokenGenerator;
use crate::generators::trait_impl_generator::TraitImplGenerator;
use crate::generators::utils::GeneratorUtils;
use crate::rust_types::{RustStruct, RustTrait};
use crate::{CodegenError, CodegenResult};

/// Classification of FHIR types for organizing into appropriate directories
#[derive(Debug, Clone, PartialEq)]
pub enum FhirTypeCategory {
    Resource,
    Profile,
    DataType,
    Extension,
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

    /// Generate a macros.rs file with all FHIR primitive macros
    pub fn generate_macros_file<P: AsRef<Path>>(&self, output_path: P) -> CodegenResult<()> {
        let macros_content = include_str!("../macros.rs");

        // Parse and reformat the macros content to ensure proper formatting
        let syntax_tree =
            syn::parse_file(macros_content).map_err(|e| CodegenError::Generation {
                message: format!("Failed to parse macros file: {e}"),
            })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Write to file
        fs::write(output_path, formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate a lib.rs file for the generated crate
    pub fn generate_lib_file<P: AsRef<Path>>(&self, output_path: P) -> CodegenResult<()> {
        let lib_tokens = quote! {
            //! Generated FHIR Rust bindings
            //!
            //! This crate contains Rust types and traits for FHIR resources and data types.
            //! It includes macros for primitive field generation and maintains FHIR compliance.

            // Allow clippy lint for derivable Default implementations
            //
            // TODO: Future optimization - derive Default when possible instead of manual impl
            //
            // Currently, we generate explicit Default implementations for all structs.
            // Many of these could use #[derive(Default)] instead, which would be more idiomatic.
            //
            // Pros of deriving Default:
            // - More idiomatic Rust code
            // - Less generated code (no manual impl blocks)
            // - Clearer intent (all fields use Default::default())
            //
            // Cons of current approach (manual impl):
            // - Clippy warns about 1,100+ derivable implementations
            // - More verbose generated code
            //
            // Pros of current approach:
            // - Explicit and predictable behavior
            // - Handles mixed initialization patterns consistently
            // - Simpler code generation logic
            //
            // To implement derive-based approach would require:
            // 1. Analyze all field types to ensure they implement Default
            // 2. Detect required fields with non-Default initializations (String::new(), Vec::new(), etc.)
            // 3. Add "Default" to struct derives only when ALL fields can use Default::default()
            // 4. Skip manual impl generation for those structs
            //
            #![allow(clippy::derivable_impls)]

            pub mod macros;
            pub mod primitives;
            pub mod datatypes;
            pub mod extensions;
            pub mod resources;
            pub mod traits;
            pub mod bindings;

            // Re-export macros and serde traits for convenience
            pub use macros::*;
            pub use serde::{Deserialize, Serialize};
        };

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(lib_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated lib tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Write to file
        fs::write(output_path, formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate module files (mod.rs) for organized directories
    pub fn generate_module_file<P: AsRef<Path>>(
        &self,
        module_dir: P,
        module_names: &[String],
    ) -> CodegenResult<()> {
        let module_dir = module_dir.as_ref();
        let mod_file_path = module_dir.join("mod.rs");

        let mut mod_tokens = proc_macro2::TokenStream::new();

        // Add module declarations only (no re-exports to avoid conflicts)
        for module_name in module_names {
            let mod_ident = format_ident!("{}", module_name);
            mod_tokens.extend(quote! {
                pub mod #mod_ident;
            });
        }

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(mod_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated mod tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Write to file
        fs::write(mod_file_path, formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate a combined primitives.rs file with all FHIR primitive type aliases
    pub fn generate_combined_primitives_file<P: AsRef<Path>>(
        &self,
        primitive_structure_defs: &[StructureDefinition],
        output_path: P,
    ) -> CodegenResult<()> {
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Add file-level documentation
        let doc_comment = quote! {
            //! FHIR Primitive Types
            //!
            //! This module contains type aliases for all FHIR primitive types.
            //! Companion elements for primitive fields use the base Element type.
        };
        all_tokens.extend(doc_comment);

        // Generate all primitive type aliases
        let mut type_cache = std::collections::HashMap::new();
        let primitive_generator = PrimitiveGenerator::new(self.config, &mut type_cache);
        let type_aliases =
            primitive_generator.generate_all_primitive_type_aliases(primitive_structure_defs)?;

        for type_alias in type_aliases {
            let type_alias_tokens = self.token_generator.generate_type_alias(&type_alias);
            all_tokens.extend(type_alias_tokens);
        }

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated primitive tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Write to file
        fs::write(output_path, formatted_code).map_err(CodegenError::Io)?;

        Ok(())
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
        // Also check if the main struct has Extension base (for nested structs)
        let mut category = self.classify_fhir_structure_def(structure_def);
        if category != FhirTypeCategory::Extension && Self::has_extension_base(rust_struct) {
            category = FhirTypeCategory::Extension;
        }

        let target_dir = match category {
            FhirTypeCategory::Resource => base_dir.join("src").join("resource"),
            FhirTypeCategory::Profile => base_dir.join("src").join("profiles"),
            FhirTypeCategory::DataType => base_dir.join("src").join("datatypes"),
            FhirTypeCategory::Extension => base_dir.join("src").join("extensions"),
            FhirTypeCategory::Primitive => base_dir.join("src").join("primitives"),
        };

        // Ensure the target directory exists
        std::fs::create_dir_all(&target_dir).map_err(CodegenError::Io)?;

        // Separate nested structs that are extensions from those that should remain
        // embedded in the parent file. Extension-like nested structs should be
        // emitted into the shared `extensions` module instead of duplicated in
        // resource files.
        let mut embedded_nested: Vec<RustStruct> = Vec::new();
        let mut external_extensions: Vec<RustStruct> = Vec::new();

        for nested in nested_structs {
            if Self::has_extension_base(nested) {
                external_extensions.push(nested.clone());
            } else {
                embedded_nested.push(nested.clone());
            }
        }

        // Generate the main file in the appropriate directory (including only
        // the nested structs that belong with the parent)
        let filename = crate::naming::Naming::filename(structure_def);
        let output_path = target_dir.join(filename);

        let result =
            self.generate_to_file(structure_def, output_path, rust_struct, &embedded_nested);

        // If there are nested extension structs discovered, write each to the
        // extensions directory as standalone files to avoid duplicate definitions
        // across resource files.
        if !external_extensions.is_empty() {
            let extensions_dir = base_dir.join("src").join("extensions");
            std::fs::create_dir_all(&extensions_dir).map_err(CodegenError::Io)?;

            for ext in external_extensions {
                // Write each extension struct to its own file. If a file already
                // exists, it will be overwritten (consistent with existing behavior).
                self.write_struct_only_file(&ext, &extensions_dir)?;
            }
        }

        result
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

    /// Check if a RustStruct has Extension as its base type
    fn has_extension_base(rust_struct: &RustStruct) -> bool {
        rust_struct.fields.iter().any(|field| {
            field.name == "base" && matches!(&field.field_type, crate::rust_types::RustType::Custom(type_name) if type_name == "Extension")
        })
    }

    /// Classify a FHIR StructureDefinition into the appropriate category
    pub fn classify_fhir_structure_def(
        &self,
        structure_def: &StructureDefinition,
    ) -> FhirTypeCategory {
        // Check if this is a profile first (derives from a core FHIR resource)
        if crate::generators::type_registry::TypeRegistry::is_profile(structure_def) {
            return FhirTypeCategory::Profile;
        }

        // Check if it's a primitive type
        if structure_def.kind == "primitive-type" {
            return FhirTypeCategory::Primitive;
        }

        // Check for known data types first (including core Extension)
        if GeneratorUtils::is_fhir_datatype(&structure_def.name)
            || structure_def.base_type == "Element"
            || structure_def.base_type == "BackboneElement"
            || structure_def.base_type == "DataType"
            || structure_def.name == "Extension"
        {
            return FhirTypeCategory::DataType;
        }

        // Check if it's an Extension-based type (but not the core Extension itself)
        if structure_def.base_type == "Extension" {
            return FhirTypeCategory::Extension;
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

        // Always include serde if enabled, but exclude primitive types
        if self.config.with_serde && structure_def.kind != "primitive-type" {
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

            // Note: No longer generating individual companion Element structs
            // All companion fields now use the base Element type directly
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

        // Add Default implementation if needed (for non-profile resources)
        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let default_impl = self.generate_default_implementation(structure_def, rust_struct);
            if !default_impl.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&default_impl);
            }

            // Also generate Default implementations for nested structs
            for nested in nested_structs {
                let nested_default_impl =
                    self.generate_nested_struct_default_implementation(structure_def, nested);
                if !nested_default_impl.is_empty() {
                    formatted_code.push_str("\n\n");
                    formatted_code.push_str(&nested_default_impl);
                }
            }
        }

        // Add trait implementations for FHIR resources
        if structure_def.kind == "resource" {
            formatted_code.push_str("\n\n");
            formatted_code.push_str(&self.generate_trait_implementations(structure_def));
        }

        // Add Resource trait impl if this is the Resource struct (legacy)
        if structure_def.name == "Resource" {
            formatted_code.push_str("\n\n");
            // let trait_impl = crate::generators::TraitGenerator::new().generate_resource_impl();
            // formatted_code.push_str(&trait_impl);
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

        // Collect imports needed for this trait
        let mut imports = std::collections::HashSet::new();
        ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);

        // Add import statements
        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

        // Generate the trait
        let trait_tokens = self.token_generator.generate_trait(rust_trait);
        all_tokens.extend(trait_tokens);

        // Debug: Print the tokens before parsing
        if std::env::var("DEBUG_TOKENS").is_ok() {
            eprintln!(
                "DEBUG: Generated tokens for trait '{}': {}",
                rust_trait.name, all_tokens
            );
        }

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!(
                "Failed to parse generated trait tokens for '{}': {e}",
                rust_trait.name
            ),
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

    /// Generate multiple traits to a single file
    pub fn generate_traits_to_file<P: AsRef<Path>>(
        &self,
        _structure_def: &StructureDefinition,
        output_path: P,
        rust_traits: &[&RustTrait],
    ) -> CodegenResult<()> {
        // Generate import tokens
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Collect imports needed for all traits
        let mut imports = std::collections::HashSet::new();
        for rust_trait in rust_traits {
            ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);
        }

        // Add import statements
        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

        // Generate all traits
        for rust_trait in rust_traits {
            let trait_tokens = self.token_generator.generate_trait(rust_trait);
            all_tokens.extend(trait_tokens);
        }

        // Debug: Print the tokens before parsing
        if std::env::var("DEBUG_TOKENS").is_ok() {
            let trait_names: Vec<&str> = rust_traits.iter().map(|t| t.name.as_str()).collect();
            eprintln!(
                "DEBUG: Generated tokens for traits [{}]: {}",
                trait_names.join(", "),
                all_tokens
            );
        }

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

        // Collect imports needed for this trait
        let mut imports = std::collections::HashSet::new();
        ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);

        // Add import statements
        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

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

    /// Write a single struct as its own file into the given directory.
    /// This is used to emit extension nested structs into the `extensions` module
    /// to avoid duplicating their definitions inside resource files.
    fn write_struct_only_file<P: AsRef<Path>>(
        &self,
        rust_struct: &RustStruct,
        dir: P,
    ) -> CodegenResult<()> {
        let dir = dir.as_ref();

        // Prepare imports for this struct
        let mut imports = HashSet::new();
        if self.config.with_serde {
            imports.insert("serde::{Deserialize, Serialize}".to_string());
        }

        // Collect custom types referenced by this struct
        let mut structs_in_file = HashSet::new();
        structs_in_file.insert(rust_struct.name.clone());
        ImportManager::collect_custom_types_from_struct(
            rust_struct,
            &mut imports,
            &structs_in_file,
        );

        // Generate tokens
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Add imports
        for import in &imports {
            let import_token: proc_macro2::TokenStream =
                format!("use {import};").parse().expect("Invalid import");
            all_tokens.extend(import_token);
        }

        // Add the struct tokens
        all_tokens.extend(self.token_generator.generate_struct(rust_struct));

        // Parse and format
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!(
                "Failed to parse generated tokens for {}: {e}",
                rust_struct.name
            ),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        // Determine filename
        let filename = format!(
            "{}.rs",
            crate::naming::Naming::to_snake_case(&rust_struct.name)
        );
        let output_path = dir.join(filename);

        // Write file
        std::fs::write(output_path, formatted_code).map_err(CodegenError::Io)?;

        Ok(())
    }

    /// Generate trait implementations for a FHIR resource
    fn generate_trait_implementations(&self, structure_def: &StructureDefinition) -> String {
        let trait_impl_generator = TraitImplGenerator::new();
        let trait_impls = match trait_impl_generator.generate_trait_impls(structure_def) {
            Ok(impls) => impls,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to generate trait implementations for {}: {}",
                    structure_def.name, e
                );
                return String::new();
            }
        };

        let mut implementations = Vec::new();

        for trait_impl in trait_impls {
            let impl_tokens = self.token_generator.generate_trait_impl(&trait_impl);

            // Parse and format the implementation
            match syn::parse2(impl_tokens.clone()) {
                Ok(syntax_tree) => {
                    let formatted_impl = prettyplease::unparse(&syntax_tree);
                    implementations.push(formatted_impl);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse trait implementation for {}: {}",
                        trait_impl.struct_name, e
                    );
                    eprintln!("Generated tokens:\n{impl_tokens}");
                }
            }
        }

        if implementations.is_empty() {
            String::new()
        } else {
            format!("// Trait implementations\n{}", implementations.join("\n\n"))
        }
    }

    /// Generate Default implementation for a struct if needed
    fn generate_default_implementation(
        &self,
        structure_def: &StructureDefinition,
        rust_struct: &RustStruct,
    ) -> String {
        // Skip if this is a profile - profiles already have Default derived
        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);
        if is_profile {
            return String::new();
        }

        // Get the struct name
        let struct_name = &rust_struct.name;

        // Check if the struct has Default derive already
        if rust_struct.derives.iter().any(|d| d == "Default") {
            return String::new();
        }

        // Generate Default implementation using StructureDefinition as source of truth
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            // No elements - struct likely only has base field, but we still need Default
            &Vec::new()
        };

        // Collect required fields (min >= 1)
        let mut required_fields = Vec::new();
        for element in elements {
            let path_parts: Vec<&str> = element.path.split('.').collect();
            if path_parts.len() == 2 && path_parts[0] == structure_def.name {
                let field_name = path_parts[1];
                if let Some(min) = element.min {
                    if min >= 1 && !field_name.ends_with("[x]") {
                        required_fields.push((field_name, element.clone()));
                    }
                }
            }
        }

        // If no required fields, we could derive Default, but we'll generate it anyway for consistency
        // Build the Default implementation
        let mut field_inits = Vec::new();

        // First, handle the base field if it exists (base fields are added by TokenGenerator, not in rust_struct.fields)
        if let Some(base_def) = &rust_struct.base_definition {
            // Extract the base type name (e.g., "DomainResource" from a URL or just "DomainResource")
            let base_type = base_def.split('/').next_back().unwrap_or(base_def);
            let base_type = crate::naming::Naming::to_rust_identifier(base_type);
            let proper_base_type = if base_type
                .chars()
                .next()
                .map(|c| c.is_lowercase())
                .unwrap_or(false)
            {
                crate::naming::Naming::capitalize_first(&base_type)
            } else {
                base_type
            };
            field_inits.push(format!("base: {proper_base_type}::default()"));
            // Base field uses ::default(), so we can potentially derive
        }

        // Then, process other fields from the struct
        for field in &rust_struct.fields {
            let field_name = &field.name;

            // Check if this is a required field
            let is_required = required_fields.iter().any(|(name, _)| {
                let snake_name = crate::naming::Naming::to_snake_case(name);
                snake_name == *field_name
            });

            if is_required {
                // Generate appropriate default for required field based on type
                let default_value = match field.field_type.to_string().as_str() {
                    // Handle enums - use Default::default() if available
                    s if s.contains("::") && !s.contains("Option") && !s.contains("Vec") => {
                        format!("{s}::default()")
                    }
                    // Handle String
                    "String" => "String::new()".to_string(),
                    // Handle primitives
                    "i32" | "i64" | "u32" | "u64" => "0".to_string(),
                    "f32" | "f64" => "0.0".to_string(),
                    "bool" => "false".to_string(),
                    // Handle Vec
                    s if s.starts_with("Vec<") => "Vec::new()".to_string(),
                    // For unknown types, try Default::default()
                    _ => format!("{}::default()", field.field_type.to_string()),
                };
                field_inits.push(format!("{field_name}: {default_value}"));
            } else {
                // Optional field - use Default
                field_inits.push(format!("{field_name}: Default::default()"));
            }
        }

        // Generate the impl block
        let impl_block = format!(
            r#"impl Default for {} {{
    fn default() -> Self {{
        Self {{
            {}
        }}
    }}
}}"#,
            struct_name,
            field_inits.join(",\n            ")
        );

        impl_block
    }

    /// Generate Default implementation for a nested struct
    /// Nested structs are BackboneElements within a parent resource, so we need to
    /// extract the relevant elements from the parent StructureDefinition using the
    /// nested struct's base path (e.g., "AuditEvent.source" for AuditEventSource)
    fn generate_nested_struct_default_implementation(
        &self,
        parent_structure_def: &StructureDefinition,
        nested_struct: &RustStruct,
    ) -> String {
        // Get the struct name
        let struct_name = &nested_struct.name;

        // Check if the struct has Default derive already
        if nested_struct.derives.iter().any(|d| d == "Default") {
            return String::new();
        }

        // Determine the base path for this nested struct from the parent StructureDefinition
        // Example: "AuditEventSource" -> "AuditEvent.source"
        let parent_name = &parent_structure_def.name;
        let nested_field_name = if struct_name.starts_with(parent_name) {
            let suffix = &struct_name[parent_name.len()..];
            crate::naming::Naming::to_snake_case(suffix)
        } else {
            // Fallback - should not happen in practice
            return String::new();
        };

        let base_path = format!("{parent_name}.{nested_field_name}");

        // Get elements from the parent StructureDefinition
        let elements = if let Some(differential) = &parent_structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &parent_structure_def.snapshot {
            &snapshot.element
        } else {
            &Vec::new()
        };

        // Collect required fields for this nested struct (elements under base_path with min >= 1)
        let mut required_fields = Vec::new();
        for element in elements {
            // Match elements like "AuditEvent.source.observer"
            if element.path.starts_with(&format!("{base_path}.")) {
                let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();
                // Only direct fields (no dots in remaining path)
                if !field_path.contains('.') && !field_path.ends_with("[x]") {
                    if let Some(min) = element.min {
                        if min >= 1 {
                            required_fields.push((field_path, element.clone()));
                        }
                    }
                }
            }
        }

        // Build the Default implementation
        let mut field_inits = Vec::new();

        // First, handle the base field (use the actual base type from the struct definition)
        if let Some(base_def) = &nested_struct.base_definition {
            // Extract the base type name (e.g., "BackboneElement", "Element", "Extension")
            let base_type = base_def.split('/').next_back().unwrap_or(base_def);
            let base_type = crate::naming::Naming::to_rust_identifier(base_type);
            let proper_base_type = if base_type
                .chars()
                .next()
                .map(|c| c.is_lowercase())
                .unwrap_or(false)
            {
                crate::naming::Naming::capitalize_first(&base_type)
            } else {
                base_type
            };
            field_inits.push(format!("base: {proper_base_type}::default()"));
        }

        // Then, process other fields from the struct
        for field in &nested_struct.fields {
            let field_name = &field.name;

            // Check if this is a required field
            let is_required = required_fields.iter().any(|(name, _)| {
                let snake_name = crate::naming::Naming::to_snake_case(name);
                snake_name == *field_name
            });

            if is_required {
                // Generate appropriate default for required field based on type
                let default_value = match field.field_type.to_string().as_str() {
                    // Handle enums - use Default::default() if available
                    s if s.contains("::") && !s.contains("Option") && !s.contains("Vec") => {
                        format!("{s}::default()")
                    }
                    // Handle String
                    "String" => "String::new()".to_string(),
                    // Handle primitives
                    "i32" | "i64" | "u32" | "u64" => "0".to_string(),
                    "f32" | "f64" => "0.0".to_string(),
                    "bool" => "false".to_string(),
                    // Handle Vec
                    s if s.starts_with("Vec<") => "Vec::new()".to_string(),
                    // For unknown types, try Default::default()
                    _ => format!("{}::default()", field.field_type.to_string()),
                };
                field_inits.push(format!("{field_name}: {default_value}"));
            } else {
                // Optional field - use Default
                field_inits.push(format!("{field_name}: Default::default()"));
            }
        }

        // Generate the impl block
        let impl_block = format!(
            r#"impl Default for {} {{
    fn default() -> Self {{
        Self {{
            {}
        }}
    }}
}}"#,
            struct_name,
            field_inits.join(",\n            ")
        );

        impl_block
    }

    /// Generate a complete crate structure with all necessary files and modules
    pub fn generate_complete_crate<P: AsRef<Path>>(
        &self,
        output_dir: P,
        crate_name: &str,
        _structures: &[StructureDefinition],
    ) -> CodegenResult<()> {
        let output_dir = output_dir.as_ref();

        // Create main directories
        let src_dir = output_dir.join("src");
        fs::create_dir_all(&src_dir).map_err(CodegenError::Io)?;

        // Create module directories
        let primitives_dir = src_dir.join("primitives");
        let datatypes_dir = src_dir.join("datatypes");
        let extensions_dir = src_dir.join("extensions");
        let resource_dir = src_dir.join("resource");
        let traits_dir = src_dir.join("traits");

        fs::create_dir_all(&primitives_dir).map_err(CodegenError::Io)?;
        fs::create_dir_all(&datatypes_dir).map_err(CodegenError::Io)?;
        fs::create_dir_all(&extensions_dir).map_err(CodegenError::Io)?;
        fs::create_dir_all(&resource_dir).map_err(CodegenError::Io)?;
        fs::create_dir_all(&traits_dir).map_err(CodegenError::Io)?;

        // Generate main lib.rs
        self.generate_lib_file(src_dir.join("lib.rs"))?;

        // Generate macros.rs
        self.generate_macros_file(src_dir.join("macros.rs"))?;

        // Generate combined primitives.rs file
        // For now, use an empty array since we're focusing on macro inclusion
        self.generate_combined_primitives_file(&[], primitives_dir.join("mod.rs"))?;

        // Generate a basic Cargo.toml if it doesn't exist
        let cargo_toml_path = output_dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            self.generate_cargo_toml(&cargo_toml_path, crate_name)?;
        }

        // Generate module files for datatypes, extensions, resource, and traits directories
        // These will be populated with actual generated types later
        self.generate_module_file(&datatypes_dir, &[])?;
        self.generate_module_file(&extensions_dir, &[])?;
        self.generate_module_file(&resource_dir, &[])?;
        self.generate_module_file(&traits_dir, &[])?;

        Ok(())
    }

    /// Generate a basic Cargo.toml for the generated crate
    fn generate_cargo_toml<P: AsRef<Path>>(
        &self,
        cargo_path: P,
        crate_name: &str,
    ) -> CodegenResult<()> {
        let cargo_content = format!(
            r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#
        );

        fs::write(cargo_path, cargo_content).map_err(CodegenError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CodegenConfig;
    use crate::generators::token_generator::TokenGenerator;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_macros_file() {
        let temp_dir = TempDir::new().unwrap();
        let macros_path = temp_dir.path().join("macros.rs");

        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new();
        let file_generator = FileGenerator::new(&config, &token_generator);

        file_generator.generate_macros_file(&macros_path).unwrap();

        assert!(macros_path.exists());
        let content = fs::read_to_string(&macros_path).unwrap();

        // Check that the file contains our macro definitions
        assert!(content.contains("macro_rules! primitive_string"));
        assert!(content.contains("macro_rules! primitive_boolean"));
        assert!(content.contains("macro_rules! primitive_id"));
    }

    #[test]
    fn test_generate_lib_file() {
        let temp_dir = TempDir::new().unwrap();
        let lib_path = temp_dir.path().join("lib.rs");

        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new();
        let file_generator = FileGenerator::new(&config, &token_generator);

        file_generator.generate_lib_file(&lib_path).unwrap();

        assert!(lib_path.exists());
        let content = fs::read_to_string(&lib_path).unwrap();

        // Check that the file contains module declarations
        assert!(content.contains("pub mod macros;"));
        assert!(content.contains("pub mod primitives;"));
        assert!(content.contains("pub mod datatypes;"));
        assert!(content.contains("pub mod resources;"));
        assert!(content.contains("pub mod traits;"));
        assert!(content.contains("pub mod bindings;"));

        // Check selective re-exports (only macros and serde)
        assert!(content.contains("pub use macros::*;"));
        assert!(content.contains("pub use serde::{Deserialize, Serialize};"));

        // Should NOT have glob re-exports for other modules
        assert!(!content.contains("pub use primitives::*;"));
        assert!(!content.contains("pub use datatypes::*;"));
        assert!(!content.contains("pub use resource::*;"));
        assert!(!content.contains("pub use traits::*;"));
        assert!(!content.contains("pub use bindings::*;"));
    }

    #[test]
    fn test_generate_complete_crate() {
        let temp_dir = TempDir::new().unwrap();
        let crate_path = temp_dir.path().join("test-crate");

        let config = CodegenConfig::default();
        let token_generator = TokenGenerator::new();
        let file_generator = FileGenerator::new(&config, &token_generator);

        file_generator
            .generate_complete_crate(
                &crate_path,
                "test-crate",
                &[], // Empty structure definitions
            )
            .unwrap();

        // Check that all required files and directories exist
        assert!(crate_path.join("Cargo.toml").exists());
        assert!(crate_path.join("src").is_dir());
        assert!(crate_path.join("src/lib.rs").exists());
        assert!(crate_path.join("src/macros.rs").exists());
        assert!(crate_path.join("src/primitives").is_dir());
        assert!(crate_path.join("src/primitives/mod.rs").exists());
        assert!(crate_path.join("src/datatypes").is_dir());
        assert!(crate_path.join("src/datatypes/mod.rs").exists());
        assert!(crate_path.join("src/resource").is_dir());
        assert!(crate_path.join("src/resource/mod.rs").exists());
        assert!(crate_path.join("src/traits").is_dir());
        assert!(crate_path.join("src/traits/mod.rs").exists());

        // Check Cargo.toml content
        let cargo_content = fs::read_to_string(crate_path.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("name = \"test-crate\""));
        assert!(cargo_content.contains("edition = \"2021\""));
        assert!(cargo_content.contains("serde"));
        assert!(!cargo_content.contains("paste")); // paste should NOT be in dependencies
    }
}
