use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/composition-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionStatus {
    /// Registered
    #[serde(rename = "registered")]
    Registered,
    /// Partial
    #[serde(rename = "partial")]
    Partial,
    /// Final
    #[serde(rename = "final")]
    FinalValue,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Deprecated
    #[serde(rename = "deprecated")]
    Deprecated,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for CompositionStatus {
    fn default() -> Self {
        Self::Registered
    }
}
