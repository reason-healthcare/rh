use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/resource-slicing-rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceSlicingRules {
    /// Closed
    #[serde(rename = "closed")]
    Closed,
    /// Open
    #[serde(rename = "open")]
    Open,
    /// Open at End
    #[serde(rename = "openAtEnd")]
    OpenAtEnd,
}
impl Default for ResourceSlicingRules {
    fn default() -> Self {
        Self::Closed
    }
}
