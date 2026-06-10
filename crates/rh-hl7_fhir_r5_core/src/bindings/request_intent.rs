use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/request-intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestIntent {
    /// Proposal
    #[serde(rename = "proposal")]
    Proposal,
    /// Plan
    #[serde(rename = "plan")]
    Plan,
    /// Directive
    #[serde(rename = "directive")]
    Directive,
    /// Order
    #[serde(rename = "order")]
    Order,
    /// Option
    #[serde(rename = "option")]
    Option,
}
impl Default for RequestIntent {
    fn default() -> Self {
        Self::Proposal
    }
}
