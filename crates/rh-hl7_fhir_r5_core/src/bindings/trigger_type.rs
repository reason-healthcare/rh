use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/trigger-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerType {
    /// Named Event
    #[serde(rename = "named-event")]
    NamedEvent,
    /// Periodic
    #[serde(rename = "periodic")]
    Periodic,
    /// Data Changed
    #[serde(rename = "data-changed")]
    DataChanged,
    /// Data Accessed
    #[serde(rename = "data-accessed")]
    DataAccessed,
    /// Data Access Ended
    #[serde(rename = "data-access-ended")]
    DataAccessEnded,
}
impl Default for TriggerType {
    fn default() -> Self {
        Self::NamedEvent
    }
}
