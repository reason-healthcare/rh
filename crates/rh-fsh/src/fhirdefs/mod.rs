//! FHIR R4 definitions lookup

use rh_hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType};
use std::collections::HashMap;
use std::sync::Arc;

/// Summary of a StructureDefinition
pub struct SdSummary {
    pub id: String,
    pub url: String,
    pub base_type: String,
    pub kind: String,
}

/// Summary of an element within a type
pub struct ElementSummary {
    pub path: String,
    pub types: Vec<String>,
    pub min: u32,
    pub max: Option<u32>,
    pub is_choice: bool,
}

/// Registry of known FHIR R4 type definitions
pub struct FhirDefs {
    sds_by_name: HashMap<String, SdSummary>,
    sds_by_url: HashMap<String, SdSummary>,
}

impl FhirDefs {
    /// Build the R4 definitions registry
    pub fn r4() -> Arc<Self> {
        let resource_names: Vec<(&str, &str)> = vec![
            ("Patient", "resource"),
            ("Observation", "resource"),
            ("Condition", "resource"),
            ("Encounter", "resource"),
            ("Procedure", "resource"),
            ("MedicationRequest", "resource"),
            ("DiagnosticReport", "resource"),
            ("AllergyIntolerance", "resource"),
            ("Appointment", "resource"),
            ("AppointmentResponse", "resource"),
            ("AuditEvent", "resource"),
            ("Basic", "resource"),
            ("Binary", "resource"),
            ("BiologicallyDerivedProduct", "resource"),
            ("BodyStructure", "resource"),
            ("Immunization", "resource"),
            ("CarePlan", "resource"),
            ("CareTeam", "resource"),
            ("CatalogEntry", "resource"),
            ("ChargeItem", "resource"),
            ("ChargeItemDefinition", "resource"),
            ("ClinicalImpression", "resource"),
            ("Communication", "resource"),
            ("CommunicationRequest", "resource"),
            ("CompartmentDefinition", "resource"),
            ("Consent", "resource"),
            ("Contract", "resource"),
            ("Goal", "resource"),
            ("Device", "resource"),
            ("DeviceDefinition", "resource"),
            ("DeviceMetric", "resource"),
            ("DeviceRequest", "resource"),
            ("DeviceUseStatement", "resource"),
            ("Organization", "resource"),
            ("OrganizationAffiliation", "resource"),
            ("Practitioner", "resource"),
            ("PractitionerRole", "resource"),
            ("Location", "resource"),
            ("Medication", "resource"),
            ("MedicationAdministration", "resource"),
            ("MedicationDispense", "resource"),
            ("MedicationStatement", "resource"),
            ("Substance", "resource"),
            ("ServiceRequest", "resource"),
            ("SupplyDelivery", "resource"),
            ("SupplyRequest", "resource"),
            ("Task", "resource"),
            ("TerminologyCapabilities", "resource"),
            ("TestReport", "resource"),
            ("TestScript", "resource"),
            ("DocumentReference", "resource"),
            ("Composition", "resource"),
            ("Bundle", "resource"),
            ("List", "resource"),
            ("RelatedPerson", "resource"),
            ("Person", "resource"),
            ("Coverage", "resource"),
            ("CoverageEligibilityRequest", "resource"),
            ("CoverageEligibilityResponse", "resource"),
            ("Claim", "resource"),
            ("ClaimResponse", "resource"),
            ("DetectedIssue", "resource"),
            ("EnrollmentRequest", "resource"),
            ("EnrollmentResponse", "resource"),
            ("EpisodeOfCare", "resource"),
            ("EventDefinition", "resource"),
            ("ExplanationOfBenefit", "resource"),
            ("FamilyMemberHistory", "resource"),
            ("Flag", "resource"),
            ("GraphDefinition", "resource"),
            ("Group", "resource"),
            ("GuidanceResponse", "resource"),
            ("HealthcareService", "resource"),
            ("ImagingStudy", "resource"),
            ("InsurancePlan", "resource"),
            ("Invoice", "resource"),
            ("Library", "resource"),
            ("Linkage", "resource"),
            ("Measure", "resource"),
            ("MeasureReport", "resource"),
            ("Media", "resource"),
            ("MolecularSequence", "resource"),
            ("NutritionOrder", "resource"),
            ("ObservationDefinition", "resource"),
            ("OperationOutcome", "resource"),
            ("Parameters", "resource"),
            ("PaymentNotice", "resource"),
            ("PaymentReconciliation", "resource"),
            ("PlanDefinition", "resource"),
            ("Provenance", "resource"),
            ("Questionnaire", "resource"),
            ("QuestionnaireResponse", "resource"),
            ("RequestGroup", "resource"),
            ("ResearchDefinition", "resource"),
            ("ResearchElementDefinition", "resource"),
            ("ResearchStudy", "resource"),
            ("ResearchSubject", "resource"),
            ("RiskAssessment", "resource"),
            ("Schedule", "resource"),
            ("Slot", "resource"),
            ("Specimen", "resource"),
            ("SpecimenDefinition", "resource"),
            ("Subscription", "resource"),
            ("SubstanceNucleicAcid", "resource"),
            ("SubstancePolymer", "resource"),
            ("SubstanceProtein", "resource"),
            ("SubstanceReferenceInformation", "resource"),
            ("SubstanceSourceMaterial", "resource"),
            ("SubstanceSpecification", "resource"),
            ("VerificationResult", "resource"),
            ("VisionPrescription", "resource"),
            ("StructureDefinition", "resource"),
            ("ValueSet", "resource"),
            ("CodeSystem", "resource"),
            ("ConceptMap", "resource"),
            ("CapabilityStatement", "resource"),
            ("ImplementationGuide", "resource"),
            ("SearchParameter", "resource"),
            ("OperationDefinition", "resource"),
            ("MessageDefinition", "resource"),
            ("NamingSystem", "resource"),
            ("DomainResource", "resource"),
            ("Resource", "resource"),
            ("Extension", "complex-type"),
            ("Element", "complex-type"),
            ("Identifier", "complex-type"),
            ("HumanName", "complex-type"),
            ("Address", "complex-type"),
            ("ContactPoint", "complex-type"),
            ("Coding", "complex-type"),
            ("CodeableConcept", "complex-type"),
            ("Quantity", "complex-type"),
            ("Range", "complex-type"),
            ("Ratio", "complex-type"),
            ("Period", "complex-type"),
            ("Timing", "complex-type"),
            ("Annotation", "complex-type"),
            ("Attachment", "complex-type"),
            ("Reference", "complex-type"),
            ("Narrative", "complex-type"),
            ("Meta", "complex-type"),
        ];

        let mut sds_by_name = HashMap::new();
        let mut sds_by_url = HashMap::new();

        for (name, kind) in resource_names {
            let url = format!("http://hl7.org/fhir/StructureDefinition/{name}");
            let summary = SdSummary {
                id: name.to_ascii_lowercase(),
                url: url.clone(),
                base_type: name.to_string(),
                kind: kind.to_string(),
            };
            sds_by_name.insert(name.to_string(), summary);
            let summary2 = SdSummary {
                id: name.to_ascii_lowercase(),
                url: url.clone(),
                base_type: name.to_string(),
                kind: kind.to_string(),
            };
            sds_by_url.insert(url, summary2);
        }

        for (id, name, base_type, kind) in [
            (
                "bodyheight",
                "observation-bodyheight",
                "Observation",
                "resource",
            ),
            (
                "bodyweight",
                "observation-bodyweight",
                "Observation",
                "resource",
            ),
            (
                "bodytemp",
                "observation-bodytemp",
                "Observation",
                "resource",
            ),
        ] {
            let url = format!("http://hl7.org/fhir/StructureDefinition/{id}");
            sds_by_name.insert(
                name.to_string(),
                SdSummary {
                    id: id.to_string(),
                    url: url.clone(),
                    base_type: base_type.to_string(),
                    kind: kind.to_string(),
                },
            );
            sds_by_url.insert(
                url,
                SdSummary {
                    id: id.to_string(),
                    url: format!("http://hl7.org/fhir/StructureDefinition/{id}"),
                    base_type: base_type.to_string(),
                    kind: kind.to_string(),
                },
            );
        }

        Arc::new(Self {
            sds_by_name,
            sds_by_url,
        })
    }

