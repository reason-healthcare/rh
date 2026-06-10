use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conformance-expectation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConformanceExpectation {
    /// SHALL
    #[serde(rename = "SHALL")]
    SHALL,
    /// SHOULD
    #[serde(rename = "SHOULD")]
    SHOULD,
    /// MAY
    #[serde(rename = "MAY")]
    MAY,
    /// SHOULD-NOT
    #[serde(rename = "SHOULD-NOT")]
    SHOULDNOT,
}
impl Default for ConformanceExpectation {
    fn default() -> Self {
        Self::SHALL
    }
}
