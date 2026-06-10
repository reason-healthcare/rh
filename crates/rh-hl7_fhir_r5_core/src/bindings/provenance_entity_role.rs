use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/provenance-entity-role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProvenanceEntityRole {
    /// Revision
    #[serde(rename = "revision")]
    Revision,
    /// Quotation
    #[serde(rename = "quotation")]
    Quotation,
    /// Source
    #[serde(rename = "source")]
    Source,
    /// Instantiates
    #[serde(rename = "instantiates")]
    Instantiates,
    /// Removal
    #[serde(rename = "removal")]
    Removal,
}
impl Default for ProvenanceEntityRole {
    fn default() -> Self {
        Self::Revision
    }
}
