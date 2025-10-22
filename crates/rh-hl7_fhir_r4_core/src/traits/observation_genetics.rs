use crate::traits::resource::ResourceExistence;
/// Observation-genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the observation resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait ObservationGeneticsAccessors {}
/// Observation-genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the observation resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait ObservationGeneticsMutators {
    /// Create a new ObservationGenetics with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::observation_genetics::ObservationGenetics;
    /// use hl7_fhir_r4_core::traits::observation_genetics::ObservationGeneticsMutators;
    ///
    /// let resource = ObservationGenetics::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// observation-genetics Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes how the observation resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait ObservationGeneticsExistence: ResourceExistence {}
