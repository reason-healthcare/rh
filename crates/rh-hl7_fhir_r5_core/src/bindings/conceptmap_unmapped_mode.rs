use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conceptmap-unmapped-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptmapUnmappedMode {
    /// Use Provided Source Code
    #[serde(rename = "use-source-code")]
    UseSourceCode,
    /// Fixed Code
    #[serde(rename = "fixed")]
    Fixed,
    /// Other Map
    #[serde(rename = "other-map")]
    OtherMap,
}
impl Default for ConceptmapUnmappedMode {
    fn default() -> Self {
        Self::UseSourceCode
    }
}
