use crate::traits::resource::ResourceExistence;
/// Computable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a computable PlanDefinition that specifies a single library and requires all expressions referenced from the PlanDefinition to be definitions in that single library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ComputableplandefinitionAccessors {}
/// Computable PlanDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines a computable PlanDefinition that specifies a single library and requires all expressions referenced from the PlanDefinition to be definitions in that single library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ComputableplandefinitionMutators {
    /// Create a new Computableplandefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::computableplandefinition::Computableplandefinition;
    /// use hl7_fhir_r4_core::traits::computableplandefinition::ComputableplandefinitionMutators;
    ///
    /// let resource = Computableplandefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// computableplandefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines a computable PlanDefinition that specifies a single library and requires all expressions referenced from the PlanDefinition to be definitions in that single library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
pub trait ComputableplandefinitionExistence: ResourceExistence {}
