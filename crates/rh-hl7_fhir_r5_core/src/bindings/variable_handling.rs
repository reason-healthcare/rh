use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/variable-handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableHandling {
    /// continuous variable
    #[serde(rename = "continuous")]
    Continuous,
    /// dichotomous variable
    #[serde(rename = "dichotomous")]
    Dichotomous,
    /// ordinal variable
    #[serde(rename = "ordinal")]
    Ordinal,
    /// polychotomous variable
    #[serde(rename = "polychotomous")]
    Polychotomous,
}
impl Default for VariableHandling {
    fn default() -> Self {
        Self::Continuous
    }
}
