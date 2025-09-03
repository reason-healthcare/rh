//! Generator for accessor traits.
use crate::fhir_types::{ElementDefinition, StructureDefinition};
use crate::generators::TypeUtilities;
use crate::rust_types::{RustTrait, RustTraitMethod, RustType};
use crate::CodegenResult;

#[derive(Default)]
pub struct AccessorTraitGenerator {}

impl AccessorTraitGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_accessor_methods(
        &self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        let elements = structure_def
            .differential
            .as_ref()
            .map_or(Vec::new(), |d| d.element.clone());

        if elements.is_empty() {
            if let Some(snapshot) = &structure_def.snapshot {
                let snapshot_elements = snapshot.element.clone();
                for element in &snapshot_elements {
                    if self.should_generate_accessor(element, structure_def) {
                        if let Some(method) = self.create_accessor_method(element)? {
                            rust_trait.add_method(method);
                        }
                    }
                }
            }
        } else {
            for element in &elements {
                if self.should_generate_accessor(element, structure_def) {
                    if let Some(method) = self.create_accessor_method(element)? {
                        rust_trait.add_method(method);
                    }
                }
            }
        }

        self.add_choice_type_accessor_methods(rust_trait, structure_def)?;

        Ok(())
    }

    fn should_generate_accessor(
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

        // We don't generate accessors for choice types here, they are handled separately.
        let field_name = path_parts[1];
        !field_name.ends_with("[x]")
    }

    fn create_accessor_method(
        &self,
        element: &ElementDefinition,
    ) -> CodegenResult<Option<RustTraitMethod>> {
        let path_parts: Vec<&str> = element.path.split('.').collect();
        let field_name = path_parts.last().unwrap().to_string();
        let rust_field_name = crate::naming::Naming::field_name(&field_name);

        let is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<i32>()
                .unwrap_or(1)
                > 1;

        let Some(element_type) = element.element_type.as_ref().and_then(|t| t.first()) else {
            return Ok(None);
        };

        let mut rust_type =
            TypeUtilities::map_fhir_type_to_rust(element_type, &field_name, &element.path)?;

        if is_array {
            rust_type = RustType::Slice(Box::new(rust_type));
        }

        if is_optional && !is_array {
            rust_type = RustType::Option(Box::new(rust_type));
        }

        let return_type = match rust_type {
            RustType::String => {
                // Check if this is actually an enum field (FHIR "code" type)
                if let Some(element_types) = &element.element_type {
                    if let Some(first_type) = element_types.first() {
                        if let Some(code) = &first_type.code {
                            if code == "code" {
                                // This is an enum field, should return String instead of &str
                                RustType::String
                            } else {
                                // This is a regular string field, return &str
                                RustType::Custom("&str".to_string())
                            }
                        } else {
                            RustType::Custom("&str".to_string())
                        }
                    } else {
                        RustType::Custom("&str".to_string())
                    }
                } else {
                    RustType::Custom("&str".to_string())
                }
            }
            RustType::Vec(inner) => RustType::Slice(inner),
            other => other,
        };

        let method = RustTraitMethod::new(rust_field_name)
            .with_doc(format!("Returns a reference to the {} field.", field_name))
            .with_return_type(if is_optional && !is_array {
                return_type.clone().wrap_in_option()
            } else {
                return_type.clone()
            })
            .with_body(format!("self.{}", field_name));

        Ok(Some(method))
    }

    fn add_choice_type_accessor_methods(
        &self,
        _rust_trait: &mut RustTrait,
        _structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Implementation for choice type accessors can be added here.
        Ok(())
    }
}
