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
    /// Ended
    #[serde(rename = "ended")]
    Ended,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
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
