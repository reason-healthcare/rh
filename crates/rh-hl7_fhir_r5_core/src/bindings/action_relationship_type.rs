use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/action-relationship-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionRelationshipType {
    /// Before
    #[serde(rename = "before")]
    Before,
    /// Concurrent
    #[serde(rename = "concurrent")]
    Concurrent,
    /// After
    #[serde(rename = "after")]
    After,
}
impl Default for ActionRelationshipType {
    fn default() -> Self {
        Self::Before
    }
}
