use crate::traits::resource::ResourceExistence;
/// Shareable Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the measure metadata required by HL7 and other organizations that share and publish measures
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablemeasure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Measure
pub trait ShareablemeasureAccessors {}
/// Shareable Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the measure metadata required by HL7 and other organizations that share and publish measures
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablemeasure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Measure
pub trait ShareablemeasureMutators {
    /// Create a new Shareablemeasure with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::shareablemeasure::Shareablemeasure;
    /// use hl7_fhir_r4_core::traits::shareablemeasure::ShareablemeasureMutators;
    ///
    /// let resource = Shareablemeasure::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareablemeasure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the measure metadata required by HL7 and other organizations that share and publish measures
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareablemeasure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Measure
pub trait ShareablemeasureExistence: ResourceExistence {}
