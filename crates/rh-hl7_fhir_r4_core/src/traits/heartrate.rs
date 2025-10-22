use crate::traits::resource::ResourceExistence;
/// Observation Heart Rate Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Heart Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/heartrate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeartrateAccessors {}
/// Observation Heart Rate Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Heart Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/heartrate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeartrateMutators {
    /// Create a new Heartrate with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::heartrate::Heartrate;
    /// use hl7_fhir_r4_core::traits::heartrate::HeartrateMutators;
    ///
    /// let resource = Heartrate::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// heartrate Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Heart Rate Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/heartrate
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeartrateExistence: ResourceExistence {}
