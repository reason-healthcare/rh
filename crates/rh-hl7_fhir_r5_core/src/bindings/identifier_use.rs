use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/identifier-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentifierUse {
    /// Usual
    #[serde(rename = "usual")]
    Usual,
    /// Official
    #[serde(rename = "official")]
    Official,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Secondary
    #[serde(rename = "secondary")]
    Secondary,
    /// Old
    #[serde(rename = "old")]
    Old,
}
impl Default for IdentifierUse {
    fn default() -> Self {
        Self::Usual
    }
}
