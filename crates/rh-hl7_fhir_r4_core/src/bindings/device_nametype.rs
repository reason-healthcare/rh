use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-nametype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceNametype {
    /// UDI Label name
    #[serde(rename = "udi-label-name")]
    UdiLabelName,
    /// User Friendly name
    #[serde(rename = "user-friendly-name")]
    UserFriendlyName,
    /// Patient Reported name
    #[serde(rename = "patient-reported-name")]
    PatientReportedName,
    /// Manufacturer name
    #[serde(rename = "manufacturer-name")]
    ManufacturerName,
    /// Model name
    #[serde(rename = "model-name")]
    ModelName,
    /// other
    #[serde(rename = "other")]
    Other,
}
impl Default for DeviceNametype {
    fn default() -> Self {
        Self::UdiLabelName
    }
}
