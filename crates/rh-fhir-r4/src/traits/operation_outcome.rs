use crate::resources::operation_outcome::OperationOutcomeIssue;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// OperationOutcome Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A collection of error, warning, or information messages that result from a system action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationOutcome
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationOutcomeAccessors: DomainResourceAccessors {
    /// Returns a reference to the issue field.
    fn issue(&self) -> &[OperationOutcomeIssue];
}
/// OperationOutcome Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A collection of error, warning, or information messages that result from a system action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationOutcome
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationOutcomeMutators: DomainResourceMutators {
    /// Create a new OperationOutcome with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::operation_outcome::OperationOutcome;
    /// use hl7_fhir_r4_core::traits::operation_outcome::OperationOutcomeMutators;
    ///
    /// let resource = OperationOutcome::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the issue field and returns self for chaining.
    fn set_issue(self, value: Vec<OperationOutcomeIssue>) -> Self;
    /// Adds an item to the issue field and returns self for chaining.
    fn add_issue(self, item: OperationOutcomeIssue) -> Self;
}
/// OperationOutcome Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A collection of error, warning, or information messages that result from a system action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationOutcome
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OperationOutcomeExistence: DomainResourceExistence {
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
    /// Returns true if the issue field is not empty.
    fn has_issue(&self) -> bool;
}
