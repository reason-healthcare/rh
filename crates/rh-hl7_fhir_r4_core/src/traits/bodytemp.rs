use crate::traits::resource::ResourceExistence;
/// Observation Body Temperature Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Temperature Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodytemp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodytempAccessors {}
/// Observation Body Temperature Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Temperature Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodytemp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodytempMutators {
    /// Create a new Bodytemp with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::bodytemp::Bodytemp;
    /// use hl7_fhir_r4_core::traits::bodytemp::BodytempMutators;
    ///
    /// let resource = Bodytemp::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// bodytemp Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Body Temperature Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodytemp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodytempExistence: ResourceExistence {}
