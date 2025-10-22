use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// EN-qualifier
///
/// A set of codes each of which specifies a certain subcategory of the name part in addition to the main name part type.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-EN-qualifier
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ENQualifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ENQualifier {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