    /// Get a StructureDefinition summary by name or URL
    pub fn get_sd(&self, name_or_url: &str) -> Option<&SdSummary> {
        self.sds_by_name
            .get(name_or_url)
            .or_else(|| self.sds_by_url.get(name_or_url))
    }

    /// Get element info for a field within a type
    pub fn get_element(&self, type_name: &str, path: &str) -> Option<ElementSummary> {
        let field = get_field_info(type_name, path)?;
        let types = match &field.field_type {
            FhirFieldType::Primitive(p) => vec![format!("{p:?}").to_lowercase()],
            FhirFieldType::Complex(t) => vec![t.to_string()],
            FhirFieldType::Reference => vec!["Reference".to_string()],
            FhirFieldType::BackboneElement(t) => vec![t.to_string()],
        };
        Some(ElementSummary {
            path: format!("{type_name}.{path}"),
            types,
            min: field.min,
            max: field.max,
            is_choice: field.is_choice_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_common_r4_observation_profile_urls() {
        let defs = FhirDefs::r4();
        let bodyheight = defs
            .get_sd("http://hl7.org/fhir/StructureDefinition/bodyheight")
            .expect("bodyheight profile is indexed");

        assert_eq!(bodyheight.base_type, "Observation");
        assert_eq!(
            defs.get_sd("observation-bodyweight")
                .map(|sd| sd.base_type.as_str()),
            Some("Observation")
        );
    }
}
