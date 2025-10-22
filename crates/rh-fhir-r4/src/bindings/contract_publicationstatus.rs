use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/contract-publicationstatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractPublicationstatus {
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Appended
    #[serde(rename = "appended")]
    Appended,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Disputed
    #[serde(rename = "disputed")]
    Disputed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Executable
    #[serde(rename = "executable")]
    Executable,
    /// Executed
    #[serde(rename = "executed")]
    Executed,
    /// Negotiable
    #[serde(rename = "negotiable")]
    Negotiable,
    /// Offered
    #[serde(rename = "offered")]
    Offered,
    /// Policy
    #[serde(rename = "policy")]
    Policy,
    /// Rejected
    #[serde(rename = "rejected")]
    Rejected,
    /// Renewed
    #[serde(rename = "renewed")]
    Renewed,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Resolved
    #[serde(rename = "resolved")]
    Resolved,
    /// Terminated
    #[serde(rename = "terminated")]
    Terminated,
}
impl Default for ContractPublicationstatus {
    fn default() -> Self {
        Self::Amended
    }
}
