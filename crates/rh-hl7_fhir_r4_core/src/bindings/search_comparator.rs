use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-comparator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchComparator {
    /// Equals
    #[serde(rename = "eq")]
    Eq,
    /// Not Equals
    #[serde(rename = "ne")]
    Ne,
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
    /// Approximately
    #[serde(rename = "ap")]
    Ap,
}
impl Default for SearchComparator {
    fn default() -> Self {
        Self::Eq
    }
}
