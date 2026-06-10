use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/request-priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestPriority {
    /// Routine
    #[serde(rename = "routine")]
    Routine,
    /// Urgent
    #[serde(rename = "urgent")]
    Urgent,
    /// ASAP
    #[serde(rename = "asap")]
    Asap,
    /// STAT
    #[serde(rename = "stat")]
    Stat,
}
impl Default for RequestPriority {
    fn default() -> Self {
        Self::Routine
    }
}
