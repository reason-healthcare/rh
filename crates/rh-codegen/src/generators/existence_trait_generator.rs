use crate::fhir_types::StructureDefinition;
use crate::rust_types::{RustTrait, RustTraitMethod, RustType};
use crate::CodegenResult;

/// Generator for existence check traits that provide has_xxx() methods
pub struct ExistenceTraitGenerator;

impl Default for ExistenceTraitGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ExistenceTraitGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate an existence trait with has_xxx() methods for checking field presence
    pub fn generate_existence_trait(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustTrait> {
        let struct_name = crate::naming::Naming::struct_name(structure_def);
        let trait_name = format!("{struct_name}Existence");
        let base_trait = self.get_base_trait_name(structure_def);

        let mut rust_trait = RustTrait::new(trait_name.clone());
        rust_trait.doc_comment = Some(self.generate_trait_documentation(structure_def));

        // Only add super trait if it's not empty (Resource has no super trait)
        if !base_trait.is_empty() {
            rust_trait.super_traits.push(base_trait.clone());
        }

        // Add existence methods for each element
        self.add_existence_methods(structure_def, &mut rust_trait)?;

        Ok(rust_trait)
    }

    /// Add existence check methods for all elements
    fn add_existence_methods(
        &self,
        structure_def: &StructureDefinition,
        rust_trait: &mut RustTrait,
    ) -> CodegenResult<()> {
        if let Some(snapshot) = &structure_def.snapshot {
            for element in &snapshot.element {
                // Skip the root element
                if element.path == structure_def.id {
                    continue;
                }

                // Extract field name from the element path
                if let Some(field_name) =
                    element.path.strip_prefix(&format!("{}.", structure_def.id))
                {
                    // Skip nested elements (those with dots in the remaining path)
                    if field_name.contains('.') {
                        continue;
                    }

                    self.add_existence_method(element, field_name, rust_trait)?;
                }
            }
        }

        Ok(())
    }

    /// Add a single existence check method
    fn add_existence_method(
        &self,
        element: &crate::fhir_types::ElementDefinition,
        field_name: &str,
        rust_trait: &mut RustTrait,
    ) -> CodegenResult<()> {
        let rust_field_name = crate::naming::Naming::field_name(field_name);
        let method_name = format!("has_{rust_field_name}");

        // Determine if this is an array field
        let is_array = element
            .max
            .as_ref()
            .map(|max| max != "1" && max != "0")
            .unwrap_or(false);

        // Generate appropriate documentation
        let doc_comment = if is_array {
            format!("Returns true if the {rust_field_name} field is not empty.")
        } else {
            format!("Returns true if the {rust_field_name} field is present (Some).")
        };

        let method = RustTraitMethod::new(method_name)
            .with_return_type(RustType::Boolean)
            .with_doc(doc_comment);

        rust_trait.add_method(method);
        Ok(())
    }

    /// Get the appropriate base trait name based on the structure definition
    fn get_base_trait_name(&self, structure_def: &StructureDefinition) -> String {
        // Special case: Resource itself should not have a super trait to avoid circular dependency
        if structure_def.id == "Resource" {
            return String::new(); // No super trait
        }

        // Check if this inherits from DomainResource
        if let Some(base_definition) = &structure_def.base_definition {
            if base_definition.contains("DomainResource") {
                return "DomainResourceExistence".to_string();
            }
        }

        // Default to Resource for other cases
        "ResourceExistence".to_string()
    }

    /// Generate comprehensive documentation for the existence trait
    fn generate_trait_documentation(&self, structure_def: &StructureDefinition) -> String {
        let mut docs = vec![
            format!("{} Existence Checks", structure_def.id),
            "".to_string(),
            "This trait provides existence check methods for this FHIR resource type.".to_string(),
            "".to_string(),
        ];

        if let Some(description) = &structure_def.description {
            docs.push(description.clone());
            docs.push("".to_string());
        }

        docs.extend(vec![
            "**Source:**".to_string(),
            format!("- URL: {}", &structure_def.url),
            format!(
                "- Version: {}",
                structure_def
                    .version
                    .as_ref()
                    .unwrap_or(&"Unknown".to_string())
            ),
            format!("- Kind: {}", &structure_def.kind),
            format!("- Type: {}", &structure_def.base_type),
        ]);

        if let Some(base_definition) = &structure_def.base_definition {
            docs.push(format!("- Base Definition: {base_definition}"));
        }

        docs.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_existence_trait_generation() {
        let _generator = ExistenceTraitGenerator::new();

        // This would need a proper StructureDefinition for a real test
        // For now, just verify the generator can be created
    }

    #[test]
    fn test_base_trait_selection() {
        let generator = ExistenceTraitGenerator::new();

        let structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "TestResource".to_string(),
            url: "http://example.com/TestResource".to_string(),
            version: None,
            name: "TestResource".to_string(),
            title: None,
            status: "draft".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "TestResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        let base_trait = generator.get_base_trait_name(&structure_def);
        assert_eq!(base_trait, "DomainResourceExistence");
    }
}
