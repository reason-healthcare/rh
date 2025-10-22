use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/report-result-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportResultCodes {
    /// Pass
    #[serde(rename = "pass")]
    Pass,
    /// Fail
    #[serde(rename = "fail")]
    Fail,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
}
impl Default for ReportResultCodes {
    fn default() -> Self {
        Self::Pass
    }
}
