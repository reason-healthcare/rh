use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/metric-calibration-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricCalibrationType {
    /// Unspecified
    #[serde(rename = "unspecified")]
    Unspecified,
    /// Offset
    #[serde(rename = "offset")]
    Offset,
    /// Gain
    #[serde(rename = "gain")]
    Gain,
    /// Two Point
    #[serde(rename = "two-point")]
    TwoPoint,
}
impl Default for MetricCalibrationType {
    fn default() -> Self {
        Self::Unspecified
    }
}
