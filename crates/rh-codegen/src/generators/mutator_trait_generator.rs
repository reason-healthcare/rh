//! Generator for mutator traits.
use crate::fhir_types::{ElementDefinition, StructureDefinition};
use crate::generators::TypeUtilities;
use crate::rust_types::{RustTrait, RustTraitMethod, RustType};
use crate::CodegenResult;

#[derive(Default)]
pub struct MutatorTraitGenerator {}

impl MutatorTraitGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mutator_methods(
        &self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Add constructor method
        self.add_constructor_method(rust_trait, structure_def)?;

        let elements = structure_def
            .differential
            .as_ref()
            .map_or(Vec::new(), |d| d.element.clone());

        if elements.is_empty() {
            if let Some(snapshot) = &structure_def.snapshot {
                let snapshot_elements = snapshot.element.clone();
                for element in &snapshot_elements {
                    if self.should_generate_mutator(element, structure_def) {
                        self.add_mutator_methods_for_element(rust_trait, element)?;
                    }
                }
            }
        } else {
            for element in &elements {
                if self.should_generate_mutator(element, structure_def) {
                    self.add_mutator_methods_for_element(rust_trait, element)?;
                }
            }
        }

        self.add_choice_type_mutator_methods(rust_trait, structure_def)?;

        Ok(())
    }

    fn should_generate_mutator(
        &self,
        element: &ElementDefinition,
        structure_def: &StructureDefinition,
    ) -> bool {
        let field_path = &element.path;
        let base_name = &structure_def.name;

        // The path must start with the base name of the structure.
        if !field_path.starts_with(base_name) {
            return false;
        }

        // We are interested in direct fields of the resource, which have paths like "Patient.active".
        // Splitting by '.' should result in exactly two parts.
        let path_parts: Vec<&str> = field_path.split('.').collect();
        if path_parts.len() != 2 {
            return false;
        }

        // The first part must match the base name.
        if path_parts[0] != base_name {
            return false;
        }

        // We don't generate mutators for choice types here, they are handled separately.
        let field_name = path_parts[1];
        !field_name.ends_with("[x]")
    }

    fn add_mutator_methods_for_element(
        &self,
        rust_trait: &mut RustTrait,
        element: &ElementDefinition,
    ) -> CodegenResult<()> {
        let path_parts: Vec<&str> = element.path.split('.').collect();
        let field_name = path_parts.last().unwrap().to_string();
        let rust_field_name = crate::naming::Naming::field_name(&field_name);

        let _is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<i32>()
                .unwrap_or(1)
                > 1;

        // Use binding-aware type mapping
        let rust_type = self.get_field_rust_type(element, &field_name)?;

        // Always add set_ method
        self.add_set_method(
            rust_trait,
            &rust_field_name,
            &field_name,
            &rust_type,
            is_array,
        )?;

        // Add add_ method for arrays
        if is_array {
            self.add_add_method(rust_trait, &rust_field_name, &field_name, &rust_type)?;
        }

        Ok(())
    }

    fn add_set_method(
        &self,
        rust_trait: &mut RustTrait,
        rust_field_name: &str,
        field_name: &str,
        rust_type: &RustType,
        is_array: bool,
    ) -> CodegenResult<()> {
        let method_name = format!("set_{rust_field_name}");

        let parameter_type = if is_array {
            // For arrays, set method takes a Vec
            RustType::Vec(Box::new(rust_type.clone()))
        } else {
            rust_type.clone()
        };

        let method = RustTraitMethod::new(method_name)
            .with_doc(format!(
                "Sets the {field_name} field and returns self for chaining."
            ))
            .with_parameter("value".to_string(), parameter_type)
            .with_return_type(RustType::Custom("Self".to_string()))
            .with_body(format!("self.{field_name} = value; self"))
            .with_self_param(Some("self".to_string())); // Take self by value for builder pattern

        rust_trait.add_method(method);
        Ok(())
    }

    fn add_add_method(
        &self,
        rust_trait: &mut RustTrait,
        rust_field_name: &str,
        field_name: &str,
        rust_type: &RustType,
    ) -> CodegenResult<()> {
        let method_name = format!("add_{rust_field_name}");

        let method = RustTraitMethod::new(method_name)
            .with_doc(format!(
                "Adds an item to the {field_name} field and returns self for chaining."
            ))
            .with_parameter("item".to_string(), rust_type.clone())
            .with_return_type(RustType::Custom("Self".to_string()))
            .with_body(format!("self.{field_name}.push(item); self"))
            .with_self_param(Some("self".to_string())); // Take self by value for builder pattern

        rust_trait.add_method(method);
        Ok(())
    }

    fn add_choice_type_mutator_methods(
        &self,
        _rust_trait: &mut RustTrait,
        _structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Implementation for choice type mutators can be added here.
        Ok(())
    }

    /// Add a constructor method that creates an instance with default/empty values
    fn add_constructor_method(
        &self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        let struct_name = crate::naming::Naming::struct_name(structure_def);

        // Determine the module path for the import
        let is_profile = crate::generators::type_registry::TypeRegistry::is_profile(structure_def);
        let module = if is_profile { "profiles" } else { "resources" };
        let snake_name = crate::naming::Naming::to_snake_case(&struct_name);
        let struct_import = format!("hl7_fhir_r4_core::{module}::{snake_name}::{struct_name}");
        let trait_import = format!("hl7_fhir_r4_core::traits::{snake_name}::{struct_name}Mutators");

        // Basic constructor with no parameters - supports method chaining
        let new_method = RustTraitMethod::new("new".to_string())
            .with_doc(format!(
                "Create a new {struct_name} with default/empty values.\n\nAll optional fields will be set to None and array fields will be empty vectors.\nSupports method chaining with set_xxx() and add_xxx() methods.\n\n# Example\n```rust\nuse {struct_import};\nuse {trait_import};\n\nlet resource = {struct_name}::new();\n// Can be used with method chaining:\n// resource.set_field(value).add_item(item);\n```"
            ))
            .with_return_type(RustType::Custom("Self".to_string()))
            .with_self_param(None); // No self parameter for constructor

        rust_trait.add_method(new_method);

        Ok(())
    }

    /// Get the Rust type for a field element, considering ValueSet bindings.
    /// For code fields with required bindings, returns the enum type name.
    /// Otherwise, delegates to TypeUtilities for standard type mapping.
    fn get_field_rust_type(
        &self,
        element: &ElementDefinition,
        field_name: &str,
    ) -> CodegenResult<RustType> {
        let Some(element_type) = element.element_type.as_ref().and_then(|t| t.first()) else {
            return Ok(RustType::String);
        };

        let Some(code) = &element_type.code else {
            return Ok(RustType::String);
        };

        // Check if this is a code type with a required binding - if so, use enum type
        if code == "code" {
            if let Some(binding) = &element.binding {
                if binding.strength == "required" {
                    if let Some(value_set_url) = &binding.value_set {
                        // Extract enum name from ValueSet URL
                        // E.g., http://hl7.org/fhir/ValueSet/account-status|4.0.1 -> AccountStatus
                        if let Some(enum_name) =
                            self.extract_enum_name_from_value_set(value_set_url)
                        {
                            return Ok(RustType::Custom(enum_name));
                        }
                    }
                }
            }
        }

        // Otherwise, use the standard type mapping
        TypeUtilities::map_fhir_type_to_rust(element_type, field_name, &element.path)
    }

    /// Extract enum type name from a ValueSet URL
    /// E.g., "http://hl7.org/fhir/ValueSet/account-status" -> "AccountStatus"
    fn extract_enum_name_from_value_set(&self, url: &str) -> Option<String> {
        // Remove version suffix if present (e.g., |4.0.1)
        let url_without_version = url.split('|').next().unwrap_or(url);

        // Extract the last part after the last /
        let value_set_name = url_without_version.split('/').next_back()?;

        // Use the same logic as ValueSetManager::generate_enum_name for consistency
        // Split on hyphens and capitalize each part to get PascalCase
        let name = value_set_name
            .split(&['-', '.'][..])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        // Ensure it's a valid Rust identifier
        if name.chars().next().unwrap_or('0').is_ascii_digit() {
            Some(format!("ValueSet{name}"))
        } else {
            Some(name)
        }
    }
}
