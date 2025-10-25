//! Resource type detection and dynamic dispatch

use crate::types::{IssueCode, Severity, ValidationIssue, ValidationResult};
use crate::validator::FhirValidator;
use anyhow::{anyhow, Result};
use serde_json::Value;

/// Extract the resourceType field from JSON
pub fn extract_resource_type(json: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;

    match value.get("resourceType") {
        Some(Value::String(resource_type)) => Ok(resource_type.clone()),
        Some(_) => Err(anyhow!("resourceType must be a string")),
        None => Err(anyhow!("Missing required field: resourceType")),
    }
}

/// Suggest similar resource type names for typos
pub fn suggest_resource_type(invalid: &str) -> Option<String> {
    let all_types = ALL_RESOURCE_TYPES;

    // Simple case-insensitive match first
    if let Some(correct) = all_types.iter().find(|t| t.eq_ignore_ascii_case(invalid)) {
        return Some(correct.to_string());
    }

    // Find closest match using simple Levenshtein-like heuristic
    let mut best_match = None;
    let mut best_score = usize::MAX;

    for resource_type in all_types {
        let score = levenshtein_distance(invalid, resource_type);
        if score < best_score && score <= 3 {
            best_score = score;
            best_match = Some(resource_type.to_string());
        }
    }

    best_match
}

/// Simple Levenshtein distance calculation
#[allow(clippy::needless_range_loop)]
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

/// Macro to generate resource type dispatch
macro_rules! dispatch_resource_validation {
    ($resource_type:expr, $json:expr, $validator:expr, { $($type_name:literal => $module:ident :: $struct:ident),* $(,)? }) => {
        match $resource_type.as_str() {
            $(
                $type_name => {
                    use hl7_fhir_r4_core::resources::$module::$struct;
                    $validator.validate_json::<$struct>($json).map_err(|e| anyhow::Error::from(e))
                }
            )*
            unknown => {
                let mut result = ValidationResult::new(unknown.to_string());
                let message = if let Some(suggestion) = suggest_resource_type(unknown) {
                    format!("Unknown resourceType: '{}'. Did you mean '{}'?", unknown, suggestion)
                } else {
                    format!("Unknown resourceType: '{}'. Must be a valid FHIR R4 resource type.", unknown)
                };
                result.add_issue(ValidationIssue::new(
                    Severity::Error,
                    IssueCode::Structure,
                    message,
                ));
                Ok(result)
            }
        }
    };
}

