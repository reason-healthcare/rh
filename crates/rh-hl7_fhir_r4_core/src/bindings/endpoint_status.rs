use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/endpoint-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Test
    #[serde(rename = "test")]
    Test,
}
impl Default for EndpointStatus {
    fn default() -> Self {
        Self::Active
    }
}
