use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/participationstatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Participationstatus {
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Tentative
    #[serde(rename = "tentative")]
    Tentative,
    /// Needs Action
    #[serde(rename = "needs-action")]
    NeedsAction,
}
impl Default for Participationstatus {
    fn default() -> Self {
        Self::Accepted
    }
}
