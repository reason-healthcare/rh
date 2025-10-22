use crate::traits::resource::ResourceExistence;
/// EHRS FM Record Lifecycle Event - Provenance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the elements to be supported within the Provenance resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-provenance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait EhrsrleProvenanceAccessors {}
/// EHRS FM Record Lifecycle Event - Provenance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines the elements to be supported within the Provenance resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-provenance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait EhrsrleProvenanceMutators {
    /// Create a new EhrsrleProvenance with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::ehrsrle_provenance::EhrsrleProvenance;
    /// use hl7_fhir_r4_core::traits::ehrsrle_provenance::EhrsrleProvenanceMutators;
    ///
    /// let resource = EhrsrleProvenance::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// ehrsrle-provenance Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines the elements to be supported within the Provenance resource in order to conform with the Electronic Health Record System Functional Model Record Lifecycle Event standard
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ehrsrle-provenance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait EhrsrleProvenanceExistence: ResourceExistence {}
