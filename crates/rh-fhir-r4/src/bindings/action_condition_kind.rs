use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-condition-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConditionKind {
    /// Applicability
    #[serde(rename = "applicability")]
    Applicability,
    /// Start
    #[serde(rename = "start")]
    Start,
    /// Stop
    #[serde(rename = "stop")]
    Stop,
}
impl Default for ActionConditionKind {
    fn default() -> Self {
        Self::Applicability
    }
}
