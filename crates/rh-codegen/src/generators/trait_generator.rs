//! Trait generation functionality
//!
//! This module handles the generation of Rust traits from FHIR StructureDefinitions.

use crate::fhir_types::StructureDefinition;
use crate::generators::{utils::GeneratorUtils, DocumentationGenerator, TypeUtilities};
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

        // Skip files that begin with underscore (auto-generated/temporary files)
        if TypeUtilities::should_skip_underscore_prefixed(structure_def) {
            return Err(CodegenError::Generation {
                message: format!(
                    "Skipping underscore-prefixed StructureDefinition '{}' - underscore prefixed files are ignored",
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

        // Add inheritance relationship if present
        self.add_inheritance_relationship(&mut rust_trait, structure_def)?;

        // Add common FHIR methods based on the type
        self.add_common_fhir_trait_methods(&mut rust_trait, structure_def)?;

        // Add specific methods based on the structure definition elements
        self.add_element_based_trait_methods(&mut rust_trait, structure_def)?;

        Ok(rust_trait)
    }

    /// Add inheritance relationship if the StructureDefinition has a base definition
    fn add_inheritance_relationship(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        if let Some(base_def) = &structure_def.base_definition {
            // Extract trait name from base definition URL
            // e.g., "http://hl7.org/fhir/StructureDefinition/Resource" -> "Resource"
            if let Some(parent_trait_name) = self.extract_trait_name_from_url(base_def) {
                // Only add inheritance for known FHIR base types
                if self.is_valid_fhir_base_type(&parent_trait_name) {
                    rust_trait.super_traits.push(parent_trait_name);
                }
            }
        }
        Ok(())
    }

    /// Extract trait name from FHIR StructureDefinition URL
    fn extract_trait_name_from_url(&self, url: &str) -> Option<String> {
        // FHIR URLs typically look like: "http://hl7.org/fhir/StructureDefinition/Resource"
        url.split('/').next_back().map(|s| s.to_string())
    }

    /// Check if the given type name is a valid FHIR base type for inheritance
    fn is_valid_fhir_base_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "Resource" | "DomainResource" | "Element" | "BackboneElement" | "Extension"
        )
    }

    /// Add common FHIR trait methods based on the structure type
    pub fn add_common_fhir_trait_methods(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Check if this trait inherits from Resource (directly or indirectly)
        let inherits_from_resource = self.trait_inherits_from_resource(rust_trait);

        // Only add basic resource methods if this IS the Resource trait itself
        if structure_def.name == "Resource" {
            // 1. resource_type method - returns the resource type name
            let resource_type_method = RustTraitMethod::new("resource_type".to_string())
                .with_doc("Returns the resource type name".to_string())
                .with_return_type(RustType::Custom("&'static str".to_string()));

            rust_trait.add_method(resource_type_method);

            // 2. id method - gets the logical ID
            let id_method = RustTraitMethod::new("id".to_string())
                .with_doc("Gets the logical ID of this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "&str".to_string(),
                ))));

            rust_trait.add_method(id_method);

            // 3. has_id convenience method with default implementation
            let has_id_method = RustTraitMethod::new("has_id".to_string())
                .with_doc("Checks if this resource has an ID".to_string())
                .with_return_type(RustType::Boolean)
                .with_default_implementation("self.id().is_some()".to_string());

            rust_trait.add_method(has_id_method);

            // 4. meta method - gets the metadata
            let meta_method = RustTraitMethod::new("meta".to_string())
                .with_doc("Gets the metadata about this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "&crate::datatypes::Meta".to_string(),
                ))));

            rust_trait.add_method(meta_method);

            // 5. has_meta convenience method with default implementation
            let has_meta_method = RustTraitMethod::new("has_meta".to_string())
                .with_doc("Checks if this resource has metadata".to_string())
                .with_return_type(RustType::Boolean)
                .with_default_implementation("self.meta().is_some()".to_string());

            rust_trait.add_method(has_meta_method);

            // 6. extensions method - gets extensions
            let extensions_method = RustTraitMethod::new("extensions".to_string())
                .with_doc("Gets the extensions for this resource".to_string())
                .with_return_type(RustType::Custom(
                    "&[crate::datatypes::Extension]".to_string(),
                ));

            rust_trait.add_method(extensions_method);

            // 7. implicit_rules method - gets implicit rules
            let implicit_rules_method = RustTraitMethod::new("implicit_rules".to_string())
                .with_doc("Gets the implicit rules for this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "&str".to_string(),
                ))));

            rust_trait.add_method(implicit_rules_method);

            // 8. language method - gets the language
            let language_method = RustTraitMethod::new("language".to_string())
                .with_doc("Gets the language of this resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "&str".to_string(),
                ))));

            rust_trait.add_method(language_method);
        } else if !inherits_from_resource {
            // If this is not Resource and doesn't inherit from Resource, add basic methods
            // This handles non-resource types like Element, Extension, etc.
            let extensions_method = RustTraitMethod::new("extensions".to_string())
                .with_doc("Gets the extensions for this resource".to_string())
                .with_return_type(RustType::Vec(Box::new(RustType::Custom(
                    "Extension".to_string(),
                ))));

            rust_trait.add_method(extensions_method);
        }

        // DomainResource should add narrative (this is specific to DomainResource)
        if structure_def.name == "DomainResource" {
            let narrative_method = RustTraitMethod::new("narrative".to_string())
                .with_doc("Gets the narrative for this domain resource".to_string())
                .with_return_type(RustType::Option(Box::new(RustType::Custom(
                    "Narrative".to_string(),
                ))));

            rust_trait.add_method(narrative_method);
        }

        Ok(())
    }

    /// Check if a trait inherits from Resource either directly or indirectly
    fn trait_inherits_from_resource(&self, rust_trait: &RustTrait) -> bool {
        // Direct inheritance
        if rust_trait.super_traits.contains(&"Resource".to_string()) {
            return true;
        }

        // Indirect inheritance through DomainResource
        if rust_trait
            .super_traits
            .contains(&"DomainResource".to_string())
        {
            return true;
        }

        // Could extend this for other inheritance chains if needed
        false
    }

    /// Add trait methods based on the specific elements in the structure definition
    pub fn add_element_based_trait_methods(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        // Add choice type methods first
        self.add_choice_type_trait_methods(rust_trait, structure_def)?;

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
                // Skip choice type fields since we handle them separately
                let field_name = element.path.split('.').next_back().unwrap_or("");
                if field_name.ends_with("[x]") {
                    continue;
                }

                // Check if this element should have a trait method
                if self.should_create_trait_method_for_element(element) {
                    if let Some(method) = self.create_trait_method_from_element(element)? {
                        // Avoid adding methods that are already defined in parent traits
                        if !self.method_conflicts_with_inheritance(&method, rust_trait) {
                            rust_trait.add_method(method);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if a method conflicts with methods defined in parent traits
    fn method_conflicts_with_inheritance(
        &self,
        method: &RustTraitMethod,
        rust_trait: &RustTrait,
    ) -> bool {
        // Check all methods that would be inherited through the trait hierarchy
        let inherited_methods = self.get_all_inherited_methods(rust_trait);

        inherited_methods.contains(&method.name)
    }

    /// Get all method names that are inherited through the trait hierarchy
    fn get_all_inherited_methods(
        &self,
        rust_trait: &RustTrait,
    ) -> std::collections::HashSet<String> {
        let mut inherited_methods = std::collections::HashSet::new();

        // Define methods for each trait type
        let base_resource_methods = [
            "id",
            "meta",
            "extensions",
            "implicit_rules",
            "language",
            "extension",
            "resource_type",
            "has_id",
            "has_meta",
        ];
        let domain_resource_methods = ["narrative"];

        // Recursively collect inherited methods
        self.collect_inherited_methods_recursive(
            rust_trait,
            &mut inherited_methods,
            &base_resource_methods,
            &domain_resource_methods,
        );

        inherited_methods
    }

    /// Recursively collect all inherited method names
    fn collect_inherited_methods_recursive(
        &self,
        rust_trait: &RustTrait,
        inherited_methods: &mut std::collections::HashSet<String>,
        base_resource_methods: &[&str],
        domain_resource_methods: &[&str],
    ) {
        for super_trait in &rust_trait.super_traits {
            match super_trait.as_str() {
                "Resource" => {
                    // Add all Resource methods
                    for method in base_resource_methods {
                        inherited_methods.insert(method.to_string());
                    }
                }
                "DomainResource" => {
                    // DomainResource inherits from Resource, so add both
                    for method in base_resource_methods {
                        inherited_methods.insert(method.to_string());
                    }
                    for method in domain_resource_methods {
                        inherited_methods.insert(method.to_string());
                    }
                }
                _ => {
                    // For other custom traits, we could extend this logic
                    // For now, we handle the main FHIR hierarchy
                }
            }
        }
    }

    /// Add choice type trait methods that provide convenient access to choice type values
    pub fn add_choice_type_trait_methods(
        &mut self,
        rust_trait: &mut RustTrait,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<()> {
        use crate::generators::FieldGenerator;

        let choice_types = FieldGenerator::extract_choice_types_from_structure(structure_def);

        for (base_name, type_codes) in choice_types {
            // Create method that returns the actual value from whichever variant is set
            let method_name = base_name.clone();
            let snake_case_method = GeneratorUtils::to_rust_field_name(&method_name);

            // Create a simpler approach - just return the formatted value as a string
            let mut method_body_lines = Vec::new();
            for (index, type_code) in type_codes.iter().enumerate() {
                let field_suffix = FieldGenerator::type_code_to_snake_case(type_code);
                let field_name = format!("{base_name}_{field_suffix}");
                let rust_field_name = GeneratorUtils::to_rust_field_name(&field_name);

                if index == 0 {
                    method_body_lines
                        .push(format!("if let Some(val) = &self.{rust_field_name} {{"));
                    method_body_lines.push("    Some(format!(\"{:?}\", val))".to_string());
                } else {
                    method_body_lines.push(format!(
                        "}} else if let Some(val) = &self.{rust_field_name} {{"
                    ));
                    method_body_lines.push("    Some(format!(\"{:?}\", val))".to_string());
                }
            }
            method_body_lines.push("} else {".to_string());
            method_body_lines.push("    None".to_string());
            method_body_lines.push("}".to_string());

            let method_body = method_body_lines.join("\n        ");

            // Return type: Option<String> - simplified to just the formatted value
            let return_type = RustType::Option(Box::new(RustType::String));

            let method = RustTraitMethod::new(snake_case_method.clone())
                .with_doc(format!(
                    "Gets the {base_name} value from whichever variant is set"
                ))
                .with_return_type(return_type)
                .with_default_implementation(format!("        {method_body}"));

            rust_trait.add_method(method);

            // Add a helper method that just checks if any variant is set
            let has_method_name = format!("has_{snake_case_method}");
            let mut has_method_body_lines = Vec::new();
            for (index, type_code) in type_codes.iter().enumerate() {
                let field_suffix = FieldGenerator::type_code_to_snake_case(type_code);
                let field_name = format!("{base_name}_{field_suffix}");
                let rust_field_name = GeneratorUtils::to_rust_field_name(&field_name);

                if index == 0 {
                    has_method_body_lines.push(format!("self.{rust_field_name}.is_some()"));
                } else {
                    has_method_body_lines.push(format!(" || self.{rust_field_name}.is_some()"));
                }
            }

            let has_method_body = has_method_body_lines.join("");

            let has_method = RustTraitMethod::new(has_method_name)
                .with_doc(format!("Returns true if any {base_name} variant is set"))
                .with_return_type(RustType::Boolean)
                .with_default_implementation(format!("        {has_method_body}"));

            rust_trait.add_method(has_method);

            // Also add individual getter methods for each specific type for type safety
            for type_code in &type_codes {
                let field_suffix = FieldGenerator::type_code_to_snake_case(type_code);
                let typed_method_name = format!("{base_name}_{field_suffix}");
                let typed_rust_method_name = GeneratorUtils::to_rust_field_name(&typed_method_name);

                // Map FHIR type to Rust type
                let rust_type = match type_code.as_str() {
                    "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical"
                    | "dateTime" | "date" | "time" | "instant" => RustType::String,
                    "boolean" => RustType::Boolean,
                    "integer" | "positiveInt" | "unsignedInt" => RustType::Integer,
                    "decimal" => RustType::Float,
                    _ => RustType::Custom(GeneratorUtils::capitalize_first_letter(type_code)),
                };

                let typed_return_type = RustType::Option(Box::new(rust_type));

                let typed_method = RustTraitMethod::new(typed_rust_method_name.clone())
                    .with_doc(format!("Gets the {base_name} value as {type_code}"))
                    .with_return_type(typed_return_type)
                    .with_default_implementation(format!(
                        "        self.{typed_rust_method_name}.clone()"
                    ));

                rust_trait.add_method(typed_method);
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
        // Create a synthetic Resource StructureDefinition using the same pattern as others
        let resource_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Resource".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Resource".to_string(),
            name: "Resource".to_string(),
            title: Some("Resource".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: true,
            description: Some("Base resource definition".to_string()),
            purpose: Some("This is the base resource type for everything".to_string()),
            base_type: "Resource".to_string(),
            base_definition: None, // Resource is the root, no parent
            version: None,
            differential: None,
            snapshot: None,
        };

        // Generate the trait using the standard pattern
        let rust_trait = self.generate_trait(&resource_structure)?;

        // Use TokenGenerator to generate the trait code with proper formatting
        let token_generator = crate::generators::TokenGenerator::new();
        let mut all_tokens = proc_macro2::TokenStream::new();

        // Add common imports for traits
        let import_tokens: proc_macro2::TokenStream = "use std::collections::HashMap;"
            .parse()
            .expect("Invalid import statement");
        all_tokens.extend(import_tokens);

        // Generate the trait
        let trait_tokens = token_generator.generate_trait(&rust_trait);
        all_tokens.extend(trait_tokens);

        // Parse the tokens into a syntax tree and format it properly
        let syntax_tree = syn::parse2(all_tokens).map_err(|e| crate::CodegenError::Generation {
            message: format!("Failed to parse generated trait tokens: {e}"),
        })?;

        let formatted_code = prettyplease::unparse(&syntax_tree);

        Ok(formatted_code)
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
