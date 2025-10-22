use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/linkage-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkageType {
    /// Source of Truth
    #[serde(rename = "source")]
    Source,
    /// Alternate Record
    #[serde(rename = "alternate")]
    Alternate,
    /// Historical/Obsolete Record
    #[serde(rename = "historical")]
    Historical,
}
impl Default for LinkageType {
    fn default() -> Self {
        Self::Source
    }
}
