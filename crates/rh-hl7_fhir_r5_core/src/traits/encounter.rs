use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::duration::Duration;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::virtual_service_detail::VirtualServiceDetail;
use crate::primitives::date_time::DateTimeType;
use crate::resources::encounter::EncounterAdmission;
use crate::resources::encounter::EncounterDiagnosis;
use crate::resources::encounter::EncounterLocation;
use crate::resources::encounter::EncounterParticipant;
use crate::resources::encounter::EncounterReason;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Encounter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EncounterStatus;
    /// Returns a reference to the class field.
    fn class(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> &[CodeableReference];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the subjectStatus field.
    fn subject_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the episodeOfCare field.
    fn episode_of_care(&self) -> &[Reference];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the careTeam field.
    fn care_team(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> Option<Reference>;
    /// Returns a reference to the serviceProvider field.
    fn service_provider(&self) -> Option<Reference>;
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[EncounterParticipant];
    /// Returns a reference to the appointment field.
    fn appointment(&self) -> &[Reference];
    /// Returns a reference to the virtualService field.
    fn virtual_service(&self) -> &[VirtualServiceDetail];
    /// Returns a reference to the actualPeriod field.
    fn actual_period(&self) -> Option<Period>;
    /// Returns a reference to the plannedStartDate field.
    fn planned_start_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the plannedEndDate field.
    fn planned_end_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the length field.
    fn length(&self) -> Option<Duration>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[EncounterReason];
    /// Returns a reference to the diagnosis field.
    fn diagnosis(&self) -> &[EncounterDiagnosis];
    /// Returns a reference to the account field.
    fn account(&self) -> &[Reference];
    /// Returns a reference to the dietPreference field.
    fn diet_preference(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialArrangement field.
    fn special_arrangement(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialCourtesy field.
    fn special_courtesy(&self) -> &[CodeableConcept];
    /// Returns a reference to the admission field.
    fn admission(&self) -> Option<EncounterAdmission>;
    /// Returns a reference to the location field.
    fn location(&self) -> &[EncounterLocation];
}
/// Encounter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::encounter::Encounter;
    /// use rh_hl7_fhir_r5_core::traits::encounter::EncounterMutators;
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
    /// Sets the class field and returns self for chaining.
    fn set_class(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the class field and returns self for chaining.
    fn add_class(self, item: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the serviceType field and returns self for chaining.
    fn set_service_type(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the serviceType field and returns self for chaining.
    fn add_service_type(self, item: CodeableReference) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the subjectStatus field and returns self for chaining.
    fn set_subject_status(self, value: CodeableConcept) -> Self;
    /// Sets the episodeOfCare field and returns self for chaining.
    fn set_episode_of_care(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the episodeOfCare field and returns self for chaining.
    fn add_episode_of_care(self, item: Reference) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the careTeam field and returns self for chaining.
    fn set_care_team(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the careTeam field and returns self for chaining.
    fn add_care_team(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Reference) -> Self;
    /// Sets the serviceProvider field and returns self for chaining.
    fn set_service_provider(self, value: Reference) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<EncounterParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: EncounterParticipant) -> Self;
    /// Sets the appointment field and returns self for chaining.
    fn set_appointment(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the appointment field and returns self for chaining.
    fn add_appointment(self, item: Reference) -> Self;
    /// Sets the virtualService field and returns self for chaining.
    fn set_virtual_service(self, value: Vec<VirtualServiceDetail>) -> Self;
    /// Adds an item to the virtualService field and returns self for chaining.
    fn add_virtual_service(self, item: VirtualServiceDetail) -> Self;
    /// Sets the actualPeriod field and returns self for chaining.
    fn set_actual_period(self, value: Period) -> Self;
    /// Sets the plannedStartDate field and returns self for chaining.
    fn set_planned_start_date(self, value: String) -> Self;
    /// Sets the plannedEndDate field and returns self for chaining.
    fn set_planned_end_date(self, value: String) -> Self;
    /// Sets the length field and returns self for chaining.
    fn set_length(self, value: Duration) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<EncounterReason>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: EncounterReason) -> Self;
    /// Sets the diagnosis field and returns self for chaining.
    fn set_diagnosis(self, value: Vec<EncounterDiagnosis>) -> Self;
    /// Adds an item to the diagnosis field and returns self for chaining.
    fn add_diagnosis(self, item: EncounterDiagnosis) -> Self;
    /// Sets the account field and returns self for chaining.
    fn set_account(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the account field and returns self for chaining.
    fn add_account(self, item: Reference) -> Self;
    /// Sets the dietPreference field and returns self for chaining.
    fn set_diet_preference(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the dietPreference field and returns self for chaining.
    fn add_diet_preference(self, item: CodeableConcept) -> Self;
    /// Sets the specialArrangement field and returns self for chaining.
    fn set_special_arrangement(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialArrangement field and returns self for chaining.
    fn add_special_arrangement(self, item: CodeableConcept) -> Self;
    /// Sets the specialCourtesy field and returns self for chaining.
    fn set_special_courtesy(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialCourtesy field and returns self for chaining.
    fn add_special_courtesy(self, item: CodeableConcept) -> Self;
    /// Sets the admission field and returns self for chaining.
    fn set_admission(self, value: EncounterAdmission) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<EncounterLocation>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: EncounterLocation) -> Self;
}
/// Encounter Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 5.0.0
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
    /// Returns true if the class field is not empty.
    fn has_class(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the service_type field is not empty.
    fn has_service_type(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the subject_status field is present (Some).
    fn has_subject_status(&self) -> bool;
    /// Returns true if the episode_of_care field is not empty.
    fn has_episode_of_care(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the care_team field is not empty.
    fn has_care_team(&self) -> bool;
    /// Returns true if the part_of field is present (Some).
    fn has_part_of(&self) -> bool;
    /// Returns true if the service_provider field is present (Some).
    fn has_service_provider(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the appointment field is not empty.
    fn has_appointment(&self) -> bool;
    /// Returns true if the virtual_service field is not empty.
    fn has_virtual_service(&self) -> bool;
    /// Returns true if the actual_period field is present (Some).
    fn has_actual_period(&self) -> bool;
    /// Returns true if the planned_start_date field is present (Some).
    fn has_planned_start_date(&self) -> bool;
    /// Returns true if the planned_end_date field is present (Some).
    fn has_planned_end_date(&self) -> bool;
    /// Returns true if the length field is present (Some).
    fn has_length(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the diagnosis field is not empty.
    fn has_diagnosis(&self) -> bool;
    /// Returns true if the account field is not empty.
    fn has_account(&self) -> bool;
    /// Returns true if the diet_preference field is not empty.
    fn has_diet_preference(&self) -> bool;
    /// Returns true if the special_arrangement field is not empty.
    fn has_special_arrangement(&self) -> bool;
    /// Returns true if the special_courtesy field is not empty.
    fn has_special_courtesy(&self) -> bool;
    /// Returns true if the admission field is present (Some).
    fn has_admission(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
}
