use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/relation-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    /// Triggers
    #[serde(rename = "triggers")]
    Triggers,
    /// Replaced By
    #[serde(rename = "is-replaced-by")]
    IsReplacedBy,
}
impl Default for RelationType {
    fn default() -> Self {
        Self::Triggers
    }
}
