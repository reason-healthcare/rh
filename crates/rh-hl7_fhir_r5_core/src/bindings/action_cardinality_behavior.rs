use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-cardinality-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionCardinalityBehavior {
    /// Single
    #[serde(rename = "single")]
    Single,
    /// Multiple
    #[serde(rename = "multiple")]
    Multiple,
}
impl Default for ActionCardinalityBehavior {
    fn default() -> Self {
        Self::Single
    }
}
