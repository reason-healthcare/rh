use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationrequestStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for MedicationrequestStatus {
    fn default() -> Self {
        Self::Active
    }
}
