use crate::bindings::devicedispense_status::DevicedispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::device_dispense::DeviceDispensePerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDispenseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> DevicedispenseStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableReference>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the device field.
    fn device(&self) -> CodeableReference;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the receiver field.
    fn receiver(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[DeviceDispensePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the preparedDate field.
    fn prepared_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the whenHandedOver field.
    fn when_handed_over(&self) -> Option<DateTimeType>;
    /// Returns a reference to the destination field.
    fn destination(&self) -> Option<Reference>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the usageInstruction field.
    fn usage_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the eventHistory field.
    fn event_history(&self) -> &[Reference];
}
/// DeviceDispense Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDispenseMutators: DomainResourceMutators {
    /// Create a new DeviceDispense with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::device_dispense::DeviceDispense;
    /// use rh_hl7_fhir_r5_core::traits::device_dispense::DeviceDispenseMutators;
    ///
    /// let resource = DeviceDispense::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DevicedispenseStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableReference) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: CodeableReference) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the receiver field and returns self for chaining.
    fn set_receiver(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the supportingInformation field and returns self for chaining.
    fn set_supporting_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInformation field and returns self for chaining.
    fn add_supporting_information(self, item: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<DeviceDispensePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: DeviceDispensePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the preparedDate field and returns self for chaining.
    fn set_prepared_date(self, value: String) -> Self;
    /// Sets the whenHandedOver field and returns self for chaining.
    fn set_when_handed_over(self, value: String) -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the usageInstruction field and returns self for chaining.
    fn set_usage_instruction(self, value: String) -> Self;
    /// Sets the eventHistory field and returns self for chaining.
    fn set_event_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the eventHistory field and returns self for chaining.
    fn add_event_history(self, item: Reference) -> Self;
}
/// DeviceDispense Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDispenseExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the receiver field is present (Some).
    fn has_receiver(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the prepared_date field is present (Some).
    fn has_prepared_date(&self) -> bool;
    /// Returns true if the when_handed_over field is present (Some).
    fn has_when_handed_over(&self) -> bool;
    /// Returns true if the destination field is present (Some).
    fn has_destination(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the usage_instruction field is present (Some).
    fn has_usage_instruction(&self) -> bool;
    /// Returns true if the event_history field is not empty.
    fn has_event_history(&self) -> bool;
}
