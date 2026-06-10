use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/detectedissue-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectedissueStatus {
    /// Preliminary
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    FinalValue,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Mitigated
    #[serde(rename = "mitigated")]
    Mitigated,
}
impl Default for DetectedissueStatus {
    fn default() -> Self {
        Self::Preliminary
    }
}
