use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/codesystem-content-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodesystemContentMode {
    /// Not Present
    #[serde(rename = "not-present")]
    NotPresent,
    /// Example
    #[serde(rename = "example")]
    Example,
    /// Fragment
    #[serde(rename = "fragment")]
    Fragment,
    /// Complete
    #[serde(rename = "complete")]
    Complete,
    /// Supplement
    #[serde(rename = "supplement")]
    Supplement,
}
impl Default for CodesystemContentMode {
    fn default() -> Self {
        Self::NotPresent
    }
}
