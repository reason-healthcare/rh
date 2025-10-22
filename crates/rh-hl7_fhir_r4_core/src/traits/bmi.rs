use crate::traits::resource::ResourceExistence;
/// Observation Body Mass Index Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Mass Index (BMI) Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bmi
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BmiAccessors {}
/// Observation Body Mass Index Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// FHIR Body Mass Index (BMI) Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bmi
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BmiMutators {
    /// Create a new Bmi with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::bmi::Bmi;
    /// use hl7_fhir_r4_core::traits::bmi::BmiMutators;
    ///
    /// let resource = Bmi::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// bmi Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// FHIR Body Mass Index (BMI) Profile
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bmi
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/vitalsigns
pub trait BmiExistence: ResourceExistence {}
