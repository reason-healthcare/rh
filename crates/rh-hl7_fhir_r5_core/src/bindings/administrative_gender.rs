use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/administrative-gender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdministrativeGender {
    /// Male
    #[serde(rename = "male")]
    Male,
    /// Female
    #[serde(rename = "female")]
    Female,
    /// Other
    #[serde(rename = "other")]
    Other,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for AdministrativeGender {
    fn default() -> Self {
        Self::Male
    }
}
