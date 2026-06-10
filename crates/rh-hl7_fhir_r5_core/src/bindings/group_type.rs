use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/group-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupType {
    /// Person
    #[serde(rename = "person")]
    Person,
    /// Animal
    #[serde(rename = "animal")]
    Animal,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// CareTeam
    #[serde(rename = "careteam")]
    Careteam,
    /// HealthcareService
    #[serde(rename = "healthcareservice")]
    Healthcareservice,
    /// Location
    #[serde(rename = "location")]
    Location,
    /// Organization
    #[serde(rename = "organization")]
    Organization,
    /// RelatedPerson
    #[serde(rename = "relatedperson")]
    Relatedperson,
    /// Specimen
    #[serde(rename = "specimen")]
    Specimen,
}
impl Default for GroupType {
    fn default() -> Self {
        Self::Person
    }
}
