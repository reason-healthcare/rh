use std::fs;
use std::path::Path;

use quote::quote;

use crate::generators::enum_generator::EnumGenerator;
use crate::{CodegenError, CodegenResult};

use super::FileGenerator;

impl<'a> FileGenerator<'a> {
    pub fn generate_enum_files<P: AsRef<Path>>(
        &self,
        enums_dir: P,
        enum_generator: &EnumGenerator,
    ) -> CodegenResult<()> {
        let enums_dir = enums_dir.as_ref();

        if !enums_dir.exists() {
            fs::create_dir_all(enums_dir)?;
        }

        for (enum_name, rust_enum) in enum_generator.get_cached_enums() {
            let enum_filename = EnumGenerator::enum_name_to_filename(enum_name);
            let enum_file_path = enums_dir.join(enum_filename);

            let import_tokens = quote! {
                use serde::{Deserialize, Serialize};
            };
            let enum_tokens = self.token_generator.generate_enum(rust_enum);
            let combined_tokens = quote! {
                #import_tokens

                #enum_tokens
            };

            let syntax_tree =
                syn::parse2(combined_tokens).map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse generated enum tokens for {enum_name}: {e}"),
                })?;

            let formatted_code = prettyplease::unparse(&syntax_tree);

            if enum_file_path.exists() {
                eprintln!(
                    "Warning: Enum file '{}' already exists and will be overwritten.",
                    enum_file_path.display()
                );
            }

            fs::write(&enum_file_path, formatted_code)?;
        }

        Ok(())
    }

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

        let mut enum_names: Vec<_> = enum_generator.get_cached_enums().keys().collect();
        enum_names.sort();

        for enum_name in enum_names {
            let module_name = EnumGenerator::enum_name_to_module_name(enum_name);
            mod_content.push(format!("pub mod {module_name};"));
        }

        let final_content = mod_content.join("\n") + "\n";

        if mod_file_path.exists() {
            eprintln!(
                "Warning: Enum mod file '{}' already exists and will be overwritten.",
                mod_file_path.display()
            );
        }

        fs::write(&mod_file_path, final_content)?;

        Ok(())
    }
}
