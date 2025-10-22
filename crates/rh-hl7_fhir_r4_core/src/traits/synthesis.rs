use crate::traits::resource::ResourceExistence;
/// Evidence Synthesis Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/synthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Evidence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Evidence
pub trait SynthesisAccessors {}
/// Evidence Synthesis Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/synthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Evidence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Evidence
pub trait SynthesisMutators {
    /// Create a new Synthesis with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::synthesis::Synthesis;
    /// use hl7_fhir_r4_core::traits::synthesis::SynthesisMutators;
    ///
    /// let resource = Synthesis::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// synthesis Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/synthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Evidence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Evidence
pub trait SynthesisExistence: ResourceExistence {}
