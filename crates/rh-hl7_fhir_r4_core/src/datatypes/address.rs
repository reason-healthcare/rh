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
        vec![
            rh_foundation::ElementBinding::new(
                "Address.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/address-type|4.0.1",
            )
            .with_description("The type of an address (physical / postal)."),
            rh_foundation::ElementBinding::new(
                "Address.use",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/address-use|4.0.1",
            )
            .with_description("The use of an address."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Address.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.extension", 0, None),
            rh_foundation::ElementCardinality::new("Address.use", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.line", 0, None),
            rh_foundation::ElementCardinality::new("Address.city", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.district", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.state", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.postalCode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.country", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Address.period", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Address {
    fn resource_type(&self) -> &'static str {
        "Address"
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
        Some("http://hl7.org/fhir/StructureDefinition/Address")
    }
}
