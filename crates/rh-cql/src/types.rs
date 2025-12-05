//! Type resolution for CQL semantic analysis.
//!
//! This module provides utilities for resolving CQL type specifiers to internal
//! `DataType` representations, and converting back to ELM `TypeSpecifier` for output.
//!
//! # Overview
//!
//! Type resolution involves:
//! 1. **AST to DataType**: Converting parsed type specifiers to internal types
//! 2. **DataType to ELM**: Converting internal types to ELM output format
//! 3. **Model Resolution**: Looking up FHIR/model types through ModelInfoProvider
//! 4. **Type Inference**: Determining result types for expressions
//!
//! # Example
//!
//! ```
//! use rh_cql::types::{TypeResolver, TypeBuilder};
//! use rh_cql::datatype::{DataType, SystemType};
//! use rh_cql::parser::ast;
//!
//! // Create a type resolver
//! let resolver = TypeResolver::new();
//!
//! // Resolve AST type specifier to DataType
//! let ast_type = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
//!     namespace: None,
//!     name: "Integer".to_string(),
//! });
//! let data_type = resolver.resolve_type_specifier(&ast_type);
//! assert!(data_type.is_ok());
//! assert_eq!(data_type.unwrap(), DataType::integer());
//!
//! // Convert DataType back to ELM TypeSpecifier
//! let elm_type = TypeBuilder::to_type_specifier(&DataType::integer());
//! assert!(elm_type.is_some());
//! ```

use crate::datatype::{DataType, SystemType, TupleElement};
use crate::elm;
use crate::parser::ast;
use crate::provider::ModelInfoProvider;

// ============================================================================
// Type Resolution Error
// ============================================================================

/// Errors that can occur during type resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    /// Unknown type name.
    UnknownType { name: String },
    /// Unknown model type.
    UnknownModelType { namespace: String, name: String },
    /// Invalid type specifier.
    InvalidTypeSpecifier { message: String },
    /// Model not found.
    ModelNotFound { name: String },
    /// Type parameter not resolved.
    UnresolvedTypeParameter { name: String },
    /// Incompatible types.
    IncompatibleTypes {
        expected: String,
        found: String,
        context: Option<String>,
    },
    /// Cannot infer type.
    CannotInferType { context: String },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnknownType { name } => {
                write!(f, "Unknown type: '{name}'")
            }
            TypeError::UnknownModelType { namespace, name } => {
                write!(f, "Unknown type '{name}' in model '{namespace}'")
            }
            TypeError::InvalidTypeSpecifier { message } => {
                write!(f, "Invalid type specifier: {message}")
            }
            TypeError::ModelNotFound { name } => {
                write!(f, "Model not found: '{name}'")
            }
            TypeError::UnresolvedTypeParameter { name } => {
                write!(f, "Unresolved type parameter: '{name}'")
            }
            TypeError::IncompatibleTypes {
                expected,
                found,
                context,
            } => {
                if let Some(ctx) = context {
                    write!(
                        f,
                        "Type mismatch in {ctx}: expected {expected}, found {found}"
                    )
                } else {
                    write!(f, "Type mismatch: expected {expected}, found {found}")
                }
            }
            TypeError::CannotInferType { context } => {
                write!(f, "Cannot infer type: {context}")
            }
        }
    }
}

impl std::error::Error for TypeError {}

/// Result type for type operations.
pub type TypeResult<T> = Result<T, TypeError>;

// ============================================================================
// Type Resolver
// ============================================================================

/// Resolves AST type specifiers to internal DataType representations.
///
/// The resolver handles:
/// - System types (Integer, String, Boolean, etc.)
/// - Model types (FHIR.Patient, FHIR.Observation, etc.)
/// - Collection types (List<T>, Interval<T>)
/// - Structural types (Tuple, Choice)
#[derive(Default)]
pub struct TypeResolver<'a> {
    /// Model information provider for resolving model types.
    model_provider: Option<&'a dyn ModelInfoProvider>,
    /// Registered model aliases (local name -> URI).
    model_aliases: std::collections::HashMap<String, String>,
    /// Default model namespace (typically "FHIR").
    default_model: Option<String>,
}

impl std::fmt::Debug for TypeResolver<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeResolver")
            .field("model_provider", &self.model_provider.is_some())
            .field("model_aliases", &self.model_aliases)
            .field("default_model", &self.default_model)
            .finish()
    }
}

