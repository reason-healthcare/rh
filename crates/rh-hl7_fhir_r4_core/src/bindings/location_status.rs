use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/location-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}
impl Default for LocationStatus {
    fn default() -> Self {
        Self::Active
    }
}
