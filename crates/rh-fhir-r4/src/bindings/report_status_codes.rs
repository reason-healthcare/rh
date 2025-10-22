use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/report-status-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportStatusCodes {
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Waiting
    #[serde(rename = "waiting")]
    Waiting,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ReportStatusCodes {
    fn default() -> Self {
        Self::Completed
    }
}