impl<'a> TypeResolver<'a> {
    /// Create a new type resolver without model support.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a type resolver with a model provider.
    pub fn with_model_provider(provider: &'a dyn ModelInfoProvider) -> Self {
        Self {
            model_provider: Some(provider),
            model_aliases: std::collections::HashMap::new(),
            default_model: None,
        }
    }

    /// Set the model information provider.
    pub fn set_model_provider(&mut self, provider: &'a dyn ModelInfoProvider) {
        self.model_provider = Some(provider);
    }

    /// Register a model alias.
    pub fn add_model_alias(&mut self, local_name: impl Into<String>, uri: impl Into<String>) {
        self.model_aliases.insert(local_name.into(), uri.into());
    }

    /// Set the default model namespace.
    pub fn set_default_model(&mut self, model: impl Into<String>) {
        self.default_model = Some(model.into());
    }

    /// Resolve an AST type specifier to a DataType.
    pub fn resolve_type_specifier(&self, spec: &ast::TypeSpecifier) -> TypeResult<DataType> {
        match spec {
            ast::TypeSpecifier::Named(named) => self.resolve_named_type(named),
            ast::TypeSpecifier::List(list) => self.resolve_list_type(list),
            ast::TypeSpecifier::Interval(interval) => self.resolve_interval_type(interval),
            ast::TypeSpecifier::Tuple(tuple) => self.resolve_tuple_type(tuple),
            ast::TypeSpecifier::Choice(choice) => self.resolve_choice_type(choice),
        }
    }

    /// Resolve a named type specifier.
    fn resolve_named_type(&self, named: &ast::NamedTypeSpecifier) -> TypeResult<DataType> {
        let name = &named.name;

        // Check for system types first (no namespace or System namespace)
        if named.namespace.is_none() || named.namespace.as_deref() == Some("System") {
            if let Some(sys_type) = self.resolve_system_type(name) {
                return Ok(DataType::System(sys_type));
            }
        }

        // If there's a namespace, try to resolve as model type
        if let Some(ref namespace) = named.namespace {
            return self.resolve_model_type(namespace, name);
        }

        // Try default model if set
        if let Some(ref default_model) = self.default_model {
            if self.is_model_type(default_model, name) {
                return Ok(DataType::model(default_model, name));
            }
        }

        // Unknown type
        Err(TypeError::UnknownType { name: name.clone() })
    }

    /// Try to resolve a system type by name.
    fn resolve_system_type(&self, name: &str) -> Option<SystemType> {
        SystemType::from_name(name)
    }

    /// Check if a type exists in a model.
    ///
    /// Note: Without comprehensive model info, we assume model types are valid.
    /// Full validation would require iterating through the model's type info
    /// and matching on each TypeInfo variant to extract the name.
    fn is_model_type(&self, namespace: &str, _name: &str) -> bool {
        if let Some(provider) = self.model_provider {
            // Look up the model URI from alias
            let model_uri = self.model_aliases.get(namespace);
            let model_name = model_uri.map(|s| s.as_str()).unwrap_or(namespace);

            // If we have a valid model, assume the type exists
            // Full validation would require matching on TypeInfo variants
            return provider.get_model(model_name, None).is_some();
        }
        // Without a provider, assume model types are valid
        true
    }

    /// Resolve a model type.
    fn resolve_model_type(&self, namespace: &str, name: &str) -> TypeResult<DataType> {
        // Validate the model exists if we have a provider
        if let Some(provider) = self.model_provider {
            let model_uri = self.model_aliases.get(namespace);
            let model_name = model_uri.map(|s| s.as_str()).unwrap_or(namespace);

            if provider.get_model(model_name, None).is_none() {
                return Err(TypeError::ModelNotFound {
                    name: namespace.to_string(),
                });
            }
        }

        Ok(DataType::model(namespace, name))
    }

    /// Resolve a list type specifier.
    fn resolve_list_type(&self, list: &ast::ListTypeSpecifier) -> TypeResult<DataType> {
        let element_type = self.resolve_type_specifier(&list.element_type)?;
        Ok(DataType::list(element_type))
    }

    /// Resolve an interval type specifier.
    fn resolve_interval_type(&self, interval: &ast::IntervalTypeSpecifier) -> TypeResult<DataType> {
        let point_type = self.resolve_type_specifier(&interval.point_type)?;
        Ok(DataType::interval(point_type))
    }

    /// Resolve a tuple type specifier.
    fn resolve_tuple_type(&self, tuple: &ast::TupleTypeSpecifier) -> TypeResult<DataType> {
        let mut elements = Vec::with_capacity(tuple.elements.len());
        for elem in &tuple.elements {
            let element_type = self.resolve_type_specifier(&elem.element_type)?;
            elements.push(TupleElement {
                name: elem.name.clone(),
                element_type: Box::new(element_type),
            });
        }
        Ok(DataType::tuple(elements))
    }

