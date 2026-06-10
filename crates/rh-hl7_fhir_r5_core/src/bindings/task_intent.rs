use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/task-intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskIntent {
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "proposal")]
    Proposal,
    #[serde(rename = "plan")]
    Plan,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "original-order")]
    OriginalOrder,
    #[serde(rename = "reflex-order")]
    ReflexOrder,
    #[serde(rename = "filler-order")]
    FillerOrder,
    #[serde(rename = "instance-order")]
    InstanceOrder,
    #[serde(rename = "option")]
    Option,
}
impl Default for TaskIntent {
    fn default() -> Self {
        Self::Unknown
    }
}
