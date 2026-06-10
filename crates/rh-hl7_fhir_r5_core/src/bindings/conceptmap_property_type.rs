use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conceptmap-property-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptmapPropertyType {
    /// Coding (external reference)
    #[serde(rename = "Coding")]
    Coding,
    /// string
    #[serde(rename = "string")]
    String,
    /// integer
    #[serde(rename = "integer")]
    Integer,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// dateTime
    #[serde(rename = "dateTime")]
    DateTime,
    /// decimal
    #[serde(rename = "decimal")]
    Decimal,
    /// code
    #[serde(rename = "code")]
    Code,
}
impl Default for ConceptmapPropertyType {
    fn default() -> Self {
        Self::Coding
    }
}
