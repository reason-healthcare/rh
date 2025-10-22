use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/operation-parameter-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationParameterUse {
    /// In
    #[serde(rename = "in")]
    In,
    /// Out
    #[serde(rename = "out")]
    Out,
}
impl Default for OperationParameterUse {
    fn default() -> Self {
        Self::In
    }
}
