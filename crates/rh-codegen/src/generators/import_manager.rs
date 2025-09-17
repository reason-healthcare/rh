//! Import management utilities
//!
//! This module handles the determination of import paths for FHIR types
//! and collection of custom types for import generation.

use crate::generators::naming_manager::NamingManager;
use crate::generators::type_registry::TypeRegistry;
use crate::rust_types::{RustStruct, RustType};
use std::collections::HashSet;

/// Utility struct for managing imports and type classifications
pub struct ImportManager;

impl ImportManager {
    /// Collect all custom types referenced by a struct and add them to the imports set
    pub fn collect_custom_types_from_struct(
        rust_struct: &RustStruct,
        imports: &mut HashSet<String>,
        structs_in_file: &HashSet<String>,
    ) {
        // Add import for base type if present
        if let Some(base_def) = &rust_struct.base_definition {
            let base_type = base_def.split('/').next_back().unwrap_or(base_def);
            // Only add import if it's not a primitive type, not the current struct, and not in the same file
            if !Self::is_primitive_type(base_type)
                && base_type != rust_struct.name
                && !structs_in_file.contains(base_type)
            {
                // For base definitions, ensure proper struct name casing but only for
                // names that are clearly in all lowercase (like "vitalsigns")
                let proper_base_type = if base_type
                    .chars()
                    .all(|c| c.is_lowercase() || c.is_numeric())
                {
                    // Convert all-lowercase names to PascalCase (e.g., "vitalsigns" -> "Vitalsigns")
                    crate::naming::Naming::capitalize_first(base_type)
                } else {
                    // Keep names that already have proper casing (e.g., "BackboneElement")
                    base_type.to_string()
                };
                let import_path = Self::get_import_path_for_type(&proper_base_type);
                imports.insert(import_path);
            }
        }

        // Collect custom types from all fields
        for field in &rust_struct.fields {
            Self::collect_custom_types_from_type(
                &field.field_type,
                imports,
                &rust_struct.name,
                structs_in_file,
            );
        }
    }

    /// Collect all custom types referenced by a trait and add them to the imports set
    pub fn collect_custom_types_from_trait(
        rust_trait: &crate::rust_types::RustTrait,
        imports: &mut HashSet<String>,
    ) {
        // Add imports for super traits
        for super_trait in &rust_trait.super_traits {
            if !Self::is_primitive_type(super_trait) {
                // For super traits that are FHIR trait types (Accessors, Mutators, Existence)
                // or FHIR resources, import from the traits module
                let import_path = if Self::is_fhir_trait_type(super_trait) {
                    // For trait types like "DomainResourceAccessors", extract the base name
                    // and use it for the module path: crate::traits::domain_resource::DomainResourceAccessors
                    let base_name = if super_trait.ends_with("Accessors") {
                        super_trait.strip_suffix("Accessors").unwrap()
                    } else if super_trait.ends_with("Mutators") {
                        super_trait.strip_suffix("Mutators").unwrap()
                    } else if super_trait.ends_with("Existence") {
                        super_trait.strip_suffix("Existence").unwrap()
                    } else {
                        super_trait
                    };

                    format!(
                        "crate::traits::{}::{}",
                        crate::naming::Naming::to_snake_case(base_name),
                        super_trait
                    )
                } else if Self::is_fhir_resource_type(super_trait) {
                    format!(
                        "crate::traits::{}::{}",
                        crate::naming::Naming::to_snake_case(super_trait),
                        super_trait
                    )
                } else {
                    Self::get_import_path_for_type(super_trait)
                };
                imports.insert(import_path);
            }
        }

        // Collect custom types from all trait methods
        for method in &rust_trait.methods {
            // Collect types from return type
            if let Some(return_type) = &method.return_type {
                Self::collect_custom_types_from_type(
                    return_type,
                    imports,
                    &rust_trait.name,
                    &HashSet::new(), // Traits don't have "structs in file"
                );
            }

            // Collect types from parameters (if any)
            for param in &method.params {
                Self::collect_custom_types_from_type(
                    &param.param_type,
                    imports,
                    &rust_trait.name,
                    &HashSet::new(),
                );
            }
        }
    }

