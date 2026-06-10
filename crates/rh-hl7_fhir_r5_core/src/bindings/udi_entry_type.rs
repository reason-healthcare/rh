use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/udi-entry-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UdiEntryType {
    /// Barcode
    #[serde(rename = "barcode")]
    Barcode,
    /// RFID
    #[serde(rename = "rfid")]
    Rfid,
    /// Manual
    #[serde(rename = "manual")]
    Manual,
    /// Card
    #[serde(rename = "card")]
    Card,
    /// Self Reported
    #[serde(rename = "self-reported")]
    SelfReported,
    /// Electronic Transmission
    #[serde(rename = "electronic-transmission")]
    ElectronicTransmission,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for UdiEntryType {
    fn default() -> Self {
        Self::Barcode
    }
}
