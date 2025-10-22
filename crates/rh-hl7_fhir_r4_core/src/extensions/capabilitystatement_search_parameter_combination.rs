use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Search Parameter Combination
///
/// This extension defines a possible search parameter combination,  by listing a set of search parameters and indicating whether they are required or optional. If a search combination is specified, clients should expect that they must submit a search that meets one of the required combinations or the search will be unsuccessful. If multiple search parameter combinations are specified, a client may pick between them, and supply the minimal required parameters for any of the combinations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/capabilitystatement-search-parameter-combination
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitystatementSearchParameterCombination {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CapabilitystatementSearchParameterCombination {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
