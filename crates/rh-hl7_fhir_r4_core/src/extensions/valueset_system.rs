use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// system
///
/// Allows a direct reference to the code system for FHIR query.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-system
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// systemRef
///
/// The formal URI for the code system.  I.e. ValueSet.codeSystem.system (or its equivalent).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-systemRef
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSystemRef {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// systemName
///
/// The human-readable name for the code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-systemName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSystemName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSystem {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
