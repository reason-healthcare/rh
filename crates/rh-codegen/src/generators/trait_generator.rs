//! Trait generation functionality
//!
//! This module handles the generation of Rust traits from FHIR StructureDefinitions.

use crate::fhir_types::StructureDefinition;
use crate::generators::accessor_trait_generator::AccessorTraitGenerator;
use crate::generators::existence_trait_generator::ExistenceTraitGenerator;
use crate::generators::mutator_trait_generator::MutatorTraitGenerator;
use crate::generators::DocumentationGenerator;
use crate::rust_types::RustTrait;
use crate::{CodegenError, CodegenResult};

/// Trait generator for FHIR StructureDefinitions
pub struct TraitGenerator {
    accessor_generator: AccessorTraitGenerator,
    mutator_generator: MutatorTraitGenerator,
    existence_generator: ExistenceTraitGenerator,
}

impl TraitGenerator {
    /// Create a new trait generator
    pub fn new() -> Self {
        Self {
            accessor_generator: AccessorTraitGenerator::new(),
            mutator_generator: MutatorTraitGenerator::new(),
            existence_generator: ExistenceTraitGenerator::new(),
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
            "Mutators" => {
                self.mutator_generator
                    .add_mutator_methods(&mut rust_trait, structure_def)?;
            }
            "Existence" => {
                // For existence traits, we need to generate it differently
                return self
                    .existence_generator
                    .generate_existence_trait(structure_def);
            }
            _ => {
                return Err(CodegenError::Generation {
                    message: format!("Unknown trait category: {}", category),
                });
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
        // Special case: Resource itself should not have super traits to avoid circular dependency
        if structure_def.id == "Resource" {
            return Ok(());
        }

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
