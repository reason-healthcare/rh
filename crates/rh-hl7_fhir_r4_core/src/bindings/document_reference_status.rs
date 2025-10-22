use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/document-reference-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentReferenceStatus {
    /// Current
    #[serde(rename = "current")]
    Current,
    /// Superseded
    #[serde(rename = "superseded")]
    Superseded,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for DocumentReferenceStatus {
    fn default() -> Self {
        Self::Current
    }
}
