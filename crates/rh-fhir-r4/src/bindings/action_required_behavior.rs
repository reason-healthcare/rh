use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-required-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionRequiredBehavior {
    /// Must
    #[serde(rename = "must")]
    Must,
    /// Could
    #[serde(rename = "could")]
    Could,
    /// Must Unless Documented
    #[serde(rename = "must-unless-documented")]
    MustUnlessDocumented,
}
impl Default for ActionRequiredBehavior {
    fn default() -> Self {
        Self::Must
    }
}
