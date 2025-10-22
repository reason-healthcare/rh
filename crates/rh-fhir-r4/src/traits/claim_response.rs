use crate::bindings::claim_use::ClaimUse;
use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::claim_response::ClaimResponseAdditem;
use crate::resources::claim_response::ClaimResponseError;
use crate::resources::claim_response::ClaimResponseInsurance;
use crate::resources::claim_response::ClaimResponseItem;
use crate::resources::claim_response::ClaimResponsePayment;
use crate::resources::claim_response::ClaimResponseProcessnote;
use crate::resources::claim_response::ClaimResponseTotal;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ClaimResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the adjudication details from the processing of a Claim resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClaimResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> CodeableConcept;
    /// Returns a reference to the subType field.
    fn sub_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the use field.
    fn use_(&self) -> ClaimUse;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the insurer field.
    fn insurer(&self) -> Reference;
    /// Returns a reference to the requestor field.
    fn requestor(&self) -> Option<Reference>;
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> RemittanceOutcome;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the preAuthRef field.
    fn pre_auth_ref(&self) -> Option<StringType>;
    /// Returns a reference to the preAuthPeriod field.
    fn pre_auth_period(&self) -> Option<Period>;
    /// Returns a reference to the payeeType field.
    fn payee_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the item field.
    fn item(&self) -> &[ClaimResponseItem];
    /// Returns a reference to the addItem field.
    fn add_item(&self) -> &[ClaimResponseAdditem];
    /// Returns a reference to the total field.
    fn total(&self) -> &[ClaimResponseTotal];
    /// Returns a reference to the payment field.
    fn payment(&self) -> Option<ClaimResponsePayment>;
    /// Returns a reference to the fundsReserve field.
    fn funds_reserve(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the formCode field.
    fn form_code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the form field.
    fn form(&self) -> Option<Attachment>;
    /// Returns a reference to the processNote field.
    fn process_note(&self) -> &[ClaimResponseProcessnote];
    /// Returns a reference to the communicationRequest field.
    fn communication_request(&self) -> &[Reference];
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[ClaimResponseInsurance];
    /// Returns a reference to the error field.
    fn error(&self) -> &[ClaimResponseError];
}
/// ClaimResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the adjudication details from the processing of a Claim resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClaimResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimResponseMutators: DomainResourceMutators {
    /// Create a new ClaimResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::claim_response::ClaimResponse;
    /// use hl7_fhir_r4_core::traits::claim_response::ClaimResponseMutators;
    ///
    /// let resource = ClaimResponse::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subType field and returns self for chaining.
    fn set_sub_type(self, value: CodeableConcept) -> Self;
    /// Sets the use field and returns self for chaining.
    fn set_use_(self, value: ClaimUse) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the insurer field and returns self for chaining.
    fn set_insurer(self, value: Reference) -> Self;
    /// Sets the requestor field and returns self for chaining.
    fn set_requestor(self, value: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: RemittanceOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the preAuthRef field and returns self for chaining.
    fn set_pre_auth_ref(self, value: String) -> Self;
    /// Sets the preAuthPeriod field and returns self for chaining.
    fn set_pre_auth_period(self, value: Period) -> Self;
    /// Sets the payeeType field and returns self for chaining.
    fn set_payee_type(self, value: CodeableConcept) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<ClaimResponseItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: ClaimResponseItem) -> Self;
    /// Sets the addItem field and returns self for chaining.
    fn set_add_item(self, value: Vec<ClaimResponseAdditem>) -> Self;
    /// Adds an item to the addItem field and returns self for chaining.
    fn add_add_item(self, item: ClaimResponseAdditem) -> Self;
    /// Sets the adjudication field and returns self for chaining.
    fn set_adjudication(self, value: Vec<String>) -> Self;
    /// Adds an item to the adjudication field and returns self for chaining.
    fn add_adjudication(self, item: String) -> Self;
    /// Sets the total field and returns self for chaining.
    fn set_total(self, value: Vec<ClaimResponseTotal>) -> Self;
    /// Adds an item to the total field and returns self for chaining.
    fn add_total(self, item: ClaimResponseTotal) -> Self;
    /// Sets the payment field and returns self for chaining.
    fn set_payment(self, value: ClaimResponsePayment) -> Self;
    /// Sets the fundsReserve field and returns self for chaining.
    fn set_funds_reserve(self, value: CodeableConcept) -> Self;
    /// Sets the formCode field and returns self for chaining.
    fn set_form_code(self, value: CodeableConcept) -> Self;
    /// Sets the form field and returns self for chaining.
    fn set_form(self, value: Attachment) -> Self;
    /// Sets the processNote field and returns self for chaining.
    fn set_process_note(self, value: Vec<ClaimResponseProcessnote>) -> Self;
    /// Adds an item to the processNote field and returns self for chaining.
    fn add_process_note(self, item: ClaimResponseProcessnote) -> Self;
    /// Sets the communicationRequest field and returns self for chaining.
    fn set_communication_request(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the communicationRequest field and returns self for chaining.
    fn add_communication_request(self, item: Reference) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<ClaimResponseInsurance>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: ClaimResponseInsurance) -> Self;
    /// Sets the error field and returns self for chaining.
    fn set_error(self, value: Vec<ClaimResponseError>) -> Self;
    /// Adds an item to the error field and returns self for chaining.
    fn add_error(self, item: ClaimResponseError) -> Self;
}
/// ClaimResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides the adjudication details from the processing of a Claim resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ClaimResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimResponseExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the sub_type field is present (Some).
    fn has_sub_type(&self) -> bool;
    /// Returns true if the use_ field is present (Some).
    fn has_use_(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the insurer field is present (Some).
    fn has_insurer(&self) -> bool;
    /// Returns true if the requestor field is present (Some).
    fn has_requestor(&self) -> bool;
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
    /// Returns true if the pre_auth_ref field is present (Some).
    fn has_pre_auth_ref(&self) -> bool;
    /// Returns true if the pre_auth_period field is present (Some).
    fn has_pre_auth_period(&self) -> bool;
    /// Returns true if the payee_type field is present (Some).
    fn has_payee_type(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
    /// Returns true if the add_item field is not empty.
    fn has_add_item(&self) -> bool;
    /// Returns true if the adjudication field is not empty.
    fn has_adjudication(&self) -> bool;
    /// Returns true if the total field is not empty.
    fn has_total(&self) -> bool;
    /// Returns true if the payment field is present (Some).
    fn has_payment(&self) -> bool;
    /// Returns true if the funds_reserve field is present (Some).
    fn has_funds_reserve(&self) -> bool;
    /// Returns true if the form_code field is present (Some).
    fn has_form_code(&self) -> bool;
    /// Returns true if the form field is present (Some).
    fn has_form(&self) -> bool;
    /// Returns true if the process_note field is not empty.
    fn has_process_note(&self) -> bool;
    /// Returns true if the communication_request field is not empty.
    fn has_communication_request(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the error field is not empty.
    fn has_error(&self) -> bool;
}
