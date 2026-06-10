use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/supplydelivery-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplydeliveryStatus {
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Delivered
    #[serde(rename = "completed")]
    Completed,
    /// Abandoned
    #[serde(rename = "abandoned")]
    Abandoned,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for SupplydeliveryStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
