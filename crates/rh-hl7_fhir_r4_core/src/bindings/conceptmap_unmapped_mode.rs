use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conceptmap-unmapped-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptmapUnmappedMode {
    /// Provided Code
    #[serde(rename = "provided")]
    Provided,
    /// Fixed Code
    #[serde(rename = "fixed")]
    Fixed,
    /// Other Map
    #[serde(rename = "other-map")]
    OtherMap,
}
impl Default for ConceptmapUnmappedMode {
    fn default() -> Self {
        Self::Provided
    }
}