    /// Resolve a choice type specifier.
    fn resolve_choice_type(&self, choice: &ast::ChoiceTypeSpecifier) -> TypeResult<DataType> {
        let mut types = Vec::with_capacity(choice.types.len());
        for t in &choice.types {
            types.push(self.resolve_type_specifier(t)?);
        }
        Ok(DataType::choice(types))
    }

    /// Resolve a type from a qualified name string.
    ///
    /// Handles formats like:
    /// - "Integer" (system type)
    /// - "FHIR.Patient" (model type)
    /// - "{urn:hl7-org:elm-types:r1}Integer" (qualified system type)
    pub fn resolve_qualified_name(&self, qname: &str) -> TypeResult<DataType> {
        // Handle qualified names like "{urn:hl7-org:elm-types:r1}Integer"
        if let Some(pos) = qname.rfind('}') {
            let namespace = &qname[1..pos];
            let name = &qname[pos + 1..];

            // System types
            if namespace == "urn:hl7-org:elm-types:r1" {
                if let Some(sys_type) = SystemType::from_name(name) {
                    return Ok(DataType::System(sys_type));
                }
            }

            return Ok(DataType::model(namespace, name));
        }

        // Handle "Namespace.Name" format
        if let Some(pos) = qname.find('.') {
            let namespace = &qname[..pos];
            let name = &qname[pos + 1..];
            return self.resolve_model_type(namespace, name);
        }

        // Try as system type
        if let Some(sys_type) = SystemType::from_name(qname) {
            return Ok(DataType::System(sys_type));
        }

        // Try default model
        if let Some(ref default_model) = self.default_model {
            return Ok(DataType::model(default_model, qname));
        }

        Err(TypeError::UnknownType {
            name: qname.to_string(),
        })
    }
}

// ============================================================================
// Type Builder
// ============================================================================

/// Builds ELM TypeSpecifier from internal DataType representations.
///
/// This is the reverse operation of TypeResolver - converting internal types
/// back to the ELM format for serialization.
#[derive(Debug, Default)]
pub struct TypeBuilder;

impl TypeBuilder {
    /// Convert a DataType to an ELM TypeSpecifier.
    pub fn to_type_specifier(data_type: &DataType) -> Option<elm::TypeSpecifier> {
        match data_type {
            DataType::System(sys_type) => Some(Self::system_type_specifier(*sys_type)),
            DataType::Model { namespace, name } => {
                Some(Self::model_type_specifier(namespace, name))
            }
            DataType::List(element_type) => Self::list_type_specifier(element_type),
            DataType::Interval(point_type) => Self::interval_type_specifier(point_type),
            DataType::Tuple(elements) => Self::tuple_type_specifier(elements),
            DataType::Choice(types) => Self::choice_type_specifier(types),
            DataType::TypeParameter(name) => Some(Self::parameter_type_specifier(name)),
            DataType::Unknown => None,
        }
    }

    /// Convert a DataType to a qualified type name string.
    pub fn to_qualified_name(data_type: &DataType) -> Option<String> {
        match data_type {
            DataType::System(sys_type) => Some(sys_type.qualified_name().to_string()),
            DataType::Model { namespace, name } => Some(format!("{{{namespace}}}{name}")),
            _ => None, // Complex types don't have simple qualified names
        }
    }

