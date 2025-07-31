//! FHIR type generation functionality
//!
//! This module contains the core logic for generating Rust types from FHIR StructureDefinitions.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use quote::quote;

use crate::config::CodegenConfig;
use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustEnum, RustStruct, RustType};
use crate::token_generator::TokenGenerator;
use crate::value_sets::ValueSetManager;
use crate::{CodegenError, CodegenResult};

/// Main code generator struct
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
        let content = fs::read_to_string(&path).map_err(CodegenError::Io)?;

        let structure_def: StructureDefinition =
            serde_json::from_str(&content).map_err(CodegenError::Json)?;

        Ok(structure_def)
    }

    /// Generate a Rust struct from a FHIR StructureDefinition
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        // Skip LogicalModels as they are conceptual models, not implementable types
        if structure_def.kind == "logical" {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping LogicalModel '{}' - logical models are not generated as Rust types",
                    structure_def.name
                ),
            });
        }

        // Generate struct name from URL or ID, not the name field
        let struct_name = self.generate_struct_name(structure_def);

        // Check if we've already generated this type
        if let Some(cached_struct) = self.type_cache.get(&struct_name) {
            return Ok(cached_struct.clone());
        }

        // Create the struct with enhanced documentation
        let mut rust_struct = RustStruct::new(struct_name.clone());
        rust_struct.doc_comment = self.generate_struct_documentation(structure_def);

        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        rust_struct.derives = derives;

        // Set the base definition for inheritance if present
        if let Some(base_def) = &structure_def.base_definition {
            rust_struct.base_definition = Some(base_def.clone());
        }

        // Extract element definitions from differential only (preferred FHIR approach)
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else {
            // No differential elements to process - this may be a base resource
            return Ok(rust_struct);
        };

        // Process each element definition to create struct fields and nested structs
        let mut nested_structs_info = HashMap::new();
        let mut direct_fields = Vec::new();

        for element in elements {
            // Skip the root element (matches the resource name or base type)
            if element.path == structure_def.name || element.path == structure_def.base_type {
                continue;
            }

            // Determine the base path to use for filtering
            // For extensions and profiles, we need to use the base type (e.g., "Extension")
            // For resources, we use the resource name
            let base_path =
                if structure_def.base_type == "Extension" && structure_def.kind == "complex-type" {
                    "Extension"
                } else if structure_def.kind == "resource" {
                    &structure_def.name
                } else {
                    // For other types, try to infer from the first element path
                    if let Some(first_dot) = element.path.find('.') {
                        &element.path[..first_dot]
                    } else {
                        &structure_def.name
                    }
                };

            // Only process elements that belong to this structure
            if !element.path.starts_with(&format!("{base_path}.")) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();

            if field_path.contains('.') {
                // This is a nested field - collect it for nested struct generation
                let nested_field_name = field_path.split('.').next().unwrap();
                nested_structs_info
                    .entry(nested_field_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            } else {
                // This is a direct field - store for later processing
                direct_fields.push(element.clone());
            }
        }

        // First pass: Generate nested structs for BackboneElements
        for (nested_field_name, nested_elements) in &nested_structs_info {
            if let Some(nested_struct) = self.generate_nested_struct(
                &struct_name,
                nested_field_name,
                nested_elements,
                structure_def,
            )? {
                // Store the nested struct in cache for later use
                self.type_cache
                    .insert(nested_struct.name.clone(), nested_struct);
            }
        }

        // Second pass: Process direct fields (now nested structs are available)
        for element in direct_fields {
            if let Some(field) = self.create_field_from_element(&element)? {
                rust_struct.add_field(field);
            }
        }

        // Cache the generated struct for future use
        self.type_cache.insert(struct_name, rust_struct.clone());

        Ok(rust_struct)
    }

    /// Generate a nested struct for BackboneElements
    fn generate_nested_struct(
        &mut self,
        parent_struct_name: &str,
        nested_field_name: &str,
        nested_elements: &[crate::fhir_types::ElementDefinition],
        parent_structure_def: &StructureDefinition,
    ) -> CodegenResult<Option<crate::rust_types::RustStruct>> {
        // Generate the nested struct name (e.g., BundleEntry, BundleLink)
        let nested_struct_name = format!(
            "{}{}",
            parent_struct_name,
            self.to_pascal_case(nested_field_name)
        );

        // Check if we've already generated this nested struct
        if self.type_cache.contains_key(&nested_struct_name) {
            return Ok(None);
        }

        // Create the nested struct
        let mut nested_struct = crate::rust_types::RustStruct::new(nested_struct_name.clone());

        // Add documentation
        nested_struct.doc_comment = Some(format!(
            "{parent_struct_name} nested structure for the '{nested_field_name}' field"
        ));

        // Use config to determine derives
        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
        }
        nested_struct.derives = derives;

        // Set base as BackboneElement since these are typically BackboneElements
        nested_struct.base_definition = Some("BackboneElement".to_string());

        // Process the nested elements
        let base_path = format!("{}.{}", parent_structure_def.name, nested_field_name);
        let mut sub_nested_structs = HashMap::new();
        let mut direct_fields = Vec::new();

        for element in nested_elements {
            if !element.path.starts_with(&base_path) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();

            if field_path.contains('.') {
                // This is a sub-nested field - collect it for recursive nested struct generation
                let sub_nested_field_name = field_path.split('.').next().unwrap();
                sub_nested_structs
                    .entry(sub_nested_field_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(element.clone());
            } else {
                // This is a direct field of this nested struct
                direct_fields.push(element.clone());
            }
        }

        // First, generate any sub-nested structs
        for (sub_nested_field_name, sub_nested_elements) in &sub_nested_structs {
            // For recursive calls, we need to create a modified context
            // The base path for sub-nested structs should be the current nested struct's path
            let sub_nested_struct_name = format!(
                "{}{}",
                nested_struct_name,
                self.to_pascal_case(sub_nested_field_name)
            );

            if !self.type_cache.contains_key(&sub_nested_struct_name) {
                let mut sub_nested_struct =
                    crate::rust_types::RustStruct::new(sub_nested_struct_name.clone());

                sub_nested_struct.doc_comment = Some(format!(
                    "{nested_struct_name} nested structure for the '{sub_nested_field_name}' field"
                ));

                // Use config to determine derives
                let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
                if self.config.with_serde {
                    derives.extend(vec!["Serialize".to_string(), "Deserialize".to_string()]);
                }
                sub_nested_struct.derives = derives;
                sub_nested_struct.base_definition = Some("BackboneElement".to_string());

                // Process the sub-nested elements
                let sub_base_path = format!("{base_path}.{sub_nested_field_name}");

                for element in sub_nested_elements {
                    if !element.path.starts_with(&sub_base_path) {
                        continue;
                    }

                    let field_path = element
                        .path
                        .strip_prefix(&format!("{sub_base_path}."))
                        .unwrap();

                    // Only process direct fields (no further recursion for now - can be extended)
                    if !field_path.contains('.') {
                        if let Some(field) = self.create_field_from_element(element)? {
                            sub_nested_struct.add_field(field);
                        }
                    }
                }

                // Store the sub-nested struct in cache
                self.type_cache
                    .insert(sub_nested_struct_name, sub_nested_struct);
            }
        }

        // Then, process direct fields (now sub-nested structs are available)
        for element in direct_fields {
            if let Some(field) = self.create_field_from_element(&element)? {
                nested_struct.add_field(field);
            }
        }

        Ok(Some(nested_struct))
    }

    /// Convert a string to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect()
    }

    /// Check if a field should use a nested struct type instead of BackboneElement
    fn should_use_nested_struct_type(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        element_types: &[crate::fhir_types::ElementType],
    ) -> bool {
        // Check if this element is a BackboneElement and we have nested elements for it
        if let Some(first_type) = element_types.first() {
            if let Some(code) = &first_type.code {
                if code == "BackboneElement" {
                    // Extract parent struct name and field name from the path
                    let path_parts: Vec<&str> = element.path.split('.').collect();
                    if path_parts.len() >= 2 {
                        let parent_struct_name = self.to_valid_rust_identifier(path_parts[0]);
                        let field_name = path_parts[path_parts.len() - 1];

                        // For nested structures, we need to build the correct nested struct name
                        // For example: Bundle.entry.search -> BundleEntrySearch
                        let nested_struct_name = if path_parts.len() == 2 {
                            // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                            format!("{}{}", parent_struct_name, self.to_pascal_case(field_name))
                        } else {
                            // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                            let mut nested_name = parent_struct_name;
                            for part in path_parts.iter().skip(1) {
                                nested_name.push_str(&self.to_pascal_case(part));
                            }
                            nested_name
                        };

                        // Check if we have generated this nested struct
                        return self.type_cache.contains_key(&nested_struct_name);
                    }
                }
            }
        }
        false
    }

    /// Create a RustField from an ElementDefinition
    fn create_field_from_element(
        &mut self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> CodegenResult<Option<crate::rust_types::RustField>> {
        use crate::rust_types::{RustField, RustType};
        use crate::type_mapper::TypeMapper;

        // Get the field name from the path (last segment)
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
        let rust_field_name = self.to_rust_field_name(field_name);

        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        // Get the field type
        let field_type = if let Some(element_types) = &element.element_type {
            // Check if this should use a nested struct type
            if self.should_use_nested_struct_type(element, element_types) {
                // Build the correct nested struct name based on the full path
                let path_parts: Vec<&str> = element.path.split('.').collect();
                let nested_struct_name = if path_parts.len() >= 2 {
                    let parent_struct_name = self.to_valid_rust_identifier(path_parts[0]);
                    if path_parts.len() == 2 {
                        // Direct nested struct (e.g., Bundle.entry -> BundleEntry)
                        format!(
                            "{}{}",
                            parent_struct_name,
                            self.to_pascal_case(path_parts[1])
                        )
                    } else {
                        // Sub-nested struct (e.g., Bundle.entry.search -> BundleEntrySearch)
                        let mut nested_name = parent_struct_name;
                        for part in path_parts.iter().skip(1) {
                            nested_name.push_str(&self.to_pascal_case(part));
                        }
                        nested_name
                    }
                } else {
                    format!("{}Unknown", self.to_valid_rust_identifier(&element.path))
                };

                if is_array {
                    RustType::Vec(Box::new(RustType::Custom(nested_struct_name)))
                } else {
                    RustType::Custom(nested_struct_name)
                }
            } else {
                let mut type_mapper = TypeMapper::new(&self.config, &mut self.value_set_manager);
                type_mapper.map_fhir_type_with_binding(
                    element_types,
                    element.binding.as_ref(),
                    is_array,
                )
            }
        } else {
            // No type specified, default to String
            if is_array {
                RustType::Vec(Box::new(RustType::String))
            } else {
                RustType::String
            }
        };

        // Create the field
        let mut field = RustField::new(rust_field_name.clone(), field_type);
        field.is_optional = is_optional;

        // Add documentation if available
        if let Some(short) = &element.short {
            field.doc_comment = Some(short.clone());
        } else if let Some(definition) = &element.definition {
            field.doc_comment = Some(definition.clone());
        }

        // Add serde rename if the field name was changed
        if rust_field_name != field_name {
            field = field.with_serde_rename(field_name.to_string());
        }

        Ok(Some(field))
    }

    /// Convert a FHIR field name to a valid Rust field name
    fn to_rust_field_name(&self, name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = clean_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        // Handle Rust keywords by appending underscore
        match snake_case.as_str() {
            "type" => "type_".to_string(),
            "use" => "use_".to_string(),
            "ref" => "ref_".to_string(),
            "mod" => "mod_".to_string(),
            "fn" => "fn_".to_string(),
            "let" => "let_".to_string(),
            "const" => "const_".to_string(),
            "static" => "static_".to_string(),
            "struct" => "struct_".to_string(),
            "enum" => "enum_".to_string(),
            "impl" => "impl_".to_string(),
            "trait" => "trait_".to_string(),
            "for" => "for_".to_string(),
            "if" => "if_".to_string(),
            "else" => "else_".to_string(),
            "while" => "while_".to_string(),
            "loop" => "loop_".to_string(),
            "match" => "match_".to_string(),
            "return" => "return_".to_string(),
            "where" => "where_".to_string(),
            "abstract" => "abstract_".to_string(),
            _ => snake_case,
        }
    }

    /// Generate a Rust struct and write it to a file
    pub fn generate_to_file<P: AsRef<Path>>(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: P,
    ) -> CodegenResult<()> {
        // Generate the main struct
        let rust_struct = self.generate_struct(structure_def)?;

        // Collect all related nested structs from cache
        let mut all_structs = vec![rust_struct.clone()];
        let struct_name_prefix = &rust_struct.name;

        // Find all nested structs that start with the main struct name
        for (cached_name, cached_struct) in &self.type_cache {
            if cached_name != &rust_struct.name && cached_name.starts_with(struct_name_prefix) {
                all_structs.push(cached_struct.clone());
            }
        }

        // Collect all imports needed for this file
        let mut imports = std::collections::HashSet::new();

        // Always include serde if enabled
        if self.config.with_serde {
            imports.insert("serde::{Deserialize, Serialize}".to_string());
        }

        // Collect custom types from all structs
        for struct_def in &all_structs {
            self.collect_custom_types_from_struct(struct_def, &mut imports);
        }

        // Generate import tokens
        let mut all_tokens = proc_macro2::TokenStream::new();
        for import in &imports {
            let import_tokens: proc_macro2::TokenStream = format!("use {import};")
                .parse()
                .expect("Invalid import statement");
            all_tokens.extend(import_tokens);
        }

        for struct_def in all_structs {
            let struct_tokens = self.token_generator.generate_struct(&struct_def);
            all_tokens.extend(struct_tokens);
        }

        // Parse the tokens into a syntax tree and format it
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| CodegenError::Generation {
            message: format!("Failed to parse generated tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

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

    /// Generate all ValueSet enums to separate files in the specified directory
    pub fn generate_enum_files<P: AsRef<Path>>(&mut self, enums_dir: P) -> CodegenResult<()> {
        let enums_dir = enums_dir.as_ref();

        // Create the enums directory if it doesn't exist
        if !enums_dir.exists() {
            fs::create_dir_all(enums_dir).map_err(CodegenError::Io)?;
        }

        // Generate a file for each cached enum
        for (enum_name, rust_enum) in self.value_set_manager.get_cached_enums() {
            let enum_filename = self.enum_name_to_filename(enum_name);
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
    pub fn generate_enums_mod_file<P: AsRef<Path>>(&self, enums_dir: P) -> CodegenResult<()> {
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
        let mut enum_names: Vec<_> = self.value_set_manager.get_cached_enums().keys().collect();
        enum_names.sort();

        // Generate module declarations and re-exports
        for enum_name in enum_names {
            let module_name = self.enum_name_to_module_name(enum_name);
            mod_content.push(format!("pub mod {module_name};"));
            mod_content.push(format!("pub use {module_name}::*;"));
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

    /// Convert an enum name to a filename (snake_case)
    fn enum_name_to_filename(&self, enum_name: &str) -> String {
        let snake_case = self.pascal_case_to_snake_case(enum_name);
        format!("{snake_case}.rs")
    }

    /// Convert an enum name to a module name (snake_case)
    fn enum_name_to_module_name(&self, enum_name: &str) -> String {
        self.pascal_case_to_snake_case(enum_name)
    }

    /// Convert PascalCase to snake_case with proper acronym handling
    fn pascal_case_to_snake_case(&self, pascal_case: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = pascal_case.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() && i > 0 {
                // Check if this is part of an acronym or start of a new word
                let is_acronym_continuation = i > 0 && chars[i - 1].is_uppercase();
                let is_followed_by_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();

                // Add underscore if:
                // 1. Previous char was lowercase (start of new word like "someWord")
                // 2. This is an acronym followed by lowercase (like "HTTPRequest" -> "http_request")
                if (i > 0 && chars[i - 1].is_lowercase())
                    || (is_acronym_continuation && is_followed_by_lowercase)
                {
                    result.push('_');
                }
            }

            result.push(ch.to_lowercase().next().unwrap());
        }

        result
    }

    /// Generate an enum for a value set binding
    pub fn generate_enum_for_value_set(
        &mut self,
        value_set_url: &str,
    ) -> CodegenResult<Option<RustEnum>> {
        // Check if we've already generated this enum
        if let Some(cached_enum) = self.enum_cache.get(value_set_url) {
            return Ok(Some(cached_enum.clone()));
        }

        // Generate a placeholder enum using the value set manager
        let enum_name = self
            .value_set_manager
            .generate_placeholder_enum(value_set_url);

        // Get the generated enum from the value set manager's cache
        if let Some(rust_enum) = self.value_set_manager.get_cached_enums().get(&enum_name) {
            // Cache it in our own cache as well
            self.enum_cache
                .insert(value_set_url.to_string(), rust_enum.clone());
            Ok(Some(rust_enum.clone()))
        } else {
            Ok(None)
        }
    }

    /// Check if any ValueSet enums have been generated
    pub fn has_cached_enums(&self) -> bool {
        !self.value_set_manager.get_cached_enums().is_empty()
    }

    fn capitalize_first_letter(&self, s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }

    /// Generate a proper Rust struct name from StructureDefinition URL or ID
    fn generate_struct_name(&self, structure_def: &StructureDefinition) -> String {
        let raw_name = if structure_def.name == "alternate" {
            // Special case for "alternate" name - use ID
            self.to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name.is_empty() {
            // No name provided - use ID
            self.to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name != structure_def.id && !structure_def.id.is_empty() {
            // Name and ID differ - prefer ID for uniqueness, especially for extensions
            // This handles cases like cqf-library where name="library" but id="cqf-library"
            self.to_valid_rust_identifier(&structure_def.id)
        } else {
            // Use name when it matches ID or ID is empty
            self.to_valid_rust_identifier(&structure_def.name)
        };

        // FHIR conventions is to have capitalized names for non-primitive types
        if structure_def.kind != "primitive-type" {
            self.capitalize_first_letter(&raw_name)
        } else {
            raw_name
        }
    }

    /// Convert a FHIR name to a valid Rust identifier while preserving the original as much as possible
    fn to_valid_rust_identifier(&self, name: &str) -> String {
        // For names that are already valid Rust identifiers, use them as-is
        if self.is_valid_rust_identifier(name) {
            return name.to_string();
        }

        // For names with spaces, dashes, or other characters, convert to PascalCase
        let mut result = String::new();
        let mut capitalize_next = true;

        for ch in name.chars() {
            if ch.is_alphanumeric() {
                if capitalize_next {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            } else {
                // Skip non-alphanumeric characters and capitalize the next letter
                capitalize_next = true;
            }
        }

        // Ensure it starts with a letter or underscore (Rust requirement)
        if result.is_empty() || result.chars().next().unwrap().is_numeric() {
            result = format!("_{result}");
        }

        // Handle common FHIR acronyms that should remain uppercase
        self.fix_acronyms(&result)
    }

    /// Fix common FHIR acronyms to maintain proper casing
    fn fix_acronyms(&self, name: &str) -> String {
        let mut result = name.to_string();

        // Common FHIR acronyms that should be uppercase
        let acronyms = [
            ("Cqf", "CQF"),     // Clinical Quality Framework
            ("Fhir", "FHIR"),   // Fast Healthcare Interoperability Resources
            ("Hl7", "HL7"),     // Health Level 7
            ("Http", "HTTP"),   // HyperText Transfer Protocol
            ("Https", "HTTPS"), // HTTP Secure
            ("Json", "JSON"),   // JavaScript Object Notation
            ("Xml", "XML"),     // eXtensible Markup Language
            ("Uuid", "UUID"),   // Universally Unique Identifier
            ("Uri", "URI"),     // Uniform Resource Identifier
            ("Url", "URL"),     // Uniform Resource Locator
            ("Api", "API"),     // Application Programming Interface
        ];

        for (from, to) in &acronyms {
            result = result.replace(from, to);
        }

        result
    }

    /// Check if a string is a valid Rust identifier
    fn is_valid_rust_identifier(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let mut chars = name.chars();
        let first_char = chars.next().unwrap();

        // First character must be a letter or underscore
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        // Remaining characters must be alphanumeric or underscore
        for ch in chars {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }

        // Check if it's a Rust keyword
        !self.is_rust_keyword(name)
    }

    /// Check if a string is a Rust keyword
    fn is_rust_keyword(&self, name: &str) -> bool {
        matches!(
            name,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "async"
                | "await"
                | "dyn"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "macro"
                | "override"
                | "priv"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "try"
        )
    }

    /// Convert a FHIR resource type name to snake_case filename
    pub fn to_filename(&self, structure_def: &StructureDefinition) -> String {
        // Use the same logic as struct name generation, then convert to snake_case
        let struct_name = self.generate_struct_name(structure_def);

        // Convert PascalCase to snake_case with proper acronym handling
        let mut result = String::new();
        let chars: Vec<char> = struct_name.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() && i > 0 {
                // Check if this is part of an acronym or start of a new word
                let is_acronym_continuation = i > 0 && chars[i - 1].is_uppercase();
                let is_followed_by_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();

                // Add underscore if:
                // 1. Previous char was lowercase (start of new word like "someWord")
                // 2. This is an acronym followed by lowercase (like "HTTPRequest" -> "http_request")
                if (i > 0 && chars[i - 1].is_lowercase())
                    || (is_acronym_continuation && is_followed_by_lowercase)
                {
                    result.push('_');
                }
            }

            result.push(ch.to_lowercase().next().unwrap());
        }

        format!("{result}.rs")
    }

    /// Collect all custom types referenced by a struct and add them to the imports set
    fn collect_custom_types_from_struct(
        &self,
        rust_struct: &RustStruct,
        imports: &mut std::collections::HashSet<String>,
    ) {
        // Add import for base type if present
        if let Some(base_def) = &rust_struct.base_definition {
            let base_type = base_def.split('/').next_back().unwrap_or(base_def);
            // Only add import if it's not a primitive type and not the current struct
            if !self.is_primitive_type(base_type) && base_type != rust_struct.name {
                // For commonly used FHIR types, we can assume they are in a common crate
                // This is a simplified approach - in a real implementation you might want
                // to track where types are defined more precisely
                let import_path = match base_type {
                    "Element" | "BackboneElement" | "DomainResource" | "Resource" => {
                        format!("crate::{}", self.to_snake_case(base_type))
                    }
                    _ => format!("crate::{}", self.to_snake_case(base_type)),
                };
                imports.insert(import_path);
            }
        }

        // Collect custom types from all fields
        for field in &rust_struct.fields {
            self.collect_custom_types_from_type(&field.field_type, imports, &rust_struct.name);
        }
    }

    /// Recursively collect custom types from a RustType
    fn collect_custom_types_from_type(
        &self,
        rust_type: &RustType,
        imports: &mut std::collections::HashSet<String>,
        current_struct_name: &str,
    ) {
        match rust_type {
            RustType::Custom(type_name) => {
                // Only add import if it's not a primitive type and not the current struct
                if !self.is_primitive_type(type_name) && type_name != current_struct_name {
                    let import_path = format!("crate::{}", self.to_snake_case(type_name));
                    imports.insert(import_path);
                }
            }
            RustType::Option(inner) => {
                self.collect_custom_types_from_type(inner, imports, current_struct_name);
            }
            RustType::Vec(inner) => {
                self.collect_custom_types_from_type(inner, imports, current_struct_name);
            }
            RustType::Reference(type_name) => {
                if !self.is_primitive_type(type_name) && type_name != current_struct_name {
                    let import_path = format!("crate::{}", self.to_snake_case(type_name));
                    imports.insert(import_path);
                }
            }
            // Primitive types don't need imports
            RustType::String | RustType::Integer | RustType::Boolean | RustType::Float => {}
        }
    }

    /// Check if a type name represents a primitive Rust type
    fn is_primitive_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "String" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "bool" | "usize" | "isize"
        )
    }

    /// Convert a PascalCase type name to snake_case for module imports
    fn to_snake_case(&self, name: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = name.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() && i > 0 {
                // Check if this is part of an acronym or start of a new word
                let is_acronym_continuation = i > 0 && chars[i - 1].is_uppercase();
                let is_followed_by_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();

                // Add underscore if:
                // 1. Previous char was lowercase (start of new word like "someWord")
                // 2. This is an acronym followed by lowercase (like "HTTPRequest" -> "http_request")
                if (i > 0 && chars[i - 1].is_lowercase())
                    || (is_acronym_continuation && is_followed_by_lowercase)
                {
                    result.push('_');
                }
            }

            result.push(ch.to_lowercase().next().unwrap());
        }

        result
    }

    /// Generate comprehensive documentation for a struct based on StructureDefinition metadata
    fn generate_struct_documentation(&self, structure_def: &StructureDefinition) -> Option<String> {
        let mut doc_lines = Vec::new();

        // Add title if available, otherwise use name
        if let Some(title) = &structure_def.title {
            doc_lines.push(title.clone());
        } else {
            doc_lines.push(structure_def.name.clone());
        }

        // Add description if available
        if let Some(description) = &structure_def.description {
            doc_lines.push("".to_string());
            // Clean up the description by removing carriage returns and other problematic characters
            let cleaned_description = description.replace('\r', "").replace('\n', " ");
            doc_lines.push(cleaned_description);
        }

        // Add source information
        doc_lines.push("".to_string());
        doc_lines.push("**Source:**".to_string());
        doc_lines.push(format!("- URL: {}", structure_def.url));

        if let Some(version) = &structure_def.version {
            doc_lines.push(format!("- Version: {version}"));
        }

        doc_lines.push(format!("- Kind: {}", structure_def.kind));
        doc_lines.push(format!("- Type: {}", structure_def.base_type));

        if let Some(base_def) = &structure_def.base_definition {
            doc_lines.push(format!("- Base Definition: {base_def}"));
        }

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_valid_rust_identifier_conversion() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        // Test FHIR resource names that should preserve original case
        assert_eq!(
            generator.to_valid_rust_identifier("StructureDefinition"),
            "StructureDefinition"
        );
        assert_eq!(generator.to_valid_rust_identifier("Patient"), "Patient");
        assert_eq!(
            generator.to_valid_rust_identifier("Observation"),
            "Observation"
        );
        assert_eq!(
            generator.to_valid_rust_identifier("CodeSystem"),
            "CodeSystem"
        );

        // Test names that need conversion due to special characters
        assert_eq!(generator.to_valid_rust_identifier("patient"), "patient");

        // Test names with spaces
        assert_eq!(
            generator.to_valid_rust_identifier("Relative Date Criteria"),
            "RelativeDateCriteria"
        );
        assert_eq!(generator.to_valid_rust_identifier("Care Plan"), "CarePlan");

        // Test names with dashes and underscores
        assert_eq!(
            generator.to_valid_rust_identifier("patient-name"),
            "PatientName"
        );
        assert_eq!(
            generator.to_valid_rust_identifier("patient_name"),
            "patient_name"
        );

        // Test mixed separators
        assert_eq!(
            generator.to_valid_rust_identifier("some-complex_name with.spaces"),
            "SomeComplexNameWithSpaces"
        );

        // Test empty and edge cases
        assert_eq!(generator.to_valid_rust_identifier(""), "_");
        assert_eq!(generator.to_valid_rust_identifier("   "), "_");
        assert_eq!(generator.to_valid_rust_identifier("a"), "a");
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
}
