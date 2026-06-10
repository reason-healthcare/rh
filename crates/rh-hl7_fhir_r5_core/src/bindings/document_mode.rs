use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/document-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentMode {
    /// Producer
    #[serde(rename = "producer")]
    Producer,
    /// Consumer
    #[serde(rename = "consumer")]
    Consumer,
}
impl Default for DocumentMode {
    fn default() -> Self {
        Self::Producer
    }
}
