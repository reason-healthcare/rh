use crate::traits::resource::ResourceExistence;
/// DiagnosticReport-Genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the DiagnosticReport resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticreport-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait DiagnosticreportGeneticsAccessors {}
/// DiagnosticReport-Genetics Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the DiagnosticReport resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticreport-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait DiagnosticreportGeneticsMutators {
    /// Create a new DiagnosticreportGenetics with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::diagnosticreport_genetics::DiagnosticreportGenetics;
    /// use hl7_fhir_r4_core::traits::diagnosticreport_genetics::DiagnosticreportGeneticsMutators;
    ///
    /// let resource = DiagnosticreportGenetics::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// diagnosticreport-genetics Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes how the DiagnosticReport resource is used to report structured genetic test results
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticreport-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait DiagnosticreportGeneticsExistence: ResourceExistence {}
