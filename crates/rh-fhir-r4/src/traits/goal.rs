use crate::bindings::goal_status::GoalStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::goal::GoalTarget;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Goal Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Goal
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Goal
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GoalAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the lifecycleStatus field.
    fn lifecycle_status(&self) -> GoalStatus;
    /// Returns a reference to the achievementStatus field.
    fn achievement_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the target field.
    fn target(&self) -> &[GoalTarget];
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<StringType>;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<StringType>;
    /// Returns a reference to the expressedBy field.
    fn expressed_by(&self) -> Option<Reference>;
    /// Returns a reference to the addresses field.
    fn addresses(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the outcomeCode field.
    fn outcome_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the outcomeReference field.
    fn outcome_reference(&self) -> &[Reference];
}
/// Goal Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Goal
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Goal
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GoalMutators: DomainResourceMutators {
    /// Create a new Goal with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::goal::Goal;
    /// use hl7_fhir_r4_core::traits::goal::GoalMutators;
    ///
    /// let resource = Goal::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the lifecycleStatus field and returns self for chaining.
    fn set_lifecycle_status(self, value: GoalStatus) -> Self;
    /// Sets the achievementStatus field and returns self for chaining.
    fn set_achievement_status(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the target field and returns self for chaining.
    fn set_target(self, value: Vec<GoalTarget>) -> Self;
    /// Adds an item to the target field and returns self for chaining.
    fn add_target(self, item: GoalTarget) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: String) -> Self;
    /// Sets the expressedBy field and returns self for chaining.
    fn set_expressed_by(self, value: Reference) -> Self;
    /// Sets the addresses field and returns self for chaining.
    fn set_addresses(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the addresses field and returns self for chaining.
    fn add_addresses(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the outcomeCode field and returns self for chaining.
    fn set_outcome_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the outcomeCode field and returns self for chaining.
    fn add_outcome_code(self, item: CodeableConcept) -> Self;
    /// Sets the outcomeReference field and returns self for chaining.
    fn set_outcome_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the outcomeReference field and returns self for chaining.
    fn add_outcome_reference(self, item: Reference) -> Self;
}
/// Goal Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Goal
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Goal
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GoalExistence: DomainResourceExistence {
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
    /// Returns true if the lifecycle_status field is present (Some).
    fn has_lifecycle_status(&self) -> bool;
    /// Returns true if the achievement_status field is present (Some).
    fn has_achievement_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the start field is present (Some).
    fn has_start(&self) -> bool;
    /// Returns true if the target field is not empty.
    fn has_target(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the expressed_by field is present (Some).
    fn has_expressed_by(&self) -> bool;
    /// Returns true if the addresses field is not empty.
    fn has_addresses(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the outcome_code field is not empty.
    fn has_outcome_code(&self) -> bool;
    /// Returns true if the outcome_reference field is not empty.
    fn has_outcome_reference(&self) -> bool;
}