/// All FHIR R4 resource types for suggestion matching
const ALL_RESOURCE_TYPES: &[&str] = &[
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
    "Parameters",
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

impl FhirValidator {
    /// Validate any FHIR resource with automatic resource type detection
    ///
    /// This method extracts the `resourceType` field from the JSON and dispatches
    /// to the appropriate validation based on the resource type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rh_validator::FhirValidator;
    ///
    /// let validator = FhirValidator::new()?;
    /// let json = r#"{"resourceType": "Patient", "id": "123"}"#;
    /// let result = validator.validate_any_resource(json)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn validate_any_resource(&self, json: &str) -> Result<ValidationResult> {
        let resource_type = extract_resource_type(json)?;

        dispatch_resource_validation!(resource_type, json, self, {
            "Account" => account::Account,
            "ActivityDefinition" => activity_definition::ActivityDefinition,
            "AdverseEvent" => adverse_event::AdverseEvent,
            "AllergyIntolerance" => allergy_intolerance::AllergyIntolerance,
            "Appointment" => appointment::Appointment,
            "AppointmentResponse" => appointment_response::AppointmentResponse,
            "AuditEvent" => audit_event::AuditEvent,
            "Basic" => basic::Basic,
            "Binary" => binary::Binary,
            "BiologicallyDerivedProduct" => biologically_derived_product::BiologicallyDerivedProduct,
            "BodyStructure" => body_structure::BodyStructure,
            "Bundle" => bundle::Bundle,
            "CapabilityStatement" => capability_statement::CapabilityStatement,
            "CarePlan" => care_plan::CarePlan,
            "CareTeam" => care_team::CareTeam,
            "CatalogEntry" => catalog_entry::CatalogEntry,
            "ChargeItem" => charge_item::ChargeItem,
            "ChargeItemDefinition" => charge_item_definition::ChargeItemDefinition,
            "Claim" => claim::Claim,
            "ClaimResponse" => claim_response::ClaimResponse,
            "ClinicalImpression" => clinical_impression::ClinicalImpression,
            "CodeSystem" => code_system::CodeSystem,
            "Communication" => communication::Communication,
            "CommunicationRequest" => communication_request::CommunicationRequest,
            "CompartmentDefinition" => compartment_definition::CompartmentDefinition,
            "Composition" => composition::Composition,
            "ConceptMap" => concept_map::ConceptMap,
            "Condition" => condition::Condition,
            "Consent" => consent::Consent,
            "Contract" => contract::Contract,
            "Coverage" => coverage::Coverage,
            "CoverageEligibilityRequest" => coverage_eligibility_request::CoverageEligibilityRequest,
            "CoverageEligibilityResponse" => coverage_eligibility_response::CoverageEligibilityResponse,
            "DetectedIssue" => detected_issue::DetectedIssue,
            "Device" => device::Device,
            "DeviceDefinition" => device_definition::DeviceDefinition,
            "DeviceMetric" => device_metric::DeviceMetric,
            "DeviceRequest" => device_request::DeviceRequest,
            "DeviceUseStatement" => device_use_statement::DeviceUseStatement,
            "DiagnosticReport" => diagnostic_report::DiagnosticReport,
            "DocumentManifest" => document_manifest::DocumentManifest,
            "DocumentReference" => document_reference::DocumentReference,
            "EffectEvidenceSynthesis" => effect_evidence_synthesis::EffectEvidenceSynthesis,
            "Encounter" => encounter::Encounter,
            "Endpoint" => endpoint::Endpoint,
            "EnrollmentRequest" => enrollment_request::EnrollmentRequest,
            "EnrollmentResponse" => enrollment_response::EnrollmentResponse,
            "EpisodeOfCare" => episode_of_care::EpisodeOfCare,
            "EventDefinition" => event_definition::EventDefinition,
            "Evidence" => evidence::Evidence,
            "EvidenceVariable" => evidence_variable::EvidenceVariable,
            "ExplanationOfBenefit" => explanation_of_benefit::ExplanationOfBenefit,
            "FamilyMemberHistory" => family_member_history::FamilyMemberHistory,
            "Flag" => flag::Flag,
            "Goal" => goal::Goal,
            "GraphDefinition" => graph_definition::GraphDefinition,
            "Group" => group::Group,
            "GuidanceResponse" => guidance_response::GuidanceResponse,
            "HealthcareService" => healthcare_service::HealthcareService,
            "ImagingStudy" => imaging_study::ImagingStudy,
            "Immunization" => immunization::Immunization,
            "ImmunizationEvaluation" => immunization_evaluation::ImmunizationEvaluation,
            "ImmunizationRecommendation" => immunization_recommendation::ImmunizationRecommendation,
            "ImplementationGuide" => implementation_guide::ImplementationGuide,
            "InsurancePlan" => insurance_plan::InsurancePlan,
            "Invoice" => invoice::Invoice,
            "Library" => library::Library,
            "Linkage" => linkage::Linkage,
            "List" => list::List,
            "Location" => location::Location,
            "Measure" => measure::Measure,
            "MeasureReport" => measure_report::MeasureReport,
            "Media" => media::Media,
            "Medication" => medication::Medication,
            "MedicationAdministration" => medication_administration::MedicationAdministration,
            "MedicationDispense" => medication_dispense::MedicationDispense,
            "MedicationKnowledge" => medication_knowledge::MedicationKnowledge,
            "MedicationRequest" => medication_request::MedicationRequest,
            "MedicationStatement" => medication_statement::MedicationStatement,
            "MedicinalProduct" => medicinal_product::MedicinalProduct,
            "MedicinalProductAuthorization" => medicinal_product_authorization::MedicinalProductAuthorization,
            "MedicinalProductContraindication" => medicinal_product_contraindication::MedicinalProductContraindication,
            "MedicinalProductIndication" => medicinal_product_indication::MedicinalProductIndication,
            "MedicinalProductIngredient" => medicinal_product_ingredient::MedicinalProductIngredient,
            "MedicinalProductInteraction" => medicinal_product_interaction::MedicinalProductInteraction,
            "MedicinalProductManufactured" => medicinal_product_manufactured::MedicinalProductManufactured,
            "MedicinalProductPackaged" => medicinal_product_packaged::MedicinalProductPackaged,
            "MedicinalProductPharmaceutical" => medicinal_product_pharmaceutical::MedicinalProductPharmaceutical,
            "MedicinalProductUndesirableEffect" => medicinal_product_undesirable_effect::MedicinalProductUndesirableEffect,
            "MessageDefinition" => message_definition::MessageDefinition,
            "MessageHeader" => message_header::MessageHeader,
            "MolecularSequence" => molecular_sequence::MolecularSequence,
            "NamingSystem" => naming_system::NamingSystem,
            "NutritionOrder" => nutrition_order::NutritionOrder,
            "Observation" => observation::Observation,
            "ObservationDefinition" => observation_definition::ObservationDefinition,
            "OperationDefinition" => operation_definition::OperationDefinition,
            "OperationOutcome" => operation_outcome::OperationOutcome,
            "Organization" => organization::Organization,
            "OrganizationAffiliation" => organization_affiliation::OrganizationAffiliation,
            "Parameters" => parameters::Parameters,
            "Patient" => patient::Patient,
            "PaymentNotice" => payment_notice::PaymentNotice,
            "PaymentReconciliation" => payment_reconciliation::PaymentReconciliation,
            "Person" => person::Person,
            "PlanDefinition" => plan_definition::PlanDefinition,
            "Practitioner" => practitioner::Practitioner,
            "PractitionerRole" => practitioner_role::PractitionerRole,
            "Procedure" => procedure::Procedure,
            "Provenance" => provenance::Provenance,
            "Questionnaire" => questionnaire::Questionnaire,
            "QuestionnaireResponse" => questionnaire_response::QuestionnaireResponse,
            "RelatedPerson" => related_person::RelatedPerson,
            "RequestGroup" => request_group::RequestGroup,
            "ResearchDefinition" => research_definition::ResearchDefinition,
            "ResearchElementDefinition" => research_element_definition::ResearchElementDefinition,
            "ResearchStudy" => research_study::ResearchStudy,
            "ResearchSubject" => research_subject::ResearchSubject,
            "RiskAssessment" => risk_assessment::RiskAssessment,
            "RiskEvidenceSynthesis" => risk_evidence_synthesis::RiskEvidenceSynthesis,
            "Schedule" => schedule::Schedule,
            "SearchParameter" => search_parameter::SearchParameter,
            "ServiceRequest" => service_request::ServiceRequest,
            "Slot" => slot::Slot,
            "Specimen" => specimen::Specimen,
            "SpecimenDefinition" => specimen_definition::SpecimenDefinition,
            "StructureDefinition" => structure_definition::StructureDefinition,
            "StructureMap" => structure_map::StructureMap,
            "Subscription" => subscription::Subscription,
            "Substance" => substance::Substance,
            "SubstanceNucleicAcid" => substance_nucleic_acid::SubstanceNucleicAcid,
            "SubstancePolymer" => substance_polymer::SubstancePolymer,
            "SubstanceProtein" => substance_protein::SubstanceProtein,
            "SubstanceReferenceInformation" => substance_reference_information::SubstanceReferenceInformation,
            "SubstanceSourceMaterial" => substance_source_material::SubstanceSourceMaterial,
            "SubstanceSpecification" => substance_specification::SubstanceSpecification,
            "SupplyDelivery" => supply_delivery::SupplyDelivery,
            "SupplyRequest" => supply_request::SupplyRequest,
            "Task" => task::Task,
            "TerminologyCapabilities" => terminology_capabilities::TerminologyCapabilities,
            "TestReport" => test_report::TestReport,
            "TestScript" => test_script::TestScript,
            "ValueSet" => value_set::ValueSet,
            "VerificationResult" => verification_result::VerificationResult,
            "VisionPrescription" => vision_prescription::VisionPrescription,
        })
    }

    /// Validate multiple FHIR resources from NDJSON with automatic type detection
    ///
    /// This method processes NDJSON (newline-delimited JSON) where each line contains
    /// a FHIR resource. Resource types are automatically detected from the `resourceType` field.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rh_validator::FhirValidator;
    ///
    /// let validator = FhirValidator::new()?;
    /// let ndjson = r#"{"resourceType": "Patient", "id": "1"}
    /// {"resourceType": "Observation", "id": "2"}
    /// {"resourceType": "Organization", "id": "3"}"#;
    ///
    /// let results = validator.validate_ndjson_any(ndjson)?;
    /// assert_eq!(results.len(), 3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn validate_ndjson_any(&self, ndjson: &str) -> Result<Vec<(usize, ValidationResult)>> {
        use rayon::prelude::*;

        // Clone config for thread-safe access
        let config = self.config().clone();

        // Parse lines with their line numbers
        let lines: Vec<(usize, String)> = ndjson
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    None
                } else {
                    Some((idx + 1, trimmed.to_string()))
                }
            })
            .collect();

        // Validate all lines in parallel
        lines
            .par_iter()
            .map(|(line_num, json)| {
                // Create a validator instance for this thread
                let validator = FhirValidator::with_config(config.clone())?;
                let result = validator.validate_any_resource(json)?;
                Ok((*line_num, result))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_resource_type() {
        let json = r#"{"resourceType": "Patient", "id": "123"}"#;
        assert_eq!(extract_resource_type(json).unwrap(), "Patient");
    }

    #[test]
    fn test_extract_resource_type_missing() {
        let json = r#"{"id": "123"}"#;
        assert!(extract_resource_type(json).is_err());
    }

    #[test]
    fn test_extract_resource_type_not_string() {
        let json = r#"{"resourceType": 123}"#;
        assert!(extract_resource_type(json).is_err());
    }

    #[test]
    fn test_suggest_resource_type_exact() {
        assert_eq!(
            suggest_resource_type("patient"),
            Some("Patient".to_string())
        );
        assert_eq!(
            suggest_resource_type("PATIENT"),
            Some("Patient".to_string())
        );
    }

    #[test]
    fn test_suggest_resource_type_typo() {
        assert_eq!(suggest_resource_type("Patint"), Some("Patient".to_string()));
        assert_eq!(
            suggest_resource_type("Observaton"),
            Some("Observation".to_string())
        );
    }

    #[test]
    fn test_suggest_resource_type_no_match() {
        assert_eq!(suggest_resource_type("CompletelyInvalid"), None);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("a", ""), 1);
        assert_eq!(levenshtein_distance("", "a"), 1);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "abd"), 1);
        assert_eq!(levenshtein_distance("Patient", "Patint"), 1);
    }
}
