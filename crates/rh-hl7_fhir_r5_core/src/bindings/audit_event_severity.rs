use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/audit-event-severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventSeverity {
    /// Emergency
    #[serde(rename = "emergency")]
    Emergency,
    /// Alert
    #[serde(rename = "alert")]
    Alert,
    /// Critical
    #[serde(rename = "critical")]
    Critical,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
    /// Notice
    #[serde(rename = "notice")]
    Notice,
    /// Informational
    #[serde(rename = "informational")]
    Informational,
    /// Debug
    #[serde(rename = "debug")]
    Debug,
}
impl Default for AuditEventSeverity {
    fn default() -> Self {
        Self::Emergency
    }
}
