use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/submit-data-update-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmitDataUpdateType {
    /// Incremental
    #[serde(rename = "incremental")]
    Incremental,
    /// Snapshot
    #[serde(rename = "snapshot")]
    Snapshot,
}
impl Default for SubmitDataUpdateType {
    fn default() -> Self {
        Self::Incremental
    }
}
