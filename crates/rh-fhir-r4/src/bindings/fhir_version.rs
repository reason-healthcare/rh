use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/FHIR-version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FHIRVersion {
    /// 0.01
    #[serde(rename = "0.01")]
    Code001,
    /// 0.05
    #[serde(rename = "0.05")]
    Code005,
    /// 0.06
    #[serde(rename = "0.06")]
    Code006,
    /// 0.11
    #[serde(rename = "0.11")]
    Code011,
    /// 0.0.80
    #[serde(rename = "0.0.80")]
    Code0080,
    /// 0.0.81
    #[serde(rename = "0.0.81")]
    Code0081,
    /// 0.0.82
    #[serde(rename = "0.0.82")]
    Code0082,
    /// 0.4.0
    #[serde(rename = "0.4.0")]
    Code040,
    /// 0.5.0
    #[serde(rename = "0.5.0")]
    Code050,
    /// 1.0.0
    #[serde(rename = "1.0.0")]
    Code100,
    /// 1.0.1
    #[serde(rename = "1.0.1")]
    Code101,
    /// 1.0.2
    #[serde(rename = "1.0.2")]
    Code102,
    /// 1.1.0
    #[serde(rename = "1.1.0")]
    Code110,
    /// 1.4.0
    #[serde(rename = "1.4.0")]
    Code140,
    /// 1.6.0
    #[serde(rename = "1.6.0")]
    Code160,
    /// 1.8.0
    #[serde(rename = "1.8.0")]
    Code180,
    /// 3.0.0
    #[serde(rename = "3.0.0")]
    Code300,
    /// 3.0.1
    #[serde(rename = "3.0.1")]
    Code301,
    /// 3.3.0
    #[serde(rename = "3.3.0")]
    Code330,
    /// 3.5.0
    #[serde(rename = "3.5.0")]
    Code350,
    /// 4.0.0
    #[serde(rename = "4.0.0")]
    Code400,
    /// 4.0.1
    #[serde(rename = "4.0.1")]
    Code401,
}
impl Default for FHIRVersion {
    fn default() -> Self {
        Self::Code001
    }
}
