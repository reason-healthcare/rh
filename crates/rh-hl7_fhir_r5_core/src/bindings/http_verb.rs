use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/http-verb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpVerb {
    /// GET
    #[serde(rename = "GET")]
    GET,
    /// HEAD
    #[serde(rename = "HEAD")]
    HEAD,
    /// POST
    #[serde(rename = "POST")]
    POST,
    /// PUT
    #[serde(rename = "PUT")]
    PUT,
    /// DELETE
    #[serde(rename = "DELETE")]
    DELETE,
    /// PATCH
    #[serde(rename = "PATCH")]
    PATCH,
}
impl Default for HttpVerb {
    fn default() -> Self {
        Self::GET
    }
}
