use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/condition-questionnaire-purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionQuestionnairePurpose {
    /// Pre-admit
    #[serde(rename = "preadmit")]
    Preadmit,
    /// Diff Diagnosis
    #[serde(rename = "diff-diagnosis")]
    DiffDiagnosis,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
}
impl Default for ConditionQuestionnairePurpose {
    fn default() -> Self {
        Self::Preadmit
    }
}
