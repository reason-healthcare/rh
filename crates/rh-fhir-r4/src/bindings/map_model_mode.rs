use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-model-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapModelMode {
    /// Source Structure Definition
    #[serde(rename = "source")]
    Source,
    /// Queried Structure Definition
    #[serde(rename = "queried")]
    Queried,
    /// Target Structure Definition
    #[serde(rename = "target")]
    Target,
    /// Produced Structure Definition
    #[serde(rename = "produced")]
    Produced,
}
impl Default for MapModelMode {
    fn default() -> Self {
        Self::Source
    }
}
