use crate::bindings::address_type::AddressType;
use crate::bindings::address_use::AddressUse;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Address
///
/// Base StructureDefinition for Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Address
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Address
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// home | work | temp | old | billing - purpose of this address
    #[serde(rename = "use")]
    pub use_: Option<AddressUse>,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// postal | physical | both
    #[serde(rename = "type")]
    pub type_: Option<AddressType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Text representation of the address
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Street name, number, direction & P.O. Box etc.
    pub line: Option<Vec<StringType>>,
    /// Extension element for the 'line' primitive field. Contains metadata and extensions.
    pub _line: Option<Element>,
    /// Name of city, town etc.
    pub city: Option<StringType>,
    /// Extension element for the 'city' primitive field. Contains metadata and extensions.
    pub _city: Option<Element>,
    /// District name (aka county)
    pub district: Option<StringType>,
    /// Extension element for the 'district' primitive field. Contains metadata and extensions.
    pub _district: Option<Element>,
    /// Sub-unit of country (abbreviations ok)
    pub state: Option<StringType>,
    /// Extension element for the 'state' primitive field. Contains metadata and extensions.
    pub _state: Option<Element>,
    /// Postal code for area
    #[serde(rename = "postalCode")]
    pub postal_code: Option<StringType>,
    /// Extension element for the 'postalCode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_postalCode")]
    pub _postal_code: Option<Element>,
    /// Country (e.g. can be ISO 3166 2 or 3 letter code)
    pub country: Option<StringType>,
    /// Extension element for the 'country' primitive field. Contains metadata and extensions.
    pub _country: Option<Element>,
    /// Time period when address was/is in use
    pub period: Option<Period>,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            base: Element::default(),
            use_: Default::default(),
            _use: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            line: Default::default(),
            _line: Default::default(),
            city: Default::default(),
            _city: Default::default(),
            district: Default::default(),
            _district: Default::default(),
            state: Default::default(),
            _state: Default::default(),
            postal_code: Default::default(),
            _postal_code: Default::default(),
            country: Default::default(),
            _country: Default::default(),
            period: Default::default(),
        }
    }
}
