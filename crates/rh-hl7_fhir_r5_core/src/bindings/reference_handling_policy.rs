use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/reference-handling-policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceHandlingPolicy {
    /// Literal References
    #[serde(rename = "literal")]
    Literal,
    /// Logical References
    #[serde(rename = "logical")]
    Logical,
    /// Resolves References
    #[serde(rename = "resolves")]
    Resolves,
    /// Reference Integrity Enforced
    #[serde(rename = "enforced")]
    Enforced,
    /// Local References Only
    #[serde(rename = "local")]
    Local,
}
impl Default for ReferenceHandlingPolicy {
    fn default() -> Self {
        Self::Literal
    }
}
