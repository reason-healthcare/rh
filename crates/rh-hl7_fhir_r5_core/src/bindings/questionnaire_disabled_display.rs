use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-disabled-display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionnaireDisabledDisplay {
    /// Hidden
    #[serde(rename = "hidden")]
    Hidden,
    /// Protected
    #[serde(rename = "protected")]
    Protected,
}
impl Default for QuestionnaireDisabledDisplay {
    fn default() -> Self {
        Self::Hidden
    }
}
