use crate::bindings::event_status::EventStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::communication::CommunicationPayload;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Communication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An occurrence of information being transmitted; e.g. an alert that was sent to a responsible provider, a public health agency that was notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Communication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Communication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CommunicationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the inResponseTo field.
    fn in_response_to(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> EventStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the medium field.
    fn medium(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the topic field.
    fn topic(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the about field.
    fn about(&self) -> &[Reference];
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the sent field.
    fn sent(&self) -> Option<DateTimeType>;
    /// Returns a reference to the received field.
    fn received(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recipient field.
    fn recipient(&self) -> &[Reference];
    /// Returns a reference to the sender field.
    fn sender(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the payload field.
    fn payload(&self) -> &[CommunicationPayload];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// Communication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An occurrence of information being transmitted; e.g. an alert that was sent to a responsible provider, a public health agency that was notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Communication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Communication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CommunicationMutators: DomainResourceMutators {
    /// Create a new Communication with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::communication::Communication;
    /// use hl7_fhir_r4_core::traits::communication::CommunicationMutators;
    ///
    /// let resource = Communication::new();
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
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the inResponseTo field and returns self for chaining.
    fn set_in_response_to(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the inResponseTo field and returns self for chaining.
    fn add_in_response_to(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EventStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the medium field and returns self for chaining.
    fn set_medium(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the medium field and returns self for chaining.
    fn add_medium(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the topic field and returns self for chaining.
    fn set_topic(self, value: CodeableConcept) -> Self;
    /// Sets the about field and returns self for chaining.
    fn set_about(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the about field and returns self for chaining.
    fn add_about(self, item: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the sent field and returns self for chaining.
    fn set_sent(self, value: String) -> Self;
    /// Sets the received field and returns self for chaining.
    fn set_received(self, value: String) -> Self;
    /// Sets the recipient field and returns self for chaining.
    fn set_recipient(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the recipient field and returns self for chaining.
    fn add_recipient(self, item: Reference) -> Self;
    /// Sets the sender field and returns self for chaining.
    fn set_sender(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the payload field and returns self for chaining.
    fn set_payload(self, value: Vec<CommunicationPayload>) -> Self;
    /// Adds an item to the payload field and returns self for chaining.
    fn add_payload(self, item: CommunicationPayload) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// Communication Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An occurrence of information being transmitted; e.g. an alert that was sent to a responsible provider, a public health agency that was notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Communication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Communication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CommunicationExistence: DomainResourceExistence {
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
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the in_response_to field is not empty.
    fn has_in_response_to(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the medium field is not empty.
    fn has_medium(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the topic field is present (Some).
    fn has_topic(&self) -> bool;
    /// Returns true if the about field is not empty.
    fn has_about(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the sent field is present (Some).
    fn has_sent(&self) -> bool;
    /// Returns true if the received field is present (Some).
    fn has_received(&self) -> bool;
    /// Returns true if the recipient field is not empty.
    fn has_recipient(&self) -> bool;
    /// Returns true if the sender field is present (Some).
    fn has_sender(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the payload field is not empty.
    fn has_payload(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
