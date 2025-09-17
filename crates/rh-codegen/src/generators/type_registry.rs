//! Type registry for tracking FHIR type classifications
//!
//! This module maintains a registry of FHIR types and their classifications
//! to enable proper import path generation based on the original StructureDefinition.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::fhir_types::StructureDefinition;

/// Classification of FHIR types for import path generation
#[derive(Debug, Clone, PartialEq)]
pub enum TypeClassification {
    /// ValueSet-based enum type (goes to bindings)
    ValueSetEnum,
    /// FHIR resource type (goes to resources)
    Resource,
    /// FHIR complex datatype (goes to datatypes)
    ComplexType,
    /// Nested structure within a resource (goes to parent resource module)
    NestedStructure { parent_resource: String },
    /// Primitive type (goes to primitives)
    Primitive,
    /// Generated trait (goes to traits)
    Trait,
}

/// Global registry of type classifications
static TYPE_REGISTRY: Lazy<Mutex<HashMap<String, TypeClassification>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Type registry for managing FHIR type classifications
pub struct TypeRegistry;

impl TypeRegistry {
    /// Register a type based on its StructureDefinition
    pub fn register_from_structure_definition(structure_def: &StructureDefinition) {
        let classification = Self::classify_from_structure_definition(structure_def);
        Self::register_type(&structure_def.name, classification);
    }

    /// Register a type with a specific classification
    pub fn register_type(type_name: &str, classification: TypeClassification) {
        if let Ok(mut registry) = TYPE_REGISTRY.lock() {
            registry.insert(type_name.to_string(), classification);
        }
    }

    /// Get the classification for a type
    pub fn get_classification(type_name: &str) -> Option<TypeClassification> {
        if let Ok(registry) = TYPE_REGISTRY.lock() {
            registry.get(type_name).cloned()
        } else {
            None
        }
    }

    /// Clear the registry (useful for testing)
    pub fn clear() {
        if let Ok(mut registry) = TYPE_REGISTRY.lock() {
            registry.clear();
        }
    }

    /// Classify a type based on its StructureDefinition
    fn classify_from_structure_definition(
        structure_def: &StructureDefinition,
    ) -> TypeClassification {
        match structure_def.kind.as_str() {
            "primitive-type" => {
                // Check if this is a ValueSet-based enum
                if Self::is_value_set_enum(structure_def) {
                    TypeClassification::ValueSetEnum
                } else {
                    TypeClassification::Primitive
                }
            }
            "complex-type" => {
                // Check if this is a nested structure (BackboneElement)
                if let Some(parent) = Self::detect_parent_resource(structure_def) {
                    TypeClassification::NestedStructure {
                        parent_resource: parent,
                    }
                } else {
                    TypeClassification::ComplexType
                }
            }
            "resource" => TypeClassification::Resource,
            "logical" => TypeClassification::Resource, // Treat logical models as resources
            _ => {
                // Fallback - try to detect based on name patterns
                if structure_def.name.ends_with("Accessors")
                    || structure_def.name.ends_with("Mutators")
                    || structure_def.name.ends_with("Helpers")
                {
                    TypeClassification::Trait
                } else {
                    TypeClassification::ComplexType
                }
            }
        }
    }

    /// Check if a StructureDefinition represents a ValueSet-based enum
    fn is_value_set_enum(structure_def: &StructureDefinition) -> bool {
        // ValueSet-based enums are typically primitive-type with base_type "code"
        // and have binding constraints
        structure_def.kind == "primitive-type" && structure_def.base_type == "code"
    }

    /// Detect if this is a nested structure and return the parent resource name
    fn detect_parent_resource(structure_def: &StructureDefinition) -> Option<String> {
        // Check if the base_definition indicates this is a BackboneElement
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("BackboneElement") {
                // Extract parent resource from the URL or name
                // E.g., "http://hl7.org/fhir/StructureDefinition/Task#Task.restriction" -> "Task"
                if let Some(fragment) = base_def.split('#').nth(1) {
                    if let Some(parent) = fragment.split('.').next() {
                        return Some(parent.to_string());
                    }
                }

                // Fallback: try to extract from the name
                // E.g., "TaskRestriction" -> "Task"
                return Self::extract_parent_from_name(&structure_def.name);
            }
        }

