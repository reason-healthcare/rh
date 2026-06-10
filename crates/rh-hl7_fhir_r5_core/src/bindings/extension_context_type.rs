use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/extension-context-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionContextType {
    /// FHIRPath
    #[serde(rename = "fhirpath")]
    Fhirpath,
    /// Element ID
    #[serde(rename = "element")]
    Element,
    /// Extension URL
    #[serde(rename = "extension")]
    Extension,
}
impl Default for ExtensionContextType {
    fn default() -> Self {
        Self::Fhirpath
    }
}
