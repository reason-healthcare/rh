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

            pub mod macros;
            pub mod primitives;
            pub mod datatypes;
            pub mod extensions;
            pub mod resource;
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

        // Generate imports
        all_tokens.extend(quote! {
            use serde::{Deserialize, Serialize};
        });

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
            FhirTypeCategory::DataType => base_dir.join("src").join("datatypes"),
            FhirTypeCategory::Extension => base_dir.join("src").join("extensions"),
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

        // Add trait implementations for FHIR resources
        if structure_def.kind == "resource" {
            formatted_code.push_str("\n\n");
            formatted_code.push_str(&self.generate_trait_implementations(structure_def));
        }

        // Add Resource trait impl if this is the Resource struct (legacy)
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
            match syn::parse2(impl_tokens) {
                Ok(syntax_tree) => {
                    let formatted_impl = prettyplease::unparse(&syntax_tree);
                    implementations.push(formatted_impl);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse trait implementation for {}: {}",
                        trait_impl.struct_name, e
                    );
                }
            }
        }

        if implementations.is_empty() {
            String::new()
        } else {
            format!("// Trait implementations\n{}", implementations.join("\n\n"))
        }
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
paste = "1.0"
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
        assert!(content.contains("pub mod resource;"));
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
        assert!(cargo_content.contains("paste"));
    }
}
