use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/list-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListStatus {
    /// Current
    #[serde(rename = "current")]
    Current,
    /// Retired
    #[serde(rename = "retired")]
    Retired,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ListStatus {
    fn default() -> Self {
        Self::Current
    }
}
