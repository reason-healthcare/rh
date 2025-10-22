use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medicationdispense-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationdispenseStatus {
    /// Preparation
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for MedicationdispenseStatus {
    fn default() -> Self {
        Self::Preparation
    }
}
