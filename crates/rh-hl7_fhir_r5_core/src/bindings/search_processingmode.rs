use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-processingmode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchProcessingmode {
    /// Normal
    #[serde(rename = "normal")]
    Normal,
    /// Phonetic
    #[serde(rename = "phonetic")]
    Phonetic,
    /// Other
    #[serde(rename = "other")]
    Other,
}
impl Default for SearchProcessingmode {
    fn default() -> Self {
        Self::Normal
    }
}
