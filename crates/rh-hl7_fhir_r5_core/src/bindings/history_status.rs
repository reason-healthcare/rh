use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/history-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoryStatus {
    /// Partial
    #[serde(rename = "partial")]
    Partial,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Health Unknown
    #[serde(rename = "health-unknown")]
    HealthUnknown,
}
impl Default for HistoryStatus {
    fn default() -> Self {
        Self::Partial
    }
}
