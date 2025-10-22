use crate::bindings::eligibilityrequest_purpose::EligibilityrequestPurpose;
use crate::bindings::fm_status::FmStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::coverage_eligibility_request::CoverageEligibilityRequestInsurance;
use crate::resources::coverage_eligibility_request::CoverageEligibilityRequestItem;
use crate::resources::coverage_eligibility_request::CoverageEligibilityRequestSupportinginfo;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CoverageEligibilityRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityRequestAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> &[EligibilityrequestPurpose];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the enterer field.
    fn enterer(&self) -> Option<Reference>;
    /// Returns a reference to the provider field.
    fn provider(&self) -> Option<Reference>;
    /// Returns a reference to the insurer field.
    fn insurer(&self) -> Reference;
    /// Returns a reference to the facility field.
    fn facility(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[CoverageEligibilityRequestSupportinginfo];
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[CoverageEligibilityRequestInsurance];
    /// Returns a reference to the item field.
    fn item(&self) -> &[CoverageEligibilityRequestItem];
}
/// CoverageEligibilityRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityRequestMutators: DomainResourceMutators {
    /// Create a new CoverageEligibilityRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::coverage_eligibility_request::CoverageEligibilityRequest;
    /// use hl7_fhir_r4_core::traits::coverage_eligibility_request::CoverageEligibilityRequestMutators;
    ///
    /// let resource = CoverageEligibilityRequest::new();
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
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: Vec<EligibilityrequestPurpose>) -> Self;
    /// Adds an item to the purpose field and returns self for chaining.
    fn add_purpose(self, item: EligibilityrequestPurpose) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the enterer field and returns self for chaining.
    fn set_enterer(self, value: Reference) -> Self;
    /// Sets the provider field and returns self for chaining.
    fn set_provider(self, value: Reference) -> Self;
    /// Sets the insurer field and returns self for chaining.
    fn set_insurer(self, value: Reference) -> Self;
    /// Sets the facility field and returns self for chaining.
    fn set_facility(self, value: Reference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<CoverageEligibilityRequestSupportinginfo>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: CoverageEligibilityRequestSupportinginfo) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<CoverageEligibilityRequestInsurance>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: CoverageEligibilityRequestInsurance) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<CoverageEligibilityRequestItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: CoverageEligibilityRequestItem) -> Self;
}
/// CoverageEligibilityRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CoverageEligibilityRequestExistence: DomainResourceExistence {
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
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the purpose field is not empty.
    fn has_purpose(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the serviced field is present (Some).
    fn has_serviced(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the enterer field is present (Some).
    fn has_enterer(&self) -> bool;
    /// Returns true if the provider field is present (Some).
    fn has_provider(&self) -> bool;
    /// Returns true if the insurer field is present (Some).
    fn has_insurer(&self) -> bool;
    /// Returns true if the facility field is present (Some).
    fn has_facility(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
}
