//! File generation and organization functionality
//!
//! This module handles writing generated code to files and organizing the output structure.
//!
//! ## Ergonomic Improvements for Trait Usage
//!
//! This module implements two key ergonomic improvements for generated FHIR resources:
//!
//! ### 1. Trait Re-exports in Resource Modules
//!
//! Each generated resource module (e.g., `resources::patient`) automatically re-exports its
//! associated traits (`PatientMutators`, `PatientAccessors`, `PatientExistence`). This allows
//! users to import just the resource module and get all necessary traits:
//!
//! ```ignore
//! // Before: Required importing from multiple modules
//! use hl7_fhir_r4_core::resources::patient::Patient;
//! use hl7_fhir_r4_core::traits::patient::PatientMutators;
//! use hl7_fhir_r4_core::traits::domain_resource::DomainResourceMutators;
//! use hl7_fhir_r4_core::traits::resource::ResourceMutators;
//!
//! // After: Single import gets everything needed
//! use hl7_fhir_r4_core::resources::patient::{Patient, PatientMutators};
//! // Note: Parent traits (ResourceMutators, DomainResourceMutators) are trait bounds,
//! // so they're brought into scope automatically when PatientMutators is used
//! ```
//!
//! ### 2. Prelude Module
//!
//! A `prelude` module is generated that re-exports commonly used base traits:
//!
//! ```ignore
//! use hl7_fhir_r4_core::prelude::*;
//! use hl7_fhir_r4_core::resources::patient::{Patient, PatientMutators};
//!
//! // Now all base traits are in scope
//! let patient = <Patient as PatientMutators>::new()
//!     .set_id("example".to_string())  // from ResourceMutators
//!     .set_active(true);               // from PatientMutators
//! ```
//!
//! These improvements follow idiomatic Rust patterns used by popular crates like
//! `serde`, `tokio`, and `diesel`, making the generated code more ergonomic and
//! reducing the cognitive load on users.

mod enum_files;
mod trait_file;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use quote::{format_ident, quote};

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::generators::binding_generator::BindingGenerator;
use crate::generators::import_manager::ImportManager;
use crate::generators::primitive_generator::PrimitiveGenerator;
use crate::generators::token_generator::TokenGenerator;
use crate::rust_types::{RustStruct, RustTrait};
use crate::{CodegenError, CodegenResult};

#[derive(Debug, Clone, PartialEq)]
pub enum FhirTypeCategory {
    Resource,
    Profile,
    DataType,
    Extension,
    Primitive,
}

pub struct FileGenerator<'a> {
    pub(crate) config: &'a CodegenConfig,
    pub(crate) token_generator: &'a TokenGenerator,
}

impl<'a> FileGenerator<'a> {
    pub fn new(config: &'a CodegenConfig, token_generator: &'a TokenGenerator) -> Self {
        Self {
            config,
            token_generator,
        }
    }

    pub fn generate_macros_file<P: AsRef<Path>>(&self, output_path: P) -> CodegenResult<()> {
        let macros_content = include_str!("../../macros.rs");

        let syntax_tree =
            syn::parse_file(macros_content).map_err(|e| CodegenError::Generation {
                message: format!("Failed to parse macros file: {e}"),
            })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        fs::write(output_path, formatted_code)?;

        Ok(())
    }

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

        let syntax_tree = syn::parse2(lib_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated lib tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        fs::write(output_path, formatted_code)?;

        Ok(())
    }

    pub fn generate_module_file<P: AsRef<Path>>(
        &self,
        module_dir: P,
        module_names: &[String],
    ) -> CodegenResult<()> {
        let module_dir = module_dir.as_ref();
        let mod_file_path = module_dir.join("mod.rs");

        let mut mod_tokens = proc_macro2::TokenStream::new();

        for module_name in module_names {
            let mod_ident = format_ident!("{}", module_name);
            mod_tokens.extend(quote! {
                pub mod #mod_ident;
            });
        }

        let syntax_tree = syn::parse2(mod_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated mod tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        fs::write(mod_file_path, formatted_code)?;

        Ok(())
    }

    pub fn generate_combined_primitives_file<P: AsRef<Path>>(
        &self,
        primitive_structure_defs: &[StructureDefinition],
        output_path: P,
    ) -> CodegenResult<()> {
        let mut all_tokens = proc_macro2::TokenStream::new();

        let doc_comment = quote! {
            //! FHIR Primitive Types
            //!
            //! This module contains type aliases for all FHIR primitive types.
            //! Companion elements for primitive fields use the base Element type.
        };
        all_tokens.extend(doc_comment);

        let mut type_cache = std::collections::HashMap::new();
        let primitive_generator = PrimitiveGenerator::new(self.config, &mut type_cache);
        let type_aliases =
            primitive_generator.generate_all_primitive_type_aliases(primitive_structure_defs)?;

        for type_alias in type_aliases {
            let type_alias_tokens = self.token_generator.generate_type_alias(&type_alias);
            all_tokens.extend(type_alias_tokens);
        }

        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated primitive tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        fs::write(output_path, formatted_code)?;

        Ok(())
    }

    pub fn generate_to_organized_directories<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
        rust_struct: &RustStruct,
        nested_structs: &[RustStruct],
    ) -> CodegenResult<()> {
        let base_dir = base_output_dir.as_ref();

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

        std::fs::create_dir_all(&target_dir)?;

        let mut embedded_nested: Vec<RustStruct> = Vec::new();
        let mut external_extensions: Vec<RustStruct> = Vec::new();

        for nested in nested_structs {
            if Self::has_extension_base(nested) {
                external_extensions.push(nested.clone());
            } else {
                embedded_nested.push(nested.clone());
            }
        }

        embedded_nested.sort_by(|left, right| left.name.cmp(&right.name));
        external_extensions.sort_by(|left, right| left.name.cmp(&right.name));

        let filename = crate::naming::Naming::filename(structure_def);
        let output_path = target_dir.join(filename);

        let result =
            self.generate_to_file(structure_def, output_path, rust_struct, &embedded_nested);

        if !external_extensions.is_empty() {
            let extensions_dir = base_dir.join("src").join("extensions");
            std::fs::create_dir_all(&extensions_dir)?;

            for ext in external_extensions {
                self.write_struct_only_file(&ext, &extensions_dir)?;
            }
        }

        result
    }

