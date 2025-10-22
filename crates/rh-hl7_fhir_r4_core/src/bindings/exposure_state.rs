use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/exposure-state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExposureState {
    /// Exposure
    #[serde(rename = "exposure")]
    Exposure,
    /// Exposure Alternative
    #[serde(rename = "exposure-alternative")]
    ExposureAlternative,
}
impl Default for ExposureState {
    fn default() -> Self {
        Self::Exposure
    }
}
