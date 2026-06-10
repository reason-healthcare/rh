use crate::bindings::narrative_status::NarrativeStatus;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Narrative
///
/// Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Narrative
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Narrative
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// generated | extensions | additional | empty
    pub status: NarrativeStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Limited xhtml content
    pub div: StringType,
    /// Extension element for the 'div' primitive field. Contains metadata and extensions.
    pub _div: Option<Element>,
}

impl Default for Narrative {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            status: NarrativeStatus::default(),
            _status: Default::default(),
            div: StringType::default(),
            _div: Default::default(),
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
    rh_foundation::Invariant::new("txt-1", rh_foundation::Severity::Error, "The narrative SHALL contain only the basic html formatting elements and attributes described in chapters 7-11 (except section 4 of chapter 9) and 15 of the HTML 4.0 standard, <a> elements (either name or href), images and internally contained style attributes", "htmlChecks()"),
    rh_foundation::Invariant::new("txt-2", rh_foundation::Severity::Error, "The narrative SHALL have some non-whitespace content", "htmlChecks()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Narrative.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/narrative-status|5.0.0",
        )
        .with_description("The status of a resource narrative.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Narrative.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Narrative.extension", 0, None),
            rh_foundation::ElementCardinality::new("Narrative.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Narrative.div", 1, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Narrative {
    fn resource_type(&self) -> &'static str {
        "Narrative"
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
        Some("http://hl7.org/fhir/StructureDefinition/Narrative")
    }
}
