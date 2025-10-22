use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// rdf-type
///
/// The XML (schema) type of a property as used in RDF - used for the value attribute of a primitive type (for which there is no type in the FHIR typing system).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-rdf-type
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionRdfType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionRdfType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
