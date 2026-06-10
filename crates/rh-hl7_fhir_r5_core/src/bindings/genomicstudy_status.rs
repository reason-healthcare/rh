use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenomicstudyStatus {
    /// Registered
    #[serde(rename = "registered")]
    Registered,
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for GenomicstudyStatus {
    fn default() -> Self {
        Self::Registered
    }
}
