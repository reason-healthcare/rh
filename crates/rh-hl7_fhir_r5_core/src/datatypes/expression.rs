use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Expression
///
/// Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Expression
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Expression
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
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
    /// Available values:
    /// - `text/cql`: CQL
    /// - `text/fhirpath`: FHIRPath
    /// - `text/x-fhir-query`: FHIR Query
    /// - `text/cql-identifier`: CQL Identifier
    /// - `text/cql-expression`: CQL Expression
    pub language: Option<StringType>,
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
            base: DataType::default(),
            description: Default::default(),
            _description: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            language: Default::default(),
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
            ),
            rh_foundation::Invariant::new(
                "exp-1",
                rh_foundation::Severity::Error,
                "An expression or a reference must be provided",
                "expression.exists() or reference.exists()",
            ),
            rh_foundation::Invariant::new(
                "exp-2",
                rh_foundation::Severity::Error,
                "The name must be a valid variable name in most computer languages",
                "name.hasValue() implies name.matches('[A-Za-z][A-Za-z0-9\\\\_]{0,63}')",
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Expression.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Expression.extension", 0, None),
            rh_foundation::ElementCardinality::new("Expression.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Expression.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Expression.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Expression.expression", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Expression.reference", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Expression {
    fn resource_type(&self) -> &'static str {
        "Expression"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Expression")
    }
}
