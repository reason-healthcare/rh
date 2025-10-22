use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/binding-strength
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindingStrength {
    /// Required
    #[serde(rename = "required")]
    Required,
    /// Extensible
    #[serde(rename = "extensible")]
    Extensible,
    /// Preferred
    #[serde(rename = "preferred")]
    Preferred,
    /// Example
    #[serde(rename = "example")]
    Example,
}
impl Default for BindingStrength {
    fn default() -> Self {
        Self::Required
    }
}
