use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/characteristic-combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacteristicCombination {
    /// All of
    #[serde(rename = "all-of")]
    AllOf,
    /// Any of
    #[serde(rename = "any-of")]
    AnyOf,
    /// At least
    #[serde(rename = "at-least")]
    AtLeast,
    /// At most
    #[serde(rename = "at-most")]
    AtMost,
    /// Statistical
    #[serde(rename = "statistical")]
    Statistical,
    /// Net effect
    #[serde(rename = "net-effect")]
    NetEffect,
    /// Dataset
    #[serde(rename = "dataset")]
    Dataset,
}
impl Default for CharacteristicCombination {
    fn default() -> Self {
        Self::AllOf
    }
}
