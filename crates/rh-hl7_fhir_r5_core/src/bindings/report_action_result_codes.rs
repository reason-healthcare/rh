use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/report-action-result-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportActionResultCodes {
    /// Pass
    #[serde(rename = "pass")]
    Pass,
    /// Skip
    #[serde(rename = "skip")]
    Skip,
    /// Fail
    #[serde(rename = "fail")]
    Fail,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Error
    #[serde(rename = "error")]
    Error,
}
impl Default for ReportActionResultCodes {
    fn default() -> Self {
        Self::Pass
    }
}
