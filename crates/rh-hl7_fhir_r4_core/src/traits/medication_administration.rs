use crate::bindings::medication_admin_status::MedicationAdminStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medication_administration::MedicationAdministrationDosage;
use crate::resources::medication_administration::MedicationAdministrationPerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicationAdministration Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion.  Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationAdministration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationAdministrationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiates field.
    fn instantiates(&self) -> &[StringType];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> MedicationAdminStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the category field.
    fn category(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the context field.
    fn context(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[MedicationAdministrationPerformer];
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the dosage field.
    fn dosage(&self) -> Option<MedicationAdministrationDosage>;
    /// Returns a reference to the eventHistory field.
    fn event_history(&self) -> &[Reference];
}
/// MedicationAdministration Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion.  Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationAdministration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationAdministrationMutators: DomainResourceMutators {
    /// Create a new MedicationAdministration with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medication_administration::MedicationAdministration;
    /// use hl7_fhir_r4_core::traits::medication_administration::MedicationAdministrationMutators;
    ///
    /// let resource = MedicationAdministration::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiates field and returns self for chaining.
    fn set_instantiates(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiates field and returns self for chaining.
    fn add_instantiates(self, item: String) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationAdminStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the statusReason field and returns self for chaining.
    fn add_status_reason(self, item: CodeableConcept) -> Self;
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
    fn set_performer(self, value: Vec<MedicationAdministrationPerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: MedicationAdministrationPerformer) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the device field and returns self for chaining.
    fn add_device(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the dosage field and returns self for chaining.
    fn set_dosage(self, value: MedicationAdministrationDosage) -> Self;
    /// Sets the eventHistory field and returns self for chaining.
    fn set_event_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the eventHistory field and returns self for chaining.
    fn add_event_history(self, item: Reference) -> Self;
}
/// MedicationAdministration Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion.  Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationAdministration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationAdministrationExistence: DomainResourceExistence {
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
    /// Returns true if the instantiates field is not empty.
    fn has_instantiates(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is not empty.
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
    /// Returns true if the effective field is present (Some).
    fn has_effective(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the device field is not empty.
    fn has_device(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the dosage field is present (Some).
    fn has_dosage(&self) -> bool;
    /// Returns true if the event_history field is not empty.
    fn has_event_history(&self) -> bool;
}
