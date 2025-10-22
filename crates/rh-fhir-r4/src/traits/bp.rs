use crate::traits::resource::ResourceExistence;
/// Observation Blood Pressure Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Blood Pressure Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BpAccessors {}
/// Observation Blood Pressure Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Blood Pressure Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BpMutators {
    /// Create a new Bp with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::bp::Bp;
    /// use hl7_fhir_r4_core::traits::bp::BpMutators;
    ///
    /// let resource = Bp::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// bp Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Blood Pressure Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bp
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BpExistence: ResourceExistence {}
