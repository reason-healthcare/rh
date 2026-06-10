use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/sequence-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SequenceType {
    /// AA Sequence
    #[serde(rename = "aa")]
    Aa,
    /// DNA Sequence
    #[serde(rename = "dna")]
    Dna,
    /// RNA Sequence
    #[serde(rename = "rna")]
    Rna,
}
impl Default for SequenceType {
    fn default() -> Self {
        Self::Aa
    }
}
