use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/vision-base-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisionBaseCodes {
    /// Up
    #[serde(rename = "up")]
    Up,
    /// Down
    #[serde(rename = "down")]
    Down,
    /// In
    #[serde(rename = "in")]
    In,
    /// Out
    #[serde(rename = "out")]
    Out,
}
impl Default for VisionBaseCodes {
    fn default() -> Self {
        Self::Up
    }
}
