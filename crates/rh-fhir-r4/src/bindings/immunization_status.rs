use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/immunization-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmunizationStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    #[serde(rename = "not-done")]
    NotDone,
}
impl Default for ImmunizationStatus {
    fn default() -> Self {
        Self::Completed
    }
}
