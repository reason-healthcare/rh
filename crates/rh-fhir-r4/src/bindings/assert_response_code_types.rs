use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/assert-response-code-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertResponseCodeTypes {
    /// okay
    #[serde(rename = "okay")]
    Okay,
    /// created
    #[serde(rename = "created")]
    Created,
    /// noContent
    #[serde(rename = "noContent")]
    NoContent,
    /// notModified
    #[serde(rename = "notModified")]
    NotModified,
    /// bad
    #[serde(rename = "bad")]
    Bad,
    /// forbidden
    #[serde(rename = "forbidden")]
    Forbidden,
    /// notFound
    #[serde(rename = "notFound")]
    NotFound,
    /// methodNotAllowed
    #[serde(rename = "methodNotAllowed")]
    MethodNotAllowed,
    /// conflict
    #[serde(rename = "conflict")]
    Conflict,
    /// gone
    #[serde(rename = "gone")]
    Gone,
    /// preconditionFailed
    #[serde(rename = "preconditionFailed")]
    PreconditionFailed,
    /// unprocessable
    #[serde(rename = "unprocessable")]
    Unprocessable,
}
impl Default for AssertResponseCodeTypes {
    fn default() -> Self {
        Self::Okay
    }
}
