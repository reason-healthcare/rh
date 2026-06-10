use crate::traits::resource::ResourceExistence;
/// Publishable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a value set, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait PublishablevaluesetAccessors {}
/// Publishable ValueSet Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a value set, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait PublishablevaluesetMutators {
    /// Create a new Publishablevalueset with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishablevalueset::Publishablevalueset;
    /// use rh_hl7_fhir_r5_core::traits::publishablevalueset::PublishablevaluesetMutators;
    ///
    /// let resource = Publishablevalueset::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishablevalueset Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a value set, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablevalueset
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablevalueset
pub trait PublishablevaluesetExistence: ResourceExistence {}
