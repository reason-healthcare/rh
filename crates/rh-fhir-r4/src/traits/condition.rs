use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::condition::ConditionEvidence;
use crate::resources::condition::ConditionStage;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Condition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Condition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Condition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the clinicalStatus field.
    fn clinical_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the verificationStatus field.
    fn verification_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the severity field.
    fn severity(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recordedDate field.
    fn recorded_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the asserter field.
    fn asserter(&self) -> Option<Reference>;
    /// Returns a reference to the stage field.
    fn stage(&self) -> &[ConditionStage];
    /// Returns a reference to the evidence field.
    fn evidence(&self) -> &[ConditionEvidence];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// Condition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Condition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Condition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionMutators: DomainResourceMutators {
    /// Create a new Condition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::condition::Condition;
    /// use hl7_fhir_r4_core::traits::condition::ConditionMutators;
    ///
    /// let resource = Condition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the clinicalStatus field and returns self for chaining.
    fn set_clinical_status(self, value: CodeableConcept) -> Self;
    /// Sets the verificationStatus field and returns self for chaining.
    fn set_verification_status(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the severity field and returns self for chaining.
    fn set_severity(self, value: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the bodySite field and returns self for chaining.
    fn add_body_site(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recordedDate field and returns self for chaining.
    fn set_recorded_date(self, value: String) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the asserter field and returns self for chaining.
    fn set_asserter(self, value: Reference) -> Self;
    /// Sets the stage field and returns self for chaining.
    fn set_stage(self, value: Vec<ConditionStage>) -> Self;
    /// Adds an item to the stage field and returns self for chaining.
    fn add_stage(self, item: ConditionStage) -> Self;
    /// Sets the evidence field and returns self for chaining.
    fn set_evidence(self, value: Vec<ConditionEvidence>) -> Self;
    /// Adds an item to the evidence field and returns self for chaining.
    fn add_evidence(self, item: ConditionEvidence) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// Condition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Condition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Condition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionExistence: DomainResourceExistence {
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
    /// Returns true if the clinical_status field is present (Some).
    fn has_clinical_status(&self) -> bool;
    /// Returns true if the verification_status field is present (Some).
    fn has_verification_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the severity field is present (Some).
    fn has_severity(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the onset field is present (Some).
    fn has_onset(&self) -> bool;
    /// Returns true if the abatement field is present (Some).
    fn has_abatement(&self) -> bool;
    /// Returns true if the recorded_date field is present (Some).
    fn has_recorded_date(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the asserter field is present (Some).
    fn has_asserter(&self) -> bool;
    /// Returns true if the stage field is not empty.
    fn has_stage(&self) -> bool;
    /// Returns true if the evidence field is not empty.
    fn has_evidence(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
