use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::duration::Duration;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::resources::encounter::EncounterClasshistory;
use crate::resources::encounter::EncounterDiagnosis;
use crate::resources::encounter::EncounterHospitalization;
use crate::resources::encounter::EncounterLocation;
use crate::resources::encounter::EncounterParticipant;
use crate::resources::encounter::EncounterStatushistory;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Encounter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EncounterStatus;
    /// Returns a reference to the statusHistory field.
    fn status_history(&self) -> &[EncounterStatushistory];
    /// Returns a reference to the class field.
    fn class(&self) -> Coding;
    /// Returns a reference to the classHistory field.
    fn class_history(&self) -> &[EncounterClasshistory];
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the episodeOfCare field.
    fn episode_of_care(&self) -> &[Reference];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[EncounterParticipant];
    /// Returns a reference to the appointment field.
    fn appointment(&self) -> &[Reference];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the length field.
    fn length(&self) -> Option<Duration>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the diagnosis field.
    fn diagnosis(&self) -> &[EncounterDiagnosis];
    /// Returns a reference to the account field.
    fn account(&self) -> &[Reference];
    /// Returns a reference to the hospitalization field.
    fn hospitalization(&self) -> Option<EncounterHospitalization>;
    /// Returns a reference to the location field.
    fn location(&self) -> &[EncounterLocation];
    /// Returns a reference to the serviceProvider field.
    fn service_provider(&self) -> Option<Reference>;
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> Option<Reference>;
}
/// Encounter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterMutators: DomainResourceMutators {
    /// Create a new Encounter with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::encounter::Encounter;
    /// use hl7_fhir_r4_core::traits::encounter::EncounterMutators;
    ///
    /// let resource = Encounter::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EncounterStatus) -> Self;
    /// Sets the statusHistory field and returns self for chaining.
    fn set_status_history(self, value: Vec<EncounterStatushistory>) -> Self;
    /// Adds an item to the statusHistory field and returns self for chaining.
    fn add_status_history(self, item: EncounterStatushistory) -> Self;
    /// Sets the class field and returns self for chaining.
    fn set_class(self, value: Coding) -> Self;
    /// Sets the classHistory field and returns self for chaining.
    fn set_class_history(self, value: Vec<EncounterClasshistory>) -> Self;
    /// Adds an item to the classHistory field and returns self for chaining.
    fn add_class_history(self, item: EncounterClasshistory) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the serviceType field and returns self for chaining.
    fn set_service_type(self, value: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the episodeOfCare field and returns self for chaining.
    fn set_episode_of_care(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the episodeOfCare field and returns self for chaining.
    fn add_episode_of_care(self, item: Reference) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<EncounterParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: EncounterParticipant) -> Self;
    /// Sets the appointment field and returns self for chaining.
    fn set_appointment(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the appointment field and returns self for chaining.
    fn add_appointment(self, item: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the length field and returns self for chaining.
    fn set_length(self, value: Duration) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the diagnosis field and returns self for chaining.
    fn set_diagnosis(self, value: Vec<EncounterDiagnosis>) -> Self;
    /// Adds an item to the diagnosis field and returns self for chaining.
    fn add_diagnosis(self, item: EncounterDiagnosis) -> Self;
    /// Sets the account field and returns self for chaining.
    fn set_account(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the account field and returns self for chaining.
    fn add_account(self, item: Reference) -> Self;
    /// Sets the hospitalization field and returns self for chaining.
    fn set_hospitalization(self, value: EncounterHospitalization) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<EncounterLocation>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: EncounterLocation) -> Self;
    /// Sets the serviceProvider field and returns self for chaining.
    fn set_service_provider(self, value: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Reference) -> Self;
}
/// Encounter Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterExistence: DomainResourceExistence {
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
    /// Returns true if the status_history field is not empty.
    fn has_status_history(&self) -> bool;
    /// Returns true if the class field is present (Some).
    fn has_class(&self) -> bool;
    /// Returns true if the class_history field is not empty.
    fn has_class_history(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the service_type field is present (Some).
    fn has_service_type(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the episode_of_care field is not empty.
    fn has_episode_of_care(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the appointment field is not empty.
    fn has_appointment(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the length field is present (Some).
    fn has_length(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the diagnosis field is not empty.
    fn has_diagnosis(&self) -> bool;
    /// Returns true if the account field is not empty.
    fn has_account(&self) -> bool;
    /// Returns true if the hospitalization field is present (Some).
    fn has_hospitalization(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
    /// Returns true if the service_provider field is present (Some).
    fn has_service_provider(&self) -> bool;
    /// Returns true if the part_of field is present (Some).
    fn has_part_of(&self) -> bool;
}
