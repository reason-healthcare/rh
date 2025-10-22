use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/metric-category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricCategory {
    /// Measurement
    #[serde(rename = "measurement")]
    Measurement,
    /// Setting
    #[serde(rename = "setting")]
    Setting,
    /// Calculation
    #[serde(rename = "calculation")]
    Calculation,
    /// Unspecified
    #[serde(rename = "unspecified")]
    Unspecified,
}
impl Default for MetricCategory {
    fn default() -> Self {
        Self::Measurement
    }
}
