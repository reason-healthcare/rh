use crate::traits::resource::ResourceExistence;
/// Observation Head Circumference Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Head Circumference Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/headcircum
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeadcircumAccessors {}
/// Observation Head Circumference Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Head Circumference Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/headcircum
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeadcircumMutators {
    /// Create a new Headcircum with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::headcircum::Headcircum;
    /// use hl7_fhir_r4_core::traits::headcircum::HeadcircumMutators;
    ///
    /// let resource = Headcircum::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// headcircum Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Head Circumference Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/headcircum
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait HeadcircumExistence: ResourceExistence {}
