use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-selection-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionSelectionBehavior {
    /// Any
    #[serde(rename = "any")]
    Any,
    /// All
    #[serde(rename = "all")]
    All,
    /// All Or None
    #[serde(rename = "all-or-none")]
    AllOrNone,
    /// Exactly One
    #[serde(rename = "exactly-one")]
    ExactlyOne,
    /// At Most One
    #[serde(rename = "at-most-one")]
    AtMostOne,
    /// One Or More
    #[serde(rename = "one-or-more")]
    OneOrMore,
}
impl Default for ActionSelectionBehavior {
    fn default() -> Self {
        Self::Any
    }
}
