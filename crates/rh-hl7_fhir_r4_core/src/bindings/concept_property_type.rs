use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/concept-property-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptPropertyType {
    /// code (internal reference)
    #[serde(rename = "code")]
    Code,
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
}
impl Default for ConceptPropertyType {
    fn default() -> Self {
        Self::Code
    }
}
