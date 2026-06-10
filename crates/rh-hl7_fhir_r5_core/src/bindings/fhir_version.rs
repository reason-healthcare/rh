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
    /// 0.0
    #[serde(rename = "0.0")]
    Code00,
    /// 0.4
    #[serde(rename = "0.4")]
    Code04,
    /// 0.5
    #[serde(rename = "0.5")]
    Code05,
    /// 1.0
    #[serde(rename = "1.0")]
    Code10,
    /// 1.1
    #[serde(rename = "1.1")]
    Code11,
    /// 1.4
    #[serde(rename = "1.4")]
    Code14,
    /// 1.6
    #[serde(rename = "1.6")]
    Code16,
    /// 1.8
    #[serde(rename = "1.8")]
    Code18,
    /// 3.0
    #[serde(rename = "3.0")]
    Code30,
    /// 3.3
    #[serde(rename = "3.3")]
    Code33,
    /// 3.5
    #[serde(rename = "3.5")]
    Code35,
    /// 4.0
    #[serde(rename = "4.0")]
    Code40,
    /// 4.1
    #[serde(rename = "4.1")]
    Code41,
    /// 4.2
    #[serde(rename = "4.2")]
    Code42,
    /// 4.3
    #[serde(rename = "4.3")]
    Code43,
    /// 4.4
    #[serde(rename = "4.4")]
    Code44,
    /// 4.5
    #[serde(rename = "4.5")]
    Code45,
    /// 4.6
    #[serde(rename = "4.6")]
    Code46,
    /// 5.0
    #[serde(rename = "5.0")]
    Code50,
}
impl Default for FHIRVersion {
    fn default() -> Self {
        Self::Code001
    }
}
