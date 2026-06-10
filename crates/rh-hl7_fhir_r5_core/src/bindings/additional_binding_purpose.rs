use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/additional-binding-purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdditionalBindingPurpose {
    /// Maximum Binding
    #[serde(rename = "maximum")]
    Maximum,
    /// Minimum Binding
    #[serde(rename = "minimum")]
    Minimum,
    /// Required Binding
    #[serde(rename = "required")]
    Required,
    /// Conformance Binding
    #[serde(rename = "extensible")]
    Extensible,
    /// Candidate Binding
    #[serde(rename = "candidate")]
    Candidate,
    /// Current Binding
    #[serde(rename = "current")]
    Current,
    /// Preferred Binding
    #[serde(rename = "preferred")]
    Preferred,
    /// UI Suggested Binding
    #[serde(rename = "ui")]
    Ui,
    /// Starter Binding
    #[serde(rename = "starter")]
    Starter,
    /// Component Binding
    #[serde(rename = "component")]
    Component,
}
impl Default for AdditionalBindingPurpose {
    fn default() -> Self {
        Self::Maximum
    }
}
