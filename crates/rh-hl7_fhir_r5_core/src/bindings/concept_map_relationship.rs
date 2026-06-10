use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/concept-map-relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptMapRelationship {
    /// Related To
    #[serde(rename = "related-to")]
    RelatedTo,
    /// Not Related To
    #[serde(rename = "not-related-to")]
    NotRelatedTo,
}
impl Default for ConceptMapRelationship {
    fn default() -> Self {
        Self::RelatedTo
    }
}
