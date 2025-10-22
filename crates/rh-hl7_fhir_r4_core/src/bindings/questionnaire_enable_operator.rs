use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-enable-operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionnaireEnableOperator {
    /// Exists
    #[serde(rename = "exists")]
    Exists,
    /// Equals
    #[serde(rename = "=")]
    Equal,
    /// Not Equals
    #[serde(rename = "!=")]
    NotEqual,
    /// Greater Than
    #[serde(rename = ">")]
    GreaterThan,
    /// Less Than
    #[serde(rename = "<")]
    LessThan,
    /// Greater or Equals
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    /// Less or Equals
    #[serde(rename = "<=")]
    LessThanOrEqual,
}
impl Default for QuestionnaireEnableOperator {
    fn default() -> Self {
        Self::Exists
    }
}
