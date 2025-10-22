use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/observation-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservationStatus {
    /// Registered
    #[serde(rename = "registered")]
    Registered,
    /// Preliminary
    #[serde(rename = "preliminary")]
    Preliminary,
    /// Final
    #[serde(rename = "final")]
    Final,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for ObservationStatus {
    fn default() -> Self {
        Self::Registered
    }
}