    /// Create a type specifier for a system type.
    fn system_type_specifier(sys_type: SystemType) -> elm::TypeSpecifier {
        elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: sys_type.qualified_name().to_string(),
            local_id: None,
            locator: None,
        })
    }

    /// Create a type specifier for a model type.
    fn model_type_specifier(namespace: &str, name: &str) -> elm::TypeSpecifier {
        elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: format!("{{{namespace}}}{name}"),
            local_id: None,
            locator: None,
        })
    }

    /// Create a list type specifier.
    fn list_type_specifier(element_type: &DataType) -> Option<elm::TypeSpecifier> {
        let element = Self::to_type_specifier(element_type)?;
        Some(elm::TypeSpecifier::List(elm::ListTypeSpecifier {
            element_type: Some(Box::new(element)),
            local_id: None,
            locator: None,
        }))
    }

    /// Create an interval type specifier.
    fn interval_type_specifier(point_type: &DataType) -> Option<elm::TypeSpecifier> {
        let point = Self::to_type_specifier(point_type)?;
        Some(elm::TypeSpecifier::Interval(elm::IntervalTypeSpecifier {
            point_type: Some(Box::new(point)),
            local_id: None,
            locator: None,
        }))
    }

    /// Create a tuple type specifier.
    fn tuple_type_specifier(elements: &[TupleElement]) -> Option<elm::TypeSpecifier> {
        let mut elm_elements = Vec::with_capacity(elements.len());
        for elem in elements {
            let element_type = Self::to_type_specifier(&elem.element_type)?;
            elm_elements.push(elm::TupleElementDefinition {
                name: elem.name.clone(),
                element_type: Some(Box::new(element_type)),
                type_specifier: None,
            });
        }
        Some(elm::TypeSpecifier::Tuple(elm::TupleTypeSpecifier {
            element: elm_elements,
            local_id: None,
            locator: None,
        }))
    }

    /// Create a choice type specifier.
    fn choice_type_specifier(types: &[DataType]) -> Option<elm::TypeSpecifier> {
        let mut elm_types = Vec::with_capacity(types.len());
        for t in types {
            elm_types.push(Self::to_type_specifier(t)?);
        }
        Some(elm::TypeSpecifier::Choice(elm::ChoiceTypeSpecifier {
            choice: elm_types,
            local_id: None,
            locator: None,
        }))
    }

    /// Create a parameter type specifier.
    fn parameter_type_specifier(name: &str) -> elm::TypeSpecifier {
        elm::TypeSpecifier::Parameter(elm::ParameterTypeSpecifier {
            parameter_name: Some(name.to_string()),
            local_id: None,
            locator: None,
        })
    }
}

// ============================================================================
// Type Inference Utilities
// ============================================================================

/// Utilities for type inference during expression analysis.
#[derive(Debug, Default)]
pub struct TypeInference;

impl TypeInference {
    /// Get the result type of a literal.
    pub fn literal_type(literal: &ast::Literal) -> DataType {
        match literal {
            ast::Literal::Null => DataType::any(),
            ast::Literal::Boolean(_) => DataType::boolean(),
            ast::Literal::Integer(_) => DataType::integer(),
            ast::Literal::Long(_) => DataType::long(),
            ast::Literal::Decimal(_) => DataType::decimal(),
            ast::Literal::String(_) => DataType::string(),
            ast::Literal::Date(_) => DataType::date(),
            ast::Literal::DateTime(_) => DataType::date_time(),
            ast::Literal::Time(_) => DataType::time(),
            ast::Literal::Quantity { .. } => DataType::quantity(),
            ast::Literal::Ratio { .. } => DataType::System(SystemType::Ratio),
            ast::Literal::Code { .. } => DataType::code(),
        }
    }

    /// Promote numeric types according to CQL rules.
    ///
    /// - Integer + Integer = Integer
    /// - Integer + Long = Long
    /// - Integer + Decimal = Decimal
    /// - Long + Decimal = Decimal
    pub fn promote_numeric(left: &DataType, right: &DataType) -> Option<DataType> {
        use SystemType::*;

        match (left, right) {
            (DataType::System(Integer), DataType::System(Integer)) => Some(DataType::integer()),
            (DataType::System(Integer), DataType::System(Long))
            | (DataType::System(Long), DataType::System(Integer)) => Some(DataType::long()),
            (DataType::System(Integer), DataType::System(Decimal))
            | (DataType::System(Decimal), DataType::System(Integer)) => Some(DataType::decimal()),
            (DataType::System(Long), DataType::System(Long)) => Some(DataType::long()),
            (DataType::System(Long), DataType::System(Decimal))
            | (DataType::System(Decimal), DataType::System(Long)) => Some(DataType::decimal()),
            (DataType::System(Decimal), DataType::System(Decimal)) => Some(DataType::decimal()),
            _ => None,
        }
    }

