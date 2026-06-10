use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/event-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    /// Preparation
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Not Done
    #[serde(rename = "not-done")]
    NotDone,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for EventStatus {
    fn default() -> Self {
        Self::Preparation
    }
}
