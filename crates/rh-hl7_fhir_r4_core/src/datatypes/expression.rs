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
                "exp-1",
                rh_foundation::Severity::Error,
                "An expression or a reference must be provided",
                "expression.exists() or reference.exists()",
            )
            .with_xpath("exists(f:expression) or exists(f:reference)"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

impl crate::validation::ValidatableResource for Expression {
    fn resource_type(&self) -> &'static str {
        "Expression"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Expression")
    }
}
