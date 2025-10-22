use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ServiceRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ServiceRequestAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the requisition field.
    fn requisition(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> RequestStatus;
    /// Returns a reference to the intent field.
    fn intent(&self) -> RequestIntent;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the orderDetail field.
    fn order_detail(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the performerType field.
    fn performer_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[Reference];
    /// Returns a reference to the locationCode field.
    fn location_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the locationReference field.
    fn location_reference(&self) -> &[Reference];
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[Reference];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> &[Reference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableConcept];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the patientInstruction field.
    fn patient_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the relevantHistory field.
    fn relevant_history(&self) -> &[Reference];
}
/// ServiceRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ServiceRequestMutators: DomainResourceMutators {
    /// Create a new ServiceRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::service_request::ServiceRequest;
    /// use hl7_fhir_r4_core::traits::service_request::ServiceRequestMutators;
    ///
    /// let resource = ServiceRequest::new();
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
    /// Sets the requisition field and returns self for chaining.
    fn set_requisition(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: RequestStatus) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: RequestIntent) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the doNotPerform field and returns self for chaining.
    fn set_do_not_perform(self, value: bool) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the orderDetail field and returns self for chaining.
    fn set_order_detail(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the orderDetail field and returns self for chaining.
    fn add_order_detail(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the requester field and returns self for chaining.
    fn set_requester(self, value: Reference) -> Self;
    /// Sets the performerType field and returns self for chaining.
    fn set_performer_type(self, value: CodeableConcept) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: Reference) -> Self;
    /// Sets the locationCode field and returns self for chaining.
    fn set_location_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the locationCode field and returns self for chaining.
    fn add_location_code(self, item: CodeableConcept) -> Self;
    /// Sets the locationReference field and returns self for chaining.
    fn set_location_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the locationReference field and returns self for chaining.
    fn add_location_reference(self, item: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: Reference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the specimen field and returns self for chaining.
    fn add_specimen(self, item: Reference) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the bodySite field and returns self for chaining.
    fn add_body_site(self, item: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the patientInstruction field and returns self for chaining.
    fn set_patient_instruction(self, value: String) -> Self;
    /// Sets the relevantHistory field and returns self for chaining.
    fn set_relevant_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the relevantHistory field and returns self for chaining.
    fn add_relevant_history(self, item: Reference) -> Self;
}
/// ServiceRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ServiceRequestExistence: DomainResourceExistence {
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
    /// Returns true if the requisition field is present (Some).
    fn has_requisition(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the order_detail field is not empty.
    fn has_order_detail(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the as_needed field is present (Some).
    fn has_as_needed(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the performer_type field is present (Some).
    fn has_performer_type(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location_code field is not empty.
    fn has_location_code(&self) -> bool;
    /// Returns true if the location_reference field is not empty.
    fn has_location_reference(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the specimen field is not empty.
    fn has_specimen(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the patient_instruction field is present (Some).
    fn has_patient_instruction(&self) -> bool;
    /// Returns true if the relevant_history field is not empty.
    fn has_relevant_history(&self) -> bool;
}
