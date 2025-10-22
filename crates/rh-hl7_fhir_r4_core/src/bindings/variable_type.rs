use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/variable-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    /// Dichotomous
    #[serde(rename = "dichotomous")]
    Dichotomous,
    /// Continuous
    #[serde(rename = "continuous")]
    Continuous,
    /// Descriptive
    #[serde(rename = "descriptive")]
    Descriptive,
}
impl Default for VariableType {
    fn default() -> Self {
        Self::Dichotomous
    }
}
