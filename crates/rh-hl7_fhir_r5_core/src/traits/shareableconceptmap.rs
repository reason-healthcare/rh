use crate::traits::resource::ResourceExistence;
/// Shareable ConceptMap Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the concept map metadata required by HL7 and other organizations that share and publish concept maps
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ConceptMap
pub trait ShareableconceptmapAccessors {}
/// Shareable ConceptMap Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Enforces the minimum information set for the concept map metadata required by HL7 and other organizations that share and publish concept maps
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ConceptMap
pub trait ShareableconceptmapMutators {
    /// Create a new Shareableconceptmap with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::shareableconceptmap::Shareableconceptmap;
    /// use rh_hl7_fhir_r5_core::traits::shareableconceptmap::ShareableconceptmapMutators;
    ///
    /// let resource = Shareableconceptmap::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// shareableconceptmap Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Enforces the minimum information set for the concept map metadata required by HL7 and other organizations that share and publish concept maps
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ConceptMap
pub trait ShareableconceptmapExistence: ResourceExistence {}
