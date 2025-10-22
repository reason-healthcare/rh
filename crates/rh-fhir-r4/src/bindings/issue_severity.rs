use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/issue-severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Fatal
    #[serde(rename = "fatal")]
    Fatal,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Information
    #[serde(rename = "information")]
    Information,
}
impl Default for IssueSeverity {
    fn default() -> Self {
        Self::Fatal
    }
}
