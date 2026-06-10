use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/reference-version-rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceVersionRules {
    /// Either Specific or independent
    #[serde(rename = "either")]
    Either,
    /// Version independent
    #[serde(rename = "independent")]
    Independent,
    /// Version Specific
    #[serde(rename = "specific")]
    Specific,
}
impl Default for ReferenceVersionRules {
    fn default() -> Self {
        Self::Either
    }
}
