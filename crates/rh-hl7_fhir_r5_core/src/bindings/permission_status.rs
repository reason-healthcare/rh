use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/permission-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
}
impl Default for PermissionStatus {
    fn default() -> Self {
        Self::Active
    }
}
