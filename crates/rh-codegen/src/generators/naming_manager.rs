//! Centralized naming management for FHIR types and modules
//!
//! This module provides centralized naming conventions and type mapping
//! to ensure consistent naming across the codebase.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Centralized mapping of FHIR resource names and their properties
#[derive(Debug, Clone)]
pub struct FhirResourceInfo {
    /// The canonical FHIR resource name (e.g., "EvidenceVariable")
    pub canonical_name: String,
    /// The module name used in imports (e.g., "evidence_variable")
    pub module_name: String,
    /// Whether this is a core FHIR resource type
    pub is_core_resource: bool,
}

impl FhirResourceInfo {
    fn new(canonical_name: &str) -> Self {
        Self {
            canonical_name: canonical_name.to_string(),
            module_name: crate::naming::Naming::to_snake_case(canonical_name),
            is_core_resource: true,
        }
    }
}

/// Static mapping of all known FHIR resources
static FHIR_RESOURCE_MAP: Lazy<HashMap<String, FhirResourceInfo>> = Lazy::new(|| {
    let resource_names = [
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
        "Parameters",
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
        // Base types
        "DomainResource",
        "Resource",
        "MetadataResource",
    ];

    let mut map = HashMap::new();
    for name in &resource_names {
        map.insert(name.to_string(), FhirResourceInfo::new(name));
    }
    map
});

/// Static mapping of known FHIR datatypes
static FHIR_DATATYPE_MAP: Lazy<HashMap<String, FhirResourceInfo>> = Lazy::new(|| {
    let datatype_names = [
        "Identifier",
        "HumanName",
        "Address",
        "ContactPoint",
        "Attachment",
        "CodeableConcept",
        "Coding",
        "Quantity",
        "Range",
        "Period",
        "Ratio",
        "SampledData",
        "Signature",
        "Age",
        "Count",
        "Distance",
        "Duration",
        "Money",
        "SimpleQuantity",
        "Extension",
        "Narrative",
        "Annotation",
        "Reference",
        "Meta",
        "ContactDetail",
        "Contributor",
        "DataRequirement",
        "Expression",
        "ParameterDefinition",
        "RelatedArtifact",
        "TriggerDefinition",
        "UsageContext",
        "Dosage",
        "Population",
        "ProductShelfLife",
        "ProdCharacteristic",
        "MarketingStatus",
        "SubstanceAmount",
        "Timing",
        "ElementDefinition",
        // Primitive types with Type suffix
        "StringType",
        "BooleanType",
        "IntegerType",
        "DecimalType",
        "UriType",
        "UrlType",
        "CanonicalType",
        "OidType",
        "UuidType",
        "InstantType",
        "DateType",
        "DateTimeType",
        "TimeType",
        "CodeType",
        "IdType",
        "MarkdownType",
        "Base64BinaryType",
        "UnsignedIntType",
        "PositiveIntType",
        "XhtmlType",
    ];

    let mut map = HashMap::new();
    for name in &datatype_names {
        let info = FhirResourceInfo {
            canonical_name: name.to_string(),
            module_name: crate::naming::Naming::to_snake_case(name),
            is_core_resource: false,
        };
        map.insert(name.to_string(), info);
    }
    map
});

/// Centralized naming manager for FHIR types
pub struct NamingManager;

impl NamingManager {
    /// Get information about a FHIR resource
    pub fn get_resource_info(resource_name: &str) -> Option<&FhirResourceInfo> {
        FHIR_RESOURCE_MAP.get(resource_name)
    }

    /// Get information about a FHIR datatype
    pub fn get_datatype_info(datatype_name: &str) -> Option<&FhirResourceInfo> {
        FHIR_DATATYPE_MAP.get(datatype_name)
    }

    /// Check if a type name represents a FHIR resource
    pub fn is_fhir_resource(type_name: &str) -> bool {
        FHIR_RESOURCE_MAP.contains_key(type_name)
    }

    /// Check if a type name represents a FHIR datatype
    pub fn is_fhir_datatype(type_name: &str) -> bool {
        FHIR_DATATYPE_MAP.contains_key(type_name)
    }

    /// Detect if a type name represents a nested structure and return the parent resource name
    /// Uses longest-prefix matching to ensure correct parent identification
    ///
    /// Examples:
    /// - "EvidenceVariableCharacteristic" -> Some("EvidenceVariable") (not "Evidence")
    /// - "MeasureReportGroup" -> Some("MeasureReport") (not "Measure")
    /// - "AccountCoverage" -> Some("Account")
    pub fn detect_nested_structure_parent(type_name: &str) -> Option<String> {
        let mut longest_match = None;
        let mut longest_length = 0;

        // Find the longest matching resource name prefix
        for resource_name in FHIR_RESOURCE_MAP.keys() {
            if type_name.starts_with(resource_name)
                && type_name.len() > resource_name.len()
                && resource_name.len() > longest_length
            {
                // Check that the character after the resource name is uppercase (indicating PascalCase)
                if let Some(next_char) = type_name.chars().nth(resource_name.len()) {
                    if next_char.is_uppercase() {
                        longest_match = Some(resource_name.clone());
                        longest_length = resource_name.len();
                    }
                }
            }
        }

        longest_match
    }

