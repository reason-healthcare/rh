use crate::traits::resource::ResourceExistence;
/// Publishable CodeSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a code system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablecodesystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
pub trait PublishablecodesystemAccessors {}
/// Publishable CodeSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a code system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablecodesystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
pub trait PublishablecodesystemMutators {
    /// Create a new Publishablecodesystem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishablecodesystem::Publishablecodesystem;
    /// use rh_hl7_fhir_r5_core::traits::publishablecodesystem::PublishablecodesystemMutators;
    ///
    /// let resource = Publishablecodesystem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishablecodesystem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a code system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablecodesystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablecodesystem
pub trait PublishablecodesystemExistence: ResourceExistence {}
