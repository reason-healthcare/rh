//! Trait generation functionality
//!
//! This module handles the generation of Rust traits from FHIR StructureDefinitions.

use crate::fhir_types::StructureDefinition;
use crate::generators::{utils::GeneratorUtils, DocumentationGenerator};
use crate::rust_types::{RustTrait, RustTraitMethod, RustType};
use crate::{CodegenError, CodegenResult};

/// Trait generator for FHIR StructureDefinitions
#[derive(Default)]
pub struct TraitGenerator {}

impl TraitGenerator {
    /// Create a new trait generator
    pub fn new() -> Self {
        Self {}
    }

    /// Generate a Rust trait from a FHIR StructureDefinition
    pub fn generate_trait(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustTrait> {
        // Skip LogicalModels as they are conceptual models, not implementable types
        if structure_def.kind == "logical" {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping LogicalModel '{}' - logical models are not generated as traits",
                    structure_def.name
                ),
            });
        }

        // Skip examples
        if structure_def.url.to_lowercase().contains("example") {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping example StructureDefinition '{}'",
                    structure_def.name
                ),
            });
        }

        // Generate trait name from the structure definition
        let trait_name = GeneratorUtils::generate_struct_name(structure_def);

        // Create the trait with enhanced documentation
        let mut rust_trait = RustTrait::new(trait_name);
        rust_trait.doc_comment =
            DocumentationGenerator::generate_trait_documentation(structure_def);

        // Add common FHIR methods based on the type
        self.add_common_fhir_trait_methods(&mut rust_trait, structure_def)?;

        // Add specific methods based on the structure definition elements
        self.add_element_based_trait_methods(&mut rust_trait, structure_def)?;

        Ok(rust_trait)
    }

    /// Add common FHIR trait methods based on the structure type
    pub fn add_common_fhir_trait_methods(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // All FHIR resources should have extensions
        let extensions_method = RustTraitMethod::new("extensions".to_string())
            .with_doc("Gets the extensions for this resource".to_string())
            .with_return_type(RustType::Vec(Box::new(RustType::Custom(
                "Extension".to_string(),
            ))));

        rust_trait.add_method(extensions_method);

        // DomainResource types should have narrative
        if structure_def.base_type == "DomainResource" || structure_def.name == "DomainResource" {
            let narrative_method = RustTraitMethod::new("narrative".to_string())
                .with_doc("Gets the narrative for this domain resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "Narrative".to_string(),
                ))));

            rust_trait.add_method(narrative_method);
        }

        // Resource types should have id and meta
        if structure_def.kind == "resource" || structure_def.base_type == "Resource" {
            let id_method = RustTraitMethod::new("id".to_string())
                .with_doc("Gets the logical ID of this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::String)));

            rust_trait.add_method(id_method);

            let meta_method = RustTraitMethod::new("meta".to_string())
                .with_doc("Gets the metadata about this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "Meta".to_string(),
                ))));

            rust_trait.add_method(meta_method);
        }

        Ok(())
    }

    /// Add trait methods based on the specific elements in the structure definition
    pub fn add_element_based_trait_methods(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Get elements from differential or snapshot
        let elements = if let Some(differential) = &structure_def.differential {
            &differential.element
        } else if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else {
            return Ok(()); // No elements to process
        };

        // Process important elements to create trait methods
        for element in elements {
            // Skip the root element
            if element.path == structure_def.name || element.path == structure_def.base_type {
                continue;
            }

            // Only create trait methods for direct fields (not nested)
            let base_path = if structure_def.base_type != structure_def.name
                && !structure_def.base_type.is_empty()
            {
                &structure_def.base_type
            } else {
                &structure_def.name
            };

            if !element.path.starts_with(&format!("{base_path}.")) {
                continue;
            }

            let field_path = element.path.strip_prefix(&format!("{base_path}.")).unwrap();

            // Only process direct fields (no nested fields)
            if !field_path.contains('.') {
                // Check if this element should have a trait method
                if self.should_create_trait_method_for_element(element) {
                    if let Some(method) = self.create_trait_method_from_element(element)? {
                        rust_trait.add_method(method);
                    }
                }
            }
        }

        Ok(())
    }

    /// Determine if an element should have a trait method created for it
    pub fn should_create_trait_method_for_element(
        &self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> bool {
        let field_name = element.path.split('.').next_back().unwrap_or("");

        // Create trait methods for commonly accessed FHIR fields
        matches!(
            field_name,
            "status"
                | "identifier"
                | "active"
                | "name"
                | "telecom"
                | "address"
                | "gender"
                | "birthDate"
                | "photo"
                | "qualification"
                | "communication"
                | "endpoint"
                | "type"
                | "category"
                | "subject"
                | "code"
                | "value"
                | "component"
                | "interpretation"
                | "note"
                | "bodySite"
                | "method"
                | "specimen"
                | "device"
                | "referenceRange"
                | "hasMember"
                | "derivedFrom"
        )
    }

    /// Create a trait method from an element definition
    pub fn create_trait_method_from_element(
        &mut self,
        element: &crate::fhir_types::ElementDefinition,
    ) -> CodegenResult<Option<RustTraitMethod>> {
        let field_name = element.path.split('.').next_back().unwrap_or("unknown");
        let rust_field_name = GeneratorUtils::to_rust_field_name(field_name);

        // Determine if this field is optional (min = 0)
        let is_optional = element.min.unwrap_or(0) == 0;

        // Determine if this field is an array (max = "*" or > 1)
        let is_array = element
            .max
            .as_ref()
            .is_some_and(|max| max == "*" || max.parse::<u32>().unwrap_or(1) > 1);

        // Get the return type based on the element type
        let return_type = if let Some(element_types) = &element.element_type {
            if let Some(first_type) = element_types.first() {
                let base_type = if let Some(code) = &first_type.code {
                    match code.as_str() {
                        "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" => {
                            RustType::String
                        }
                        "boolean" => RustType::Boolean,
                        "integer" | "positiveInt" | "unsignedInt" => RustType::Integer,
                        "decimal" => RustType::Float,
                        _ => RustType::Custom(GeneratorUtils::capitalize_first_letter(code)),
                    }
                } else {
                    RustType::String
                };

                if is_array {
                    if is_optional {
                        RustType::Option(Box::new(RustType::Vec(Box::new(base_type))))
                    } else {
                        RustType::Vec(Box::new(base_type))
                    }
                } else if is_optional {
                    RustType::Option(Box::new(base_type))
                } else {
                    base_type
                }
            } else {
                RustType::Option(Box::new(RustType::String))
            }
        } else {
            RustType::Option(Box::new(RustType::String))
        };

        // Create the trait method
        let method = RustTraitMethod::new(rust_field_name.clone())
            .with_doc(format!("Gets the {field_name} field"))
            .with_return_type(return_type);

        Ok(Some(method))
    }

    /// Generate a comprehensive Resource trait with common FHIR resource methods
    pub fn generate_resource_trait(&mut self) -> CodegenResult<String> {
        let trait_content = r#"/// Trait for FHIR resources providing common functionality for all resource types.
///
/// This trait defines the core interface that all FHIR resources must implement,
/// providing access to common fields like id, meta, extensions, and basic resource operations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Kind: resource
/// - Type: Resource
pub trait Resource {
    /// Returns the resource type name
    fn resource_type(&self) -> &'static str;

    /// Gets the logical ID of this resource
    fn id(&self) -> Option<&str>;

    /// Checks if this resource has an ID
    fn has_id(&self) -> bool {
        self.id().is_some()
    }

    /// Gets the metadata about this resource
    fn meta(&self) -> Option<&crate::datatypes::Meta>;

    /// Checks if this resource has metadata
    fn has_meta(&self) -> bool {
        self.meta().is_some()
    }

    /// Gets the extensions for this resource
    fn extensions(&self) -> &[crate::datatypes::Extension];

    /// Gets the implicit rules for this resource
    fn implicit_rules(&self) -> Option<&str>;

    /// Gets the language of this resource
    fn language(&self) -> Option<&str>;
}"#;

        Ok(trait_content.to_string())
    }

    /// Generate a Resource trait implementation for the Resource struct
    pub fn generate_resource_impl(&self) -> String {
        r#"impl Resource for Resource {
    fn resource_type(&self) -> &'static str {
        "Resource"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn has_id(&self) -> bool {
        self.id.is_some()
    }

    fn meta(&self) -> Option<&crate::datatypes::Meta> {
        self.meta.as_ref()
    }

    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    fn extensions(&self) -> &[crate::datatypes::Extension] {
        &self.extension.as_deref().unwrap_or(&[])
    }

    fn implicit_rules(&self) -> Option<&str> {
        self.implicit_rules.as_deref()
    }

    fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}"#
        .to_string()
    }
}
