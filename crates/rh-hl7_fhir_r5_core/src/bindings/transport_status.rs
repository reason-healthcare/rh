use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/transport-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportStatus {
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Abandoned
    #[serde(rename = "abandoned")]
    Abandoned,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for TransportStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
