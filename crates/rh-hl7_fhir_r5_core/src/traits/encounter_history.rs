use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::duration::Duration;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::encounter_history::EncounterHistoryLocation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// EncounterHistory Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of significant events/milestones key data throughout the history of an Encounter
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EncounterHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterHistoryAccessors: DomainResourceAccessors {
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EncounterStatus;
    /// Returns a reference to the class field.
    fn class(&self) -> CodeableConcept;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> &[CodeableReference];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the subjectStatus field.
    fn subject_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the actualPeriod field.
    fn actual_period(&self) -> Option<Period>;
    /// Returns a reference to the plannedStartDate field.
    fn planned_start_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the plannedEndDate field.
    fn planned_end_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the length field.
    fn length(&self) -> Option<Duration>;
    /// Returns a reference to the location field.
    fn location(&self) -> &[EncounterHistoryLocation];
}
/// EncounterHistory Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of significant events/milestones key data throughout the history of an Encounter
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EncounterHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterHistoryMutators: DomainResourceMutators {
    /// Create a new EncounterHistory with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::encounter_history::EncounterHistory;
    /// use rh_hl7_fhir_r5_core::traits::encounter_history::EncounterHistoryMutators;
    ///
    /// let resource = EncounterHistory::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EncounterStatus) -> Self;
    /// Sets the class field and returns self for chaining.
    fn set_class(self, value: CodeableConcept) -> Self;
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
    /// Sets the actualPeriod field and returns self for chaining.
    fn set_actual_period(self, value: Period) -> Self;
    /// Sets the plannedStartDate field and returns self for chaining.
    fn set_planned_start_date(self, value: String) -> Self;
    /// Sets the plannedEndDate field and returns self for chaining.
    fn set_planned_end_date(self, value: String) -> Self;
    /// Sets the length field and returns self for chaining.
    fn set_length(self, value: Duration) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<EncounterHistoryLocation>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: EncounterHistoryLocation) -> Self;
}
/// EncounterHistory Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of significant events/milestones key data throughout the history of an Encounter
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EncounterHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EncounterHistoryExistence: DomainResourceExistence {
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the class field is present (Some).
    fn has_class(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the service_type field is not empty.
    fn has_service_type(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the subject_status field is present (Some).
    fn has_subject_status(&self) -> bool;
    /// Returns true if the actual_period field is present (Some).
    fn has_actual_period(&self) -> bool;
    /// Returns true if the planned_start_date field is present (Some).
    fn has_planned_start_date(&self) -> bool;
    /// Returns true if the planned_end_date field is present (Some).
    fn has_planned_end_date(&self) -> bool;
    /// Returns true if the length field is present (Some).
    fn has_length(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
}
