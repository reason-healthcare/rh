use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-participant-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionParticipantType {
    /// Patient
    #[serde(rename = "patient")]
    Patient,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Related Person
    #[serde(rename = "related-person")]
    RelatedPerson,
    /// Device
    #[serde(rename = "device")]
    Device,
}
impl Default for ActionParticipantType {
    fn default() -> Self {
        Self::Patient
    }
}
