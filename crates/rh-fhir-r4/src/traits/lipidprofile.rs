use crate::traits::resource::ResourceExistence;
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Lipid Lab Report
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/lipidprofile
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait LipidprofileAccessors {}
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Lipid Lab Report
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/lipidprofile
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait LipidprofileMutators {
    /// Create a new Lipidprofile with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::lipidprofile::Lipidprofile;
    /// use hl7_fhir_r4_core::traits::lipidprofile::LipidprofileMutators;
    ///
    /// let resource = Lipidprofile::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// lipidprofile Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Lipid Lab Report
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/lipidprofile
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DiagnosticReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
pub trait LipidprofileExistence: ResourceExistence {}
