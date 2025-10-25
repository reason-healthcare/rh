use crate::datatypes::element::Element;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Coding
///
/// Base StructureDefinition for Coding Type: A reference to a code defined by a terminology system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Coding
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Coding
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Identity of the terminology system
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// Version of the system - if relevant
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Symbol in syntax defined by the system
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Representation defined by the system
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// If this coding was chosen directly by the user
    #[serde(rename = "userSelected")]
    pub user_selected: Option<BooleanType>,
    /// Extension element for the 'userSelected' primitive field. Contains metadata and extensions.
    #[serde(rename = "_userSelected")]
    pub _user_selected: Option<Element>,
}

impl Default for Coding {
    fn default() -> Self {
        Self {
            base: Element::default(),
            system: Default::default(),
            _system: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            user_selected: Default::default(),
            _user_selected: Default::default(),
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

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Coding.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coding.extension", 0, None),
            rh_foundation::ElementCardinality::new("Coding.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coding.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coding.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coding.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coding.userSelected", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Coding {
    fn resource_type(&self) -> &'static str {
        "Coding"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Coding")
    }
}
