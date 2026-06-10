use crate::traits::resource::ResourceExistence;
/// Provenance  Relevant  History Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Guidance on using Provenance for related history elements to provide key events that have happened over the lifespan of the resource  - see the use of this pattern in the [Request Pattern](request.html#history)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/provenance-relevant-history
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait ProvenanceRelevantHistoryAccessors {}
/// Provenance  Relevant  History Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Guidance on using Provenance for related history elements to provide key events that have happened over the lifespan of the resource  - see the use of this pattern in the [Request Pattern](request.html#history)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/provenance-relevant-history
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait ProvenanceRelevantHistoryMutators {
    /// Create a new ProvenanceRelevantHistory with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::provenance_relevant_history::ProvenanceRelevantHistory;
    /// use rh_hl7_fhir_r5_core::traits::provenance_relevant_history::ProvenanceRelevantHistoryMutators;
    ///
    /// let resource = ProvenanceRelevantHistory::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// provenance-relevant-history Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Guidance on using Provenance for related history elements to provide key events that have happened over the lifespan of the resource  - see the use of this pattern in the [Request Pattern](request.html#history)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/provenance-relevant-history
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Provenance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Provenance
pub trait ProvenanceRelevantHistoryExistence: ResourceExistence {}
