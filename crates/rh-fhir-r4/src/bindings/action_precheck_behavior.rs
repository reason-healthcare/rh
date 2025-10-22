use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-precheck-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionPrecheckBehavior {
    /// Yes
    #[serde(rename = "yes")]
    Yes,
    /// No
    #[serde(rename = "no")]
    No,
}
impl Default for ActionPrecheckBehavior {
    fn default() -> Self {
        Self::Yes
    }
}
