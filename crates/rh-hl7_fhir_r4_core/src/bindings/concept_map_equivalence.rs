use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/concept-map-equivalence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptMapEquivalence {
    /// Related To
    #[serde(rename = "relatedto")]
    Relatedto,
    /// Unmatched
    #[serde(rename = "unmatched")]
    Unmatched,
}
impl Default for ConceptMapEquivalence {
    fn default() -> Self {
        Self::Relatedto
    }
}
