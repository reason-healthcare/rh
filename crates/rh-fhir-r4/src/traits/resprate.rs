use crate::traits::resource::ResourceExistence;
/// Observation Respiratory Rate Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Respiratory Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resprate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait ResprateAccessors {}
/// Observation Respiratory Rate Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Respiratory Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resprate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait ResprateMutators {
    /// Create a new Resprate with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::resprate::Resprate;
    /// use hl7_fhir_r4_core::traits::resprate::ResprateMutators;
    ///
    /// let resource = Resprate::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// resprate Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Respiratory Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resprate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait ResprateExistence: ResourceExistence {}
