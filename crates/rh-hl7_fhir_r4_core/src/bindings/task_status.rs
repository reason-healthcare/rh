use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/task-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Received
    #[serde(rename = "received")]
    Received,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Ready
    #[serde(rename = "ready")]
    Ready,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Failed
    #[serde(rename = "failed")]
    Failed,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for TaskStatus {
    fn default() -> Self {
        Self::Draft
    }
}
