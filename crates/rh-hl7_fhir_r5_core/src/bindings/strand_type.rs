use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/strand-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrandType {
    /// Watson strand of starting sequence
    #[serde(rename = "watson")]
    Watson,
    /// Crick strand of starting sequence
    #[serde(rename = "crick")]
    Crick,
}
impl Default for StrandType {
    fn default() -> Self {
        Self::Watson
    }
}