    /// Recursively collect custom types from a RustType
    pub fn collect_custom_types_from_type(
        rust_type: &RustType,
        imports: &mut HashSet<String>,
        current_struct_name: &str,
        structs_in_file: &HashSet<String>,
    ) {
        match rust_type {
            RustType::Custom(type_name) => {
                // Only add import if it's not a primitive type, not the current struct, and not in the same file
                if !Self::is_primitive_type(type_name)
                    && type_name != current_struct_name
                    && !structs_in_file.contains(type_name)
                {
                    // Get the correct import path for this type
                    let import_path = Self::get_import_path_for_type(type_name);
                    imports.insert(import_path);
                }
            }
            RustType::Option(inner) => {
                Self::collect_custom_types_from_type(
                    inner,
                    imports,
                    current_struct_name,
                    structs_in_file,
                );
            }
            RustType::Vec(inner) => {
                Self::collect_custom_types_from_type(
                    inner,
                    imports,
                    current_struct_name,
                    structs_in_file,
                );
            }
            RustType::Box(inner) => {
                Self::collect_custom_types_from_type(
                    inner,
                    imports,
                    current_struct_name,
                    structs_in_file,
                );
            }
            RustType::Slice(inner) => {
                Self::collect_custom_types_from_type(
                    inner,
                    imports,
                    current_struct_name,
                    structs_in_file,
                );
            }
            RustType::Reference(name) => {
                if !Self::is_primitive_type(name)
                    && name != current_struct_name
                    && !structs_in_file.contains(name)
                {
                    let import_path = Self::get_import_path_for_type(name);
                    imports.insert(import_path);
                }
            }
            // Primitive types don't need imports
            RustType::String | RustType::Integer | RustType::Boolean | RustType::Float => {}
        }
    }

    /// Determine the correct import path for a given type name
    pub fn get_import_path_for_type(type_name: &str) -> String {
        // First try the TypeRegistry for accurate classification based on StructureDefinition
        TypeRegistry::get_import_path_for_type(type_name)
    }

    /// Check if a type is a FHIR resource type
    pub fn is_fhir_resource_type(type_name: &str) -> bool {
        NamingManager::is_fhir_resource(type_name)
    }

    /// Check if a type is a FHIR trait type (ends with Accessors, Mutators, or Existence)
    pub fn is_fhir_trait_type(type_name: &str) -> bool {
        type_name.ends_with("Accessors")
            || type_name.ends_with("Mutators")
            || type_name.ends_with("Existence")
    }

    /// Check if a type name represents a known FHIR data type
    pub fn is_fhir_datatype(type_name: &str) -> bool {
        NamingManager::is_fhir_datatype(type_name)
    }

    /// Check if a type is a FHIR primitive type
    pub fn is_fhir_primitive_type(type_name: &str) -> bool {
        // FHIR primitive types that have extensions - matching actual generated type names
        matches!(
            type_name,
            "StringType"
                | "BooleanType"
                | "IntegerType"
                | "DecimalType"
                | "UriType"
                | "UrlType"
                | "CanonicalType"
                | "OidType"
                | "UuidType"
                | "InstantType"
                | "DateType"
                | "DateTimeType"
                | "TimeType"
                | "CodeType"
                | "IdType"
                | "MarkdownType"
                | "Base64BinaryType"
                | "UnsignedIntType"
                | "PositiveIntType"
                | "XhtmlType"
                // Also handle the non-Type variants that appear in trait methods
                | "String"
                | "Boolean"
                | "Integer"
                | "Decimal"
                | "Uri"
                | "Url"
                | "Canonical"
                | "Oid"
                | "Uuid"
                | "Instant"
                | "Date"
                | "DateTime"
                | "Time"
                | "Code"
                | "Id"
                | "Markdown"
                | "Base64Binary"
                | "UnsignedInt"
                | "PositiveInt"
                | "Xhtml"
        )
    }

