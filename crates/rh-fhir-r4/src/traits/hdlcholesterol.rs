use crate::traits::resource::ResourceExistence;
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// HDL Cholesterol Result
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hdlcholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait HdlcholesterolAccessors {}
/// Example Lipid Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// HDL Cholesterol Result
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hdlcholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait HdlcholesterolMutators {
    /// Create a new Hdlcholesterol with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::hdlcholesterol::Hdlcholesterol;
    /// use hl7_fhir_r4_core::traits::hdlcholesterol::HdlcholesterolMutators;
    ///
    /// let resource = Hdlcholesterol::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// hdlcholesterol Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// HDL Cholesterol Result
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hdlcholesterol
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Observation
pub trait HdlcholesterolExistence: ResourceExistence {}
