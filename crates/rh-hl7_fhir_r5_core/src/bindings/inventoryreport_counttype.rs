use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/inventoryreport-counttype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryreportCounttype {
    /// Snapshot
    #[serde(rename = "snapshot")]
    Snapshot,
    /// Difference
    #[serde(rename = "difference")]
    Difference,
}
impl Default for InventoryreportCounttype {
    fn default() -> Self {
        Self::Snapshot
    }
}
