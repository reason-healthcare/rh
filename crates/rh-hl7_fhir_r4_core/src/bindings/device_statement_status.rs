use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-statement-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceStatementStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Intended
    #[serde(rename = "intended")]
    Intended,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
}
impl Default for DeviceStatementStatus {
    fn default() -> Self {
        Self::Active
    }
}
