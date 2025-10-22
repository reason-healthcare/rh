use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-grouping-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionGroupingBehavior {
    /// Visual Group
    #[serde(rename = "visual-group")]
    VisualGroup,
    /// Logical Group
    #[serde(rename = "logical-group")]
    LogicalGroup,
    /// Sentence Group
    #[serde(rename = "sentence-group")]
    SentenceGroup,
}
impl Default for ActionGroupingBehavior {
    fn default() -> Self {
        Self::VisualGroup
    }
}
