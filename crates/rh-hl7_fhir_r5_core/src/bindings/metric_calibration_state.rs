use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/metric-calibration-state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricCalibrationState {
    /// Not Calibrated
    #[serde(rename = "not-calibrated")]
    NotCalibrated,
    /// Calibration Required
    #[serde(rename = "calibration-required")]
    CalibrationRequired,
    /// Calibrated
    #[serde(rename = "calibrated")]
    Calibrated,
    /// Unspecified
    #[serde(rename = "unspecified")]
    Unspecified,
}
impl Default for MetricCalibrationState {
    fn default() -> Self {
        Self::NotCalibrated
    }
}
