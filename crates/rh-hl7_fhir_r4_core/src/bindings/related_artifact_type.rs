use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/related-artifact-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelatedArtifactType {
    /// Documentation
    #[serde(rename = "documentation")]
    Documentation,
    /// Justification
    #[serde(rename = "justification")]
    Justification,
    /// Citation
    #[serde(rename = "citation")]
    Citation,
    /// Predecessor
    #[serde(rename = "predecessor")]
    Predecessor,
    /// Successor
    #[serde(rename = "successor")]
    Successor,
    /// Derived From
    #[serde(rename = "derived-from")]
    DerivedFrom,
    /// Depends On
    #[serde(rename = "depends-on")]
    DependsOn,
    /// Composed Of
    #[serde(rename = "composed-of")]
    ComposedOf,
}
impl Default for RelatedArtifactType {
    fn default() -> Self {
        Self::Documentation
    }
}