        None
    }

    /// Extract parent resource name from a nested structure name
    pub fn extract_parent_from_name(type_name: &str) -> Option<String> {
        // This is a simplified heuristic - ideally we'd have more context
        // Look for known resource prefixes
        let mut known_resources = vec![
            "Account",
            "ActivityDefinition",
            "AdverseEvent",
            "AllergyIntolerance",
            "Appointment",
            "AppointmentResponse",
            "AuditEvent",
            "Basic",
            "Binary",
            "BiologicallyDerivedProduct",
            "BodyStructure",
            "Bundle",
            "CapabilityStatement",
            "CarePlan",
            "CareTeam",
            "CatalogEntry",
            "ChargeItem",
            "ChargeItemDefinition",
            "Claim",
            "ClaimResponse",
            "ClinicalImpression",
            "CodeSystem",
            "Communication",
            "CommunicationRequest",
            "CompartmentDefinition",
            "Composition",
            "ConceptMap",
            "Condition",
            "Consent",
            "Contract",
            "Coverage",
            "CoverageEligibilityRequest",
            "CoverageEligibilityResponse",
            "DetectedIssue",
            "Device",
            "DeviceDefinition",
            "DeviceMetric",
            "DeviceRequest",
            "DeviceUseStatement",
            "DiagnosticReport",
            "DocumentManifest",
            "DocumentReference",
            "EffectEvidenceSynthesis",
            "Encounter",
            "Endpoint",
            "EnrollmentRequest",
            "EnrollmentResponse",
            "EpisodeOfCare",
            "EventDefinition",
            "Evidence",
            "EvidenceVariable",
            "ExampleScenario",
            "ExplanationOfBenefit",
            "FamilyMemberHistory",
            "Flag",
            "Goal",
            "GraphDefinition",
            "Group",
            "GuidanceResponse",
            "HealthcareService",
            "ImagingStudy",
            "Immunization",
            "ImmunizationEvaluation",
            "ImmunizationRecommendation",
            "ImplementationGuide",
            "InsurancePlan",
            "Invoice",
            "Library",
            "Linkage",
            "List",
            "Location",
            "Measure",
            "MeasureReport",
            "Media",
            "Medication",
            "MedicationAdministration",
            "MedicationDispense",
            "MedicationKnowledge",
            "MedicationRequest",
            "MedicationStatement",
            "MedicinalProduct",
            "MedicinalProductAuthorization",
            "MedicinalProductContraindication",
            "MedicinalProductIndication",
            "MedicinalProductIngredient",
            "MedicinalProductInteraction",
            "MedicinalProductManufactured",
            "MedicinalProductPackaged",
            "MedicinalProductPharmaceutical",
            "MedicinalProductUndesirableEffect",
            "MessageDefinition",
            "MessageHeader",
            "MolecularSequence",
            "NamingSystem",
            "NutritionOrder",
            "Observation",
            "ObservationDefinition",
            "OperationDefinition",
            "OperationOutcome",
            "Organization",
            "OrganizationAffiliation",
            "Patient",
            "PaymentNotice",
            "PaymentReconciliation",
            "Person",
            "PlanDefinition",
            "Practitioner",
            "PractitionerRole",
            "Procedure",
            "Provenance",
            "Questionnaire",
            "QuestionnaireResponse",
            "RelatedPerson",
            "RequestGroup",
            "ResearchDefinition",
            "ResearchElementDefinition",
            "ResearchStudy",
            "ResearchSubject",
            "RiskAssessment",
            "RiskEvidenceSynthesis",
            "Schedule",
            "SearchParameter",
            "ServiceRequest",
            "Slot",
            "Specimen",
            "SpecimenDefinition",
            "StructureDefinition",
            "StructureMap",
            "Subscription",
            "Substance",
            "SubstanceNucleicAcid",
            "SubstancePolymer",
            "SubstanceProtein",
            "SubstanceReferenceInformation",
            "SubstanceSourceMaterial",
            "SubstanceSpecification",
            "SupplyDelivery",
            "SupplyRequest",
            "Task",
            "TerminologyCapabilities",
            "TestReport",
            "TestScript",
            "ValueSet",
            "VerificationResult",
            "VisionPrescription",
        ];

        // Sort by length in descending order to match longer resource names first
        // This ensures "ClaimResponse" is matched before "Claim" for types like "ClaimResponseAdditem"
        known_resources.sort_by(|a, b| b.len().cmp(&a.len()));

        for resource in &known_resources {
            if type_name.starts_with(resource) && type_name.len() > resource.len() {
                // Check that the next character is uppercase (PascalCase)
                if let Some(next_char) = type_name.chars().nth(resource.len()) {
                    if next_char.is_uppercase() {
                        return Some(resource.to_string());
                    }
                }
            }
        }

        None
    }

    /// Get the correct import path for a type using the registry
    pub fn get_import_path_for_type(type_name: &str) -> String {
        if let Some(classification) = Self::get_classification(type_name) {
            match classification {
                TypeClassification::ValueSetEnum => {
                    format!(
                        "crate::bindings::{}::{}",
                        crate::naming::Naming::to_snake_case(type_name),
                        type_name
                    )
                }
                TypeClassification::Resource => {
                    format!(
                        "crate::resources::{}::{}",
                        crate::naming::Naming::to_snake_case(type_name),
                        type_name
                    )
                }
                TypeClassification::ComplexType => {
                    format!(
                        "crate::datatypes::{}::{}",
                        crate::naming::Naming::to_snake_case(type_name),
                        type_name
                    )
                }
                TypeClassification::NestedStructure { parent_resource } => {
                    format!(
                        "crate::resources::{}::{}",
                        crate::naming::Naming::to_snake_case(&parent_resource),
                        type_name
                    )
                }
                TypeClassification::Primitive => {
                    // Handle primitive types specially
                    Self::get_primitive_import_path(type_name).unwrap_or_else(|| {
                        format!(
                            "crate::primitives::{}::{}",
                            crate::naming::Naming::to_snake_case(type_name),
                            type_name
                        )
                    })
                }
                TypeClassification::Trait => Self::get_trait_import_path(type_name),
            }
        } else {
            // Fallback to NamingManager for unregistered types
            // But for some well-known types, provide better defaults
            if crate::generators::naming_manager::NamingManager::is_fhir_resource(type_name) {
                format!(
                    "crate::resources::{}::{}",
                    crate::naming::Naming::to_snake_case(type_name),
                    type_name
                )
            } else if crate::generators::naming_manager::NamingManager::is_fhir_datatype(type_name)
            {
                format!(
                    "crate::datatypes::{}::{}",
                    crate::naming::Naming::to_snake_case(type_name),
                    type_name
                )
            } else if let Some(parent_resource) = Self::extract_parent_from_name(type_name) {
                // This looks like a nested structure - route it to the parent resource module
                format!(
                    "crate::resources::{}::{}",
                    crate::naming::Naming::to_snake_case(&parent_resource),
                    type_name
                )
            } else {
                // Use the original NamingManager logic as final fallback
                crate::generators::naming_manager::NamingManager::get_import_path_for_type(
                    type_name,
                )
            }
        }
    }

    /// Get import path for primitive types (copied from NamingManager)
    fn get_primitive_import_path(type_name: &str) -> Option<String> {
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
            _ => return None,
        };

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
            _ => type_name,
        };

        Some(format!("crate::primitives::{module_name}::{type_variant}"))
    }

    /// Get import path for generated traits (copied from NamingManager)
    fn get_trait_import_path(type_name: &str) -> String {
        let module_name = if type_name.ends_with("Accessors") {
            type_name.strip_suffix("Accessors").unwrap_or(type_name)
        } else if type_name.ends_with("Mutators") {
            type_name.strip_suffix("Mutators").unwrap_or(type_name)
        } else if type_name.ends_with("Helpers") {
            type_name.strip_suffix("Helpers").unwrap_or(type_name)
        } else {
            type_name
        };

        format!(
            "crate::traits::{}::{}",
            crate::naming::Naming::to_snake_case(module_name),
            type_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valueset_enum_registration() {
        TypeRegistry::clear();

        let task_intent = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "task-intent".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/task-intent".to_string(),
            version: None,
            name: "TaskIntent".to_string(),
            title: Some("Task Intent".to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "primitive-type".to_string(),
            is_abstract: false,
            base_type: "code".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/code".to_string()),
            differential: None,
            snapshot: None,
        };

        TypeRegistry::register_from_structure_definition(&task_intent);

        assert_eq!(
            TypeRegistry::get_classification("TaskIntent"),
            Some(TypeClassification::ValueSetEnum)
        );

        assert_eq!(
            TypeRegistry::get_import_path_for_type("TaskIntent"),
            "crate::bindings::task_intent::TaskIntent"
        );
    }

    #[test]
    fn test_nested_structure_registration() {
        TypeRegistry::clear();

        let account_coverage = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "account-coverage".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Account#Account.coverage".to_string(),
            version: None,
            name: "AccountCoverage".to_string(),
            title: Some("Account Coverage".to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "complex-type".to_string(),
            is_abstract: false,
            base_type: "BackboneElement".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/BackboneElement#Account.coverage"
                    .to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        TypeRegistry::register_from_structure_definition(&account_coverage);

        assert_eq!(
            TypeRegistry::get_classification("AccountCoverage"),
            Some(TypeClassification::NestedStructure {
                parent_resource: "Account".to_string()
            })
        );

        assert_eq!(
            TypeRegistry::get_import_path_for_type("AccountCoverage"),
            "crate::resources::account::AccountCoverage"
        );
    }

    #[test]
    fn test_resource_registration() {
        TypeRegistry::clear();

        let patient = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            version: None,
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            description: None,
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        TypeRegistry::register_from_structure_definition(&patient);

        assert_eq!(
            TypeRegistry::get_classification("Patient"),
            Some(TypeClassification::Resource)
        );

        assert_eq!(
            TypeRegistry::get_import_path_for_type("Patient"),
            "crate::resources::patient::Patient"
        );
    }

    #[test]
    fn test_parent_resource_extraction_with_composite_names() {
        // Test that "ClaimResponse" is extracted correctly, not "Claim"
        assert_eq!(
            TypeRegistry::extract_parent_from_name("ClaimResponseAdditem"),
            Some("ClaimResponse".to_string())
        );

        // Test that shorter matches don't interfere with longer ones
        assert_eq!(
            TypeRegistry::extract_parent_from_name("MedicinalProductName"),
            Some("MedicinalProduct".to_string())
        );

        // Test ActivityDefinition still works
        assert_eq!(
            TypeRegistry::extract_parent_from_name("ActivityDefinitionParticipant"),
            Some("ActivityDefinition".to_string())
        );
    }
}
