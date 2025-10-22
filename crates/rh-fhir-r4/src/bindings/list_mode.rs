use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/list-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListMode {
    /// Working List
    #[serde(rename = "working")]
    Working,
    /// Snapshot List
    #[serde(rename = "snapshot")]
    Snapshot,
    /// Change List
    #[serde(rename = "changes")]
    Changes,
}
impl Default for ListMode {
    fn default() -> Self {
        Self::Working
    }
}
