use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/consent-state-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentStateCodes {
    /// Pending
    #[serde(rename = "draft")]
    Draft,
    /// Proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ConsentStateCodes {
    fn default() -> Self {
        Self::Draft
    }
}
