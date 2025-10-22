use crate::traits::resource::ResourceExistence;
/// ServiceRequest-Genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the ServiceRequest resource is used to for genetics
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ServiceRequest
pub trait ServicerequestGeneticsAccessors {}
/// ServiceRequest-Genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the ServiceRequest resource is used to for genetics
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ServiceRequest
pub trait ServicerequestGeneticsMutators {
    /// Create a new ServicerequestGenetics with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::servicerequest_genetics::ServicerequestGenetics;
    /// use hl7_fhir_r4_core::traits::servicerequest_genetics::ServicerequestGeneticsMutators;
    ///
    /// let resource = ServicerequestGenetics::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// servicerequest-genetics Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes how the ServiceRequest resource is used to for genetics
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ServiceRequest
pub trait ServicerequestGeneticsExistence: ResourceExistence {}
