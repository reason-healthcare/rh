use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/resource-aggregation-mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceAggregationMode {
    /// Contained
    #[serde(rename = "contained")]
    Contained,
    /// Referenced
    #[serde(rename = "referenced")]
    Referenced,
}
impl Default for ResourceAggregationMode {
    fn default() -> Self {
        Self::Contained
    }
}
