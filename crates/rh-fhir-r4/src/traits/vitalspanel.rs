use crate::traits::resource::ResourceExistence;
/// Observation Vital Signs Panel Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Vital Signs Panel Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalspanel
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait VitalspanelAccessors {}
/// Observation Vital Signs Panel Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Vital Signs Panel Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalspanel
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait VitalspanelMutators {
    /// Create a new Vitalspanel with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::vitalspanel::Vitalspanel;
    /// use hl7_fhir_r4_core::traits::vitalspanel::VitalspanelMutators;
    ///
    /// let resource = Vitalspanel::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// vitalspanel Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Vital Signs Panel Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/vitalspanel
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait VitalspanelExistence: ResourceExistence {}
