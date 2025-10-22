use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-relationship-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionRelationshipType {
    /// Before Start
    #[serde(rename = "before-start")]
    BeforeStart,
    /// Before
    #[serde(rename = "before")]
    Before,
    /// Before End
    #[serde(rename = "before-end")]
    BeforeEnd,
    /// Concurrent With Start
    #[serde(rename = "concurrent-with-start")]
    ConcurrentWithStart,
    /// Concurrent
    #[serde(rename = "concurrent")]
    Concurrent,
    /// Concurrent With End
    #[serde(rename = "concurrent-with-end")]
    ConcurrentWithEnd,
    /// After Start
    #[serde(rename = "after-start")]
    AfterStart,
    /// After
    #[serde(rename = "after")]
    After,
    /// After End
    #[serde(rename = "after-end")]
    AfterEnd,
}
impl Default for ActionRelationshipType {
    fn default() -> Self {
        Self::BeforeStart
    }
}
