use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/name-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NameUse {
    /// Usual
    #[serde(rename = "usual")]
    Usual,
    /// Official
    #[serde(rename = "official")]
    Official,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Nickname
    #[serde(rename = "nickname")]
    Nickname,
    /// Anonymous
    #[serde(rename = "anonymous")]
    Anonymous,
    /// Old
    #[serde(rename = "old")]
    Old,
}
impl Default for NameUse {
    fn default() -> Self {
        Self::Usual
    }
}
