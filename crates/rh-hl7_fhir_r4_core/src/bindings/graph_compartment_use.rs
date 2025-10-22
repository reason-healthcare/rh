use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/graph-compartment-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphCompartmentUse {
    /// Condition
    #[serde(rename = "condition")]
    Condition,
    /// Requirement
    #[serde(rename = "requirement")]
    Requirement,
}
impl Default for GraphCompartmentUse {
    fn default() -> Self {
        Self::Condition
    }
}
