use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/inventoryreport-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryreportStatus {
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for InventoryreportStatus {
    fn default() -> Self {
        Self::Draft
    }
}
