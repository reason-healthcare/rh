use crate::traits::resource::ResourceExistence;
/// CDS Hooks RequestGroup Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a RequestGroup that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestGroup
pub trait CdshooksrequestgroupAccessors {}
/// CDS Hooks RequestGroup Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a RequestGroup that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestGroup
pub trait CdshooksrequestgroupMutators {
    /// Create a new Cdshooksrequestgroup with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cdshooksrequestgroup::Cdshooksrequestgroup;
    /// use hl7_fhir_r4_core::traits::cdshooksrequestgroup::CdshooksrequestgroupMutators;
    ///
    /// let resource = Cdshooksrequestgroup::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cdshooksrequestgroup Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a RequestGroup that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestgroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestGroup
pub trait CdshooksrequestgroupExistence: ResourceExistence {}
