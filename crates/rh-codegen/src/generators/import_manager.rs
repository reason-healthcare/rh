//! Import management utilities
//!
//! This module handles the determination of import paths for FHIR types
//! and collection of custom types for import generation.

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
                // Determine the correct module path for the type
                let import_path = Self::get_import_path_for_type(base_type);
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
                // For super traits that are FHIR resources, import the trait version not the struct
                let import_path = if Self::is_fhir_resource_type(super_trait) {
                    format!(
                        "crate::traits::{}::{}",
                        crate::generators::utils::GeneratorUtils::to_snake_case(super_trait),
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
            RustType::Reference(type_name) => {
                if !Self::is_primitive_type(type_name)
                    && type_name != current_struct_name
                    && !structs_in_file.contains(type_name)
                {
                    // Get the correct import path for this type
                    let import_path = Self::get_import_path_for_type(type_name);
                    imports.insert(import_path);
                }
            }
            // Primitive types don't need imports
            RustType::String | RustType::Integer | RustType::Boolean | RustType::Float => {}
        }
    }

    /// Determine the correct import path for a given type name
    pub fn get_import_path_for_type(type_name: &str) -> String {
        // Check if it's a known FHIR resource type
        if Self::is_fhir_resource_type(type_name) {
            return format!(
                "crate::resource::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Check if it's a known FHIR datatype (use existing method for consistency)
        if Self::is_fhir_datatype(type_name) {
            return format!(
                "crate::datatypes::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Check if it's a known primitive type extension
        if Self::is_fhir_primitive_type(type_name) {
            // Map type names to correct module names - handle both Type and non-Type variants
            let module_name = match type_name {
                "StringType" | "String" => "string",
                "BooleanType" | "Boolean" => "boolean",
                "IntegerType" | "Integer" => "integer",
                "DecimalType" | "Decimal" => "decimal",
                "UriType" | "Uri" => "uri",
                "UrlType" | "Url" => "url",
                "CanonicalType" | "Canonical" => "canonical",
                "OidType" | "Oid" => "oid",
                "UuidType" | "Uuid" => "uuid",
                "InstantType" | "Instant" => "instant",
                "DateType" | "Date" => "date",
                "DateTimeType" | "DateTime" => "date_time",
                "TimeType" | "Time" => "time",
                "CodeType" | "Code" => "code",
                "IdType" | "Id" => "id",
                "MarkdownType" | "Markdown" => "markdown",
                "Base64BinaryType" | "Base64Binary" => "base64binary",
                "UnsignedIntType" | "UnsignedInt" => "unsigned_int",
                "PositiveIntType" | "PositiveInt" => "positive_int",
                "XhtmlType" | "Xhtml" => "xhtml",
                _ => "unknown_primitive",
            };
            // Map the type name to the correct Type variant for import
            let type_variant = match type_name {
                "String" => "StringType",
                "Boolean" => "BooleanType",
                "Integer" => "IntegerType",
                "Decimal" => "DecimalType",
                "Uri" => "UriType",
                "Url" => "UrlType",
                "Canonical" => "CanonicalType",
                "Oid" => "OidType",
                "Uuid" => "UuidType",
                "Instant" => "InstantType",
                "Date" => "DateType",
                "DateTime" => "DateTimeType",
                "Time" => "TimeType",
                "Code" => "CodeType",
                "Id" => "IdType",
                "Markdown" => "MarkdownType",
                "Base64Binary" => "Base64BinaryType",
                "UnsignedInt" => "UnsignedIntType",
                "PositiveInt" => "PositiveIntType",
                "Xhtml" => "XhtmlType",
                _ => type_name, // Already has Type suffix
            };
            return format!("crate::primitives::{module_name}::{type_variant}");
        }

        // Check if it's a generated trait
        if Self::is_generated_trait(type_name) {
            return format!(
                "crate::traits::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Default to bindings for unknown types (likely enums)
        format!(
            "crate::bindings::{}::{}",
            Self::to_snake_case(type_name),
            type_name
        )
    }

    /// Check if a type is a FHIR resource type
    pub fn is_fhir_resource_type(type_name: &str) -> bool {
        // Common FHIR resource types - using case-insensitive comparison
        matches!(
            type_name.to_lowercase().as_str(),
            "patient"
                | "observation"
                | "practitioner"
                | "organization"
                | "location"
                | "encounter"
                | "diagnosticreport"
                | "medication"
                | "medicationrequest"
                | "activitydefinition"
                | "plandefinition"
                | "measure"
                | "library"
                | "valueset"
                | "codesystem"
                | "conceptmap"
                | "structuredefinition"
                | "implementationguide"
                | "capabilitystatement"
                | "operationdefinition"
                | "searchparameter"
                | "compartmentdefinition"
                | "examplescenario"
                | "graphdefinition"
                | "messagedefinition"
                | "namingsystem"
                | "terminologycapabilities"
                | "testscript"
                | "testreport"
                | "domainresource"
                | "resource"
                | "metadataresource"
                // Additional resource types that were missing
                | "auditevent"
                | "composition"
                | "questionnaire"
                | "requestgroup"
                | "servicerequest"
                | "evidencevariable"
                | "guidanceresponse"
                | "familymemberhistory"
                | "evidence"
                | "provenance"
                | "group"
                | "vitalsigns"
                | "substance"
                // More resource types from the generated traits
                | "account"
                | "adverseevent"
                | "allergyintolerance"
                | "appointment"
                | "appointmentresponse"
                | "basic"
                | "binary"
                | "bodystructure"
                | "bundle"
                | "careplan"
                | "careteam"
                | "chargeitem"
                | "chargeitemdefinition"
                | "claim"
                | "claimresponse"
                | "clinicalimpression"
                | "communication"
                | "communicationrequest"
                | "condition"
                | "consent"
                | "contract"
                | "coverage"
                | "coverageeligibilityrequest"
                | "coverageeligibilityresponse"
                | "detectedissue"
                | "device"
                | "devicedefinition"
                | "devicemetric"
                | "devicerequest"
                | "deviceusestatement"
                | "documentmanifest"
                | "documentreference"
                | "endpoint"
                | "enrollmentrequest"
                | "enrollmentresponse"
                | "episodeofcare"
                | "eventdefinition"
                | "explanationofbenefit"
                | "flag"
                | "goal"
                | "healthcareservice"
                | "imagingstudy"
                | "immunization"
                | "immunizationevaluation"
                | "immunizationrecommendation"
                | "insuranceplan"
                | "invoice"
                | "linkage"
                | "list"
                | "measurereport"
                | "media"
                | "medicationadministration"
                | "medicationdispense"
                | "medicationknowledge"
                | "medicationstatement"
                | "medicinalproduct"
                | "medicinalproductauthorization"
                | "medicinalproductcontraindication"
                | "medicinalproductindication"
                | "medicinalproductingredient"
                | "medicinalproductinteraction"
                | "medicinalproductmanufactured"
                | "medicinalproductpackaged"
                | "medicinalproductpharmaceutical"
                | "medicinalproductundesirableeffect"
                | "messageheader"
                | "molecularsequence"
                | "nutritionorder"
                | "observationdefinition"
                | "operationoutcome"
                | "organizationaffiliation"
                | "parameters"
                | "paymentnotice"
                | "paymentreconciliation"
                | "person"
                | "practitionerrole"
                | "procedure"
                | "questionnaireresponse"
                | "relatedperson"
                | "researchdefinition"
                | "researchelementdefinition"
                | "researchstudy"
                | "researchsubject"
                | "riskassessment"
                | "schedule"
                | "slot"
                | "specimen"
                | "specimendefinition"
                | "structuremap"
                | "subscription"
                | "substancenucleicacid"
                | "substancepolymer"
                | "substanceprotein"
                | "substancereferenceinformation"
                | "substancesourcematerial"
                | "substancespecification"
                | "supplydelivery"
                | "supplyrequest"
                | "task"
                | "verificationresult"
                | "visionprescription"
        )
    }

    /// Check if a type name represents a known FHIR data type
    pub fn is_fhir_datatype(type_name: &str) -> bool {
        matches!(
            type_name.to_lowercase().as_str(),
            "identifier"
                | "humanname"
                | "address"
                | "contactpoint"
                | "attachment"
                | "codeableconcept"
                | "coding"
                | "quantity"
                | "range"
                | "period"
                | "ratio"
                | "sampleddata"
                | "signature"
                | "age"
                | "count"
                | "distance"
                | "duration"
                | "money"
                | "simplequantity"
                | "extension"
                | "narrative"
                | "annotation"
                | "reference"
                | "meta"
                | "timing"
                | "dosage"
                | "relatedartifact"
                | "contactdetail"
                | "contributor"
                | "datarequirement"
                | "parameterdef"
                | "triggerdefinition"
                | "usagecontext"
                | "expression"
                | "backboneelement"
                | "element"
                // Additional complex datatypes
                | "parameterdefinition"
                | "prodcharacteristic"
                | "productshelflife"
                | "marketingstatus"
                | "population"
                | "substanceamount"
                | "elementdefinition"
                // Complex nested types that may be generated
                | "implementationguidedefinition"
        )
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
            || matches!(
                lower_name.as_str(),
                "resourcetrait"
                    | "domainresourcetrait"
                    | "backboneelementtrait"
                    | "elementtrait"
                    | "metadataresourcetrait"
            )
    }

    /// Check if a type name represents a primitive Rust type
    pub fn is_primitive_type(type_name: &str) -> bool {
        matches!(
            type_name,
            "String" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "bool" | "usize" | "isize"
        )
    }

    /// Convert a PascalCase type name to snake_case for module imports
    /// This is a simplified version - for full functionality use NameGenerator
    fn to_snake_case(name: &str) -> String {
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
            "crate::resource::domain_resource::DomainResource"
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
}
