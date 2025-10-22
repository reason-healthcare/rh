use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/namingsystem-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NamingsystemType {
    /// Code System
    #[serde(rename = "codesystem")]
    Codesystem,
    /// Identifier
    #[serde(rename = "identifier")]
    Identifier,
    /// Root
    #[serde(rename = "root")]
    Root,
}
impl Default for NamingsystemType {
    fn default() -> Self {
        Self::Codesystem
    }
}
