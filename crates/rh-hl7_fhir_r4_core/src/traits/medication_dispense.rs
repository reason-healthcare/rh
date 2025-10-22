use crate::bindings::medicationdispense_status::MedicationdispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::medication_dispense::MedicationDispensePerformer;
use crate::resources::medication_dispense::MedicationDispenseSubstitution;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicationDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationDispenseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> MedicationdispenseStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the context field.
    fn context(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[MedicationDispensePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the authorizingPrescription field.
    fn authorizing_prescription(&self) -> &[Reference];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the daysSupply field.
    fn days_supply(&self) -> Option<Quantity>;
    /// Returns a reference to the whenPrepared field.
    fn when_prepared(&self) -> Option<DateTimeType>;
    /// Returns a reference to the whenHandedOver field.
    fn when_handed_over(&self) -> Option<DateTimeType>;
    /// Returns a reference to the destination field.
    fn destination(&self) -> Option<Reference>;
    /// Returns a reference to the receiver field.
    fn receiver(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the dosageInstruction field.
    fn dosage_instruction(&self) -> &[Dosage];
    /// Returns a reference to the substitution field.
    fn substitution(&self) -> Option<MedicationDispenseSubstitution>;
    /// Returns a reference to the detectedIssue field.
    fn detected_issue(&self) -> &[Reference];
    /// Returns a reference to the eventHistory field.
    fn event_history(&self) -> &[Reference];
}
/// MedicationDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationDispenseMutators: DomainResourceMutators {
    /// Create a new MedicationDispense with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medication_dispense::MedicationDispense;
    /// use hl7_fhir_r4_core::traits::medication_dispense::MedicationDispenseMutators;
    ///
    /// let resource = MedicationDispense::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationdispenseStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: Reference) -> Self;
    /// Sets the supportingInformation field and returns self for chaining.
    fn set_supporting_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInformation field and returns self for chaining.
    fn add_supporting_information(self, item: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<MedicationDispensePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: MedicationDispensePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the authorizingPrescription field and returns self for chaining.
    fn set_authorizing_prescription(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the authorizingPrescription field and returns self for chaining.
    fn add_authorizing_prescription(self, item: Reference) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the daysSupply field and returns self for chaining.
    fn set_days_supply(self, value: Quantity) -> Self;
    /// Sets the whenPrepared field and returns self for chaining.
    fn set_when_prepared(self, value: String) -> Self;
    /// Sets the whenHandedOver field and returns self for chaining.
    fn set_when_handed_over(self, value: String) -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Reference) -> Self;
    /// Sets the receiver field and returns self for chaining.
    fn set_receiver(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the receiver field and returns self for chaining.
    fn add_receiver(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the dosageInstruction field and returns self for chaining.
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self;
    /// Adds an item to the dosageInstruction field and returns self for chaining.
    fn add_dosage_instruction(self, item: Dosage) -> Self;
    /// Sets the substitution field and returns self for chaining.
    fn set_substitution(self, value: MedicationDispenseSubstitution) -> Self;
    /// Sets the detectedIssue field and returns self for chaining.
    fn set_detected_issue(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the detectedIssue field and returns self for chaining.
    fn add_detected_issue(self, item: Reference) -> Self;
    /// Sets the eventHistory field and returns self for chaining.
    fn set_event_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the eventHistory field and returns self for chaining.
    fn add_event_history(self, item: Reference) -> Self;
}
/// MedicationDispense Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationDispenseExistence: DomainResourceExistence {
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
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the medication field is present (Some).
    fn has_medication(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the context field is present (Some).
    fn has_context(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the authorizing_prescription field is not empty.
    fn has_authorizing_prescription(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the days_supply field is present (Some).
    fn has_days_supply(&self) -> bool;
    /// Returns true if the when_prepared field is present (Some).
    fn has_when_prepared(&self) -> bool;
    /// Returns true if the when_handed_over field is present (Some).
    fn has_when_handed_over(&self) -> bool;
    /// Returns true if the destination field is present (Some).
    fn has_destination(&self) -> bool;
    /// Returns true if the receiver field is not empty.
    fn has_receiver(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the dosage_instruction field is not empty.
    fn has_dosage_instruction(&self) -> bool;
    /// Returns true if the substitution field is present (Some).
    fn has_substitution(&self) -> bool;
    /// Returns true if the detected_issue field is not empty.
    fn has_detected_issue(&self) -> bool;
    /// Returns true if the event_history field is not empty.
    fn has_event_history(&self) -> bool;
}
