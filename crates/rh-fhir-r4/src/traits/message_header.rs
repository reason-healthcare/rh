use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::message_header::MessageHeaderDestination;
use crate::resources::message_header::MessageHeaderResponse;
use crate::resources::message_header::MessageHeaderSource;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MessageHeader Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageHeader
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageHeaderAccessors: DomainResourceAccessors {
    /// Returns a reference to the destination field.
    fn destination(&self) -> &[MessageHeaderDestination];
    /// Returns a reference to the sender field.
    fn sender(&self) -> Option<Reference>;
    /// Returns a reference to the enterer field.
    fn enterer(&self) -> Option<Reference>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the source field.
    fn source(&self) -> MessageHeaderSource;
    /// Returns a reference to the responsible field.
    fn responsible(&self) -> Option<Reference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the response field.
    fn response(&self) -> Option<MessageHeaderResponse>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[Reference];
    /// Returns a reference to the definition field.
    fn definition(&self) -> Option<StringType>;
}
/// MessageHeader Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageHeader
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageHeaderMutators: DomainResourceMutators {
    /// Create a new MessageHeader with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::message_header::MessageHeader;
    /// use hl7_fhir_r4_core::traits::message_header::MessageHeaderMutators;
    ///
    /// let resource = MessageHeader::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Vec<MessageHeaderDestination>) -> Self;
    /// Adds an item to the destination field and returns self for chaining.
    fn add_destination(self, item: MessageHeaderDestination) -> Self;
    /// Sets the sender field and returns self for chaining.
    fn set_sender(self, value: Reference) -> Self;
    /// Sets the enterer field and returns self for chaining.
    fn set_enterer(self, value: Reference) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: MessageHeaderSource) -> Self;
    /// Sets the responsible field and returns self for chaining.
    fn set_responsible(self, value: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: CodeableConcept) -> Self;
    /// Sets the response field and returns self for chaining.
    fn set_response(self, value: MessageHeaderResponse) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: Reference) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: String) -> Self;
}
/// MessageHeader Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageHeader
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MessageHeaderExistence: DomainResourceExistence {
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
    /// Returns true if the event field is present (Some).
    fn has_event(&self) -> bool;
    /// Returns true if the destination field is not empty.
    fn has_destination(&self) -> bool;
    /// Returns true if the sender field is present (Some).
    fn has_sender(&self) -> bool;
    /// Returns true if the enterer field is present (Some).
    fn has_enterer(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the responsible field is present (Some).
    fn has_responsible(&self) -> bool;
    /// Returns true if the reason field is present (Some).
    fn has_reason(&self) -> bool;
    /// Returns true if the response field is present (Some).
    fn has_response(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the definition field is present (Some).
    fn has_definition(&self) -> bool;
}