    /// Get the correct import path for a given type name
    pub fn get_import_path_for_type(type_name: &str) -> String {
        // Check if it's a generated trait FIRST (highest priority)
        if Self::is_generated_trait(type_name) {
            return Self::get_trait_import_path(type_name);
        }

        // Check if it's a nested structure (most specific for non-traits)
        if let Some(parent_resource) = Self::detect_nested_structure_parent(type_name) {
            if let Some(resource_info) = Self::get_resource_info(&parent_resource) {
                return format!(
                    "crate::resources::{}::{}",
                    resource_info.module_name, type_name
                );
            }
        }

        // Check if it's a known FHIR resource type
        if let Some(resource_info) = Self::get_resource_info(type_name) {
            return format!(
                "crate::resources::{}::{}",
                resource_info.module_name, type_name
            );
        }

        // Check if it's a known FHIR datatype
        if let Some(datatype_info) = Self::get_datatype_info(type_name) {
            return format!(
                "crate::datatypes::{}::{}",
                datatype_info.module_name, type_name
            );
        }

        // Check if it's a primitive type with special module mapping
        if let Some(primitive_path) = Self::get_primitive_import_path(type_name) {
            return primitive_path;
        }

        // Default to bindings for unknown types (likely enums)
        format!(
            "crate::bindings::{}::{}",
            crate::naming::Naming::to_snake_case(type_name),
            type_name
        )
    }

    /// Get import path for primitive types
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

        Some(format!("crate::primitives::{module_name}::{type_variant}"))
    }

    /// Check if a type is a generated trait
    fn is_generated_trait(type_name: &str) -> bool {
        type_name.ends_with("Accessors")
            || type_name.ends_with("Mutators")
            || type_name.ends_with("Helpers")
    }

    /// Get import path for generated traits
    fn get_trait_import_path(type_name: &str) -> String {
        // Strip the suffix (Accessors, Mutators, Helpers) to get the module name
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
    fn test_longest_prefix_matching() {
        // Test that longest prefix wins for nested structure detection
        assert_eq!(
            NamingManager::detect_nested_structure_parent("EvidenceVariableCharacteristic"),
            Some("EvidenceVariable".to_string())
        );
        assert_eq!(
            NamingManager::detect_nested_structure_parent("MeasureReportGroup"),
            Some("MeasureReport".to_string())
        );
        assert_eq!(
            NamingManager::detect_nested_structure_parent("AccountCoverage"),
            Some("Account".to_string())
        );
        assert_eq!(
            NamingManager::detect_nested_structure_parent("ImplementationGuideGlobal"),
            Some("ImplementationGuide".to_string())
        );
    }

    #[test]
    fn test_import_path_generation() {
        // Test correct import paths for nested structures
        assert_eq!(
            NamingManager::get_import_path_for_type("EvidenceVariableCharacteristic"),
            "crate::resources::evidence_variable::EvidenceVariableCharacteristic"
        );
        assert_eq!(
            NamingManager::get_import_path_for_type("MeasureReportGroup"),
            "crate::resources::measure_report::MeasureReportGroup"
        );
        assert_eq!(
            NamingManager::get_import_path_for_type("AccountCoverage"),
            "crate::resources::account::AccountCoverage"
        );

        // Test regular resources and datatypes still work
        assert_eq!(
            NamingManager::get_import_path_for_type("Patient"),
            "crate::resources::patient::Patient"
        );
        assert_eq!(
            NamingManager::get_import_path_for_type("Identifier"),
            "crate::datatypes::identifier::Identifier"
        );

        // Test trait import paths
        assert_eq!(
            NamingManager::get_import_path_for_type("DomainResourceAccessors"),
            "crate::traits::domain_resource::DomainResourceAccessors"
        );
        assert_eq!(
            NamingManager::get_import_path_for_type("ResourceAccessors"),
            "crate::traits::resource::ResourceAccessors"
        );

        // Test nested structure that might be falling through to bindings
        assert_eq!(
            NamingManager::get_import_path_for_type("RiskEvidenceSynthesisCertainty"),
            "crate::resources::risk_evidence_synthesis::RiskEvidenceSynthesisCertainty"
        );
        assert_eq!(
            NamingManager::get_import_path_for_type("ParametersParameter"),
            "crate::resources::parameters::ParametersParameter"
        );
    }

    #[test]
    fn test_resource_and_datatype_detection() {
        assert!(NamingManager::is_fhir_resource("Patient"));
        assert!(NamingManager::is_fhir_resource("EvidenceVariable"));
        assert!(NamingManager::is_fhir_resource("MeasureReport"));
        assert!(!NamingManager::is_fhir_resource("Identifier"));

        assert!(NamingManager::is_fhir_datatype("Identifier"));
        assert!(NamingManager::is_fhir_datatype("CodeableConcept"));
        assert!(!NamingManager::is_fhir_datatype("Patient"));
    }

    #[test]
    fn test_edge_cases() {
        // Test that non-nested structures return None
        assert_eq!(
            NamingManager::detect_nested_structure_parent("Patient"),
            None
        );
        assert_eq!(
            NamingManager::detect_nested_structure_parent("Identifier"),
            None
        );

        // Test invalid nested structure names
        assert_eq!(
            NamingManager::detect_nested_structure_parent("PatientSomething"),
            Some("Patient".to_string())
        );
    }
}
