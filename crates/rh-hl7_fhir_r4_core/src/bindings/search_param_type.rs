use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-param-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchParamType {
    /// Number
    #[serde(rename = "number")]
    Number,
    /// Date/DateTime
    #[serde(rename = "date")]
    Date,
    /// String
    #[serde(rename = "string")]
    String,
    /// Token
    #[serde(rename = "token")]
    Token,
    /// Reference
    #[serde(rename = "reference")]
    Reference,
    /// Composite
    #[serde(rename = "composite")]
    Composite,
    /// Quantity
    #[serde(rename = "quantity")]
    Quantity,
    /// URI
    #[serde(rename = "uri")]
    Uri,
    /// Special
    #[serde(rename = "special")]
    Special,
}
impl Default for SearchParamType {
    fn default() -> Self {
        Self::Number
    }
}
