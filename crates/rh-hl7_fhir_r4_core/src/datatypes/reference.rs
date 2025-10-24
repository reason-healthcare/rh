use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Reference
///
/// Base StructureDefinition for Reference Type: A reference from one resource to another.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Reference
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Reference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Type the reference refers to (e.g. "Patient")
    ///
    /// Binding: extensible (Aa resource (or, for logical models, the URI of the logical model).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/resource-types
    #[serde(rename = "type")]
    pub type_: Option<StringType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Logical reference, when literal reference is not known
    pub identifier: Option<Box<Identifier>>,
    /// Text alternative for the resource
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            base: Element::default(),
            reference: Default::default(),
            _reference: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            identifier: Default::default(),
            display: Default::default(),
            _display: Default::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("ref-1", rh_foundation::Severity::Error, "SHALL have a contained resource if a local reference is provided", "reference.startsWith('#').not() or (reference.substring(1).trace('url') in %rootResource.contained.id.trace('ids'))").with_xpath("not(starts-with(f:reference/@value, '#')) or exists(ancestor::*[self::f:entry or self::f:parameter]/f:resource/f:*/f:contained/f:*[f:id/@value=substring-after(current()/f:reference/@value, '#')]|/*/f:contained/f:*[f:id/@value=substring-after(current()/f:reference/@value, '#')])"),
]
    });

impl crate::validation::ValidatableResource for Reference {
    fn resource_type(&self) -> &'static str {
        "Reference"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Reference")
    }
}
