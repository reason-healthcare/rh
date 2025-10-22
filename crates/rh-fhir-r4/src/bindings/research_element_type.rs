use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/research-element-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchElementType {
    /// Population
    #[serde(rename = "population")]
    Population,
    /// Exposure
    #[serde(rename = "exposure")]
    Exposure,
    /// Outcome
    #[serde(rename = "outcome")]
    Outcome,
}
impl Default for ResearchElementType {
    fn default() -> Self {
        Self::Population
    }
}
