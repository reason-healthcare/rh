use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-answer-constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionnaireAnswerConstraint {
    /// Options only
    #[serde(rename = "optionsOnly")]
    OptionsOnly,
    /// Options or 'type'
    #[serde(rename = "optionsOrType")]
    OptionsOrType,
    /// Options or string
    #[serde(rename = "optionsOrString")]
    OptionsOrString,
}
impl Default for QuestionnaireAnswerConstraint {
    fn default() -> Self {
        Self::OptionsOnly
    }
}
