use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/operation-parameter-scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationParameterScope {
    /// Instance
    #[serde(rename = "instance")]
    Instance,
    /// Type
    #[serde(rename = "type")]
    TypeValue,
    /// System
    #[serde(rename = "system")]
    System,
}
impl Default for OperationParameterScope {
    fn default() -> Self {
        Self::Instance
    }
}
