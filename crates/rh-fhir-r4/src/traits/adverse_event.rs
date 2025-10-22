use crate::bindings::adverse_event_actuality::AdverseEventActuality;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::adverse_event::AdverseEventSuspectentity;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AdverseEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdverseEventAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the actuality field.
    fn actuality(&self) -> AdverseEventActuality;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the event field.
    fn event(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the detected field.
    fn detected(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recordedDate field.
    fn recorded_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the resultingCondition field.
    fn resulting_condition(&self) -> &[Reference];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the seriousness field.
    fn seriousness(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the severity field.
    fn severity(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the contributor field.
    fn contributor(&self) -> &[Reference];
    /// Returns a reference to the suspectEntity field.
    fn suspect_entity(&self) -> &[AdverseEventSuspectentity];
    /// Returns a reference to the subjectMedicalHistory field.
    fn subject_medical_history(&self) -> &[Reference];
    /// Returns a reference to the referenceDocument field.
    fn reference_document(&self) -> &[Reference];
    /// Returns a reference to the study field.
    fn study(&self) -> &[Reference];
}
/// AdverseEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdverseEventMutators: DomainResourceMutators {
    /// Create a new AdverseEvent with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::adverse_event::AdverseEvent;
    /// use hl7_fhir_r4_core::traits::adverse_event::AdverseEventMutators;
    ///
    /// let resource = AdverseEvent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the actuality field and returns self for chaining.
    fn set_actuality(self, value: AdverseEventActuality) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the event field and returns self for chaining.
    fn set_event(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the detected field and returns self for chaining.
    fn set_detected(self, value: String) -> Self;
    /// Sets the recordedDate field and returns self for chaining.
    fn set_recorded_date(self, value: String) -> Self;
    /// Sets the resultingCondition field and returns self for chaining.
    fn set_resulting_condition(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the resultingCondition field and returns self for chaining.
    fn add_resulting_condition(self, item: Reference) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the seriousness field and returns self for chaining.
    fn set_seriousness(self, value: CodeableConcept) -> Self;
    /// Sets the severity field and returns self for chaining.
    fn set_severity(self, value: CodeableConcept) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: CodeableConcept) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the contributor field and returns self for chaining.
    fn set_contributor(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the contributor field and returns self for chaining.
    fn add_contributor(self, item: Reference) -> Self;
    /// Sets the suspectEntity field and returns self for chaining.
    fn set_suspect_entity(self, value: Vec<AdverseEventSuspectentity>) -> Self;
    /// Adds an item to the suspectEntity field and returns self for chaining.
    fn add_suspect_entity(self, item: AdverseEventSuspectentity) -> Self;
    /// Sets the subjectMedicalHistory field and returns self for chaining.
    fn set_subject_medical_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subjectMedicalHistory field and returns self for chaining.
    fn add_subject_medical_history(self, item: Reference) -> Self;
    /// Sets the referenceDocument field and returns self for chaining.
    fn set_reference_document(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the referenceDocument field and returns self for chaining.
    fn add_reference_document(self, item: Reference) -> Self;
    /// Sets the study field and returns self for chaining.
    fn set_study(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the study field and returns self for chaining.
    fn add_study(self, item: Reference) -> Self;
}
/// AdverseEvent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdverseEventExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the actuality field is present (Some).
    fn has_actuality(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the event field is present (Some).
    fn has_event(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the detected field is present (Some).
    fn has_detected(&self) -> bool;
    /// Returns true if the recorded_date field is present (Some).
    fn has_recorded_date(&self) -> bool;
    /// Returns true if the resulting_condition field is not empty.
    fn has_resulting_condition(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the seriousness field is present (Some).
    fn has_seriousness(&self) -> bool;
    /// Returns true if the severity field is present (Some).
    fn has_severity(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the contributor field is not empty.
    fn has_contributor(&self) -> bool;
    /// Returns true if the suspect_entity field is not empty.
    fn has_suspect_entity(&self) -> bool;
    /// Returns true if the subject_medical_history field is not empty.
    fn has_subject_medical_history(&self) -> bool;
    /// Returns true if the reference_document field is not empty.
    fn has_reference_document(&self) -> bool;
    /// Returns true if the study field is not empty.
    fn has_study(&self) -> bool;
}
