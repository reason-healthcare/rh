use crate::bindings::identifier_use::IdentifierUse;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Identifier
///
/// Base StructureDefinition for Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Identifier
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Identifier
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// usual | official | temp | secondary | old (If known)
    #[serde(rename = "use")]
    pub use_: Option<IdentifierUse>,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Description of identifier
    ///
    /// Binding: extensible (A coded type for an identifier that can be used to determine which identifier to use for a specific purpose.)
    ///
    /// Available values:
    /// - `DL`
    /// - `PPN`
    /// - `BRN`
    /// - `MR`
    /// - `MCN`
    /// - `EN`
    /// - `TAX`
    /// - `NIIP`
    /// - `PRN`
    /// - `MD`
    /// - ... and 8 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The namespace for the identifier value
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// The value that is unique
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Time period when id is/was valid for use
    pub period: Option<Period>,
    /// Organization that issued id (may be just text)
    pub assigner: Option<Box<Reference>>,
}
/// validDate
///
/// Indicates a date on which this identifier value was deemed to apply to this instance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/identifier-validDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierValidDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            base: Element::default(),
            use_: Default::default(),
            _use: Default::default(),
            type_: Default::default(),
            system: Default::default(),
            _system: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            period: Default::default(),
            assigner: Default::default(),
        }
    }
}

impl Default for IdentifierValidDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
