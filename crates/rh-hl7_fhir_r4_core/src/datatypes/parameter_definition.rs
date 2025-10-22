use crate::bindings::all_types::AllTypes;
use crate::bindings::operation_parameter_use::OperationParameterUse;
use crate::datatypes::element::Element;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ParameterDefinition
///
/// Base StructureDefinition for ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ParameterDefinition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ParameterDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Name used to access the parameter value
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// in | out
    #[serde(rename = "use")]
    pub use_: OperationParameterUse,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Minimum cardinality
    pub min: Option<IntegerType>,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Maximum cardinality (a number of *)
    pub max: Option<StringType>,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
    /// A brief description of the parameter
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
    /// What type of value
    #[serde(rename = "type")]
    pub type_: AllTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// What profile the value is expected to be
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
}

impl Default for ParameterDefinition {
    fn default() -> Self {
        Self {
            base: Element::default(),
            name: Default::default(),
            _name: Default::default(),
            use_: Default::default(),
            _use: Default::default(),
            min: Default::default(),
            _min: Default::default(),
            max: Default::default(),
            _max: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
        }
    }
}
