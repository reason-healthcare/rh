use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/type-derivation-rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeDerivationRule {
    /// Specialization
    #[serde(rename = "specialization")]
    Specialization,
    /// Constraint
    #[serde(rename = "constraint")]
    Constraint,
}
impl Default for TypeDerivationRule {
    fn default() -> Self {
        Self::Specialization
    }
}
