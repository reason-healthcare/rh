use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/graph-compartment-rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphCompartmentRule {
    /// Identical
    #[serde(rename = "identical")]
    Identical,
    /// Matching
    #[serde(rename = "matching")]
    Matching,
    /// Different
    #[serde(rename = "different")]
    Different,
    /// Custom
    #[serde(rename = "custom")]
    Custom,
}
impl Default for GraphCompartmentRule {
    fn default() -> Self {
        Self::Identical
    }
}
