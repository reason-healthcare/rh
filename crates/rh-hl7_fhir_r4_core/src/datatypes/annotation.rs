use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Annotation
///
/// Base StructureDefinition for Annotation Type: A  text note which also  contains information about who made the statement and when.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Annotation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Annotation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Individual responsible for the annotation (Reference)
    #[serde(rename = "authorReference")]
    pub author_reference: Option<Reference>,
    /// Individual responsible for the annotation (string)
    #[serde(rename = "authorString")]
    pub author_string: Option<StringType>,
    /// When the annotation was made
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// The annotation  - text content (as markdown)
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            base: Element::default(),
            author_reference: Default::default(),
            author_string: Default::default(),
            time: Default::default(),
            _time: Default::default(),
            text: StringType::default(),
            _text: Default::default(),
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

impl crate::validation::ValidatableResource for Annotation {
    fn resource_type(&self) -> &'static str {
        "Annotation"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Annotation")
    }
}
