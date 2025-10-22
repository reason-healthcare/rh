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
    /// Original Order
    #[serde(rename = "original-order")]
    OriginalOrder,
    /// Reflex Order
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    /// Filler Order
    #[serde(rename = "filler-order")]
    FillerOrder,
    /// Instance Order
    #[serde(rename = "instance-order")]
    InstanceOrder,
    /// Option
    #[serde(rename = "option")]
    Option,
}
impl Default for MedicationrequestIntent {
    fn default() -> Self {
        Self::Proposal
    }
}
