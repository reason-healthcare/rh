use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Preferred Contact
///
/// This Contact is the preferred contact at this organization for the purpose of the contact  There can be multiple contacts on an Organizations record with this value set to true, but these should all have different purpose values.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/organization-preferredContact
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationPreferredContact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OrganizationPreferredContact {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
