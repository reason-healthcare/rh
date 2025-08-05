//! Type classification and utility functions for FHIR code generation
//!
//! This module provides utility functions for classifying FHIR types, converting names,
//! and other common operations needed during code generation.

use crate::fhir_types::StructureDefinition;
use crate::value_sets::ValueSetManager;

/// Utility functions for type classification and name conversion
pub struct TypeUtilities;

impl TypeUtilities {
    /// Check if a type name represents a known FHIR data type
    pub fn is_fhir_datatype(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "narrative"
                | "extension"
                | "coding"
                | "codeableconcept"
                | "identifier"
                | "humanname"
                | "address"
                | "contactpoint"
                | "period"
                | "quantity"
                | "range"
                | "ratio"
                | "sampleddata"
                | "attachment"
                | "signature"
                | "annotation"
                | "meta"
                | "reference"
                | "dosage"
                | "age"
                | "count"
                | "distance"
                | "duration"
                | "money"
                | "simplequantity"
                | "timing"
                | "element"
                | "backboneelement"
                | "datatype"
                | "primitivetype"
                // Additional complex datatypes
                | "contactdetail"
                | "usagecontext"
                | "expression"
                | "relatedartifact"
                | "contributor"
                | "datarequirement"
                | "parameterdefinition"
                | "triggerdefinition"
                | "prodcharacteristic"
                | "substance"
                | "productshelflife"
                | "marketingstatus"
                | "population"
                | "substanceamount"
        )
    }

    /// Check if any ValueSet enums have been generated
    pub fn has_cached_enums(value_set_manager: &ValueSetManager) -> bool {
        !value_set_manager.get_cached_enums().is_empty()
    }

    /// Check if a FHIR type is a primitive type
    pub fn is_primitive_type(structure_def: &StructureDefinition) -> bool {
        structure_def.kind == "primitive-type"
    }

    /// Check if a FHIR type is a resource type
    pub fn is_resource_type(structure_def: &StructureDefinition) -> bool {
        structure_def.kind == "resource"
    }

    /// Check if a FHIR type is a complex data type
    pub fn is_complex_type(structure_def: &StructureDefinition) -> bool {
        structure_def.kind == "complex-type"
    }

    /// Check if a FHIR type is a logical model (should be skipped)
    pub fn is_logical_model(structure_def: &StructureDefinition) -> bool {
        structure_def.kind == "logical"
    }

    /// Check if a FHIR type should be skipped due to underscore prefix
    /// Files beginning with underscore are typically auto-generated or temporary
    pub fn should_skip_underscore_prefixed(structure_def: &StructureDefinition) -> bool {
        use crate::generators::name_generator::NameGenerator;
        
        // Check if the original name or id starts with underscore
        if structure_def.name.starts_with('_') || structure_def.id.starts_with('_') {
            return true;
        }
        
        // Check if the generated struct name would start with underscore
        let generated_name = NameGenerator::generate_struct_name(structure_def);
        generated_name.starts_with('_')
    }

    /// Get the base type hierarchy for a structure definition
    pub fn get_base_type_hierarchy(structure_def: &StructureDefinition) -> Vec<String> {
        let mut hierarchy = vec![structure_def.base_type.clone()];

        // Add known base type hierarchies
        match structure_def.base_type.as_str() {
            "DomainResource" => {
                hierarchy.push("Resource".to_string());
                hierarchy.push("BaseResource".to_string());
            }
            "Resource" => {
                hierarchy.push("BaseResource".to_string());
            }
            "BackboneElement" => {
                hierarchy.push("Element".to_string());
                hierarchy.push("Base".to_string());
            }
            "Element" => {
                hierarchy.push("Base".to_string());
            }
            _ => {}
        }

        hierarchy
    }

    /// Determine if a structure definition should generate a struct or type alias
    pub fn should_generate_struct(structure_def: &StructureDefinition) -> bool {
        !Self::is_primitive_type(structure_def) && !Self::is_logical_model(structure_def)
    }

    /// Determine if a structure definition should generate a trait
    pub fn should_generate_trait(structure_def: &StructureDefinition) -> bool {
        Self::is_resource_type(structure_def) || Self::is_complex_type(structure_def)
    }

    /// Get the appropriate file extension for generated code
    pub fn get_file_extension() -> &'static str {
        "rs"
    }

    /// Check if a type name is a known FHIR resource type
    pub fn is_fhir_resource_type(name: &str) -> bool {
        matches!(
            name,
            "Resource"
                | "DomainResource"
                | "Patient"
                | "Practitioner"
                | "Organization"
                | "Location"
                | "Observation"
                | "Condition"
                | "Procedure"
                | "MedicationRequest"
                | "MedicationAdministration"
                | "DiagnosticReport"
                | "Encounter"
                | "AllergyIntolerance"
                | "CarePlan"
                | "CareTeam"
                | "Goal"
                | "ServiceRequest"
                | "Specimen"
                | "Device"
                | "Medication"
                | "Substance"
                | "Bundle"
                | "Composition"
                | "DocumentManifest"
                | "DocumentReference"
                | "List"
                | "MessageHeader"
                | "OperationOutcome"
                | "Parameters"
                | "Binary"
                | "Provenance"
                | "AuditEvent"
                | "Consent"
                | "Contract"
                | "Coverage"
                | "ExplanationOfBenefit"
                | "Claim"
                | "ClaimResponse"
                | "Account"
                | "Invoice"
                | "PaymentNotice"
                | "PaymentReconciliation"
                | "EnrollmentRequest"
                | "EnrollmentResponse"
                | "EligibilityRequest"
                | "EligibilityResponse"
                | "CoverageEligibilityRequest"
                | "CoverageEligibilityResponse"
                | "InsurancePlan"
                | "HealthcareService"
                | "Endpoint"
                | "Schedule"
                | "Slot"
                | "Appointment"
                | "AppointmentResponse"
                | "Flag"
                | "RiskAssessment"
                | "DetectedIssue"
                | "ClinicalImpression"
                | "FamilyMemberHistory"
                | "Group"
                | "ImmunizationRecommendation"
                | "Immunization"
                | "SupplyRequest"
                | "SupplyDelivery"
                | "VisionPrescription"
                | "NutritionOrder"
                | "DeviceRequest"
                | "DeviceUseStatement"
                | "RequestGroup"
                | "Task"
                | "Communication"
                | "CommunicationRequest"
                | "Basic"
                | "QuestionnaireResponse"
                | "Questionnaire"
                | "ResearchStudy"
                | "ResearchSubject"
                | "ActivityDefinition"
                | "PlanDefinition"
                | "Measure"
                | "MeasureReport"
                | "Library"
                | "ImplementationGuide"
                | "StructureDefinition"
                | "StructureMap"
                | "GraphDefinition"
                | "ExampleScenario"
                | "CodeSystem"
                | "ValueSet"
                | "ConceptMap"
                | "NamingSystem"
                | "TerminologyCapabilities"
                | "CapabilityStatement"
                | "SearchParameter"
                | "CompartmentDefinition"
                | "OperationDefinition"
                | "MessageDefinition"
                | "EventDefinition"
                | "ChargeItemDefinition"
                | "ChargeItem"
                | "SubstanceSpecification"
                | "SubstancePolymer"
                | "SubstanceReferenceInformation"
                | "SubstanceSourceMaterial"
                | "MedicinalProduct"
                | "MedicinalProductAuthorization"
                | "MedicinalProductContraindication"
                | "MedicinalProductIndication"
                | "MedicinalProductIngredient"
                | "MedicinalProductInteraction"
                | "MedicinalProductManufactured"
                | "MedicinalProductPackaged"
                | "MedicinalProductPharmaceutical"
                | "MedicinalProductUndesirableEffect"
                | "DeviceDefinition"
                | "DeviceMetric"
                | "BiologicallyDerivedProduct"
                | "CatalogEntry"
                | "EffectEvidenceSynthesis"
                | "Evidence"
                | "EvidenceVariable"
                | "MolecularSequence"
                | "RiskEvidenceSynthesis"
                | "VerificationResult"
                | "AdverseEvent"
                | "Media"
                | "ImagingStudy"
                | "BodyStructure"
                | "Person"
                | "RelatedPerson"
                | "PractitionerRole"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhir_types::StructureDefinition;

    #[test]
    fn test_is_fhir_datatype() {
        // Test known FHIR datatypes
        assert!(TypeUtilities::is_fhir_datatype("Identifier"));
        assert!(TypeUtilities::is_fhir_datatype("CodeableConcept"));
        assert!(TypeUtilities::is_fhir_datatype("HumanName"));
        assert!(TypeUtilities::is_fhir_datatype("Address"));
        assert!(TypeUtilities::is_fhir_datatype("Period"));
        assert!(TypeUtilities::is_fhir_datatype("Reference"));
        assert!(TypeUtilities::is_fhir_datatype("BackboneElement"));

        // Test case insensitivity
        assert!(TypeUtilities::is_fhir_datatype("identifier"));
        assert!(TypeUtilities::is_fhir_datatype("IDENTIFIER"));
        assert!(TypeUtilities::is_fhir_datatype("codeableconcept"));

        // Test non-FHIR types
        assert!(!TypeUtilities::is_fhir_datatype("Patient"));
        assert!(!TypeUtilities::is_fhir_datatype("String"));
        assert!(!TypeUtilities::is_fhir_datatype("CustomType"));
    }

    #[test]
    fn test_is_fhir_resource_type() {
        // Test known FHIR resource types
        assert!(TypeUtilities::is_fhir_resource_type("Patient"));
        assert!(TypeUtilities::is_fhir_resource_type("Observation"));
        assert!(TypeUtilities::is_fhir_resource_type("DomainResource"));
        assert!(TypeUtilities::is_fhir_resource_type("Bundle"));
        assert!(TypeUtilities::is_fhir_resource_type("StructureDefinition"));

        // Test non-resource types
        assert!(!TypeUtilities::is_fhir_resource_type("Identifier"));
        assert!(!TypeUtilities::is_fhir_resource_type("String"));
        assert!(!TypeUtilities::is_fhir_resource_type("CustomType"));
    }

    #[test]
    fn test_type_classification() {
        // Test primitive type
        let primitive_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "string".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/string".to_string(),
            name: "string".to_string(),
            title: Some("string".to_string()),
            status: "active".to_string(),
            kind: "primitive-type".to_string(),
            is_abstract: false,
            description: Some("A sequence of Unicode characters".to_string()),
            purpose: None,
            base_type: "string".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Element".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert!(TypeUtilities::is_primitive_type(&primitive_structure));
        assert!(!TypeUtilities::is_resource_type(&primitive_structure));
        assert!(!TypeUtilities::is_complex_type(&primitive_structure));
        assert!(!TypeUtilities::is_logical_model(&primitive_structure));
        assert!(!TypeUtilities::should_generate_struct(&primitive_structure));

        // Test resource type
        let resource_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("Demographics and other administrative information".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert!(!TypeUtilities::is_primitive_type(&resource_structure));
        assert!(TypeUtilities::is_resource_type(&resource_structure));
        assert!(!TypeUtilities::is_complex_type(&resource_structure));
        assert!(!TypeUtilities::is_logical_model(&resource_structure));
        assert!(TypeUtilities::should_generate_struct(&resource_structure));
        assert!(TypeUtilities::should_generate_trait(&resource_structure));
    }

    #[test]
    fn test_base_type_hierarchy() {
        let patient_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("Demographics and other administrative information".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string(),
            ),
            version: None,
            differential: None,
            snapshot: None,
        };

        let hierarchy = TypeUtilities::get_base_type_hierarchy(&patient_structure);
        assert_eq!(
            hierarchy,
            vec!["DomainResource", "Resource", "BaseResource"]
        );
    }

    #[test]
    fn test_logical_model_detection() {
        let logical_model = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "test-logical-model".to_string(),
            url: "http://example.org/fhir/StructureDefinition/test-logical-model".to_string(),
            name: "test-logical-model".to_string(),
            title: Some("Test Logical Model".to_string()),
            status: "active".to_string(),
            kind: "logical".to_string(),
            is_abstract: false,
            description: Some("A test logical model".to_string()),
            purpose: None,
            base_type: "Base".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Base".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert!(TypeUtilities::is_logical_model(&logical_model));
        assert!(!TypeUtilities::should_generate_struct(&logical_model));
    }

    #[test]
    fn test_underscore_prefixed_detection() {
        // Test structure with underscore prefixed name
        let underscore_name_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "normal-id".to_string(),
            url: "http://example.org/fhir/StructureDefinition/_11179object_class".to_string(),
            name: "_11179object_class".to_string(),
            title: Some("Auto-generated class".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("An auto-generated resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test structure with underscore prefixed id
        let underscore_id_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "_temp_generated".to_string(),
            url: "http://example.org/fhir/StructureDefinition/temp-generated".to_string(),
            name: "temp-generated".to_string(),
            title: Some("Temporary resource".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A temporary resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test structure with numeric prefix (gets underscore in generated name)
        let numeric_prefix_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "11179-objectClass".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/11179-objectClass".to_string(),
            name: "ObjectClass".to_string(),
            title: Some("Object Class extension".to_string()),
            status: "active".to_string(),
            kind: "complex-type".to_string(),
            is_abstract: false,
            description: Some("A concept that represents a set of ideas".to_string()),
            purpose: None,
            base_type: "Extension".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Extension".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        // Test normal structure (no underscore prefix)
        let normal_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            name: "Patient".to_string(),
            title: Some("Patient".to_string()),
            status: "active".to_string(),
            kind: "resource".to_string(),
            is_abstract: false,
            description: Some("A patient resource".to_string()),
            purpose: None,
            base_type: "DomainResource".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            version: None,
            differential: None,
            snapshot: None,
        };

        assert!(TypeUtilities::should_skip_underscore_prefixed(&underscore_name_structure));
        assert!(TypeUtilities::should_skip_underscore_prefixed(&underscore_id_structure));
        assert!(TypeUtilities::should_skip_underscore_prefixed(&numeric_prefix_structure));
        assert!(!TypeUtilities::should_skip_underscore_prefixed(&normal_structure));
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(TypeUtilities::get_file_extension(), "rs");
    }
}
