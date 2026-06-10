use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/event-capability-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCapabilityMode {
    /// Sender
    #[serde(rename = "sender")]
    Sender,
    /// Receiver
    #[serde(rename = "receiver")]
    Receiver,
}
impl Default for EventCapabilityMode {
    fn default() -> Self {
        Self::Sender
    }
}
