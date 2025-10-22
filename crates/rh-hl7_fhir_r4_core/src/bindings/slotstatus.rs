use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/slotstatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Slotstatus {
    /// Busy
    #[serde(rename = "busy")]
    Busy,
    /// Free
    #[serde(rename = "free")]
    Free,
    /// Busy (Unavailable)
    #[serde(rename = "busy-unavailable")]
    BusyUnavailable,
    /// Busy (Tentative)
    #[serde(rename = "busy-tentative")]
    BusyTentative,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for Slotstatus {
    fn default() -> Self {
        Self::Busy
    }
}
