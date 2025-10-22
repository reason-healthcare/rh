use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/code-search-support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeSearchSupport {
    /// Explicit Codes
    #[serde(rename = "explicit")]
    Explicit,
    /// Implicit Codes
    #[serde(rename = "all")]
    All,
}
impl Default for CodeSearchSupport {
    fn default() -> Self {
        Self::Explicit
    }
}
