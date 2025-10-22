use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/metric-operational-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricOperationalStatus {
    /// On
    #[serde(rename = "on")]
    On,
    /// Off
    #[serde(rename = "off")]
    Off,
    /// Standby
    #[serde(rename = "standby")]
    Standby,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for MetricOperationalStatus {
    fn default() -> Self {
        Self::On
    }
}
