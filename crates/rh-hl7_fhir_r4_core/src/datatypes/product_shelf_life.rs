use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// ProductShelfLife
///
/// Base StructureDefinition for ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ProductShelfLife
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ProductShelfLife
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductShelfLife {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Unique identifier for the packaged Medicinal Product
    pub identifier: Option<Identifier>,
    /// This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    pub period: Quantity,
    /// Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "specialPrecautionsForStorage")]
    pub special_precautions_for_storage: Option<Vec<CodeableConcept>>,
}

impl Default for ProductShelfLife {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            type_: Default::default(),
            period: Quantity::default(),
            special_precautions_for_storage: Default::default(),
        }
    }
}
