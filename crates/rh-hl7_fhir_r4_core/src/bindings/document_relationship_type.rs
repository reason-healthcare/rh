use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/document-relationship-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentRelationshipType {
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Signs
    #[serde(rename = "signs")]
    Signs,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
}
impl Default for DocumentRelationshipType {
    fn default() -> Self {
        Self::Replaces
    }
}
