use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-participant-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionParticipantType {
    /// CareTeam
    #[serde(rename = "careteam")]
    Careteam,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Group
    #[serde(rename = "group")]
    Group,
    /// HealthcareService
    #[serde(rename = "healthcareservice")]
    Healthcareservice,
    /// Location
    #[serde(rename = "location")]
    Location,
    /// Organization
    #[serde(rename = "organization")]
    Organization,
    /// Patient
    #[serde(rename = "patient")]
    Patient,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// PractitionerRole
    #[serde(rename = "practitionerrole")]
    Practitionerrole,
    /// RelatedPerson
    #[serde(rename = "relatedperson")]
    Relatedperson,
}
impl Default for ActionParticipantType {
    fn default() -> Self {
        Self::Careteam
    }
}
