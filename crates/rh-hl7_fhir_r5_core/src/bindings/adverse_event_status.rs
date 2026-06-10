use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdverseEventStatus {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for AdverseEventStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
