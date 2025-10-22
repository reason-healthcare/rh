use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/supplyrequest-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplyrequestStatus {
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Suspended
    #[serde(rename = "suspended")]
    Suspended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for SupplyrequestStatus {
    fn default() -> Self {
        Self::Draft
    }
}
