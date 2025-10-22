use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/goal-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    /// Proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
}
impl Default for GoalStatus {
    fn default() -> Self {
        Self::Proposed
    }
}
