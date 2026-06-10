use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/interaction-trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionTrigger {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}
impl Default for InteractionTrigger {
    fn default() -> Self {
        Self::Create
    }
}
