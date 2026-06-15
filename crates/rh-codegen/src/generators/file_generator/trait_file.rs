use std::fs;
use std::path::Path;

use crate::generators::import_manager::ImportManager;
use crate::rust_types::RustTrait;
use crate::{CodegenError, CodegenResult};

use super::FileGenerator;

impl<'a> FileGenerator<'a> {
    pub fn generate_trait_to_file<P: AsRef<Path>>(
        &self,
        _structure_def: &crate::fhir_types::StructureDefinition,
        output_path: P,
        rust_trait: &RustTrait,
    ) -> CodegenResult<()> {
        let mut all_tokens = proc_macro2::TokenStream::new();

        let mut imports = std::collections::HashSet::new();
        ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);

        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

        let trait_tokens = self.token_generator.generate_trait(rust_trait);
        all_tokens.extend(trait_tokens);

        if std::env::var("DEBUG_TOKENS").is_ok() {
            eprintln!(
                "DEBUG: Generated tokens for trait '{}': {}",
                rust_trait.name, all_tokens
            );
        }

        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!(
                "Failed to parse generated trait tokens for '{}': {e}",
                rust_trait.name
            ),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: Trait file '{}' already exists and will be overwritten.",
                output_path.as_ref().display()
            );
        }

        fs::write(output_path.as_ref(), formatted_code)?;

        Ok(())
    }

    pub fn generate_traits_to_file<P: AsRef<Path>>(
        &self,
        _structure_def: &crate::fhir_types::StructureDefinition,
        output_path: P,
        rust_traits: &[&RustTrait],
    ) -> CodegenResult<()> {
        let mut all_tokens = proc_macro2::TokenStream::new();

        let mut imports = std::collections::HashSet::new();
        for rust_trait in rust_traits {
            ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);
        }

        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

        for rust_trait in rust_traits {
            let trait_tokens = self.token_generator.generate_trait(rust_trait);
            all_tokens.extend(trait_tokens);
        }

        if std::env::var("DEBUG_TOKENS").is_ok() {
            let trait_names: Vec<&str> = rust_traits.iter().map(|t| t.name.as_str()).collect();
            eprintln!(
                "DEBUG: Generated tokens for traits [{}]: {}",
                trait_names.join(", "),
                all_tokens
            );
        }

        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated trait tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: Trait file '{}' already exists and will be overwritten.",
                output_path.as_ref().display()
            );
        }

        fs::write(output_path.as_ref(), formatted_code)?;

        Ok(())
    }

    pub fn generate_trait_file_from_trait<P: AsRef<Path>>(
        &self,
        rust_trait: &RustTrait,
        output_path: P,
    ) -> CodegenResult<()> {
        let mut all_tokens = proc_macro2::TokenStream::new();

        let mut imports = std::collections::HashSet::new();
        ImportManager::collect_custom_types_from_trait(rust_trait, &mut imports);

        for import_path in imports {
            let import_stmt = format!("use {import_path};");
            let import_tokens: proc_macro2::TokenStream =
                import_stmt.parse().map_err(|e| CodegenError::Generation {
                    message: format!("Failed to parse import statement '{import_stmt}': {e}"),
                })?;
            all_tokens.extend(import_tokens);
        }

        let trait_tokens = self.token_generator.generate_trait(rust_trait);
        all_tokens.extend(trait_tokens);

        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated trait tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        if output_path.as_ref().exists() {
            eprintln!(
                "Warning: Trait file '{}' already exists and will be overwritten.",
                output_path.as_ref().display()
            );
        }

        fs::write(output_path.as_ref(), formatted_code)?;

        Ok(())
    }

    pub(crate) fn generate_trait_implementations(
        &self,
        structure_def: &crate::fhir_types::StructureDefinition,
    ) -> String {
        let trait_impl_generator =
            crate::generators::trait_impl_generator::TraitImplGenerator::new();
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

    pub(crate) fn generate_trait_reexports(
        &self,
        structure_def: &crate::fhir_types::StructureDefinition,
    ) -> String {
        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);

        let (trait_module_name, trait_prefix) = if is_profile {
            let struct_name = crate::naming::Naming::struct_name(structure_def);
            let snake_module = crate::naming::Naming::to_rust_identifier(
                &crate::naming::Naming::to_snake_case(&struct_name),
            );
            (snake_module, struct_name)
        } else {
            let resource_name = crate::naming::Naming::to_rust_identifier(&structure_def.name);
            let snake_name = crate::naming::Naming::to_rust_identifier(
                &crate::naming::Naming::to_snake_case(&resource_name),
            );
            (snake_name, resource_name)
        };

        format!(
            r#"// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::{trait_module_name}::{{
    {trait_prefix}Mutators,
    {trait_prefix}Accessors,
    {trait_prefix}Existence,
}};"#
        )
    }

    pub(crate) fn generate_default_implementation(
        &self,
        structure_def: &crate::fhir_types::StructureDefinition,
        rust_struct: &crate::rust_types::RustStruct,
    ) -> String {
        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);
        if is_profile {
            return String::new();
        }

        let struct_name = &rust_struct.name;

        if rust_struct.derives.iter().any(|d| d == "Default") {
            return String::new();
        }

        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            &Vec::new()
        };

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

        let mut field_inits = Vec::new();

        if let Some(base_def) = &rust_struct.base_definition {
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

        for field in &rust_struct.fields {
            let field_name = &field.name;

            let is_required = required_fields.iter().any(|(name, _)| {
                let snake_name = crate::naming::Naming::to_snake_case(name);
                snake_name == *field_name
            });

            if is_required {
                let default_value = match field.field_type.to_string().as_str() {
                    s if s.contains("::") && !s.contains("Option") && !s.contains("Vec") => {
                        format!("{s}::default()")
                    }
                    "String" => "String::new()".to_string(),
                    "i32" | "i64" | "u32" | "u64" => "0".to_string(),
                    "f32" | "f64" => "0.0".to_string(),
                    "bool" => "false".to_string(),
                    s if s.starts_with("Vec<") => "Vec::new()".to_string(),
                    _ => format!("{}::default()", field.field_type.to_string()),
                };
                field_inits.push(format!("{field_name}: {default_value}"));
            } else {
                field_inits.push(format!("{field_name}: Default::default()"));
            }
        }

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

    pub(crate) fn generate_nested_struct_default_implementation(
        &self,
        parent_structure_def: &crate::fhir_types::StructureDefinition,
        nested_struct: &crate::rust_types::RustStruct,
    ) -> String {
        let struct_name = &nested_struct.name;

        if nested_struct.derives.iter().any(|d| d == "Default") {
            return String::new();
        }

        let parent_name = &parent_structure_def.name;
        let nested_field_name = if struct_name.starts_with(parent_name) {
            let suffix = &struct_name[parent_name.len()..];
            crate::naming::Naming::to_snake_case(suffix)
        } else {
            return String::new();
        };

        let base_path = format!("{parent_name}.{nested_field_name}");

        let elements = if let Some(differential) = &parent_structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &parent_structure_def.snapshot {
            &snapshot.element
        } else {
            &Vec::new()
        };

        let mut required_fields = Vec::new();
        for element in elements {
            if element.path.starts_with(&format!("{base_path}.")) {
                let field_path = element
                    .path
                    .strip_prefix(&format!("{base_path}."))
                    .unwrap_or_else(|| {
                        panic!(
                            "codegen bug: element path '{}' does not start with '{base_path}.'",
                            element.path
                        )
                    });
                if !field_path.contains('.') && !field_path.ends_with("[x]") {
                    if let Some(min) = element.min {
                        if min >= 1 {
                            required_fields.push((field_path, element.clone()));
                        }
                    }
                }
            }
        }

        let mut field_inits = Vec::new();

        if let Some(base_def) = &nested_struct.base_definition {
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

        for field in &nested_struct.fields {
            let field_name = &field.name;

            let is_required = required_fields.iter().any(|(name, _)| {
                let snake_name = crate::naming::Naming::to_snake_case(name);
                snake_name == *field_name
            });

            if is_required {
                let default_value = match field.field_type.to_string().as_str() {
                    s if s.contains("::") && !s.contains("Option") && !s.contains("Vec") => {
                        format!("{s}::default()")
                    }
                    "String" => "String::new()".to_string(),
                    "i32" | "i64" | "u32" | "u64" => "0".to_string(),
                    "f32" | "f64" => "0.0".to_string(),
                    "bool" => "false".to_string(),
                    s if s.starts_with("Vec<") => "Vec::new()".to_string(),
                    _ => format!("{}::default()", field.field_type.to_string()),
                };
                field_inits.push(format!("{field_name}: {default_value}"));
            } else {
                field_inits.push(format!("{field_name}: Default::default()"));
            }
        }

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
}
