use crate::traits::resource::ResourceExistence;
/// Observation Body Height Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Height Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodyheight
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodyheightAccessors {}
/// Observation Body Height Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Height Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodyheight
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodyheightMutators {
    /// Create a new Bodyheight with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::bodyheight::Bodyheight;
    /// use hl7_fhir_r4_core::traits::bodyheight::BodyheightMutators;
    ///
    /// let resource = Bodyheight::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// bodyheight Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Body Height Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodyheight
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BodyheightExistence: ResourceExistence {}
