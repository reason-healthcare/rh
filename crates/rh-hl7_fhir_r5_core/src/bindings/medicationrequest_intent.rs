use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicationrequestIntent {
    /// Proposal
    #[serde(rename = "proposal")]
    Proposal,
    /// Plan
    #[serde(rename = "plan")]
    Plan,
    /// Order
    #[serde(rename = "order")]
    Order,
    /// Option
    #[serde(rename = "option")]
    Option,
}
impl Default for MedicationrequestIntent {
    fn default() -> Self {
        Self::Proposal
    }
}
