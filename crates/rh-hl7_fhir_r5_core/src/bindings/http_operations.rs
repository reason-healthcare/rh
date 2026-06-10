use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/http-operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpOperations {
    /// DELETE
    #[serde(rename = "delete")]
    Delete,
    /// GET
    #[serde(rename = "get")]
    Get,
    /// OPTIONS
    #[serde(rename = "options")]
    Options,
    /// PATCH
    #[serde(rename = "patch")]
    Patch,
    /// POST
    #[serde(rename = "post")]
    Post,
    /// PUT
    #[serde(rename = "put")]
    Put,
    /// HEAD
    #[serde(rename = "head")]
    Head,
}
impl Default for HttpOperations {
    fn default() -> Self {
        Self::Delete
    }
}
