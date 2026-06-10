use crate::traits::resource::ResourceExistence;
/// C D S  Hooks  Request Orchestration Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a RequestOrchestration that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestorchestration
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RequestOrchestration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
pub trait CdshooksrequestorchestrationAccessors {}
/// C D S  Hooks  Request Orchestration Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a RequestOrchestration that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestorchestration
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RequestOrchestration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
pub trait CdshooksrequestorchestrationMutators {
    /// Create a new Cdshooksrequestorchestration with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::cdshooksrequestorchestration::Cdshooksrequestorchestration;
    /// use rh_hl7_fhir_r5_core::traits::cdshooksrequestorchestration::CdshooksrequestorchestrationMutators;
    ///
    /// let resource = Cdshooksrequestorchestration::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cdshooksrequestorchestration Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a RequestOrchestration that can represent a CDS Hooks response
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksrequestorchestration
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RequestOrchestration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
pub trait CdshooksrequestorchestrationExistence: ResourceExistence {}
