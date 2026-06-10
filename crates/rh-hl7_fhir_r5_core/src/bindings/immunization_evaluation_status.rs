use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/immunization-evaluation-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmunizationEvaluationStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ImmunizationEvaluationStatus {
    fn default() -> Self {
        Self::Completed
    }
}
