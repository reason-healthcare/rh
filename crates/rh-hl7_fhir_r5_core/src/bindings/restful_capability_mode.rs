use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/restful-capability-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestfulCapabilityMode {
    /// Client
    #[serde(rename = "client")]
    Client,
    /// Server
    #[serde(rename = "server")]
    Server,
}
impl Default for RestfulCapabilityMode {
    fn default() -> Self {
        Self::Client
    }
}
