use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/enrollment-outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnrollmentOutcome {
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
impl Default for EnrollmentOutcome {
    fn default() -> Self {
        Self::Queued
    }
}
