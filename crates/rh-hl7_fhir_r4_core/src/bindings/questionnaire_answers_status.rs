use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-answers-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionnaireAnswersStatus {
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Amended
    #[serde(rename = "amended")]
    Amended,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Stopped
    #[serde(rename = "stopped")]
    Stopped,
}
impl Default for QuestionnaireAnswersStatus {
    fn default() -> Self {
        Self::InProgress
    }
}
