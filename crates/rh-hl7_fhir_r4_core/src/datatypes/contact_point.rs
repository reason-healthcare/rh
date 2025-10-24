use crate::bindings::contact_point_system::ContactPointSystem;
use crate::bindings::contact_point_use::ContactPointUse;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ContactPoint
///
/// Base StructureDefinition for ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ContactPoint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ContactPoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// phone | fax | email | pager | url | sms | other
    pub system: Option<ContactPointSystem>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// The actual contact point details
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// home | work | temp | old | mobile - purpose of this contact point
    #[serde(rename = "use")]
    pub use_: Option<ContactPointUse>,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Specify preferred order of use (1 = highest)
    pub rank: Option<PositiveIntType>,
    /// Extension element for the 'rank' primitive field. Contains metadata and extensions.
    pub _rank: Option<Element>,
    /// Time period when the contact point was/is in use
    pub period: Option<Period>,
}

impl Default for ContactPoint {
    fn default() -> Self {
        Self {
            base: Element::default(),
            system: Default::default(),
            _system: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            use_: Default::default(),
            _use: Default::default(),
            rank: Default::default(),
            _rank: Default::default(),
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
                "cpt-2",
                rh_foundation::Severity::Error,
                "A system is required if a value is provided.",
                "value.empty() or system.exists()",
            )
            .with_xpath("not(exists(f:value)) or exists(f:system)"),
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

impl crate::validation::ValidatableResource for ContactPoint {
    fn resource_type(&self) -> &'static str {
        "ContactPoint"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ContactPoint")
    }
}
