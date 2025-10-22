use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/vision-eye-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisionEyeCodes {
    /// Right Eye
    #[serde(rename = "right")]
    Right,
    /// Left Eye
    #[serde(rename = "left")]
    Left,
}
impl Default for VisionEyeCodes {
    fn default() -> Self {
        Self::Right
    }
}
