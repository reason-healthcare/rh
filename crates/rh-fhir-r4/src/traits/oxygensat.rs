use crate::traits::resource::ResourceExistence;
/// Observation Oxygen Saturation Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Oxygen Saturation Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/oxygensat
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait OxygensatAccessors {}
/// Observation Oxygen Saturation Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Oxygen Saturation Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/oxygensat
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait OxygensatMutators {
    /// Create a new Oxygensat with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::oxygensat::Oxygensat;
    /// use hl7_fhir_r4_core::traits::oxygensat::OxygensatMutators;
    ///
    /// let resource = Oxygensat::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// oxygensat Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Oxygen Saturation Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/oxygensat
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait OxygensatExistence: ResourceExistence {}
