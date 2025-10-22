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
