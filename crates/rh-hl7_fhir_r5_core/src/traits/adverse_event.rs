use crate::bindings::adverse_event_actuality::AdverseEventActuality;
use crate::bindings::adverse_event_status::AdverseEventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::adverse_event::AdverseEventContributingfactor;
use crate::resources::adverse_event::AdverseEventMitigatingaction;
use crate::resources::adverse_event::AdverseEventParticipant;
use crate::resources::adverse_event::AdverseEventPreventiveaction;
use crate::resources::adverse_event::AdverseEventSupportinginfo;
use crate::resources::adverse_event::AdverseEventSuspectentity;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AdverseEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdverseEventAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> AdverseEventStatus;
    /// Returns a reference to the actuality field.
    fn actuality(&self) -> AdverseEventActuality;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the detected field.
    fn detected(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recordedDate field.
    fn recorded_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the resultingEffect field.
    fn resulting_effect(&self) -> &[Reference];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the seriousness field.
    fn seriousness(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> &[CodeableConcept];
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[AdverseEventParticipant];
    /// Returns a reference to the study field.
    fn study(&self) -> &[Reference];
    /// Returns a reference to the expectedInResearchStudy field.
    fn expected_in_research_study(&self) -> Option<BooleanType>;
    /// Returns a reference to the suspectEntity field.
    fn suspect_entity(&self) -> &[AdverseEventSuspectentity];
    /// Returns a reference to the contributingFactor field.
    fn contributing_factor(&self) -> &[AdverseEventContributingfactor];
    /// Returns a reference to the preventiveAction field.
    fn preventive_action(&self) -> &[AdverseEventPreventiveaction];
    /// Returns a reference to the mitigatingAction field.
    fn mitigating_action(&self) -> &[AdverseEventMitigatingaction];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[AdverseEventSupportinginfo];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// AdverseEvent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::adverse_event::AdverseEvent;
    /// use rh_hl7_fhir_r5_core::traits::adverse_event::AdverseEventMutators;
    ///
    /// let resource = AdverseEvent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: AdverseEventStatus) -> Self;
    /// Sets the actuality field and returns self for chaining.
    fn set_actuality(self, value: AdverseEventActuality) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the detected field and returns self for chaining.
    fn set_detected(self, value: String) -> Self;
    /// Sets the recordedDate field and returns self for chaining.
    fn set_recorded_date(self, value: String) -> Self;
    /// Sets the resultingEffect field and returns self for chaining.
    fn set_resulting_effect(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the resultingEffect field and returns self for chaining.
    fn add_resulting_effect(self, item: Reference) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the seriousness field and returns self for chaining.
    fn set_seriousness(self, value: CodeableConcept) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the outcome field and returns self for chaining.
    fn add_outcome(self, item: CodeableConcept) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<AdverseEventParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: AdverseEventParticipant) -> Self;
    /// Sets the study field and returns self for chaining.
    fn set_study(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the study field and returns self for chaining.
    fn add_study(self, item: Reference) -> Self;
    /// Sets the expectedInResearchStudy field and returns self for chaining.
    fn set_expected_in_research_study(self, value: bool) -> Self;
    /// Sets the suspectEntity field and returns self for chaining.
    fn set_suspect_entity(self, value: Vec<AdverseEventSuspectentity>) -> Self;
    /// Adds an item to the suspectEntity field and returns self for chaining.
    fn add_suspect_entity(self, item: AdverseEventSuspectentity) -> Self;
    /// Sets the contributingFactor field and returns self for chaining.
    fn set_contributing_factor(self, value: Vec<AdverseEventContributingfactor>) -> Self;
    /// Adds an item to the contributingFactor field and returns self for chaining.
    fn add_contributing_factor(self, item: AdverseEventContributingfactor) -> Self;
    /// Sets the preventiveAction field and returns self for chaining.
    fn set_preventive_action(self, value: Vec<AdverseEventPreventiveaction>) -> Self;
    /// Adds an item to the preventiveAction field and returns self for chaining.
    fn add_preventive_action(self, item: AdverseEventPreventiveaction) -> Self;
    /// Sets the mitigatingAction field and returns self for chaining.
    fn set_mitigating_action(self, value: Vec<AdverseEventMitigatingaction>) -> Self;
    /// Adds an item to the mitigatingAction field and returns self for chaining.
    fn add_mitigating_action(self, item: AdverseEventMitigatingaction) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<AdverseEventSupportinginfo>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: AdverseEventSupportinginfo) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// AdverseEvent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AdverseEventExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the actuality field is present (Some).
    fn has_actuality(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the detected field is present (Some).
    fn has_detected(&self) -> bool;
    /// Returns true if the recorded_date field is present (Some).
    fn has_recorded_date(&self) -> bool;
    /// Returns true if the resulting_effect field is not empty.
    fn has_resulting_effect(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the seriousness field is present (Some).
    fn has_seriousness(&self) -> bool;
    /// Returns true if the outcome field is not empty.
    fn has_outcome(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the study field is not empty.
    fn has_study(&self) -> bool;
    /// Returns true if the expected_in_research_study field is present (Some).
    fn has_expected_in_research_study(&self) -> bool;
    /// Returns true if the suspect_entity field is not empty.
    fn has_suspect_entity(&self) -> bool;
    /// Returns true if the contributing_factor field is not empty.
    fn has_contributing_factor(&self) -> bool;
    /// Returns true if the preventive_action field is not empty.
    fn has_preventive_action(&self) -> bool;
    /// Returns true if the mitigating_action field is not empty.
    fn has_mitigating_action(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
