use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/narrative-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeStatus {
    /// Generated
    #[serde(rename = "generated")]
    Generated,
    /// Extensions
    #[serde(rename = "extensions")]
    Extensions,
    /// Additional
    #[serde(rename = "additional")]
    Additional,
    /// Empty
    #[serde(rename = "empty")]
    Empty,
}
impl Default for NarrativeStatus {
    fn default() -> Self {
        Self::Generated
    }
}
