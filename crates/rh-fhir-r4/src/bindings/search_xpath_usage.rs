use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-xpath-usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchXpathUsage {
    /// Normal
    #[serde(rename = "normal")]
    Normal,
    /// Phonetic
    #[serde(rename = "phonetic")]
    Phonetic,
    /// Nearby
    #[serde(rename = "nearby")]
    Nearby,
    /// Distance
    #[serde(rename = "distance")]
    Distance,
    /// Other
    #[serde(rename = "other")]
    Other,
}
impl Default for SearchXpathUsage {
    fn default() -> Self {
        Self::Normal
    }
}
