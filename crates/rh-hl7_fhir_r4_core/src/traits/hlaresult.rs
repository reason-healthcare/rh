use crate::traits::resource::ResourceExistence;
/// Profile for HLA Genotyping Results Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the HLA genotyping results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hlaresult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait HlaresultAccessors {}
/// Profile for HLA Genotyping Results Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the HLA genotyping results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hlaresult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait HlaresultMutators {
    /// Create a new Hlaresult with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::hlaresult::Hlaresult;
    /// use hl7_fhir_r4_core::traits::hlaresult::HlaresultMutators;
    ///
    /// let resource = Hlaresult::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// hlaresult Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes how the HLA genotyping results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hlaresult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait HlaresultExistence: ResourceExistence {}
