use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/message-significance-category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageSignificanceCategory {
    /// Consequence
    #[serde(rename = "consequence")]
    Consequence,
    /// Currency
    #[serde(rename = "currency")]
    Currency,
    /// Notification
    #[serde(rename = "notification")]
    Notification,
}
impl Default for MessageSignificanceCategory {
    fn default() -> Self {
        Self::Consequence
    }
}
