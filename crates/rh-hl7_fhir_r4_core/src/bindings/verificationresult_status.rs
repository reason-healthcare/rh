use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/verificationresult-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationresultStatus {
    Unknown,
}
impl Default for VerificationresultStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
