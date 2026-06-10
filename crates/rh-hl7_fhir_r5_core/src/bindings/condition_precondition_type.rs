use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/condition-precondition-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionPreconditionType {
    /// Sensitive
    #[serde(rename = "sensitive")]
    Sensitive,
    /// Specific
    #[serde(rename = "specific")]
    Specific,
}
impl Default for ConditionPreconditionType {
    fn default() -> Self {
        Self::Sensitive
    }
}
