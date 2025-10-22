use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-input-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapInputMode {
    /// Source Instance
    #[serde(rename = "source")]
    Source,
    /// Target Instance
    #[serde(rename = "target")]
    Target,
}
impl Default for MapInputMode {
    fn default() -> Self {
        Self::Source
    }
}
