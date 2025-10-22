use crate::bindings::history_status::HistoryStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::family_member_history::FamilyMemberHistoryCondition;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// FamilyMemberHistory Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Significant health conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FamilyMemberHistoryAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> HistoryStatus;
    /// Returns a reference to the dataAbsentReason field.
    fn data_absent_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the relationship field.
    fn relationship(&self) -> CodeableConcept;
    /// Returns a reference to the sex field.
    fn sex(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the estimatedAge field.
    fn estimated_age(&self) -> Option<BooleanType>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the condition field.
    fn condition(&self) -> &[FamilyMemberHistoryCondition];
}
/// FamilyMemberHistory Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Significant health conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FamilyMemberHistoryMutators: DomainResourceMutators {
    /// Create a new FamilyMemberHistory with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::family_member_history::FamilyMemberHistory;
    /// use hl7_fhir_r4_core::traits::family_member_history::FamilyMemberHistoryMutators;
    ///
    /// let resource = FamilyMemberHistory::new();
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
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: HistoryStatus) -> Self;
    /// Sets the dataAbsentReason field and returns self for chaining.
    fn set_data_absent_reason(self, value: CodeableConcept) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the relationship field and returns self for chaining.
    fn set_relationship(self, value: CodeableConcept) -> Self;
    /// Sets the sex field and returns self for chaining.
    fn set_sex(self, value: CodeableConcept) -> Self;
    /// Sets the estimatedAge field and returns self for chaining.
    fn set_estimated_age(self, value: bool) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the condition field and returns self for chaining.
    fn set_condition(self, value: Vec<FamilyMemberHistoryCondition>) -> Self;
    /// Adds an item to the condition field and returns self for chaining.
    fn add_condition(self, item: FamilyMemberHistoryCondition) -> Self;
}
/// FamilyMemberHistory Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Significant health conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FamilyMemberHistory
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: FamilyMemberHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FamilyMemberHistoryExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the data_absent_reason field is present (Some).
    fn has_data_absent_reason(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the relationship field is present (Some).
    fn has_relationship(&self) -> bool;
    /// Returns true if the sex field is present (Some).
    fn has_sex(&self) -> bool;
    /// Returns true if the born field is present (Some).
    fn has_born(&self) -> bool;
    /// Returns true if the age field is present (Some).
    fn has_age(&self) -> bool;
    /// Returns true if the estimated_age field is present (Some).
    fn has_estimated_age(&self) -> bool;
    /// Returns true if the deceased field is present (Some).
    fn has_deceased(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the condition field is not empty.
    fn has_condition(&self) -> bool;
}
