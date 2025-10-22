use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/detectedissue-severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectedissueSeverity {
    /// High
    #[serde(rename = "high")]
    High,
    /// Moderate
    #[serde(rename = "moderate")]
    Moderate,
    /// Low
    #[serde(rename = "low")]
    Low,
}
impl Default for DetectedissueSeverity {
    fn default() -> Self {
        Self::High
    }
}
