use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/value-filter-comparator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueFilterComparator {
    /// Equals
    #[serde(rename = "eq")]
    Eq,
    /// Greater Than
    #[serde(rename = "gt")]
    Gt,
    /// Less Than
    #[serde(rename = "lt")]
    Lt,
    /// Greater or Equals
    #[serde(rename = "ge")]
    Ge,
    /// Less of Equal
    #[serde(rename = "le")]
    Le,
    /// Starts After
    #[serde(rename = "sa")]
    Sa,
    /// Ends Before
    #[serde(rename = "eb")]
    Eb,
}
impl Default for ValueFilterComparator {
    fn default() -> Self {
        Self::Eq
    }
}
