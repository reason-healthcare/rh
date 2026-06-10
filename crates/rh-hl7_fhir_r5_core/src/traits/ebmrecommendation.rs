use crate::traits::resource::ResourceExistence;
/// EBM Recommendation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents justification for a recommendation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ebmrecommendation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
pub trait EbmrecommendationAccessors {}
/// EBM Recommendation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents justification for a recommendation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ebmrecommendation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
pub trait EbmrecommendationMutators {
    /// Create a new Ebmrecommendation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::ebmrecommendation::Ebmrecommendation;
    /// use rh_hl7_fhir_r5_core::traits::ebmrecommendation::EbmrecommendationMutators;
    ///
    /// let resource = Ebmrecommendation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// ebmrecommendation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents justification for a recommendation
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ebmrecommendation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
pub trait EbmrecommendationExistence: ResourceExistence {}
