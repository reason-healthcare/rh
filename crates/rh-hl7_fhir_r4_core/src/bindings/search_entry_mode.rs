use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-entry-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchEntryMode {
    /// Match
    #[serde(rename = "match")]
    Match,
    /// Include
    #[serde(rename = "include")]
    Include,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
}
impl Default for SearchEntryMode {
    fn default() -> Self {
        Self::Match
    }
}
