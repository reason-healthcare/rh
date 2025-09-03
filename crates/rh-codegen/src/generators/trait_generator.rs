//! Trait generation functionality
//!
//! This module handles the generation of Rust traits from FHIR StructureDefinitions.

use crate::fhir_types::StructureDefinition;
use crate::generators::accessor_trait_generator::AccessorTraitGenerator;
use crate::generators::DocumentationGenerator;
use crate::rust_types::RustTrait;
use crate::{CodegenError, CodegenResult};

/// Trait generator for FHIR StructureDefinitions
#[derive(Default)]
pub struct TraitGenerator {
    accessor_generator: AccessorTraitGenerator,
}

impl TraitGenerator {
    /// Create a new trait generator
    pub fn new() -> Self {
        Self {
            accessor_generator: AccessorTraitGenerator::new(),
        }
    }

    /// Generate a Rust trait from a FHIR StructureDefinition
    pub fn generate_trait(
        &mut self,
        structure_def: &StructureDefinition,
        category: &str,
    ) -> CodegenResult<RustTrait> {
        if structure_def.kind == "logical" {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping LogicalModel '{}' - logical models are not generated as traits",
                    structure_def.name
                ),
            });
        }

        let trait_name = format!(
            "{}{}",
            crate::naming::Naming::struct_name(structure_def),
            category
        );
        let mut rust_trait = RustTrait::new(trait_name);
        rust_trait.doc_comment =
            DocumentationGenerator::generate_trait_documentation(structure_def);

        self.add_inheritance_relationship(&mut rust_trait, structure_def, category)?;

        match category {
            "Accessors" => {
                self.accessor_generator
                    .add_accessor_methods(&mut rust_trait, structure_def)?;
            }
            _ => {
                // Other categories will be handled here
            }
        }

        Ok(rust_trait)
    }

    /// Add inheritance relationship if the StructureDefinition has a base definition
    fn add_inheritance_relationship(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
        category: &str,
    ) -> CodegenResult<()> {
        if let Some(base_def) = &structure_def.base_definition {
            if let Some(parent_trait_name) = self.extract_trait_name_from_url(base_def) {
                if self.is_valid_fhir_base_type(&parent_trait_name) {
                    rust_trait
                        .super_traits
                        .push(format!("{}{}", parent_trait_name, category));
                }
            }
        }
        Ok(())
    }

    /// Extract trait name from FHIR StructureDefinition URL
    fn extract_trait_name_from_url(&self, url: &str) -> Option<String> {
        url.split('/').last().map(|s| s.to_string())
    }

    /// Check if the given type name is a valid FHIR base type for inheritance
    fn is_valid_fhir_base_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "Resource" | "DomainResource" | "Element" | "BackboneElement"
        )
    }
}
