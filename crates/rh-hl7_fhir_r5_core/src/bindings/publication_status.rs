use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/publication-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicationStatus {
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Retired
    #[serde(rename = "retired")]
    Retired,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for PublicationStatus {
    fn default() -> Self {
        Self::Draft
    }
}
