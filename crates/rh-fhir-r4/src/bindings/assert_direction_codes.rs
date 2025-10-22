use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/assert-direction-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertDirectionCodes {
    /// response
    #[serde(rename = "response")]
    Response,
    /// request
    #[serde(rename = "request")]
    Request,
}
impl Default for AssertDirectionCodes {
    fn default() -> Self {
        Self::Response
    }
}
