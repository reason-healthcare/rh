use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medication-statement-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationStatementStatus {
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
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
    /// Not Taken
    #[serde(rename = "not-taken")]
    NotTaken,
}
impl Default for MedicationStatementStatus {
    fn default() -> Self {
        Self::Active
    }
}
