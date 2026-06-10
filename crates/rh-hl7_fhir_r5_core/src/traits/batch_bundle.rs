use crate::traits::resource::ResourceExistence;
/// Batch bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR batch bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/batch-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait BatchBundleAccessors {}
/// Batch bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR batch bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/batch-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait BatchBundleMutators {
    /// Create a new BatchBundle with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::batch_bundle::BatchBundle;
    /// use rh_hl7_fhir_r5_core::traits::batch_bundle::BatchBundleMutators;
    ///
    /// let resource = BatchBundle::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// batch-bundle Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR batch bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/batch-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait BatchBundleExistence: ResourceExistence {}
