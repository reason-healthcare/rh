use crate::traits::resource::ResourceExistence;
/// CDS Hooks Service PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a PlanDefinition that implements the behavior for a CDS Hooks service
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksserviceplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait CdshooksserviceplandefinitionAccessors {}
/// CDS Hooks Service PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a PlanDefinition that implements the behavior for a CDS Hooks service
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksserviceplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait CdshooksserviceplandefinitionMutators {
    /// Create a new Cdshooksserviceplandefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cdshooksserviceplandefinition::Cdshooksserviceplandefinition;
    /// use hl7_fhir_r4_core::traits::cdshooksserviceplandefinition::CdshooksserviceplandefinitionMutators;
    ///
    /// let resource = Cdshooksserviceplandefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cdshooksserviceplandefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a PlanDefinition that implements the behavior for a CDS Hooks service
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cdshooksserviceplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait CdshooksserviceplandefinitionExistence: ResourceExistence {}