    /// Find the common type of two types.
    ///
    /// Returns the most specific type that both types can be converted to.
    pub fn common_type(left: &DataType, right: &DataType) -> Option<DataType> {
        // Same type
        if left == right {
            return Some(left.clone());
        }

        // Any absorbs all
        if left.is_any() {
            return Some(right.clone());
        }
        if right.is_any() {
            return Some(left.clone());
        }

        // Numeric promotion
        if left.is_numeric() && right.is_numeric() {
            return Self::promote_numeric(left, right);
        }

        // Temporal promotion: Date -> DateTime
        if matches!(
            (left, right),
            (
                DataType::System(SystemType::Date),
                DataType::System(SystemType::DateTime)
            ) | (
                DataType::System(SystemType::DateTime),
                DataType::System(SystemType::Date)
            )
        ) {
            return Some(DataType::date_time());
        }

        // List types - find common element type
        if let (DataType::List(left_elem), DataType::List(right_elem)) = (left, right) {
            if let Some(common_elem) = Self::common_type(left_elem, right_elem) {
                return Some(DataType::list(common_elem));
            }
        }

        // Interval types - find common point type
        if let (DataType::Interval(left_point), DataType::Interval(right_point)) = (left, right) {
            if let Some(common_point) = Self::common_type(left_point, right_point) {
                return Some(DataType::interval(common_point));
            }
        }

        // Choice types - merge the choices
        match (left, right) {
            (DataType::Choice(left_types), DataType::Choice(right_types)) => {
                let mut merged = left_types.clone();
                for t in right_types {
                    if !merged.contains(t) {
                        merged.push(t.clone());
                    }
                }
                Some(DataType::choice(merged))
            }
            (DataType::Choice(types), other) | (other, DataType::Choice(types)) => {
                let mut merged = types.clone();
                if !merged.contains(other) {
                    merged.push(other.clone());
                }
                Some(DataType::choice(merged))
            }
            _ => None,
        }
    }

    /// Check if a type can be implicitly converted to another.
    pub fn can_convert_implicitly(from: &DataType, to: &DataType) -> bool {
        // Same type
        if from == to {
            return true;
        }

        // Anything can convert to Any
        if to.is_any() {
            return true;
        }

        // Numeric conversions
        use SystemType::*;
        matches!(
            (from, to),
            (DataType::System(Integer), DataType::System(Long | Decimal))
                | (DataType::System(Long), DataType::System(Decimal))
                | (DataType::System(Date), DataType::System(DateTime))
                | (DataType::System(Code), DataType::System(Concept))
        )
    }

