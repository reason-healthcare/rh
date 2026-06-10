use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/encounter-location-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncounterLocationStatus {
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Reserved
    #[serde(rename = "reserved")]
    Reserved,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
}
impl Default for EncounterLocationStatus {
    fn default() -> Self {
        Self::Planned
    }
}
