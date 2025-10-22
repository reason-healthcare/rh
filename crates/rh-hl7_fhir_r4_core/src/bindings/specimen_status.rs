use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/specimen-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecimenStatus {
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Unavailable
    #[serde(rename = "unavailable")]
    Unavailable,
    /// Unsatisfactory
    #[serde(rename = "unsatisfactory")]
    Unsatisfactory,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for SpecimenStatus {
    fn default() -> Self {
        Self::Available
    }
}
