use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/devicedefinition-regulatory-identifier-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevicedefinitionRegulatoryIdentifierType {
    /// Basic
    #[serde(rename = "basic")]
    Basic,
    /// Master
    #[serde(rename = "master")]
    Master,
    /// License
    #[serde(rename = "license")]
    License,
}
impl Default for DevicedefinitionRegulatoryIdentifierType {
    fn default() -> Self {
        Self::Basic
    }
}
