use crate::bindings::eligibilityresponse_purpose::EligibilityresponsePurpose;
use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::coverage_eligibility_response::CoverageEligibilityResponseError;
use crate::resources::coverage_eligibility_response::CoverageEligibilityResponseInsurance;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CoverageEligibilityResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> &[EligibilityresponsePurpose];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the requestor field.
    fn requestor(&self) -> Option<Reference>;
    /// Returns a reference to the request field.
    fn request(&self) -> Reference;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> RemittanceOutcome;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the insurer field.
    fn insurer(&self) -> Reference;
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[CoverageEligibilityResponseInsurance];
    /// Returns a reference to the preAuthRef field.
    fn pre_auth_ref(&self) -> Option<StringType>;
    /// Returns a reference to the form field.
    fn form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the error field.
    fn error(&self) -> &[CoverageEligibilityResponseError];
}
/// CoverageEligibilityResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityResponseMutators: DomainResourceMutators {
    /// Create a new CoverageEligibilityResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::coverage_eligibility_response::CoverageEligibilityResponse;
    /// use hl7_fhir_r4_core::traits::coverage_eligibility_response::CoverageEligibilityResponseMutators;
    ///
    /// let resource = CoverageEligibilityResponse::new();
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
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: Vec<EligibilityresponsePurpose>) -> Self;
    /// Adds an item to the purpose field and returns self for chaining.
    fn add_purpose(self, item: EligibilityresponsePurpose) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the requestor field and returns self for chaining.
    fn set_requestor(self, value: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: RemittanceOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the insurer field and returns self for chaining.
    fn set_insurer(self, value: Reference) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<CoverageEligibilityResponseInsurance>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: CoverageEligibilityResponseInsurance) -> Self;
    /// Sets the preAuthRef field and returns self for chaining.
    fn set_pre_auth_ref(self, value: String) -> Self;
    /// Sets the form field and returns self for chaining.
    fn set_form(self, value: CodeableConcept) -> Self;
    /// Sets the error field and returns self for chaining.
    fn set_error(self, value: Vec<CoverageEligibilityResponseError>) -> Self;
    /// Adds an item to the error field and returns self for chaining.
    fn add_error(self, item: CoverageEligibilityResponseError) -> Self;
}
/// CoverageEligibilityResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityResponseExistence: DomainResourceExistence {
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
    /// Returns true if the purpose field is not empty.
    fn has_purpose(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the serviced field is present (Some).
    fn has_serviced(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the requestor field is present (Some).
    fn has_requestor(&self) -> bool;
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
    /// Returns true if the insurer field is present (Some).
    fn has_insurer(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the pre_auth_ref field is present (Some).
    fn has_pre_auth_ref(&self) -> bool;
    /// Returns true if the form field is present (Some).
    fn has_form(&self) -> bool;
    /// Returns true if the error field is not empty.
    fn has_error(&self) -> bool;
}
