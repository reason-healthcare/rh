use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::request_group::RequestGroupAction;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// RequestGroup Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A group of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RequestGroupAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the groupIdentifier field.
    fn group_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> RequestStatus;
    /// Returns a reference to the intent field.
    fn intent(&self) -> RequestIntent;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the action field.
    fn action(&self) -> &[RequestGroupAction];
}
/// RequestGroup Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A group of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RequestGroupMutators: DomainResourceMutators {
    /// Create a new RequestGroup with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::request_group::RequestGroup;
    /// use hl7_fhir_r4_core::traits::request_group::RequestGroupMutators;
    ///
    /// let resource = RequestGroup::new();
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
    /// Sets the groupIdentifier field and returns self for chaining.
    fn set_group_identifier(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: RequestStatus) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: RequestIntent) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
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
    /// Sets the action field and returns self for chaining.
    fn set_action(self, value: Vec<RequestGroupAction>) -> Self;
    /// Adds an item to the action field and returns self for chaining.
    fn add_action(self, item: RequestGroupAction) -> Self;
}
/// RequestGroup Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A group of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RequestGroupExistence: DomainResourceExistence {
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
    /// Returns true if the group_identifier field is present (Some).
    fn has_group_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the action field is not empty.
    fn has_action(&self) -> bool;
}
