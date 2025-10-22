use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/permitted-data-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermittedDataType {
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
    /// CodeableConcept
    #[serde(rename = "CodeableConcept")]
    CodeableConcept,
    /// string
    #[serde(rename = "string")]
    String,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// Range
    #[serde(rename = "Range")]
    Range,
    /// Ratio
    #[serde(rename = "Ratio")]
    Ratio,
    /// SampledData
    #[serde(rename = "SampledData")]
    SampledData,
    /// time
    #[serde(rename = "time")]
    Time,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// Period
    #[serde(rename = "Period")]
    Period,
}
impl Default for PermittedDataType {
    fn default() -> Self {
        Self::Quantity
    }
}
