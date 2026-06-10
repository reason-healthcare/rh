use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/encounter-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncounterStatus {
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Discharged
    #[serde(rename = "discharged")]
    Discharged,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Discontinued
    #[serde(rename = "discontinued")]
    Discontinued,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for EncounterStatus {
    fn default() -> Self {
        Self::Planned
    }
}
