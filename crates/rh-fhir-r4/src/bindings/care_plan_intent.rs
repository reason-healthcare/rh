use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/care-plan-intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarePlanIntent {
    #[serde(rename = "proposal")]
    Proposal,
    #[serde(rename = "plan")]
    Plan,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "option")]
    Option,
}
impl Default for CarePlanIntent {
    fn default() -> Self {
        Self::Proposal
    }
}
