use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// EnrollmentResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<FmStatus>;
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<RemittanceOutcome>;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateTimeType>;
    /// Returns a reference to the organization field.
    fn organization(&self) -> Option<Reference>;
    /// Returns a reference to the requestProvider field.
    fn request_provider(&self) -> Option<Reference>;
}
/// EnrollmentResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentResponseMutators: DomainResourceMutators {
    /// Create a new EnrollmentResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::enrollment_response::EnrollmentResponse;
    /// use hl7_fhir_r4_core::traits::enrollment_response::EnrollmentResponseMutators;
    ///
    /// let resource = EnrollmentResponse::new();
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
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: RemittanceOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the organization field and returns self for chaining.
    fn set_organization(self, value: Reference) -> Self;
    /// Sets the requestProvider field and returns self for chaining.
    fn set_request_provider(self, value: Reference) -> Self;
}
/// EnrollmentResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentResponseExistence: DomainResourceExistence {
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
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the organization field is present (Some).
    fn has_organization(&self) -> bool;
    /// Returns true if the request_provider field is present (Some).
    fn has_request_provider(&self) -> bool;
}
