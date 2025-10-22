use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/response-code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseCode {
    /// OK
    #[serde(rename = "ok")]
    Ok,
    /// Transient Error
    #[serde(rename = "transient-error")]
    TransientError,
    /// Fatal Error
    #[serde(rename = "fatal-error")]
    FatalError,
}
impl Default for ResponseCode {
    fn default() -> Self {
        Self::Ok
    }
}
