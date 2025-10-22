use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/subscription-channel-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionChannelType {
    /// Rest Hook
    #[serde(rename = "rest-hook")]
    RestHook,
    /// Websocket
    #[serde(rename = "websocket")]
    Websocket,
    /// Email
    #[serde(rename = "email")]
    Email,
    /// SMS
    #[serde(rename = "sms")]
    Sms,
    /// Message
    #[serde(rename = "message")]
    Message,
}
impl Default for SubscriptionChannelType {
    fn default() -> Self {
        Self::RestHook
    }
}
