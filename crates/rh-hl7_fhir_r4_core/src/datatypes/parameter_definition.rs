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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ParameterDefinition.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-types|4.0.1").with_description("A list of all the concrete types defined in this version of the FHIR specification - Abstract Types, Data Types and Resource Types."),
    rh_foundation::ElementBinding::new("ParameterDefinition.use", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/operation-parameter-use|4.0.1").with_description("Whether the parameter is input or output."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ParameterDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ParameterDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.use", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.min", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.max", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.documentation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ParameterDefinition.profile", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ParameterDefinition {
    fn resource_type(&self) -> &'static str {
        "ParameterDefinition"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ParameterDefinition")
    }
}
