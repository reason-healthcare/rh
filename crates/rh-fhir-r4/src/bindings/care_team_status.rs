use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/care-team-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CareTeamStatus {
    /// Proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for CareTeamStatus {
    fn default() -> Self {
        Self::Proposed
    }
}
