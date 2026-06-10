use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-productidentifierinudi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceProductidentifierinudi {
    /// Lot Number
    #[serde(rename = "lot-number")]
    LotNumber,
    /// Manufactured date
    #[serde(rename = "manufactured-date")]
    ManufacturedDate,
    /// Serial Number
    #[serde(rename = "serial-number")]
    SerialNumber,
    /// Expiration date
    #[serde(rename = "expiration-date")]
    ExpirationDate,
    /// Biological source
    #[serde(rename = "biological-source")]
    BiologicalSource,
    /// Software Version
    #[serde(rename = "software-version")]
    SoftwareVersion,
}
impl Default for DeviceProductidentifierinudi {
    fn default() -> Self {
        Self::LotNumber
    }
}
