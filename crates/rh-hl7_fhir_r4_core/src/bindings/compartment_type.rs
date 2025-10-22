use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/compartment-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompartmentType {
    /// Patient
    #[serde(rename = "Patient")]
    Patient,
    /// Encounter
    #[serde(rename = "Encounter")]
    Encounter,
    /// RelatedPerson
    #[serde(rename = "RelatedPerson")]
    RelatedPerson,
    /// Practitioner
    #[serde(rename = "Practitioner")]
    Practitioner,
    /// Device
    #[serde(rename = "Device")]
    Device,
}
impl Default for CompartmentType {
    fn default() -> Self {
        Self::Patient
    }
}
