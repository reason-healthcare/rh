use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/search-modifier-code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchModifierCode {
    /// Missing
    #[serde(rename = "missing")]
    Missing,
    /// Exact
    #[serde(rename = "exact")]
    Exact,
    /// Contains
    #[serde(rename = "contains")]
    Contains,
    /// Not
    #[serde(rename = "not")]
    Not,
    /// Text
    #[serde(rename = "text")]
    Text,
    /// In
    #[serde(rename = "in")]
    InValue,
    /// Not In
    #[serde(rename = "not-in")]
    NotIn,
    /// Below
    #[serde(rename = "below")]
    Below,
    /// Above
    #[serde(rename = "above")]
    Above,
    /// Type
    #[serde(rename = "type")]
    TypeValue,
    /// Identifier
    #[serde(rename = "identifier")]
    Identifier,
    /// Of Type
    #[serde(rename = "of-type")]
    OfType,
    /// Code Text
    #[serde(rename = "code-text")]
    CodeText,
    /// Text Advanced
    #[serde(rename = "text-advanced")]
    TextAdvanced,
    /// Iterate
    #[serde(rename = "iterate")]
    Iterate,
}
impl Default for SearchModifierCode {
    fn default() -> Self {
        Self::Missing
    }
}
