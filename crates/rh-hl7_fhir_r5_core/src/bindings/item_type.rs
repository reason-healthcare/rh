use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/item-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    /// Group
    #[serde(rename = "group")]
    Group,
    /// Display
    #[serde(rename = "display")]
    Display,
    /// Question
    #[serde(rename = "question")]
    Question,
}
impl Default for ItemType {
    fn default() -> Self {
        Self::Group
    }
}
