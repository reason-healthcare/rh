use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/capability-statement-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityStatementKind {
    /// Instance
    #[serde(rename = "instance")]
    Instance,
    /// Capability
    #[serde(rename = "capability")]
    Capability,
    /// Requirements
    #[serde(rename = "requirements")]
    Requirements,
}
impl Default for CapabilityStatementKind {
    fn default() -> Self {
        Self::Instance
    }
}
