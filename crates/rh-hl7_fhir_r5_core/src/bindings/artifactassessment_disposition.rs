use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/artifactassessment-disposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactassessmentDisposition {
    /// Unresolved
    #[serde(rename = "unresolved")]
    Unresolved,
    /// Not Persuasive
    #[serde(rename = "not-persuasive")]
    NotPersuasive,
    /// Persuasive
    #[serde(rename = "persuasive")]
    Persuasive,
    /// Persuasive with Modification
    #[serde(rename = "persuasive-with-modification")]
    PersuasiveWithModification,
    /// Not Persuasive with Modification
    #[serde(rename = "not-persuasive-with-modification")]
    NotPersuasiveWithModification,
}
impl Default for ArtifactassessmentDisposition {
    fn default() -> Self {
        Self::Unresolved
    }
}
