use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use serde::{Deserialize, Serialize};
/// UsageContext
///
/// Base StructureDefinition for UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/UsageContext
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: UsageContext
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Type of context being specified
    ///
    /// Binding: extensible (A code that specifies a type of context being specified by a usage context.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/usage-context-type
    pub code: Coding,
    /// Value that defines the context (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Value that defines the context (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value that defines the context (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value that defines the context (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
}

impl Default for UsageContext {
    fn default() -> Self {
        Self {
            base: Element::default(),
            code: Coding::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_reference: Default::default(),
        }
    }
}
