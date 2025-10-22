use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Expression
///
/// Base StructureDefinition for Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Expression
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Expression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Natural language description of the condition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Short name assigned to expression for reuse
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// text/cql | text/fhirpath | application/x-fhir-query | etc.
    ///
    /// Binding: extensible (The media type of the expression language.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/expression-language
    pub language: StringType,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Expression in specified language
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// Where the expression is found
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            base: Element::default(),
            description: Default::default(),
            _description: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            language: StringType::default(),
            _language: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
        }
    }
}
