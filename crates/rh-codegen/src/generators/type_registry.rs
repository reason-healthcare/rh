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
    /// FHIR profile (goes to profiles)
    Profile,
    /// FHIR complex datatype (goes to datatypes)
    ComplexType,
    /// Nested structure within a resource (goes to parent resource module)
    NestedStructure { parent_resource: String },
    /// Primitive type (goes to primitives)
    Primitive,
    /// Generated trait (goes to traits)
    Trait,
}

/// Global registry of type classifications and structure definitions
static TYPE_REGISTRY: Lazy<Mutex<HashMap<String, (TypeClassification, StructureDefinition)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Type registry for managing FHIR type classifications
pub struct TypeRegistry;

impl TypeRegistry {
    /// Register a type based on its StructureDefinition
    pub fn register_from_structure_definition(structure_def: &StructureDefinition) {
        let classification = Self::classify_from_structure_definition(structure_def);
        Self::register_type(
            &structure_def.name,
            classification.clone(),
            structure_def.clone(),
        );

        // Also register the actual Rust type name for import path lookups
        // The Rust type name is the PascalCase version of the structure ID
        let rust_type_name = crate::naming::Naming::to_pascal_case(&structure_def.id);
        if rust_type_name != structure_def.name {
            Self::register_type(&rust_type_name, classification, structure_def.clone());
        }
    }

    /// Register a type with a specific classification and structure definition
    pub fn register_type(
        type_name: &str,
        classification: TypeClassification,
        structure_def: StructureDefinition,
    ) {
        if let Ok(mut registry) = TYPE_REGISTRY.lock() {
            registry.insert(type_name.to_string(), (classification, structure_def));
        }
    }

    /// Register a type with just a classification (creates a minimal placeholder structure definition)
    pub fn register_type_classification_only(type_name: &str, classification: TypeClassification) {
        // Check for collisions and give nested structures priority over ValueSet enums
        let should_skip_registration = {
            if let Ok(registry) = TYPE_REGISTRY.lock() {
                if let Some((existing_classification, _)) = registry.get(type_name) {
                    match (existing_classification, &classification) {
                        // If we're trying to register a ValueSet enum but a nested structure already exists,
                        // don't overwrite the nested structure
                        (
                            TypeClassification::NestedStructure { .. },
                            TypeClassification::ValueSetEnum,
                        ) => {
                            // Keep existing nested structure, don't register ValueSet enum
                            true // Skip registration
                        }
                        // If we're trying to register a nested structure but a ValueSet enum exists,
                        // allow the overwrite (nested structure takes priority)
                        (
                            TypeClassification::ValueSetEnum,
                            TypeClassification::NestedStructure { .. },
                        ) => {
                            // Nested structure takes priority over ValueSet enum
                            false // Proceed with registration
                        }
                        // For any other collision, allow the overwrite
                        _ => false, // Proceed with registration
                    }
                } else {
                    false // No existing entry, proceed with registration
                }
            } else {
                false // Can't lock, proceed with registration
            }
        }; // Drop the read lock here

        if should_skip_registration {
            return; // Keep the existing nested structure, don't register the ValueSet enum
        }

        // Add debug output for what we're registering
        match &classification {
            TypeClassification::ValueSetEnum => {
                println!("DEBUG: Registering ValueSetEnum '{type_name}'");
            }
            TypeClassification::NestedStructure { parent_resource } => {
                println!(
                    "DEBUG: Registering NestedStructure '{type_name}' with parent '{parent_resource}'"
                );
            }
            _ => {
                // Don't add debug for other types to reduce noise
            }
        }

        // Create a minimal placeholder structure definition
        let placeholder_structure_def = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: type_name.to_lowercase(),
            url: format!("http://placeholder/{type_name}"),
            version: None,
            name: type_name.to_string(),
            title: Some(type_name.to_string()),
            status: "placeholder".to_string(),
            description: None,
            purpose: None,
            kind: match classification {
                TypeClassification::ValueSetEnum => "primitive-type".to_string(),
                TypeClassification::Resource => "resource".to_string(),
                TypeClassification::Profile => "resource".to_string(),
                TypeClassification::ComplexType => "complex-type".to_string(),
                TypeClassification::NestedStructure { .. } => "complex-type".to_string(),
                TypeClassification::Primitive => "primitive-type".to_string(),
                TypeClassification::Trait => "logical".to_string(),
            },
            is_abstract: false,
            base_type: match classification {
                TypeClassification::ValueSetEnum => "code".to_string(),
                TypeClassification::Primitive => "Element".to_string(),
                _ => "Element".to_string(),
            },
            base_definition: None,
            differential: None,
            snapshot: None,
        };

