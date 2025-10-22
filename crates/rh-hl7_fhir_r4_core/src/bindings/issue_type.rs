use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/issue-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    /// Invalid Content
    #[serde(rename = "invalid")]
    Invalid,
    /// Security Problem
    #[serde(rename = "security")]
    Security,
    /// Processing Failure
    #[serde(rename = "processing")]
    Processing,
    /// Transient Issue
    #[serde(rename = "transient")]
    Transient,
    /// Informational Note
    #[serde(rename = "informational")]
    Informational,
}
impl Default for IssueType {
    fn default() -> Self {
        Self::Invalid
    }
}
