use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/consent-data-meaning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentDataMeaning {
    /// Instance
    #[serde(rename = "instance")]
    Instance,
    /// Related
    #[serde(rename = "related")]
    Related,
    /// Dependents
    #[serde(rename = "dependents")]
    Dependents,
    /// AuthoredBy
    #[serde(rename = "authoredby")]
    Authoredby,
}
impl Default for ConsentDataMeaning {
    fn default() -> Self {
        Self::Instance
    }
}
