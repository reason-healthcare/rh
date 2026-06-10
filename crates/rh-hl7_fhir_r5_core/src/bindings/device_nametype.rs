use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-nametype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceNametype {
    /// Registered name
    #[serde(rename = "registered-name")]
    RegisteredName,
    /// User Friendly name
    #[serde(rename = "user-friendly-name")]
    UserFriendlyName,
    /// Patient Reported name
    #[serde(rename = "patient-reported-name")]
    PatientReportedName,
}
impl Default for DeviceNametype {
    fn default() -> Self {
        Self::RegisteredName
    }
}
