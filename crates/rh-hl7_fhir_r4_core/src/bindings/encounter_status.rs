use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/encounter-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncounterStatus {
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Arrived
    #[serde(rename = "arrived")]
    Arrived,
    /// Triaged
    #[serde(rename = "triaged")]
    Triaged,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// On Leave
    #[serde(rename = "onleave")]
    Onleave,
    /// Finished
    #[serde(rename = "finished")]
    Finished,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
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
