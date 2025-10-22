use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medication-admin-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationAdminStatus {
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Not Done
    #[serde(rename = "not-done")]
    NotDone,
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
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for MedicationAdminStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
