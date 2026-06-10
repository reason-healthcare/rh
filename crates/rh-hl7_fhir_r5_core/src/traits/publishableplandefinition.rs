use crate::traits::resource::ResourceExistence;
/// Publishable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the PlanDefinition metadata required by HL7 and other organizations that share and publish plandefinitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableplandefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
pub trait PublishableplandefinitionAccessors {}
/// Publishable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the PlanDefinition metadata required by HL7 and other organizations that share and publish plandefinitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableplandefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
pub trait PublishableplandefinitionMutators {
    /// Create a new Publishableplandefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishableplandefinition::Publishableplandefinition;
    /// use rh_hl7_fhir_r5_core::traits::publishableplandefinition::PublishableplandefinitionMutators;
    ///
    /// let resource = Publishableplandefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishableplandefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Supports declaration of the PlanDefinition metadata required by HL7 and other organizations that share and publish plandefinitions with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableplandefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
pub trait PublishableplandefinitionExistence: ResourceExistence {}
