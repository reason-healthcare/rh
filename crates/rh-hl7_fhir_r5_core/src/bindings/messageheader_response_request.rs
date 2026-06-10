use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/messageheader-response-request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageheaderResponseRequest {
    /// Always
    #[serde(rename = "always")]
    Always,
    /// Error/reject conditions only
    #[serde(rename = "on-error")]
    OnError,
    /// Never
    #[serde(rename = "never")]
    Never,
    /// Successful completion only
    #[serde(rename = "on-success")]
    OnSuccess,
}
impl Default for MessageheaderResponseRequest {
    fn default() -> Self {
        Self::Always
    }
}
