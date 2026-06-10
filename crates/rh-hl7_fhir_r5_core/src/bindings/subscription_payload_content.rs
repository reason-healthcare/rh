use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/subscription-payload-content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionPayloadContent {
    /// Empty
    #[serde(rename = "empty")]
    Empty,
    /// Id-only
    #[serde(rename = "id-only")]
    IdOnly,
    /// Full-resource
    #[serde(rename = "full-resource")]
    FullResource,
}
impl Default for SubscriptionPayloadContent {
    fn default() -> Self {
        Self::Empty
    }
}
