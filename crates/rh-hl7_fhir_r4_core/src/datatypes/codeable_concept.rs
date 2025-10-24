use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// CodeableConcept
///
/// Base StructureDefinition for CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CodeableConcept
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: CodeableConcept
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Code defined by a terminology system
    pub coding: Option<Vec<Coding>>,
    /// Plain text representation of the concept
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}

impl Default for CodeableConcept {
    fn default() -> Self {
        Self {
            base: Element::default(),
            coding: Default::default(),
            text: Default::default(),
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

impl crate::validation::ValidatableResource for CodeableConcept {
    fn resource_type(&self) -> &'static str {
        "CodeableConcept"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/CodeableConcept")
    }
}