    pub fn generate_trait_to_organized_directory<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        base_output_dir: P,
        rust_trait: &RustTrait,
    ) -> CodegenResult<()> {
        let traits_dir = base_output_dir.as_ref().join("src").join("traits");

        std::fs::create_dir_all(&traits_dir)?;

        let struct_name = crate::naming::Naming::struct_name(structure_def);
        let snake_case_name = crate::naming::Naming::to_snake_case(&struct_name);
        let filename = format!("{snake_case_name}.rs");
        let output_path = traits_dir.join(filename);

        self.generate_trait_to_file(structure_def, output_path, rust_trait)
    }

    fn has_extension_base(rust_struct: &RustStruct) -> bool {
        rust_struct.fields.iter().any(|field| {
            field.name == "base" && matches!(&field.field_type, crate::rust_types::RustType::Custom(type_name) if type_name == "Extension")
        })
    }

    pub fn classify_fhir_structure_def(
        &self,
        structure_def: &StructureDefinition,
    ) -> FhirTypeCategory {
        if crate::generators::type_registry::TypeRegistry::is_profile(structure_def) {
            return FhirTypeCategory::Profile;
        }

        if structure_def.kind == "primitive-type" {
            return FhirTypeCategory::Primitive;
        }

        if crate::generators::utils::GeneratorUtils::is_fhir_datatype(&structure_def.name)
            || structure_def.base_type == "Element"
            || structure_def.base_type == "BackboneElement"
            || structure_def.base_type == "DataType"
            || structure_def.name == "Extension"
        {
            return FhirTypeCategory::DataType;
        }

        if structure_def.base_type == "Extension" {
            return FhirTypeCategory::Extension;
        }

        if structure_def.kind == "resource"
            || structure_def.base_type == "Resource"
            || structure_def.base_type == "DomainResource"
        {
            return FhirTypeCategory::Resource;
        }

        if structure_def.kind == "complex-type" {
            return FhirTypeCategory::DataType;
        }

        FhirTypeCategory::Resource
    }

    pub fn generate_to_file<P: AsRef<Path>>(
        &self,
        structure_def: &StructureDefinition,
        output_path: P,
        rust_struct: &RustStruct,
        nested_structs: &[RustStruct],
    ) -> CodegenResult<()> {
        let mut imports = HashSet::new();

        if self.config.with_serde && structure_def.kind != "primitive-type" {
            imports.insert("serde::{Deserialize, Serialize}".to_string());
        }

        let has_macro_calls = rust_struct
            .fields
            .iter()
            .any(|field| field.macro_call.is_some())
            || nested_structs
                .iter()
                .any(|s| s.fields.iter().any(|field| field.macro_call.is_some()));

        if has_macro_calls {
            imports.insert("crate::{primitive_string, primitive_boolean, primitive_integer, primitive_decimal, primitive_datetime, primitive_date, primitive_time, primitive_uri, primitive_canonical, primitive_base64binary, primitive_instant, primitive_positiveint, primitive_unsignedint, primitive_id, primitive_oid, primitive_uuid, primitive_code, primitive_markdown, primitive_url}".to_string());
        }

        let mut all_tokens = proc_macro2::TokenStream::new();

        if structure_def.kind == "primitive-type" {
            let mut type_cache = std::collections::HashMap::new();
            let primitive_generator = PrimitiveGenerator::new(self.config, &mut type_cache);
            let type_alias = primitive_generator.generate_primitive_type_alias(structure_def)?;
            let type_alias_tokens = self.token_generator.generate_type_alias(&type_alias);
            all_tokens.extend(type_alias_tokens);
        } else {
            let mut all_structs = vec![rust_struct.clone()];
            all_structs.extend(nested_structs.iter().cloned());

            let structs_in_file: HashSet<String> =
                all_structs.iter().map(|s| s.name.clone()).collect();

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

        let mut import_tokens = proc_macro2::TokenStream::new();
        let mut sorted_imports: Vec<_> = imports.iter().collect();
        sorted_imports.sort();
        for import in sorted_imports {
            let import_token: proc_macro2::TokenStream = format!("use {import};")
                .parse()
                .expect("codegen bug: invalid import statement in generated file imports");
            import_tokens.extend(import_token);
        }

        let mut final_tokens = proc_macro2::TokenStream::new();
        final_tokens.extend(import_tokens);
        final_tokens.extend(all_tokens);

        let syntax_tree = syn::parse2(final_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated tokens: {e}"),
        })?;

        let mut formatted_code = prettyplease::unparse(&syntax_tree);

        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let default_impl = self.generate_default_implementation(structure_def, rust_struct);
            if !default_impl.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&default_impl);
            }

            let mut sorted_nested_structs = nested_structs.to_vec();
            sorted_nested_structs.sort_by(|left, right| left.name.cmp(&right.name));

            for nested in &sorted_nested_structs {
                let nested_default_impl =
                    self.generate_nested_struct_default_implementation(structure_def, nested);
                if !nested_default_impl.is_empty() {
                    formatted_code.push_str("\n\n");
                    formatted_code.push_str(&nested_default_impl);
                }
            }
        }

        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let invariants_const =
                crate::generators::InvariantGenerator::generate_invariants_constant(structure_def);
            if !invariants_const.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&invariants_const);
            }
        }

        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let bindings_const = BindingGenerator::generate_bindings_constant(structure_def);
            if !bindings_const.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&bindings_const);
            }
        }

        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let cardinalities_const =
                crate::generators::cardinality_generator::CardinalityGenerator::generate_cardinalities_constant(
                    structure_def,
                );
            if !cardinalities_const.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&cardinalities_const);
            }
        }

        if structure_def.kind == "resource" {
            formatted_code.push_str("\n\n");
            formatted_code.push_str(&self.generate_trait_implementations(structure_def));
        }

        if structure_def.kind == "resource" || structure_def.kind == "complex-type" {
            let validation_impl =
                crate::generators::ValidationTraitGenerator::generate_trait_impl(structure_def);
            if !validation_impl.is_empty() {
                formatted_code.push_str("\n\n");
                formatted_code.push_str(&validation_impl);
            }
        }

        if structure_def.kind == "resource" {
            formatted_code.push_str("\n\n");
            formatted_code.push_str(&self.generate_trait_reexports(structure_def));
        }

        if structure_def.name == "Resource" {
            formatted_code.push_str("\n\n");
        }

        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: File '{}' already exists and will be overwritten. This may indicate a naming collision between FHIR StructureDefinitions.",
                output_path.as_ref().display()
            );
        }

        fs::write(output_path.as_ref(), formatted_code)?;

        Ok(())
    }

    fn write_struct_only_file<P: AsRef<Path>>(
        &self,
        rust_struct: &RustStruct,
        dir: P,
    ) -> CodegenResult<()> {
        let dir = dir.as_ref();

        let mut imports = HashSet::new();
        if self.config.with_serde {
            imports.insert("serde::{Deserialize, Serialize}".to_string());
        }

        let mut structs_in_file = HashSet::new();
        structs_in_file.insert(rust_struct.name.clone());
        ImportManager::collect_custom_types_from_struct(
            rust_struct,
            &mut imports,
            &structs_in_file,
        );

        let mut all_tokens = proc_macro2::TokenStream::new();

        for import in &imports {
            let import_token: proc_macro2::TokenStream = format!("use {import};")
                .parse()
                .expect("codegen bug: invalid import statement in struct-only file");
            all_tokens.extend(import_token);
        }

        all_tokens.extend(self.token_generator.generate_struct(rust_struct));

        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!(
                "Failed to parse generated tokens for {}: {e}",
                rust_struct.name
            ),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        let filename = format!(
            "{}.rs",
            crate::naming::Naming::to_snake_case(&rust_struct.name)
        );
        let output_path = dir.join(filename);

        std::fs::write(output_path, formatted_code)?;

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

        assert!(content.contains("pub mod macros;"));
        assert!(content.contains("pub mod primitives;"));
        assert!(content.contains("pub mod datatypes;"));
        assert!(content.contains("pub mod resources;"));
        assert!(content.contains("pub mod traits;"));
        assert!(content.contains("pub mod bindings;"));

        assert!(content.contains("pub use macros::*;"));
        assert!(content.contains("pub use serde::{Deserialize, Serialize};"));

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

        let cargo_content = fs::read_to_string(crate_path.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("name = \"test-crate\""));
        assert!(cargo_content.contains("edition = \"2021\""));
        assert!(cargo_content.contains("serde"));
        assert!(!cargo_content.contains("paste"));
    }
}
