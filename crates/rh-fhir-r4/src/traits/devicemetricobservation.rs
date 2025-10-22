use crate::traits::resource::ResourceExistence;
/// Device Metric Observation Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile describes the direct or derived, qualitative or quantitative physiological measurement, setting, or calculation data produced by a medical device or a device component.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/devicemetricobservation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait DevicemetricobservationAccessors {}
/// Device Metric Observation Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile describes the direct or derived, qualitative or quantitative physiological measurement, setting, or calculation data produced by a medical device or a device component.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/devicemetricobservation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait DevicemetricobservationMutators {
    /// Create a new Devicemetricobservation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::devicemetricobservation::Devicemetricobservation;
    /// use hl7_fhir_r4_core::traits::devicemetricobservation::DevicemetricobservationMutators;
    ///
    /// let resource = Devicemetricobservation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// devicemetricobservation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This profile describes the direct or derived, qualitative or quantitative physiological measurement, setting, or calculation data produced by a medical device or a device component.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/devicemetricobservation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait DevicemetricobservationExistence: ResourceExistence {}
