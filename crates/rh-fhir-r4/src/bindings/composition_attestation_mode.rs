use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/composition-attestation-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionAttestationMode {
    /// Personal
    #[serde(rename = "personal")]
    Personal,
    /// Professional
    #[serde(rename = "professional")]
    Professional,
    /// Legal
    #[serde(rename = "legal")]
    Legal,
    /// Official
    #[serde(rename = "official")]
    Official,
}
impl Default for CompositionAttestationMode {
    fn default() -> Self {
        Self::Personal
    }
}
