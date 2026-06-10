use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medication-statement-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationStatementStatus {
    /// Recorded
    #[serde(rename = "recorded")]
    Recorded,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
}
impl Default for MedicationStatementStatus {
    fn default() -> Self {
        Self::Recorded
    }
}
