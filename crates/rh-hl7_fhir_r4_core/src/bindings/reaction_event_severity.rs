use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/reaction-event-severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionEventSeverity {
    /// Mild
    #[serde(rename = "mild")]
    Mild,
    /// Moderate
    #[serde(rename = "moderate")]
    Moderate,
    /// Severe
    #[serde(rename = "severe")]
    Severe,
}
impl Default for ReactionEventSeverity {
    fn default() -> Self {
        Self::Mild
    }
}
