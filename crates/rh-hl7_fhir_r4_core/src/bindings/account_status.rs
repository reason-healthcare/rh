use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/account-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// On Hold
    #[serde(rename = "on-hold")]
    OnHold,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for AccountStatus {
    fn default() -> Self {
        Self::Active
    }
}
