use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/episode-of-care-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpisodeOfCareStatus {
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Waitlist
    #[serde(rename = "waitlist")]
    Waitlist,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// On Hold
    #[serde(rename = "onhold")]
    Onhold,
    /// Finished
    #[serde(rename = "finished")]
    Finished,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for EpisodeOfCareStatus {
    fn default() -> Self {
        Self::Planned
    }
}
