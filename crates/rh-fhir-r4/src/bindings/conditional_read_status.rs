use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conditional-read-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalReadStatus {
    /// Not Supported
    #[serde(rename = "not-supported")]
    NotSupported,
    /// If-Modified-Since
    #[serde(rename = "modified-since")]
    ModifiedSince,
    /// If-None-Match
    #[serde(rename = "not-match")]
    NotMatch,
    /// Full Support
    #[serde(rename = "full-support")]
    FullSupport,
}
impl Default for ConditionalReadStatus {
    fn default() -> Self {
        Self::NotSupported
    }
}