    /// Check if a type is a generated trait
    pub fn is_generated_trait(type_name: &str) -> bool {
        // Traits are typically generated for base types or common interfaces
        let lower_name = type_name.to_lowercase();
        lower_name.ends_with("trait")
            || lower_name.ends_with("accessors")
            || lower_name.ends_with("mutators")
            || lower_name.ends_with("helpers")
            || matches!(
                lower_name.as_str(),
                "resourcetrait"
                    | "domainresourcetrait"
                    | "backboneelementtrait"
                    | "elementtrait"
                    | "metadataresourcetrait"
                    | "resourceaccessors"
                    | "domainresourceaccessors"
                    | "backboneelementaccessors"
                    | "elementaccessors"
                    | "metadataresourceaccessors"
            )
    }

    /// Check if a type name represents a primitive Rust type
    pub fn is_primitive_type(type_name: &str) -> bool {
        matches!(
            type_name,
            "String"
                | "&str"
                | "str"
                | "i32"
                | "u32"
                | "i64"
                | "u64"
                | "f32"
                | "f64"
                | "bool"
                | "usize"
                | "isize"
                | "Self" // Built-in Rust keyword, should not be imported
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_classification() {
        // Test resource classification
        assert!(ImportManager::is_fhir_resource_type("DomainResource"));
        assert!(ImportManager::is_fhir_resource_type("Patient"));
        assert!(ImportManager::is_fhir_resource_type("ActivityDefinition"));
        assert!(!ImportManager::is_fhir_resource_type("Identifier"));

        // Test datatype classification
        assert!(ImportManager::is_fhir_datatype("Identifier"));
        assert!(ImportManager::is_fhir_datatype("CodeableConcept"));
        assert!(ImportManager::is_fhir_datatype("Reference"));
        assert!(!ImportManager::is_fhir_datatype("DomainResource"));

        // Test import path generation
        assert_eq!(
            ImportManager::get_import_path_for_type("DomainResource"),
            "crate::resources::domain_resource::DomainResource"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("Identifier"),
            "crate::datatypes::identifier::Identifier"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("PublicationStatus"),
            "crate::bindings::publication_status::PublicationStatus"
        );
    }

    #[test]
    fn test_primitive_type_detection() {
        assert!(ImportManager::is_primitive_type("String"));
        assert!(ImportManager::is_primitive_type("i32"));
        assert!(ImportManager::is_primitive_type("bool"));
        assert!(!ImportManager::is_primitive_type("Patient"));
        assert!(!ImportManager::is_primitive_type("Identifier"));
    }

    #[test]
    fn test_nested_structure_detection() {
        // Test nested structure import path detection with longest prefix matching
        assert_eq!(
            ImportManager::get_import_path_for_type("EvidenceVariableCharacteristic"),
            "crate::resources::evidence_variable::EvidenceVariableCharacteristic"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("MeasureReportGroup"),
            "crate::resources::measure_report::MeasureReportGroup"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("AccountCoverage"),
            "crate::resources::account::AccountCoverage"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("AccountGuarantor"),
            "crate::resources::account::AccountGuarantor"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("BundleEntry"),
            "crate::resources::bundle::BundleEntry"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("ImplementationGuideGlobal"),
            "crate::resources::implementation_guide::ImplementationGuideGlobal"
        );

        // Test ConditionStage specifically - this is the failing case
        assert_eq!(
            ImportManager::get_import_path_for_type("ConditionStage"),
            "crate::resources::condition::ConditionStage"
        );

        // Test that non-nested structures still work correctly
        assert_eq!(
            ImportManager::get_import_path_for_type("Patient"),
            "crate::resources::patient::Patient"
        );
        assert_eq!(
            ImportManager::get_import_path_for_type("Identifier"),
            "crate::datatypes::identifier::Identifier"
        );
    }
}
