use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-source-list-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapSourceListMode {
    /// First
    #[serde(rename = "first")]
    First,
    /// All but the first
    #[serde(rename = "not_first")]
    NotFirst,
    /// Last
    #[serde(rename = "last")]
    Last,
    /// All but the last
    #[serde(rename = "not_last")]
    NotLast,
    /// Enforce only one
    #[serde(rename = "only_one")]
    OnlyOne,
}
impl Default for MapSourceListMode {
    fn default() -> Self {
        Self::First
    }
}
