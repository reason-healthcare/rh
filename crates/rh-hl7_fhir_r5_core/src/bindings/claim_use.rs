use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/claim-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimUse {
    /// Claim
    #[serde(rename = "claim")]
    Claim,
    /// Preauthorization
    #[serde(rename = "preauthorization")]
    Preauthorization,
    /// Predetermination
    #[serde(rename = "predetermination")]
    Predetermination,
}
impl Default for ClaimUse {
    fn default() -> Self {
        Self::Claim
    }
}
