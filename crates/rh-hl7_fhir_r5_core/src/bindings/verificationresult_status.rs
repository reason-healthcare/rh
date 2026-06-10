use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationresultStatus {
    /// Attested
    #[serde(rename = "attested")]
    Attested,
    /// Validated
    #[serde(rename = "validated")]
    Validated,
    /// In process
    #[serde(rename = "in-process")]
    InProcess,
    /// Requires revalidation
    #[serde(rename = "req-revalid")]
    ReqRevalid,
    /// Validation failed
    #[serde(rename = "val-fail")]
    ValFail,
    /// Re-Validation failed
    #[serde(rename = "reval-fail")]
    RevalFail,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for VerificationresultStatus {
    fn default() -> Self {
        Self::Attested
    }
}
