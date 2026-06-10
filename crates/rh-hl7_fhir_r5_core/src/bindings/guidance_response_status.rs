use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/guidance-response-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuidanceResponseStatus {
    /// Success
    #[serde(rename = "success")]
    Success,
    /// Data Requested
    #[serde(rename = "data-requested")]
    DataRequested,
    /// Data Required
    #[serde(rename = "data-required")]
    DataRequired,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Failure
    #[serde(rename = "failure")]
    Failure,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for GuidanceResponseStatus {
    fn default() -> Self {
        Self::Success
    }
}
