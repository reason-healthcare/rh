use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/clinicalimpression-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClinicalimpressionStatus {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ClinicalimpressionStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
