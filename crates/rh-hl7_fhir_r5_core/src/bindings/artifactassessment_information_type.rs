use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/artifactassessment-information-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactassessmentInformationType {
    /// Comment
    #[serde(rename = "comment")]
    Comment,
    /// Classifier
    #[serde(rename = "classifier")]
    Classifier,
    /// Rating
    #[serde(rename = "rating")]
    Rating,
    /// Container
    #[serde(rename = "container")]
    Container,
    /// Response
    #[serde(rename = "response")]
    Response,
    /// Change Request
    #[serde(rename = "change-request")]
    ChangeRequest,
}
impl Default for ArtifactassessmentInformationType {
    fn default() -> Self {
        Self::Comment
    }
}
