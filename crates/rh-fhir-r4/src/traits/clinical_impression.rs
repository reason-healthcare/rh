use crate::bindings::clinicalimpression_status::ClinicalimpressionStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::clinical_impression::ClinicalImpressionFinding;
use crate::resources::clinical_impression::ClinicalImpressionInvestigation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ClinicalImpression Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion with the recording of assessment tools such as Apgar score.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClinicalImpression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalImpressionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ClinicalimpressionStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the assessor field.
    fn assessor(&self) -> Option<Reference>;
    /// Returns a reference to the previous field.
    fn previous(&self) -> Option<Reference>;
    /// Returns a reference to the problem field.
    fn problem(&self) -> &[Reference];
    /// Returns a reference to the investigation field.
    fn investigation(&self) -> &[ClinicalImpressionInvestigation];
    /// Returns a reference to the protocol field.
    fn protocol(&self) -> &[StringType];
    /// Returns a reference to the summary field.
    fn summary(&self) -> Option<StringType>;
    /// Returns a reference to the finding field.
    fn finding(&self) -> &[ClinicalImpressionFinding];
    /// Returns a reference to the prognosisCodeableConcept field.
    fn prognosis_codeable_concept(&self) -> &[CodeableConcept];
    /// Returns a reference to the prognosisReference field.
    fn prognosis_reference(&self) -> &[Reference];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// ClinicalImpression Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion with the recording of assessment tools such as Apgar score.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClinicalImpression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalImpressionMutators: DomainResourceMutators {
    /// Create a new ClinicalImpression with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::clinical_impression::ClinicalImpression;
    /// use hl7_fhir_r4_core::traits::clinical_impression::ClinicalImpressionMutators;
    ///
    /// let resource = ClinicalImpression::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ClinicalimpressionStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the assessor field and returns self for chaining.
    fn set_assessor(self, value: Reference) -> Self;
    /// Sets the previous field and returns self for chaining.
    fn set_previous(self, value: Reference) -> Self;
    /// Sets the problem field and returns self for chaining.
    fn set_problem(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the problem field and returns self for chaining.
    fn add_problem(self, item: Reference) -> Self;
    /// Sets the investigation field and returns self for chaining.
    fn set_investigation(self, value: Vec<ClinicalImpressionInvestigation>) -> Self;
    /// Adds an item to the investigation field and returns self for chaining.
    fn add_investigation(self, item: ClinicalImpressionInvestigation) -> Self;
    /// Sets the protocol field and returns self for chaining.
    fn set_protocol(self, value: Vec<String>) -> Self;
    /// Adds an item to the protocol field and returns self for chaining.
    fn add_protocol(self, item: String) -> Self;
    /// Sets the summary field and returns self for chaining.
    fn set_summary(self, value: String) -> Self;
    /// Sets the finding field and returns self for chaining.
    fn set_finding(self, value: Vec<ClinicalImpressionFinding>) -> Self;
    /// Adds an item to the finding field and returns self for chaining.
    fn add_finding(self, item: ClinicalImpressionFinding) -> Self;
    /// Sets the prognosisCodeableConcept field and returns self for chaining.
    fn set_prognosis_codeable_concept(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the prognosisCodeableConcept field and returns self for chaining.
    fn add_prognosis_codeable_concept(self, item: CodeableConcept) -> Self;
    /// Sets the prognosisReference field and returns self for chaining.
    fn set_prognosis_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the prognosisReference field and returns self for chaining.
    fn add_prognosis_reference(self, item: Reference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// ClinicalImpression Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion with the recording of assessment tools such as Apgar score.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClinicalImpression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClinicalImpressionExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the effective field is present (Some).
    fn has_effective(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the assessor field is present (Some).
    fn has_assessor(&self) -> bool;
    /// Returns true if the previous field is present (Some).
    fn has_previous(&self) -> bool;
    /// Returns true if the problem field is not empty.
    fn has_problem(&self) -> bool;
    /// Returns true if the investigation field is not empty.
    fn has_investigation(&self) -> bool;
    /// Returns true if the protocol field is not empty.
    fn has_protocol(&self) -> bool;
    /// Returns true if the summary field is present (Some).
    fn has_summary(&self) -> bool;
    /// Returns true if the finding field is not empty.
    fn has_finding(&self) -> bool;
    /// Returns true if the prognosis_codeable_concept field is not empty.
    fn has_prognosis_codeable_concept(&self) -> bool;
    /// Returns true if the prognosis_reference field is not empty.
    fn has_prognosis_reference(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
