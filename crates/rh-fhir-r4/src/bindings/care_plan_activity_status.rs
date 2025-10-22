use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/care-plan-activity-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarePlanActivityStatus {
    /// Not Started
    #[serde(rename = "not-started")]
    NotStarted,
    /// Scheduled
    #[serde(rename = "scheduled")]
    Scheduled,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for CarePlanActivityStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}
