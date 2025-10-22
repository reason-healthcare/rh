use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/system-restful-interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemRestfulInteraction {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "search-system")]
    SearchSystem,
    #[serde(rename = "history-system")]
    HistorySystem,
}
impl Default for SystemRestfulInteraction {
    fn default() -> Self {
        Self::Transaction
    }
}
