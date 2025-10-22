use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/measure-report-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasureReportStatus {
    /// Complete
    #[serde(rename = "complete")]
    Complete,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
    /// Error
    #[serde(rename = "error")]
    Error,
}
impl Default for MeasureReportStatus {
    fn default() -> Self {
        Self::Complete
    }
}
