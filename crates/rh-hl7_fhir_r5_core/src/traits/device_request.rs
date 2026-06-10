use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::device_request::DeviceRequestParameter;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be delivered and subsequently be used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceRequestAccessors: DomainResourceAccessors {
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
    fn status(&self) -> Option<RequestStatus>;
    /// Returns a reference to the intent field.
    fn intent(&self) -> RequestIntent;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableReference;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<IntegerType>;
    /// Returns a reference to the parameter field.
    fn parameter(&self) -> &[DeviceRequestParameter];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<CodeableReference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the asNeeded field.
    fn as_needed(&self) -> Option<BooleanType>;
    /// Returns a reference to the asNeededFor field.
    fn as_needed_for(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[Reference];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the relevantHistory field.
    fn relevant_history(&self) -> &[Reference];
}
/// DeviceRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be delivered and subsequently be used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceRequestMutators: DomainResourceMutators {
    /// Create a new DeviceRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::device_request::DeviceRequest;
    /// use rh_hl7_fhir_r5_core::traits::device_request::DeviceRequestMutators;
    ///
    /// let resource = DeviceRequest::new();
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
    /// Sets the doNotPerform field and returns self for chaining.
    fn set_do_not_perform(self, value: bool) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableReference) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: i32) -> Self;
    /// Sets the parameter field and returns self for chaining.
    fn set_parameter(self, value: Vec<DeviceRequestParameter>) -> Self;
    /// Adds an item to the parameter field and returns self for chaining.
    fn add_parameter(self, item: DeviceRequestParameter) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the requester field and returns self for chaining.
    fn set_requester(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: CodeableReference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the asNeeded field and returns self for chaining.
    fn set_as_needed(self, value: bool) -> Self;
    /// Sets the asNeededFor field and returns self for chaining.
    fn set_as_needed_for(self, value: CodeableConcept) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: Reference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the relevantHistory field and returns self for chaining.
    fn set_relevant_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the relevantHistory field and returns self for chaining.
    fn add_relevant_history(self, item: Reference) -> Self;
}
/// DeviceRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be delivered and subsequently be used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceRequestExistence: DomainResourceExistence {
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
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the parameter field is not empty.
    fn has_parameter(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the as_needed field is present (Some).
    fn has_as_needed(&self) -> bool;
    /// Returns true if the as_needed_for field is present (Some).
    fn has_as_needed_for(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the relevant_history field is not empty.
    fn has_relevant_history(&self) -> bool;
}
