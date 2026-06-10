use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/code-search-support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeSearchSupport {
    /// In Compose
    #[serde(rename = "in-compose")]
    InCompose,
    /// In Expansion
    #[serde(rename = "in-expansion")]
    InExpansion,
    /// In Compose Or Expansion
    #[serde(rename = "in-compose-or-expansion")]
    InComposeOrExpansion,
}
impl Default for CodeSearchSupport {
    fn default() -> Self {
        Self::InCompose
    }
}