    /// Get the result type of a unary operator.
    pub fn unary_result_type(operator: &str, operand: &DataType) -> Option<DataType> {
        match operator {
            "not" => {
                if operand == &DataType::boolean() {
                    Some(DataType::boolean())
                } else {
                    None
                }
            }
            "negate" | "-" => {
                if operand.is_numeric() {
                    Some(operand.clone())
                } else if operand == &DataType::quantity() {
                    Some(DataType::quantity())
                } else {
                    None
                }
            }
            "exists" | "is null" | "is true" | "is false" => Some(DataType::boolean()),
            "singleton from" => {
                if let DataType::List(elem) = operand {
                    Some((**elem).clone())
                } else {
                    None
                }
            }
            "flatten" => {
                if let DataType::List(elem) = operand {
                    if let DataType::List(_) = **elem {
                        Some((**elem).clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "distinct" | "collapse" => {
                if operand.is_list() {
                    Some(operand.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the result type of a binary comparison operator.
    pub fn comparison_result_type(left: &DataType, right: &DataType) -> Option<DataType> {
        // Comparisons return Boolean if types are compatible
        if Self::can_convert_implicitly(left, right)
            || Self::can_convert_implicitly(right, left)
            || Self::common_type(left, right).is_some()
        {
            Some(DataType::boolean())
        } else {
            None
        }
    }

    /// Get the result type of a binary arithmetic operator.
    pub fn arithmetic_result_type(
        operator: &str,
        left: &DataType,
        right: &DataType,
    ) -> Option<DataType> {
        match operator {
            "+" | "-" | "*" => {
                // Numeric operations
                if left.is_numeric() && right.is_numeric() {
                    Self::promote_numeric(left, right)
                } else if left == &DataType::quantity() && right == &DataType::quantity() {
                    Some(DataType::quantity())
                } else {
                    None
                }
            }
            "/" => {
                // Division always returns Decimal for numeric types
                if left.is_numeric() && right.is_numeric() {
                    Some(DataType::decimal())
                } else if left == &DataType::quantity() && right == &DataType::quantity() {
                    Some(DataType::quantity())
                } else {
                    None
                }
            }
            "div" | "mod" => {
                // Integer division and modulo
                if left.is_numeric() && right.is_numeric() {
                    Some(DataType::integer())
                } else {
                    None
                }
            }
            "&" => {
                // String concatenation
                if left == &DataType::string() && right == &DataType::string() {
                    Some(DataType::string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the result type of a logical operator.
    pub fn logical_result_type(left: &DataType, right: &DataType) -> Option<DataType> {
        if left == &DataType::boolean() && right == &DataType::boolean() {
            Some(DataType::boolean())
        } else {
            None
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_error_display() {
        let err = TypeError::UnknownType {
            name: "Foo".to_string(),
        };
        assert!(err.to_string().contains("Foo"));

        let err = TypeError::IncompatibleTypes {
            expected: "Integer".to_string(),
            found: "String".to_string(),
            context: Some("assignment".to_string()),
        };
        assert!(err.to_string().contains("assignment"));
    }

    #[test]
    fn test_resolve_system_types() {
        let resolver = TypeResolver::new();

        // Integer
        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Integer".to_string(),
        });
        assert_eq!(
            resolver.resolve_type_specifier(&spec).unwrap(),
            DataType::integer()
        );

        // Boolean
        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Boolean".to_string(),
        });
        assert_eq!(
            resolver.resolve_type_specifier(&spec).unwrap(),
            DataType::boolean()
        );

        // String
        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "String".to_string(),
        });
        assert_eq!(
            resolver.resolve_type_specifier(&spec).unwrap(),
            DataType::string()
        );

        // DateTime
        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "DateTime".to_string(),
        });
        assert_eq!(
            resolver.resolve_type_specifier(&spec).unwrap(),
            DataType::date_time()
        );
    }

    #[test]
    fn test_resolve_system_namespace() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: Some("System".to_string()),
            name: "Integer".to_string(),
        });
        assert_eq!(
            resolver.resolve_type_specifier(&spec).unwrap(),
            DataType::integer()
        );
    }

    #[test]
    fn test_resolve_model_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: Some("FHIR".to_string()),
            name: "Patient".to_string(),
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert_eq!(
            result,
            DataType::Model {
                namespace: "FHIR".to_string(),
                name: "Patient".to_string()
            }
        );
    }

    #[test]
    fn test_resolve_list_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::List(ast::ListTypeSpecifier {
            element_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert_eq!(result, DataType::list(DataType::integer()));
    }

    #[test]
    fn test_resolve_interval_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Interval(ast::IntervalTypeSpecifier {
            point_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "DateTime".to_string(),
            })),
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert_eq!(result, DataType::interval(DataType::date_time()));
    }

    #[test]
    fn test_resolve_tuple_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Tuple(ast::TupleTypeSpecifier {
            elements: vec![
                ast::TupleElementDef {
                    name: "name".to_string(),
                    element_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                        namespace: None,
                        name: "String".to_string(),
                    }),
                },
                ast::TupleElementDef {
                    name: "age".to_string(),
                    element_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                        namespace: None,
                        name: "Integer".to_string(),
                    }),
                },
            ],
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert!(result.is_tuple());
        let elements = result.tuple_elements().unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0].name, "name");
        assert_eq!(elements[1].name, "age");
    }

    #[test]
    fn test_resolve_choice_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Choice(ast::ChoiceTypeSpecifier {
            types: vec![
                ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "Integer".to_string(),
                }),
                ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "String".to_string(),
                }),
            ],
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert!(result.is_choice());
        let types = result.choice_types().unwrap();
        assert_eq!(types.len(), 2);
    }

    #[test]
    fn test_resolve_qualified_name() {
        let resolver = TypeResolver::new();

        // System type with qualified name
        let result = resolver
            .resolve_qualified_name("{urn:hl7-org:elm-types:r1}Integer")
            .unwrap();
        assert_eq!(result, DataType::integer());

        // Model type with dot notation
        let result = resolver.resolve_qualified_name("FHIR.Patient").unwrap();
        assert_eq!(
            result,
            DataType::Model {
                namespace: "FHIR".to_string(),
                name: "Patient".to_string()
            }
        );

        // Simple system type
        let result = resolver.resolve_qualified_name("Boolean").unwrap();
        assert_eq!(result, DataType::boolean());
    }

    #[test]
    fn test_resolve_unknown_type() {
        let resolver = TypeResolver::new();

        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "UnknownType".to_string(),
        });
        let result = resolver.resolve_type_specifier(&spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_type_builder_system_types() {
        let spec = TypeBuilder::to_type_specifier(&DataType::integer()).unwrap();
        if let elm::TypeSpecifier::Named(named) = spec {
            assert!(named.name.contains("Integer"));
        } else {
            panic!("Expected NamedTypeSpecifier");
        }
    }

    #[test]
    fn test_type_builder_model_type() {
        let data_type = DataType::model("FHIR", "Patient");
        let spec = TypeBuilder::to_type_specifier(&data_type).unwrap();
        if let elm::TypeSpecifier::Named(named) = spec {
            assert!(named.name.contains("FHIR"));
            assert!(named.name.contains("Patient"));
        } else {
            panic!("Expected NamedTypeSpecifier");
        }
    }

    #[test]
    fn test_type_builder_list_type() {
        let data_type = DataType::list(DataType::integer());
        let spec = TypeBuilder::to_type_specifier(&data_type).unwrap();
        assert!(matches!(spec, elm::TypeSpecifier::List(_)));
    }

    #[test]
    fn test_type_builder_interval_type() {
        let data_type = DataType::interval(DataType::date_time());
        let spec = TypeBuilder::to_type_specifier(&data_type).unwrap();
        assert!(matches!(spec, elm::TypeSpecifier::Interval(_)));
    }

    #[test]
    fn test_type_builder_tuple_type() {
        let data_type = DataType::tuple(vec![TupleElement {
            name: "x".to_string(),
            element_type: Box::new(DataType::integer()),
        }]);
        let spec = TypeBuilder::to_type_specifier(&data_type).unwrap();
        assert!(matches!(spec, elm::TypeSpecifier::Tuple(_)));
    }

    #[test]
    fn test_type_builder_choice_type() {
        let data_type = DataType::choice(vec![DataType::integer(), DataType::string()]);
        let spec = TypeBuilder::to_type_specifier(&data_type).unwrap();
        assert!(matches!(spec, elm::TypeSpecifier::Choice(_)));
    }

    #[test]
    fn test_type_builder_unknown_returns_none() {
        let spec = TypeBuilder::to_type_specifier(&DataType::Unknown);
        assert!(spec.is_none());
    }

    #[test]
    fn test_to_qualified_name() {
        let name = TypeBuilder::to_qualified_name(&DataType::integer()).unwrap();
        assert_eq!(name, "{urn:hl7-org:elm-types:r1}Integer");

        let name = TypeBuilder::to_qualified_name(&DataType::model("FHIR", "Patient")).unwrap();
        assert_eq!(name, "{FHIR}Patient");

        let name = TypeBuilder::to_qualified_name(&DataType::list(DataType::integer()));
        assert!(name.is_none()); // Complex types don't have qualified names
    }

    #[test]
    fn test_literal_type() {
        assert_eq!(
            TypeInference::literal_type(&ast::Literal::Boolean(true)),
            DataType::boolean()
        );
        assert_eq!(
            TypeInference::literal_type(&ast::Literal::Integer(42)),
            DataType::integer()
        );
        assert_eq!(
            TypeInference::literal_type(&ast::Literal::Decimal(3.5)),
            DataType::decimal()
        );
        assert_eq!(
            TypeInference::literal_type(&ast::Literal::String("hello".to_string())),
            DataType::string()
        );
        assert_eq!(
            TypeInference::literal_type(&ast::Literal::Null),
            DataType::any()
        );
    }

    #[test]
    fn test_promote_numeric() {
        // Integer + Integer = Integer
        assert_eq!(
            TypeInference::promote_numeric(&DataType::integer(), &DataType::integer()),
            Some(DataType::integer())
        );

        // Integer + Long = Long
        assert_eq!(
            TypeInference::promote_numeric(&DataType::integer(), &DataType::long()),
            Some(DataType::long())
        );

        // Integer + Decimal = Decimal
        assert_eq!(
            TypeInference::promote_numeric(&DataType::integer(), &DataType::decimal()),
            Some(DataType::decimal())
        );

        // Non-numeric returns None
        assert_eq!(
            TypeInference::promote_numeric(&DataType::string(), &DataType::integer()),
            None
        );
    }

    #[test]
    fn test_common_type() {
        // Same type
        assert_eq!(
            TypeInference::common_type(&DataType::integer(), &DataType::integer()),
            Some(DataType::integer())
        );

        // Numeric promotion
        assert_eq!(
            TypeInference::common_type(&DataType::integer(), &DataType::decimal()),
            Some(DataType::decimal())
        );

        // Date -> DateTime
        assert_eq!(
            TypeInference::common_type(&DataType::date(), &DataType::date_time()),
            Some(DataType::date_time())
        );

        // Any absorbs
        assert_eq!(
            TypeInference::common_type(&DataType::any(), &DataType::string()),
            Some(DataType::string())
        );

        // List types
        assert_eq!(
            TypeInference::common_type(
                &DataType::list(DataType::integer()),
                &DataType::list(DataType::decimal())
            ),
            Some(DataType::list(DataType::decimal()))
        );
    }

    #[test]
    fn test_can_convert_implicitly() {
        // Same type
        assert!(TypeInference::can_convert_implicitly(
            &DataType::integer(),
            &DataType::integer()
        ));

        // To Any
        assert!(TypeInference::can_convert_implicitly(
            &DataType::string(),
            &DataType::any()
        ));

        // Numeric widening
        assert!(TypeInference::can_convert_implicitly(
            &DataType::integer(),
            &DataType::long()
        ));
        assert!(TypeInference::can_convert_implicitly(
            &DataType::integer(),
            &DataType::decimal()
        ));
        assert!(TypeInference::can_convert_implicitly(
            &DataType::long(),
            &DataType::decimal()
        ));

        // Date to DateTime
        assert!(TypeInference::can_convert_implicitly(
            &DataType::date(),
            &DataType::date_time()
        ));

        // Code to Concept
        assert!(TypeInference::can_convert_implicitly(
            &DataType::code(),
            &DataType::concept()
        ));

        // Invalid conversions
        assert!(!TypeInference::can_convert_implicitly(
            &DataType::string(),
            &DataType::integer()
        ));
        assert!(!TypeInference::can_convert_implicitly(
            &DataType::decimal(),
            &DataType::integer()
        ));
    }

    #[test]
    fn test_unary_result_type() {
        // not Boolean -> Boolean
        assert_eq!(
            TypeInference::unary_result_type("not", &DataType::boolean()),
            Some(DataType::boolean())
        );

        // negate Integer -> Integer
        assert_eq!(
            TypeInference::unary_result_type("negate", &DataType::integer()),
            Some(DataType::integer())
        );

        // exists -> Boolean
        assert_eq!(
            TypeInference::unary_result_type("exists", &DataType::list(DataType::integer())),
            Some(DataType::boolean())
        );

        // singleton from List<T> -> T
        assert_eq!(
            TypeInference::unary_result_type("singleton from", &DataType::list(DataType::string())),
            Some(DataType::string())
        );
    }

    #[test]
    fn test_comparison_result_type() {
        // Same types
        assert_eq!(
            TypeInference::comparison_result_type(&DataType::integer(), &DataType::integer()),
            Some(DataType::boolean())
        );

        // Compatible types
        assert_eq!(
            TypeInference::comparison_result_type(&DataType::integer(), &DataType::decimal()),
            Some(DataType::boolean())
        );

        // Incompatible
        assert_eq!(
            TypeInference::comparison_result_type(&DataType::string(), &DataType::integer()),
            None
        );
    }

    #[test]
    fn test_arithmetic_result_type() {
        // Integer + Integer = Integer
        assert_eq!(
            TypeInference::arithmetic_result_type("+", &DataType::integer(), &DataType::integer()),
            Some(DataType::integer())
        );

        // Integer + Decimal = Decimal
        assert_eq!(
            TypeInference::arithmetic_result_type("+", &DataType::integer(), &DataType::decimal()),
            Some(DataType::decimal())
        );

        // Division always Decimal
        assert_eq!(
            TypeInference::arithmetic_result_type("/", &DataType::integer(), &DataType::integer()),
            Some(DataType::decimal())
        );

        // Integer division
        assert_eq!(
            TypeInference::arithmetic_result_type(
                "div",
                &DataType::integer(),
                &DataType::integer()
            ),
            Some(DataType::integer())
        );

        // String concatenation
        assert_eq!(
            TypeInference::arithmetic_result_type("&", &DataType::string(), &DataType::string()),
            Some(DataType::string())
        );
    }

    #[test]
    fn test_logical_result_type() {
        assert_eq!(
            TypeInference::logical_result_type(&DataType::boolean(), &DataType::boolean()),
            Some(DataType::boolean())
        );

        assert_eq!(
            TypeInference::logical_result_type(&DataType::boolean(), &DataType::integer()),
            None
        );
    }

    #[test]
    fn test_resolver_with_default_model() {
        let mut resolver = TypeResolver::new();
        resolver.set_default_model("FHIR");

        // Unknown type should resolve to default model
        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: "Patient".to_string(),
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert_eq!(
            result,
            DataType::Model {
                namespace: "FHIR".to_string(),
                name: "Patient".to_string()
            }
        );
    }

    #[test]
    fn test_resolver_with_model_alias() {
        let mut resolver = TypeResolver::new();
        resolver.add_model_alias("FHIR", "http://hl7.org/fhir");

        let spec = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: Some("FHIR".to_string()),
            name: "Patient".to_string(),
        });
        let result = resolver.resolve_type_specifier(&spec).unwrap();
        assert_eq!(
            result,
            DataType::Model {
                namespace: "FHIR".to_string(),
                name: "Patient".to_string()
            }
        );
    }
}
