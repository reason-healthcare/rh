use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/observation-triggeredbytype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservationTriggeredbytype {
    /// Reflex
    #[serde(rename = "reflex")]
    Reflex,
    /// Repeat (per policy)
    #[serde(rename = "repeat")]
    Repeat,
    /// Re-run (per policy)
    #[serde(rename = "re-run")]
    ReRun,
}
impl Default for ObservationTriggeredbytype {
    fn default() -> Self {
        Self::Reflex
    }
}
