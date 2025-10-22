use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medicationknowledge-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationknowledgeStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for MedicationknowledgeStatus {
    fn default() -> Self {
        Self::Active
    }
}
