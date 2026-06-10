use crate::traits::resource::ResourceExistence;
/// Publishable ConceptMap Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a concept map, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
pub trait PublishableconceptmapAccessors {}
/// Publishable ConceptMap Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a concept map, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
pub trait PublishableconceptmapMutators {
    /// Create a new Publishableconceptmap with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::publishableconceptmap::Publishableconceptmap;
    /// use rh_hl7_fhir_r5_core::traits::publishableconceptmap::PublishableconceptmapMutators;
    ///
    /// let resource = Publishableconceptmap::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// publishableconceptmap Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Defines and enforces the minimum expectations for publication and distribution of a concept map, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
pub trait PublishableconceptmapExistence: ResourceExistence {}
