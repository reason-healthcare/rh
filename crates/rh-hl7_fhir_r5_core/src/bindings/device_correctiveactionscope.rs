use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-correctiveactionscope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCorrectiveactionscope {
    /// Model
    #[serde(rename = "model")]
    Model,
    /// Lot Numbers
    #[serde(rename = "lot-numbers")]
    LotNumbers,
    /// Serial Numbers
    #[serde(rename = "serial-numbers")]
    SerialNumbers,
}
impl Default for DeviceCorrectiveactionscope {
    fn default() -> Self {
        Self::Model
    }
}
