use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/quality-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityType {
    /// INDEL Comparison
    #[serde(rename = "indel")]
    Indel,
    /// SNP Comparison
    #[serde(rename = "snp")]
    Snp,
    /// UNKNOWN Comparison
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for QualityType {
    fn default() -> Self {
        Self::Indel
    }
}
