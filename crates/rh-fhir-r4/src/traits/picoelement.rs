use crate::traits::resource::ResourceExistence;
/// PICO Element Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/picoelement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EvidenceVariable
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
pub trait PicoelementAccessors {}
/// PICO Element Profile Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/picoelement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EvidenceVariable
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
pub trait PicoelementMutators {
    /// Create a new Picoelement with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::picoelement::Picoelement;
    /// use hl7_fhir_r4_core::traits::picoelement::PicoelementMutators;
    ///
    /// let resource = Picoelement::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// picoelement Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Explanation of what this profile contains/is for.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/picoelement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EvidenceVariable
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
pub trait PicoelementExistence: ResourceExistence {}
