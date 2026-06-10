use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-enable-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionnaireEnableBehavior {
    /// All
    #[serde(rename = "all")]
    All,
    /// Any
    #[serde(rename = "any")]
    Any,
}
impl Default for QuestionnaireEnableBehavior {
    fn default() -> Self {
        Self::All
    }
}
