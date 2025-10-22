use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/location-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationMode {
    /// Instance
    #[serde(rename = "instance")]
    Instance,
    /// Kind
    #[serde(rename = "kind")]
    Kind,
}
impl Default for LocationMode {
    fn default() -> Self {
        Self::Instance
    }
}
