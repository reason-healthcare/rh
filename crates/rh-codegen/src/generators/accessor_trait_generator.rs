//! Generator for accessor traits.
use crate::fhir_types::{ElementDefinition, ElementType, StructureDefinition};
use crate::rust_types::{RustTrait, RustTraitMethod, RustType};
use crate::type_mapper::TypeMapper;
use crate::value_sets::ValueSetManager;
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
        // Create TypeMapper for proper type resolution
        let config = crate::config::CodegenConfig::default();
        let mut value_set_manager = ValueSetManager::new();
        let mut type_mapper = TypeMapper::new(&config, &mut value_set_manager);

        let elements = structure_def
            .differential
            .as_ref()
            .map_or(Vec::new(), |d| d.element.clone());

        if elements.is_empty() {
            if let Some(snapshot) = &structure_def.snapshot {
                let snapshot_elements = snapshot.element.clone();
                for element in &snapshot_elements {
                    if self.should_generate_accessor(element, structure_def) {
                        if let Some(method) =
                            self.create_accessor_method(element, &mut type_mapper)?
                        {
                            rust_trait.add_method(method);
                        }
                    }
                }
            }
        } else {
            for element in &elements {
                if self.should_generate_accessor(element, structure_def) {
                    if let Some(method) = self.create_accessor_method(element, &mut type_mapper)? {
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
        type_mapper: &mut TypeMapper,
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

        let Some(element_types) = element.element_type.as_ref() else {
            return Ok(None);
        };

        // Check if this is a BackboneElement that should use a specific nested type
        let rust_type = if self.is_backbone_element(element_types) {
            self.get_nested_type_for_backbone_element(element, is_array)
        } else {
            // Use TypeMapper to get the correct type including enum bindings
            type_mapper.map_fhir_type_with_binding(
                element_types,
                element.binding.as_ref(),
                is_array,
            )
        };

        // Convert the type for trait return types
        let return_type = match rust_type {
            RustType::Custom(_) => {
                // For enum types or custom types, keep as-is
                // Don't convert StringType to &str to maintain compatibility with implementations
                rust_type.clone()
            }
            RustType::Vec(inner) => RustType::Slice(inner),
            other => other,
        };

        let method = RustTraitMethod::new(rust_field_name)
            .with_doc(format!("Returns a reference to the {field_name} field."))
            .with_return_type(if is_optional && !is_array {
                return_type.clone().wrap_in_option()
            } else {
                return_type.clone()
            })
            .with_body(format!("self.{field_name}"));

        Ok(Some(method))
    }

    /// Check if the element types contain BackboneElement
    fn is_backbone_element(&self, element_types: &[ElementType]) -> bool {
        element_types
            .iter()
            .any(|et| et.code.as_deref() == Some("BackboneElement"))
    }

    /// Get the specific nested type for a BackboneElement field
    fn get_nested_type_for_backbone_element(
        &self,
        element: &ElementDefinition,
        is_array: bool,
    ) -> RustType {
        let path_parts: Vec<&str> = element.path.split('.').collect();

        if path_parts.len() == 2 {
            let resource_name = path_parts[0];
            let field_name = path_parts[1];

            // Generate the expected nested type name: ResourceFieldName (e.g., AccountCoverage)
            let field_name_pascal = crate::naming::Naming::to_pascal_case(field_name);
            let nested_type_name = format!("{resource_name}{field_name_pascal}");

            let rust_type = RustType::Custom(nested_type_name);

            if is_array {
                RustType::Vec(Box::new(rust_type))
            } else {
                rust_type
            }
        } else {
            // Fallback to BackboneElement if we can't determine the specific type
            let rust_type = RustType::Custom("BackboneElement".to_string());
            if is_array {
                RustType::Vec(Box::new(rust_type))
            } else {
                rust_type
            }
        }
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
