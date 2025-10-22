use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/operation-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationKind {
    /// Operation
    #[serde(rename = "operation")]
    Operation,
    /// Query
    #[serde(rename = "query")]
    Query,
}
impl Default for OperationKind {
    fn default() -> Self {
        Self::Operation
    }
}
