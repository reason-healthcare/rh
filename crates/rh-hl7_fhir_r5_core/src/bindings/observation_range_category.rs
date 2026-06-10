use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/observation-range-category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservationRangeCategory {
    /// reference range
    #[serde(rename = "reference")]
    Reference,
    /// critical range
    #[serde(rename = "critical")]
    Critical,
    /// absolute range
    #[serde(rename = "absolute")]
    Absolute,
}
impl Default for ObservationRangeCategory {
    fn default() -> Self {
        Self::Reference
    }
}
