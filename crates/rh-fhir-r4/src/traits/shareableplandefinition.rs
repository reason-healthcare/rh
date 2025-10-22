use crate::traits::resource::ResourceExistence;
/// Shareable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the plan definition metadata required by HL7 and other organizations that share and publish plan definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ShareableplandefinitionAccessors {}
/// Shareable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the plan definition metadata required by HL7 and other organizations that share and publish plan definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ShareableplandefinitionMutators {
    /// Create a new Shareableplandefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::shareableplandefinition::Shareableplandefinition;
    /// use hl7_fhir_r4_core::traits::shareableplandefinition::ShareableplandefinitionMutators;
    ///
    /// let resource = Shareableplandefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareableplandefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the plan definition metadata required by HL7 and other organizations that share and publish plan definitions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ShareableplandefinitionExistence: ResourceExistence {}
