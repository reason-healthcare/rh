use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/fm-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FmStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for FmStatus {
    fn default() -> Self {
        Self::Active
    }
}
