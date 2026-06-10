use crate::traits::resource::ResourceExistence;
/// Search Set Bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR search bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/search-set-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SearchSetBundleAccessors {}
/// Search Set Bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR search bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/search-set-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SearchSetBundleMutators {
    /// Create a new SearchSetBundle with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::search_set_bundle::SearchSetBundle;
    /// use rh_hl7_fhir_r5_core::traits::search_set_bundle::SearchSetBundleMutators;
    ///
    /// let resource = SearchSetBundle::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// search-set-bundle Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR search bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/search-set-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SearchSetBundleExistence: ResourceExistence {}
