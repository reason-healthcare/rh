use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-group-type-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapGroupTypeMode {
    /// Not a Default
    #[serde(rename = "none")]
    None,
    /// Default for Type Combination
    #[serde(rename = "types")]
    Types,
    /// Default for type + combination
    #[serde(rename = "type-and-types")]
    TypeAndTypes,
}
impl Default for MapGroupTypeMode {
    fn default() -> Self {
        Self::None
    }
}
