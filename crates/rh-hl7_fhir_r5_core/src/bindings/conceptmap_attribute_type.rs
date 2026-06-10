use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conceptmap-attribute-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptmapAttributeType {
    /// code
    #[serde(rename = "code")]
    Code,
    /// Coding
    #[serde(rename = "Coding")]
    Coding,
    /// string
    #[serde(rename = "string")]
    String,
    /// boolean
    #[serde(rename = "boolean")]
    Boolean,
    /// Quantity
    #[serde(rename = "Quantity")]
    Quantity,
}
impl Default for ConceptmapAttributeType {
    fn default() -> Self {
        Self::Code
    }
}
