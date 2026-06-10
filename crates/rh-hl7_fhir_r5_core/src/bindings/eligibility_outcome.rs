use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/eligibility-outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EligibilityOutcome {
    /// Queued
    #[serde(rename = "queued")]
    Queued,
    /// Processing Complete
    #[serde(rename = "complete")]
    Complete,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Partial Processing
    #[serde(rename = "partial")]
    Partial,
}
impl Default for EligibilityOutcome {
    fn default() -> Self {
        Self::Queued
    }
}
