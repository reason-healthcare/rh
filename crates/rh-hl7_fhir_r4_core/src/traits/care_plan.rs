use crate::bindings::care_plan_intent::CarePlanIntent;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::care_plan::CarePlanActivity;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CarePlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CarePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CarePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CarePlanAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the replaces field.
    fn replaces(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> RequestStatus;
    /// Returns a reference to the intent field.
    fn intent(&self) -> CarePlanIntent;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateTimeType>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the contributor field.
    fn contributor(&self) -> &[Reference];
    /// Returns a reference to the careTeam field.
    fn care_team(&self) -> &[Reference];
    /// Returns a reference to the addresses field.
    fn addresses(&self) -> &[Reference];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
    /// Returns a reference to the goal field.
    fn goal(&self) -> &[Reference];
    /// Returns a reference to the activity field.
    fn activity(&self) -> &[CarePlanActivity];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// CarePlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CarePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CarePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CarePlanMutators: DomainResourceMutators {
    /// Create a new CarePlan with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::care_plan::CarePlan;
    /// use hl7_fhir_r4_core::traits::care_plan::CarePlanMutators;
    ///
    /// let resource = CarePlan::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesCanonical field and returns self for chaining.
    fn add_instantiates_canonical(self, item: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesUri field and returns self for chaining.
    fn add_instantiates_uri(self, item: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the replaces field and returns self for chaining.
    fn set_replaces(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the replaces field and returns self for chaining.
    fn add_replaces(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: RequestStatus) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: CarePlanIntent) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the contributor field and returns self for chaining.
    fn set_contributor(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the contributor field and returns self for chaining.
    fn add_contributor(self, item: Reference) -> Self;
    /// Sets the careTeam field and returns self for chaining.
    fn set_care_team(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the careTeam field and returns self for chaining.
    fn add_care_team(self, item: Reference) -> Self;
    /// Sets the addresses field and returns self for chaining.
    fn set_addresses(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the addresses field and returns self for chaining.
    fn add_addresses(self, item: Reference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
    /// Sets the goal field and returns self for chaining.
    fn set_goal(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the goal field and returns self for chaining.
    fn add_goal(self, item: Reference) -> Self;
    /// Sets the activity field and returns self for chaining.
    fn set_activity(self, value: Vec<CarePlanActivity>) -> Self;
    /// Adds an item to the activity field and returns self for chaining.
    fn add_activity(self, item: CarePlanActivity) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// CarePlan Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CarePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CarePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CarePlanExistence: DomainResourceExistence {
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
    /// Returns true if the instantiates_canonical field is not empty.
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is not empty.
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the replaces field is not empty.
    fn has_replaces(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the contributor field is not empty.
    fn has_contributor(&self) -> bool;
    /// Returns true if the care_team field is not empty.
    fn has_care_team(&self) -> bool;
    /// Returns true if the addresses field is not empty.
    fn has_addresses(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the goal field is not empty.
    fn has_goal(&self) -> bool;
    /// Returns true if the activity field is not empty.
    fn has_activity(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
