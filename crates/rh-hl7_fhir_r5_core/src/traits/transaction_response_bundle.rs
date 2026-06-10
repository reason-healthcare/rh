use crate::traits::resource::ResourceExistence;
/// Transaction response bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR transaction response.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/transaction-response-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait TransactionResponseBundleAccessors {}
/// Transaction response bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR transaction response.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/transaction-response-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait TransactionResponseBundleMutators {
    /// Create a new TransactionResponseBundle with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::transaction_response_bundle::TransactionResponseBundle;
    /// use rh_hl7_fhir_r5_core::traits::transaction_response_bundle::TransactionResponseBundleMutators;
    ///
    /// let resource = TransactionResponseBundle::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// transaction-response-bundle Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a FHIR transaction response.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/transaction-response-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait TransactionResponseBundleExistence: ResourceExistence {}
