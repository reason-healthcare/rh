use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/device-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for DeviceStatus {
    fn default() -> Self {
        Self::Active
    }
}
