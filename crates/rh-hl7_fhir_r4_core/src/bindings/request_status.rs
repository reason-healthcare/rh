use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/request-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
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
impl Default for RequestStatus {
    fn default() -> Self {
        Self::Draft
    }
}
