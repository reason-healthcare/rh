use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/provenance-entity-role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProvenanceEntityRole {
    /// Derivation
    #[serde(rename = "derivation")]
    Derivation,
}
impl Default for ProvenanceEntityRole {
    fn default() -> Self {
        Self::Derivation
    }
}
