use crate::bindings::fm_status::FmStatus;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// EnrollmentRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentRequestAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<FmStatus>;
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateTimeType>;
    /// Returns a reference to the insurer field.
    fn insurer(&self) -> Option<Reference>;
    /// Returns a reference to the provider field.
    fn provider(&self) -> Option<Reference>;
    /// Returns a reference to the candidate field.
    fn candidate(&self) -> Option<Reference>;
    /// Returns a reference to the coverage field.
    fn coverage(&self) -> Option<Reference>;
}
/// EnrollmentRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentRequestMutators: DomainResourceMutators {
    /// Create a new EnrollmentRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::enrollment_request::EnrollmentRequest;
    /// use hl7_fhir_r4_core::traits::enrollment_request::EnrollmentRequestMutators;
    ///
    /// let resource = EnrollmentRequest::new();
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
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the insurer field and returns self for chaining.
    fn set_insurer(self, value: Reference) -> Self;
    /// Sets the provider field and returns self for chaining.
    fn set_provider(self, value: Reference) -> Self;
    /// Sets the candidate field and returns self for chaining.
    fn set_candidate(self, value: Reference) -> Self;
    /// Sets the coverage field and returns self for chaining.
    fn set_coverage(self, value: Reference) -> Self;
}
/// EnrollmentRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EnrollmentRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EnrollmentRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EnrollmentRequestExistence: DomainResourceExistence {
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
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the insurer field is present (Some).
    fn has_insurer(&self) -> bool;
    /// Returns true if the provider field is present (Some).
    fn has_provider(&self) -> bool;
    /// Returns true if the candidate field is present (Some).
    fn has_candidate(&self) -> bool;
    /// Returns true if the coverage field is present (Some).
    fn has_coverage(&self) -> bool;
}
