use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/clinical-use-definition-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClinicalUseDefinitionType {
    /// Indication
    #[serde(rename = "indication")]
    Indication,
    /// Contraindication
    #[serde(rename = "contraindication")]
    Contraindication,
    /// Interaction
    #[serde(rename = "interaction")]
    Interaction,
    /// Undesirable Effect
    #[serde(rename = "undesirable-effect")]
    UndesirableEffect,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
}
impl Default for ClinicalUseDefinitionType {
    fn default() -> Self {
        Self::Indication
    }
}
