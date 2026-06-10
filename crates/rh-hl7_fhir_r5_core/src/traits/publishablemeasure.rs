use crate::traits::resource::ResourceExistence;
/// Publishable Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the Measure metadata required by HL7 and other organizations that share and publish measures with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablemeasure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablemeasure
pub trait PublishablemeasureAccessors {}
/// Publishable Measure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Supports declaration of the Measure metadata required by HL7 and other organizations that share and publish measures with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablemeasure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablemeasure
pub trait PublishablemeasureMutators {
    /// Create a new Publishablemeasure with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishablemeasure::Publishablemeasure;
    /// use rh_hl7_fhir_r5_core::traits::publishablemeasure::PublishablemeasureMutators;
    ///
    /// let resource = Publishablemeasure::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishablemeasure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Supports declaration of the Measure metadata required by HL7 and other organizations that share and publish measures with a focus on the aspects of that metadata that are important for post-publication activities including distribution, inclusion in repositories, consumption, and implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablemeasure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablemeasure
pub trait PublishablemeasureExistence: ResourceExistence {}
