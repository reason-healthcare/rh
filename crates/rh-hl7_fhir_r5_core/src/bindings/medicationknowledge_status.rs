use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medicationknowledge-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationknowledgeStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}
impl Default for MedicationknowledgeStatus {
    fn default() -> Self {
        Self::Active
    }
}
