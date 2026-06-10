use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Reference
///
/// Reference Type: A reference from one resource to another.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Reference
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Reference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Type the reference refers to (e.g. "Patient") - must be a resource in resources
    ///
    /// Binding: extensible (Aa resource (or, for logical models, the URI of the logical model).)
    ///
    /// Available values:
    /// - `Account`
    /// - `ActivityDefinition`
    /// - `ActorDefinition`
    /// - `AdministrableProductDefinition`
    /// - `AdverseEvent`
    /// - `AllergyIntolerance`
    /// - `Appointment`
    /// - `AppointmentResponse`
    /// - `ArtifactAssessment`
    /// - `AuditEvent`
    /// - ... and 148 more values
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
            base: DataType::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("ref-1", rh_foundation::Severity::Error, "SHALL have a contained resource if a local reference is provided", "reference.exists()  implies (reference.startsWith('#').not() or (reference.substring(1).trace('url') in %rootResource.contained.id.trace('ids')) or (reference='#' and %rootResource!=%resource))"),
    rh_foundation::Invariant::new("ref-2", rh_foundation::Severity::Error, "At least one of reference, identifier and display SHALL be present (unless an extension is provided).", "reference.exists() or identifier.exists() or display.exists() or extension.exists()"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Reference.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Reference.extension", 0, None),
            rh_foundation::ElementCardinality::new("Reference.reference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Reference.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Reference.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Reference.display", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Reference {
    fn resource_type(&self) -> &'static str {
        "Reference"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Reference")
    }
}
