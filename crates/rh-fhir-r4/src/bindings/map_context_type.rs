use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-context-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapContextType {
    /// Type
    #[serde(rename = "type")]
    Type,
    /// Variable
    #[serde(rename = "variable")]
    Variable,
}
impl Default for MapContextType {
    fn default() -> Self {
        Self::Type
    }
}
