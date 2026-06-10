use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/coverage-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoverageKind {
    /// Insurance
    #[serde(rename = "insurance")]
    Insurance,
    /// Self-pay
    #[serde(rename = "self-pay")]
    SelfPay,
    /// Other
    #[serde(rename = "other")]
    Other,
}
impl Default for CoverageKind {
    fn default() -> Self {
        Self::Insurance
    }
}
