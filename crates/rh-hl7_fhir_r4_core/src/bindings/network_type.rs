use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/network-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    /// Machine Name
    #[serde(rename = "1")]
    Code1,
    /// IP Address
    #[serde(rename = "2")]
    Code2,
    /// Telephone Number
    #[serde(rename = "3")]
    Code3,
    /// Email address
    #[serde(rename = "4")]
    Code4,
    /// URI
    #[serde(rename = "5")]
    Code5,
}
impl Default for NetworkType {
    fn default() -> Self {
        Self::Code1
    }
}
