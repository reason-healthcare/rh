use crate::traits::resource::ResourceExistence;
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the lab report is used for a standard Lipid Profile - Cholesterol, Triglyceride and Cholesterol fractions. Uses LOINC codes
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait CholesterolAccessors {}
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes how the lab report is used for a standard Lipid Profile - Cholesterol, Triglyceride and Cholesterol fractions. Uses LOINC codes
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait CholesterolMutators {
    /// Create a new Cholesterol with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cholesterol::Cholesterol;
    /// use hl7_fhir_r4_core::traits::cholesterol::CholesterolMutators;
    ///
    /// let resource = Cholesterol::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cholesterol Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes how the lab report is used for a standard Lipid Profile - Cholesterol, Triglyceride and Cholesterol fractions. Uses LOINC codes
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait CholesterolExistence: ResourceExistence {}
