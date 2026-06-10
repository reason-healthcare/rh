use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/assert-manual-completion-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertManualCompletionCodes {
    /// Fail
    #[serde(rename = "fail")]
    Fail,
    /// Pass
    #[serde(rename = "pass")]
    Pass,
    /// Skip
    #[serde(rename = "skip")]
    Skip,
    /// Stop
    #[serde(rename = "stop")]
    Stop,
}
impl Default for AssertManualCompletionCodes {
    fn default() -> Self {
        Self::Fail
    }
}
