use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::instant::InstantType;
use serde::{Deserialize, Serialize};
/// Signature
///
/// Base StructureDefinition for Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Signature
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Signature
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Indication of the reason the entity signed the object(s)
    ///
    /// Binding: preferred (An indication of the reason that an entity signed the object.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/signature-type
    #[serde(rename = "type")]
    pub type_: Vec<Coding>,
    /// When the signature was created
    pub when: InstantType,
    /// Extension element for the 'when' primitive field. Contains metadata and extensions.
    pub _when: Option<Element>,
    /// Who signed
    pub who: Reference,
    /// The party represented
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
    /// The technical format of the signed resources
    #[serde(rename = "targetFormat")]
    pub target_format: Option<Mimetypes>,
    /// Extension element for the 'targetFormat' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetFormat")]
    pub _target_format: Option<Element>,
    /// The technical format of the signature
    #[serde(rename = "sigFormat")]
    pub sig_format: Option<Mimetypes>,
    /// Extension element for the 'sigFormat' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sigFormat")]
    pub _sig_format: Option<Element>,
    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    pub data: Option<Base64BinaryType>,
    /// Extension element for the 'data' primitive field. Contains metadata and extensions.
    pub _data: Option<Element>,
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            base: Element::default(),
            type_: Default::default(),
            when: InstantType::default(),
            _when: Default::default(),
            who: Reference::default(),
            on_behalf_of: Default::default(),
            target_format: Default::default(),
            _target_format: Default::default(),
            sig_format: Default::default(),
            _sig_format: Default::default(),
            data: Default::default(),
            _data: Default::default(),
        }
    }
}
