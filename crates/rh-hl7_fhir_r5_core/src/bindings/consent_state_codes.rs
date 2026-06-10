use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/consent-state-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentStateCodes {
    /// Pending
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Abandoned
    #[serde(rename = "not-done")]
    NotDone,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for ConsentStateCodes {
    fn default() -> Self {
        Self::Draft
    }
}
