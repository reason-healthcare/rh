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
            ("Immunization", "resource"),
            ("CarePlan", "resource"),
            ("CareTeam", "resource"),
            ("Goal", "resource"),
            ("Device", "resource"),
            ("Organization", "resource"),
            ("Practitioner", "resource"),
            ("PractitionerRole", "resource"),
            ("Location", "resource"),
            ("Medication", "resource"),
            ("Substance", "resource"),
            ("ServiceRequest", "resource"),
            ("DocumentReference", "resource"),
            ("Composition", "resource"),
            ("Bundle", "resource"),
            ("List", "resource"),
            ("RelatedPerson", "resource"),
            ("Coverage", "resource"),
            ("Claim", "resource"),
            ("ExplanationOfBenefit", "resource"),
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
