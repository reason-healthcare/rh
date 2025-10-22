use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-target-list-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapTargetListMode {
    /// First
    #[serde(rename = "first")]
    First,
    /// Share
    #[serde(rename = "share")]
    Share,
    /// Last
    #[serde(rename = "last")]
    Last,
    /// Collate
    #[serde(rename = "collate")]
    Collate,
}
impl Default for MapTargetListMode {
    fn default() -> Self {
        Self::First
    }
}
