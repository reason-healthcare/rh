use crate::traits::resource::ResourceExistence;
/// Vital Signs Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Vital Signs Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalsigns
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait VitalsignsAccessors {}
/// Vital Signs Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Vital Signs Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalsigns
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait VitalsignsMutators {
    /// Create a new Vitalsigns with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::vitalsigns::Vitalsigns;
    /// use hl7_fhir_r4_core::traits::vitalsigns::VitalsignsMutators;
    ///
    /// let resource = Vitalsigns::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// vitalsigns Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Vital Signs Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalsigns
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait VitalsignsExistence: ResourceExistence {}
