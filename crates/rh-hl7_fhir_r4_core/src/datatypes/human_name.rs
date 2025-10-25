use crate::bindings::name_use::NameUse;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// HumanName
///
/// Base StructureDefinition for HumanName Type: A human's name with the ability to identify parts and usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HumanName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: HumanName
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// usual | official | temp | nickname | anonymous | old | maiden
    #[serde(rename = "use")]
    pub use_: Option<NameUse>,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Text representation of the full name
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Family name (often called 'Surname')
    pub family: Option<StringType>,
    /// Extension element for the 'family' primitive field. Contains metadata and extensions.
    pub _family: Option<Element>,
    /// Given names (not always 'first'). Includes middle names
    pub given: Option<Vec<StringType>>,
    /// Extension element for the 'given' primitive field. Contains metadata and extensions.
    pub _given: Option<Element>,
    /// Parts that come before the name
    pub prefix: Option<Vec<StringType>>,
    /// Extension element for the 'prefix' primitive field. Contains metadata and extensions.
    pub _prefix: Option<Element>,
    /// Parts that come after the name
    pub suffix: Option<Vec<StringType>>,
    /// Extension element for the 'suffix' primitive field. Contains metadata and extensions.
    pub _suffix: Option<Element>,
    /// Time period when name was/is in use
    pub period: Option<Period>,
}

impl Default for HumanName {
    fn default() -> Self {
        Self {
            base: Element::default(),
            use_: Default::default(),
            _use: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            family: Default::default(),
            _family: Default::default(),
            given: Default::default(),
            _given: Default::default(),
            prefix: Default::default(),
            _prefix: Default::default(),
            suffix: Default::default(),
            _suffix: Default::default(),
            period: Default::default(),
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
        vec![rh_foundation::ElementBinding::new(
            "HumanName.use",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/name-use|4.0.1",
        )
        .with_description("The use of a human name.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("HumanName.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("HumanName.extension", 0, None),
            rh_foundation::ElementCardinality::new("HumanName.use", 0, Some(1)),
            rh_foundation::ElementCardinality::new("HumanName.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("HumanName.family", 0, Some(1)),
            rh_foundation::ElementCardinality::new("HumanName.given", 0, None),
            rh_foundation::ElementCardinality::new("HumanName.prefix", 0, None),
            rh_foundation::ElementCardinality::new("HumanName.suffix", 0, None),
            rh_foundation::ElementCardinality::new("HumanName.period", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for HumanName {
    fn resource_type(&self) -> &'static str {
        "HumanName"
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
        Some("http://hl7.org/fhir/StructureDefinition/HumanName")
    }
}
