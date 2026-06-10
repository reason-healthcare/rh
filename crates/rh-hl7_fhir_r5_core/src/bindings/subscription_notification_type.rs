use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/subscription-notification-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionNotificationType {
    /// Handshake
    #[serde(rename = "handshake")]
    Handshake,
    /// Heartbeat
    #[serde(rename = "heartbeat")]
    Heartbeat,
    /// Event Notification
    #[serde(rename = "event-notification")]
    EventNotification,
    /// Query Status
    #[serde(rename = "query-status")]
    QueryStatus,
    /// Query Event
    #[serde(rename = "query-event")]
    QueryEvent,
}
impl Default for SubscriptionNotificationType {
    fn default() -> Self {
        Self::Handshake
    }
}
