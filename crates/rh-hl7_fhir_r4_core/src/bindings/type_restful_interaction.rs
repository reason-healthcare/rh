use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/type-restful-interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeRestfulInteraction {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "vread")]
    Vread,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "history-instance")]
    HistoryInstance,
    #[serde(rename = "history-type")]
    HistoryType,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "search-type")]
    SearchType,
}
impl Default for TypeRestfulInteraction {
    fn default() -> Self {
        Self::Read
    }
}
