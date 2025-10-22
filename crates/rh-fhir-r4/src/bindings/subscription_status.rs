use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/subscription-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    /// Requested
    #[serde(rename = "requested")]
    Requested,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Off
    #[serde(rename = "off")]
    Off,
}
impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Requested
    }
}
