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
