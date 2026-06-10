use crate::bindings::request_priority::RequestPriority;
use crate::bindings::task_intent::TaskIntent;
use crate::bindings::task_status::TaskStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::task::TaskInput;
use crate::resources::task::TaskOutput;
use crate::resources::task::TaskPerformer;
use crate::resources::task::TaskRestriction;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Task Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A task to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Task
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Task
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TaskAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> Option<StringType>;
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> Option<StringType>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the groupIdentifier field.
    fn group_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> TaskStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableReference>;
    /// Returns a reference to the businessStatus field.
    fn business_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the intent field.
    fn intent(&self) -> TaskIntent;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> Option<Reference>;
    /// Returns a reference to the for field.
    fn for_(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the requestedPeriod field.
    fn requested_period(&self) -> Option<Period>;
    /// Returns a reference to the executionPeriod field.
    fn execution_period(&self) -> Option<Period>;
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the lastModified field.
    fn last_modified(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the requestedPerformer field.
    fn requested_performer(&self) -> &[CodeableReference];
    /// Returns a reference to the owner field.
    fn owner(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[TaskPerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the relevantHistory field.
    fn relevant_history(&self) -> &[Reference];
    /// Returns a reference to the restriction field.
    fn restriction(&self) -> Option<TaskRestriction>;
    /// Returns a reference to the input field.
    fn input(&self) -> &[TaskInput];
    /// Returns a reference to the output field.
    fn output(&self) -> &[TaskOutput];
}
/// Task Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A task to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Task
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Task
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TaskMutators: DomainResourceMutators {
    /// Create a new Task with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::task::Task;
    /// use rh_hl7_fhir_r5_core::traits::task::TaskMutators;
    ///
    /// let resource = Task::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the groupIdentifier field and returns self for chaining.
    fn set_group_identifier(self, value: Identifier) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: TaskStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableReference) -> Self;
    /// Sets the businessStatus field and returns self for chaining.
    fn set_business_status(self, value: CodeableConcept) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: TaskIntent) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the doNotPerform field and returns self for chaining.
    fn set_do_not_perform(self, value: bool) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Reference) -> Self;
    /// Sets the for field and returns self for chaining.
    fn set_for_(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the requestedPeriod field and returns self for chaining.
    fn set_requested_period(self, value: Period) -> Self;
    /// Sets the executionPeriod field and returns self for chaining.
    fn set_execution_period(self, value: Period) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the lastModified field and returns self for chaining.
    fn set_last_modified(self, value: String) -> Self;
    /// Sets the requester field and returns self for chaining.
    fn set_requester(self, value: Reference) -> Self;
    /// Sets the requestedPerformer field and returns self for chaining.
    fn set_requested_performer(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the requestedPerformer field and returns self for chaining.
    fn add_requested_performer(self, item: CodeableReference) -> Self;
    /// Sets the owner field and returns self for chaining.
    fn set_owner(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<TaskPerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: TaskPerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the relevantHistory field and returns self for chaining.
    fn set_relevant_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the relevantHistory field and returns self for chaining.
    fn add_relevant_history(self, item: Reference) -> Self;
    /// Sets the restriction field and returns self for chaining.
    fn set_restriction(self, value: TaskRestriction) -> Self;
    /// Sets the input field and returns self for chaining.
    fn set_input(self, value: Vec<TaskInput>) -> Self;
    /// Adds an item to the input field and returns self for chaining.
    fn add_input(self, item: TaskInput) -> Self;
    /// Sets the output field and returns self for chaining.
    fn set_output(self, value: Vec<TaskOutput>) -> Self;
    /// Adds an item to the output field and returns self for chaining.
    fn add_output(self, item: TaskOutput) -> Self;
}
/// Task Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A task to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Task
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Task
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TaskExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the instantiates_canonical field is present (Some).
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is present (Some).
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the group_identifier field is present (Some).
    fn has_group_identifier(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the business_status field is present (Some).
    fn has_business_status(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the focus field is present (Some).
    fn has_focus(&self) -> bool;
    /// Returns true if the for_ field is present (Some).
    fn has_for_(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the requested_period field is present (Some).
    fn has_requested_period(&self) -> bool;
    /// Returns true if the execution_period field is present (Some).
    fn has_execution_period(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the last_modified field is present (Some).
    fn has_last_modified(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the requested_performer field is not empty.
    fn has_requested_performer(&self) -> bool;
    /// Returns true if the owner field is present (Some).
    fn has_owner(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the relevant_history field is not empty.
    fn has_relevant_history(&self) -> bool;
    /// Returns true if the restriction field is present (Some).
    fn has_restriction(&self) -> bool;
    /// Returns true if the input field is not empty.
    fn has_input(&self) -> bool;
    /// Returns true if the output field is not empty.
    fn has_output(&self) -> bool;
}