        Self::register_type(type_name, classification, placeholder_structure_def);
    }

    /// Get the classification for a type
    pub fn get_classification(type_name: &str) -> Option<TypeClassification> {
        if let Ok(registry) = TYPE_REGISTRY.lock() {
            registry
                .get(type_name)
                .map(|(classification, _)| classification.clone())
        } else {
            None
        }
    }

    /// Get the structure definition for a type
    pub fn get_structure_definition(type_name: &str) -> Option<StructureDefinition> {
        if let Ok(registry) = TYPE_REGISTRY.lock() {
            registry
                .get(type_name)
                .map(|(_, structure_def)| structure_def.clone())
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
        // Check if this is a profile first (derives from a core FHIR resource)
        if Self::is_profile(structure_def) {
            return TypeClassification::Profile;
        }

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

    /// Check if a StructureDefinition represents a profile (derived from a core FHIR resource)
    pub fn is_profile(structure_def: &StructureDefinition) -> bool {
        // Debug logging for profile detection
        if structure_def.id.contains("bmi") || structure_def.id.contains("bodyheight") {
            println!(
                "Profile detection debug for '{}': base_definition = {:?}",
                structure_def.id, structure_def.base_definition
            );
        }

        // A profile must have a base_definition
        let Some(base_def) = &structure_def.base_definition else {
            if structure_def.id.contains("bmi") || structure_def.id.contains("bodyheight") {
                println!("No base_definition found for '{}'", structure_def.id);
            }
            return false;
        };

        // Extract the resource type from the base definition URL
        // e.g., "http://hl7.org/fhir/StructureDefinition/Observation" -> "Observation"
        if let Some(resource_type) = Self::extract_resource_type_from_base_definition(base_def) {
            if structure_def.id.contains("bmi") || structure_def.id.contains("bodyheight") {
                println!(
                    "Extracted resource type '{}' from base_definition '{}' for '{}'",
                    resource_type, base_def, structure_def.id
                );
                println!(
                    "is_core_fhir_resource('{}') = {}",
                    resource_type,
                    Self::is_core_fhir_resource(&resource_type)
                );
            }
            // Check if this is a core FHIR resource
            if Self::is_core_fhir_resource(&resource_type) {
                return true;
            }

            // If it's not a core resource, it might be a profile of a profile
            // (e.g., BMI -> vitalsigns -> Observation)
            // We consider anything that derives from a non-core type to also be a profile
            // This handles chains like: BMI inherits from vitalsigns which inherits from Observation
            if structure_def.kind == "resource" {
                // For resources, if they have a base_definition that's not a core resource,
                // they're likely a profile (unless they're the base types like DomainResource)
                match resource_type.as_str() {
                    "DomainResource" | "Resource" | "Element" | "BackboneElement" => false,
                    _ => true, // Any other base_definition for a resource is likely a profile
                }
            } else {
                false
            }
        } else {
            if structure_def.id.contains("bmi") || structure_def.id.contains("bodyheight") {
                println!(
                    "Failed to extract resource type from base_definition '{}' for '{}'",
                    base_def, structure_def.id
                );
            }
            false
        }
    }

    /// Extract resource type from a base definition URL
    fn extract_resource_type_from_base_definition(base_def: &str) -> Option<String> {
        // Handle the standard FHIR StructureDefinition URLs
        if let Some(last_segment) = base_def.split('/').next_back() {
            // Skip common base types that aren't core resources
            match last_segment {
                "DomainResource" | "Resource" | "Element" | "BackboneElement" => None,
                _ => Some(last_segment.to_string()),
            }
        } else {
            None
        }
    }

    /// Check if a resource type is a core FHIR resource
    fn is_core_fhir_resource(resource_type: &str) -> bool {
        // Use the same list we have for parent resource detection
        let core_resources = vec![
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

        core_resources.contains(&resource_type)
    }

    /// Check if a StructureDefinition represents a ValueSet-based enum
    fn is_value_set_enum(structure_def: &StructureDefinition) -> bool {
        // First, check if this is a known FHIR primitive type
        // These should NOT be treated as ValueSet enums
        if Self::is_fhir_primitive_type(&structure_def.name) {
            return false;
        }

        // ValueSet-based enums are typically primitive-type with base_type "code"
        // and have binding constraints, but they are NOT FHIR primitive types
        structure_def.kind == "primitive-type" && structure_def.base_type == "code"
    }

    /// Check if a type name represents a FHIR primitive type
    pub fn is_fhir_primitive_type(type_name: &str) -> bool {
        // Known FHIR primitive types (with and without "Type" suffix)
        matches!(
            type_name,
            // With "Type" suffix
            "BooleanType" | "StringType" | "IntegerType" | "DecimalType" |
            "UriType" | "UrlType" | "CanonicalType" | "OidType" | "UuidType" |
            "InstantType" | "DateType" | "DateTimeType" | "TimeType" |
            "CodeType" | "IdType" | "MarkdownType" | "Base64BinaryType" |
            "UnsignedIntType" | "PositiveIntType" | "XhtmlType" |
            // Without "Type" suffix  
            "Boolean" | "String" | "Integer" | "Decimal" |
            "Uri" | "Url" | "Canonical" | "Oid" | "Uuid" |
            "Instant" | "Date" | "DateTime" | "Time" |
            "Code" | "Id" | "Markdown" | "Base64Binary" |
            "UnsignedInt" | "PositiveInt" | "Xhtml"
        )
    }

    /// Check if a type is a binding enum using the TypeRegistry classification.
    /// This is more reliable than heuristics based on naming patterns.
    ///
    /// Binding enums are typically registered as ValueSetEnum in the TypeRegistry.
    pub fn is_binding_enum(type_name: &str) -> bool {
        // First exclude FHIR primitive types
        if Self::is_fhir_primitive_type(type_name) {
            return false;
        }

        // Check if this is registered as a ValueSetEnum in the TypeRegistry
        matches!(
            Self::get_classification(type_name),
            Some(TypeClassification::ValueSetEnum)
        )
    }

    /// Detect if this is a nested structure and return the parent resource name
    fn detect_parent_resource(structure_def: &StructureDefinition) -> Option<String> {
        // Check if the base_definition indicates this is a BackboneElement
        if let Some(base_def) = &structure_def.base_definition {
            if base_def.contains("BackboneElement") {
                // Extract parent resource from the URL fragment
                // E.g., "http://hl7.org/fhir/StructureDefinition/Task#Task.restriction" -> "Task"
                if let Some(fragment) = base_def.split('#').nth(1) {
                    if let Some(parent) = fragment.split('.').next() {
                        return Some(parent.to_string());
                    }
                }

                // If there's no fragment, this is a standalone reusable BackboneElement type
                // (like SubstanceAmount), not a nested structure within a specific resource.
                // Return None to classify it as ComplexType instead of NestedStructure.
                return None;
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
        known_resources.sort_by_key(|b| std::cmp::Reverse(b.len()));

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
                TypeClassification::Profile => {
                    format!(
                        "crate::profiles::{}::{}",
                        crate::naming::Naming::to_snake_case(type_name),
                        type_name
                    )
                }
            }
        } else {
            // Fallback to NamingManager for unregistered types
            // But for some well-known types, provide better defaults

            // First check if it's a FHIR primitive type
            if Self::is_fhir_primitive_type(type_name) {
                Self::get_primitive_import_path(type_name).unwrap_or_else(|| {
                    format!(
                        "crate::primitives::{}::{}",
                        crate::naming::Naming::to_snake_case(type_name),
                        type_name
                    )
                })
            } else if Self::is_binding_enum(type_name) {
                // Check for binding enums BEFORE checking nested structures
                // to prevent enums like "AccountStatus" from being misclassified
                format!(
                    "crate::bindings::{}::{}",
                    crate::naming::Naming::to_snake_case(type_name),
                    type_name
                )
            } else if crate::generators::naming_manager::NamingManager::is_fhir_resource(type_name)
            {
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

        // Test ConditionStage specifically
        assert_eq!(
            TypeRegistry::extract_parent_from_name("ConditionStage"),
            Some("Condition".to_string())
        );
    }

    #[test]
    fn test_primitive_type_import_paths() {
        // Test that FHIR primitive types are correctly routed to primitives module
        // and not to bindings module

        let primitive_types = vec![
            "BooleanType",
            "StringType",
            "IntegerType",
            "PositiveIntType",
            "DecimalType",
            "UriType",
            "DateTimeType",
        ];

        for type_name in primitive_types {
            let import_path = TypeRegistry::get_import_path_for_type(type_name);

            // Should be routed to primitives, not bindings
            assert!(
                import_path.contains("crate::primitives::"),
                "Type {type_name} should be routed to primitives module, got: {import_path}"
            );

            // Should not be routed to bindings
            assert!(
                !import_path.contains("crate::bindings::"),
                "Type {type_name} should not be routed to bindings module, got: {import_path}"
            );
        }
    }

    #[test]
    fn test_is_fhir_primitive_type() {
        // Test the primitive type detection function
        assert!(TypeRegistry::is_fhir_primitive_type("BooleanType"));
        assert!(TypeRegistry::is_fhir_primitive_type("StringType"));
        assert!(TypeRegistry::is_fhir_primitive_type("PositiveIntType"));
        assert!(TypeRegistry::is_fhir_primitive_type("Boolean"));
        assert!(TypeRegistry::is_fhir_primitive_type("String"));

        // These should not be primitive types
        assert!(!TypeRegistry::is_fhir_primitive_type("Patient"));
        assert!(!TypeRegistry::is_fhir_primitive_type("AddressType"));
        assert!(!TypeRegistry::is_fhir_primitive_type("HumanName"));
    }

    #[test]
    fn test_is_binding_enum() {
        TypeRegistry::clear();

        // Test with actual StructureDefinition for AccountStatus (a binding enum)
        let account_status = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "account-status".to_string(),
            url: "http://hl7.org/fhir/ValueSet/account-status".to_string(),
            version: None,
            name: "AccountStatus".to_string(),
            title: Some("AccountStatus".to_string()),
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

        TypeRegistry::register_from_structure_definition(&account_status);

        // Test binding enum detection
        assert!(TypeRegistry::is_binding_enum("AccountStatus"));

        // Test that FHIR primitives are not binding enums
        assert!(!TypeRegistry::is_binding_enum("BooleanType"));
        assert!(!TypeRegistry::is_binding_enum("StringType"));

        // Test that non-registered types without structure definitions don't match
        assert!(!TypeRegistry::is_binding_enum("Patient"));
        assert!(!TypeRegistry::is_binding_enum("AccountCoverage"));
        assert!(!TypeRegistry::is_binding_enum("HumanName"));
    }

    #[test]
    fn test_enum_import_paths() {
        TypeRegistry::clear();

        // Register some actual binding enums with structure definitions
        let task_intent = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "task-intent".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/task-intent".to_string(),
            version: None,
            name: "TaskIntent".to_string(),
            title: Some("TaskIntent".to_string()),
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

        let publication_status = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "publication-status".to_string(),
            url: "http://hl7.org/fhir/ValueSet/publication-status".to_string(),
            version: None,
            name: "PublicationStatus".to_string(),
            title: Some("PublicationStatus".to_string()),
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
        TypeRegistry::register_from_structure_definition(&publication_status);

        // Test that binding enums are correctly routed to bindings module
        let enum_types = vec!["TaskIntent", "PublicationStatus"];

        for type_name in enum_types {
            let import_path = TypeRegistry::get_import_path_for_type(type_name);

            // Should be routed to bindings, not resources
            assert!(
                import_path.contains("crate::bindings::"),
                "Type {type_name} should be routed to bindings module, got: {import_path}"
            );

            // Should not be routed to resources
            assert!(
                !import_path.contains("crate::resources::"),
                "Type {type_name} should not be routed to resources module, got: {import_path}"
            );
        }
    }

    #[test]
    fn test_nested_structure_import_paths() {
        TypeRegistry::clear();

        // Test that registering nested structures works correctly
        TypeRegistry::register_type_classification_only(
            "ConditionStage",
            TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            },
        );

        assert_eq!(
            TypeRegistry::get_import_path_for_type("ConditionStage"),
            "crate::resources::condition::ConditionStage"
        );

        TypeRegistry::register_type_classification_only(
            "MedicationDosage",
            TypeClassification::NestedStructure {
                parent_resource: "Medication".to_string(),
            },
        );

        assert_eq!(
            TypeRegistry::get_import_path_for_type("MedicationDosage"),
            "crate::resources::medication::MedicationDosage"
        );
    }

    #[test]
    fn test_naming_collision_valueset_vs_nested_structure() {
        // Test the collision scenario: ValueSet registered first, then nested structure
        TypeRegistry::clear();

        // Step 1: Register as ValueSet enum (this happens during pre-registration)
        TypeRegistry::register_type_classification_only(
            "ConditionStage",
            TypeClassification::ValueSetEnum,
        );

        // Verify it's registered as ValueSet
        assert_eq!(
            TypeRegistry::get_classification("ConditionStage"),
            Some(TypeClassification::ValueSetEnum)
        );
        assert_eq!(
            TypeRegistry::get_import_path_for_type("ConditionStage"),
            "crate::bindings::condition_stage::ConditionStage"
        );

        // Step 2: Register as nested structure (this happens during struct generation)
        TypeRegistry::register_type_classification_only(
            "ConditionStage",
            TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            },
        );

        // Verify it's now registered as nested structure (should overwrite)
        assert_eq!(
            TypeRegistry::get_classification("ConditionStage"),
            Some(TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            })
        );
        assert_eq!(
            TypeRegistry::get_import_path_for_type("ConditionStage"),
            "crate::resources::condition::ConditionStage"
        );
    }

    #[test]
    fn test_priority_nested_structure_over_valueset() {
        // Test that once a nested structure is registered, ValueSets cannot overwrite it
        TypeRegistry::clear();

        // Step 1: Register as nested structure first
        TypeRegistry::register_type_classification_only(
            "ConditionStage",
            TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            },
        );

        // Verify it's registered as nested structure
        assert_eq!(
            TypeRegistry::get_classification("ConditionStage"),
            Some(TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            })
        );

        // Step 2: Try to register as ValueSet enum (should be ignored)
        TypeRegistry::register_type_classification_only(
            "ConditionStage",
            TypeClassification::ValueSetEnum,
        );

        // Verify it's still registered as nested structure (ValueSet registration ignored)
        assert_eq!(
            TypeRegistry::get_classification("ConditionStage"),
            Some(TypeClassification::NestedStructure {
                parent_resource: "Condition".to_string(),
            })
        );
        assert_eq!(
            TypeRegistry::get_import_path_for_type("ConditionStage"),
            "crate::resources::condition::ConditionStage"
        );
    }
}
