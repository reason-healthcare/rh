use crate::bindings::observation_status::ObservationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::risk_assessment::RiskAssessmentPrediction;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// RiskAssessment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskAssessmentAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> Option<Reference>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> Option<Reference>;
    /// Returns a reference to the status field.
    fn status(&self) -> ObservationStatus;
    /// Returns a reference to the method field.
    fn method(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the condition field.
    fn condition(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the basis field.
    fn basis(&self) -> &[Reference];
    /// Returns a reference to the prediction field.
    fn prediction(&self) -> &[RiskAssessmentPrediction];
    /// Returns a reference to the mitigation field.
    fn mitigation(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// RiskAssessment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskAssessmentMutators: DomainResourceMutators {
    /// Create a new RiskAssessment with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::risk_assessment::RiskAssessment;
    /// use hl7_fhir_r4_core::traits::risk_assessment::RiskAssessmentMutators;
    ///
    /// let resource = RiskAssessment::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Reference) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ObservationStatus) -> Self;
    /// Sets the method field and returns self for chaining.
    fn set_method(self, value: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the condition field and returns self for chaining.
    fn set_condition(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the basis field and returns self for chaining.
    fn set_basis(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basis field and returns self for chaining.
    fn add_basis(self, item: Reference) -> Self;
    /// Sets the prediction field and returns self for chaining.
    fn set_prediction(self, value: Vec<RiskAssessmentPrediction>) -> Self;
    /// Adds an item to the prediction field and returns self for chaining.
    fn add_prediction(self, item: RiskAssessmentPrediction) -> Self;
    /// Sets the mitigation field and returns self for chaining.
    fn set_mitigation(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// RiskAssessment Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RiskAssessmentExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is present (Some).
    fn has_based_on(&self) -> bool;
    /// Returns true if the parent field is present (Some).
    fn has_parent(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the method field is present (Some).
    fn has_method(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the condition field is present (Some).
    fn has_condition(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the basis field is not empty.
    fn has_basis(&self) -> bool;
    /// Returns true if the prediction field is not empty.
    fn has_prediction(&self) -> bool;
    /// Returns true if the mitigation field is present (Some).
    fn has_mitigation(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
