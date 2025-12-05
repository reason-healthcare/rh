//! Expression Translator for CQL-to-ELM conversion.
//!
//! This module translates CQL AST expressions to ELM expressions.
//! The translation is done in a type-directed manner, using the
//! LibraryBuilder for context and symbol resolution.
//!
//! # Example
//!
//! ```
//! use rh_cql::translator::ExpressionTranslator;
//! use rh_cql::parser::ast;
//! use rh_cql::elm;
//!
//! let mut translator = ExpressionTranslator::new();
//!
//! // Translate a literal
//! let ast_lit = ast::Literal::Integer(42);
//! let elm_expr = translator.translate_literal(&ast_lit);
//! ```

use crate::datatype::{DataType, SystemType};
use crate::elm;
use crate::parser::ast;

// ============================================================================
// Translator Error
// ============================================================================

/// Errors that can occur during expression translation.
#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorError {
    /// Unsupported expression type.
    UnsupportedExpression { description: String },
    /// Type error during translation.
    TypeError { message: String },
    /// Invalid literal value.
    InvalidLiteral { message: String },
}

impl std::fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslatorError::UnsupportedExpression { description } => {
                write!(f, "Unsupported expression: {description}")
            }
            TranslatorError::TypeError { message } => {
                write!(f, "Type error: {message}")
            }
            TranslatorError::InvalidLiteral { message } => {
                write!(f, "Invalid literal: {message}")
            }
        }
    }
}

impl std::error::Error for TranslatorError {}

/// Result type for translator operations.
pub type TranslatorResult<T> = Result<T, TranslatorError>;

// ============================================================================
// Reference Resolution Types
// ============================================================================

/// The kind of reference an identifier resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedRefKind {
    /// An expression definition.
    Expression,
    /// A parameter definition.
    Parameter,
    /// A function operand.
    Operand,
    /// A query alias.
    QueryAlias,
    /// A let binding.
    Let,
    /// A code system definition.
    CodeSystem,
    /// A value set definition.
    ValueSet,
    /// A code definition.
    Code,
    /// A concept definition.
    Concept,
    /// A context variable (e.g., Patient).
    Context,
    /// Unknown/unresolved reference.
    Unknown,
}

/// The kind of qualified reference.
#[derive(Debug, Clone, PartialEq)]
pub enum QualifiedRefKind {
    /// Expression from an included library.
    LibraryExpression,
    /// Parameter from an included library.
    LibraryParameter,
    /// Code system from an included library.
    LibraryCodeSystem,
    /// Value set from an included library.
    LibraryValueSet,
    /// Code from an included library.
    LibraryCode,
    /// Concept from an included library.
    LibraryConcept,
    /// Property access on a source expression.
    Property {
        /// The source expression (if not a simple alias scope).
        source: Option<Box<elm::Expression>>,
    },
    /// Unknown/unresolved reference.
    Unknown,
}

// ============================================================================
// Expression Translator
// ============================================================================

/// Translates CQL AST expressions to ELM expressions.
///
/// The translator handles:
/// - Literal translation (integers, strings, dates, etc.)
/// - Reference translation (identifiers, qualified refs)
/// - Operator translation (unary, binary, ternary)
/// - Query translation
/// - Type operations
pub struct ExpressionTranslator {
    /// Counter for generating local IDs.
    local_id_counter: u32,
    /// Whether to generate local IDs.
    generate_local_ids: bool,
}

impl Default for ExpressionTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionTranslator {
    /// Create a new expression translator.
    pub fn new() -> Self {
        Self {
            local_id_counter: 0,
            generate_local_ids: false,
        }
    }

    /// Enable local ID generation.
    pub fn with_local_ids(mut self) -> Self {
        self.generate_local_ids = true;
        self
    }

    /// Generate a new local ID.
    fn next_local_id(&mut self) -> Option<String> {
        if self.generate_local_ids {
            self.local_id_counter += 1;
            Some(self.local_id_counter.to_string())
        } else {
            None
        }
    }

    /// Create element fields with optional local ID.
    fn element_fields(&mut self) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            ..Default::default()
        }
    }

    /// Create element fields with a result type.
    fn element_fields_typed(&mut self, result_type: &DataType) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            result_type_name: Some(datatype_to_qname(result_type)),
            ..Default::default()
        }
    }

    // ========================================================================
    // Literal Translation
    // ========================================================================

    /// Translate a CQL AST literal to an ELM expression.
    pub fn translate_literal(&mut self, lit: &ast::Literal) -> elm::Expression {
        match lit {
            ast::Literal::Null => self.translate_null(),
            ast::Literal::Boolean(b) => self.translate_boolean(*b),
            ast::Literal::Integer(i) => self.translate_integer(*i),
            ast::Literal::Long(l) => self.translate_long(*l),
            ast::Literal::Decimal(d) => self.translate_decimal(*d),
            ast::Literal::String(s) => self.translate_string(s),
            ast::Literal::Date(d) => self.translate_date(d),
            ast::Literal::DateTime(dt) => self.translate_datetime(dt),
            ast::Literal::Time(t) => self.translate_time(t),
            ast::Literal::Quantity { value, unit } => self.translate_quantity(*value, unit),
            ast::Literal::Ratio {
                numerator,
                denominator,
            } => self.translate_ratio(numerator, denominator),
            ast::Literal::Code {
                code,
                system,
                display,
            } => self.translate_code(code, system.as_deref(), display.as_deref()),
        }
    }

    /// Translate null literal.
    fn translate_null(&mut self) -> elm::Expression {
        elm::Expression::Null(elm::Null {
            element: self.element_fields(),
            value_type: None,
        })
    }

    /// Translate boolean literal.
    fn translate_boolean(&mut self, value: bool) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::boolean()),
            value_type: Some(qname_system("Boolean")),
            value: Some(value.to_string()),
        })
    }

    /// Translate integer literal.
    fn translate_integer(&mut self, value: i64) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::integer()),
            value_type: Some(qname_system("Integer")),
            value: Some(value.to_string()),
        })
    }

    /// Translate long literal.
    fn translate_long(&mut self, value: i64) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::long()),
            value_type: Some(qname_system("Long")),
            value: Some(format!("{value}L")),
        })
    }

    /// Translate decimal literal.
    fn translate_decimal(&mut self, value: f64) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::decimal()),
            value_type: Some(qname_system("Decimal")),
            value: Some(format_decimal(value)),
        })
    }

    /// Translate string literal.
    fn translate_string(&mut self, value: &str) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::string()),
            value_type: Some(qname_system("String")),
            value: Some(value.to_string()),
        })
    }

    /// Translate date literal.
    fn translate_date(&mut self, value: &str) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::date()),
            value_type: Some(qname_system("Date")),
            value: Some(value.to_string()),
        })
    }

    /// Translate datetime literal.
    fn translate_datetime(&mut self, value: &str) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::date_time()),
            value_type: Some(qname_system("DateTime")),
            value: Some(value.to_string()),
        })
    }

    /// Translate time literal.
    fn translate_time(&mut self, value: &str) -> elm::Expression {
        elm::Expression::Literal(elm::Literal {
            element: self.element_fields_typed(&DataType::time()),
            value_type: Some(qname_system("Time")),
            value: Some(value.to_string()),
        })
    }

    /// Translate quantity literal.
    fn translate_quantity(&mut self, value: f64, unit: &str) -> elm::Expression {
        elm::Expression::Quantity(elm::QuantityExpr {
            element: self.element_fields_typed(&DataType::quantity()),
            value: Some(value),
            unit: Some(unit.to_string()),
        })
    }

    /// Translate ratio literal.
    fn translate_ratio(
        &mut self,
        numerator: &ast::Literal,
        denominator: &ast::Literal,
    ) -> elm::Expression {
        let num_expr = self.translate_literal(numerator);
        let denom_expr = self.translate_literal(denominator);

        elm::Expression::Ratio(elm::RatioExpr {
            element: self.element_fields_typed(&DataType::System(SystemType::Ratio)),
            numerator: Some(Box::new(num_expr)),
            denominator: Some(Box::new(denom_expr)),
        })
    }

    /// Translate code literal.
    fn translate_code(
        &mut self,
        code: &str,
        system: Option<&str>,
        display: Option<&str>,
    ) -> elm::Expression {
        elm::Expression::Code(elm::CodeExpr {
            element: self.element_fields_typed(&DataType::code()),
            code: Some(code.to_string()),
            system: system.map(|s| elm::CodeSystemRef {
                element: elm::ElementFields::default(),
                name: Some(s.to_string()),
                library_name: None,
            }),
            display: display.map(String::from),
        })
    }

    /// Get the DataType for a literal.
    pub fn literal_type(lit: &ast::Literal) -> DataType {
        match lit {
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

    // ========================================================================
    // Reference Translation
    // ========================================================================

    /// Translate an identifier reference to an ELM expression.
    ///
    /// The kind of reference depends on what the identifier resolves to:
    /// - Expression definitions → ExpressionRef
    /// - Parameters → ParameterRef
    /// - Function operands → OperandRef
    /// - Query aliases → AliasRef
    /// - Let bindings → AliasRef (treated same as query aliases in ELM)
    /// - Code systems → CodeSystemRef
    /// - Value sets → ValueSetRef
    /// - Codes → CodeRef
    /// - Concepts → ConceptRef
    /// - Unknown → IdentifierRef (unresolved)
    pub fn translate_identifier_ref(
        &mut self,
        name: &str,
        kind: ResolvedRefKind,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        match kind {
            ResolvedRefKind::Expression => elm::Expression::ExpressionRef(elm::ExpressionRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::Parameter => elm::Expression::ParameterRef(elm::ParameterRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::Operand => elm::Expression::OperandRef(elm::OperandRef {
                element,
                name: Some(name.to_string()),
            }),
            ResolvedRefKind::QueryAlias | ResolvedRefKind::Let => {
                elm::Expression::AliasRef(elm::AliasRef {
                    element,
                    name: Some(name.to_string()),
                })
            }
            ResolvedRefKind::CodeSystem => elm::Expression::CodeSystemRef(elm::CodeSystemRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::ValueSet => elm::Expression::ValueSetRef(elm::ValueSetRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
                preserve: None,
            }),
            ResolvedRefKind::Code => elm::Expression::CodeRef(elm::CodeRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::Concept => elm::Expression::ConceptRef(elm::ConceptRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::Context => elm::Expression::ExpressionRef(elm::ExpressionRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
            ResolvedRefKind::Unknown => elm::Expression::IdentifierRef(elm::IdentifierRef {
                element,
                name: Some(name.to_string()),
                library_name: None,
            }),
        }
    }

    /// Translate a qualified identifier reference to an ELM expression.
    ///
    /// Qualified identifiers have the form `qualifier.name` where qualifier could be:
    /// - A library alias → ExpressionRef/ParameterRef with library_name
    /// - A model alias → Property access on a type
    /// - A query alias → Property on AliasRef
    pub fn translate_qualified_ref(
        &mut self,
        qualifier: &str,
        name: &str,
        kind: QualifiedRefKind,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        match kind {
            QualifiedRefKind::LibraryExpression => {
                elm::Expression::ExpressionRef(elm::ExpressionRef {
                    element,
                    name: Some(name.to_string()),
                    library_name: Some(qualifier.to_string()),
                })
            }
            QualifiedRefKind::LibraryParameter => {
                elm::Expression::ParameterRef(elm::ParameterRef {
                    element,
                    name: Some(name.to_string()),
                    library_name: Some(qualifier.to_string()),
                })
            }
            QualifiedRefKind::LibraryCodeSystem => {
                elm::Expression::CodeSystemRef(elm::CodeSystemRef {
                    element,
                    name: Some(name.to_string()),
                    library_name: Some(qualifier.to_string()),
                })
            }
            QualifiedRefKind::LibraryValueSet => elm::Expression::ValueSetRef(elm::ValueSetRef {
                element,
                name: Some(name.to_string()),
                library_name: Some(qualifier.to_string()),
                preserve: None,
            }),
            QualifiedRefKind::LibraryCode => elm::Expression::CodeRef(elm::CodeRef {
                element,
                name: Some(name.to_string()),
                library_name: Some(qualifier.to_string()),
            }),
            QualifiedRefKind::LibraryConcept => elm::Expression::ConceptRef(elm::ConceptRef {
                element,
                name: Some(name.to_string()),
                library_name: Some(qualifier.to_string()),
            }),
            QualifiedRefKind::Property { source } => elm::Expression::Property(elm::Property {
                element,
                path: Some(name.to_string()),
                scope: Some(qualifier.to_string()),
                source,
            }),
            QualifiedRefKind::Unknown => elm::Expression::IdentifierRef(elm::IdentifierRef {
                element,
                name: Some(format!("{qualifier}.{name}")),
                library_name: None,
            }),
        }
    }

    /// Translate a function invocation to an ELM FunctionRef.
    pub fn translate_function_ref(
        &mut self,
        name: &str,
        library: Option<&str>,
        operands: Vec<elm::Expression>,
        signature: Vec<elm::TypeSpecifier>,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        elm::Expression::FunctionRef(elm::FunctionRef {
            element,
            name: Some(name.to_string()),
            library_name: library.map(String::from),
            operand: operands,
            signature,
        })
    }

    /// Translate an AST identifier reference (unresolved).
    ///
    /// This creates an IdentifierRef that will need resolution during semantic analysis.
    pub fn translate_ast_identifier_ref(&mut self, id_ref: &ast::IdentifierRef) -> elm::Expression {
        elm::Expression::IdentifierRef(elm::IdentifierRef {
            element: self.element_fields(),
            name: Some(id_ref.name.clone()),
            library_name: None,
        })
    }

    /// Translate an AST qualified identifier reference (unresolved).
    ///
    /// This creates a qualified IdentifierRef that will need resolution during semantic analysis.
    pub fn translate_ast_qualified_ref(
        &mut self,
        qual_ref: &ast::QualifiedIdentifierRef,
    ) -> elm::Expression {
        elm::Expression::IdentifierRef(elm::IdentifierRef {
            element: self.element_fields(),
            name: Some(qual_ref.name.clone()),
            library_name: Some(qual_ref.qualifier.clone()),
        })
    }

    /// Translate an AST function invocation (unresolved).
    ///
    /// This creates a FunctionRef with operands translated but without signature resolution.
    pub fn translate_ast_function_invocation(
        &mut self,
        func: &ast::FunctionInvocation,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let operands: Vec<elm::Expression> = func
            .arguments
            .iter()
            .map(|arg| translate_expr(self, arg))
            .collect();

        elm::Expression::FunctionRef(elm::FunctionRef {
            element: self.element_fields(),
            name: Some(func.name.clone()),
            library_name: func.library.clone(),
            operand: operands,
            signature: Vec::new(),
        })
    }

    // ========================================================================
    // Operator Translation
    // ========================================================================

    /// Translate an AST unary expression to an ELM expression.
    ///
    /// This translates the operand and wraps it in the appropriate ELM operator.
    pub fn translate_unary_operator(
        &mut self,
        operator: ast::UnaryOperator,
        operand: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Handle Expand specially - it uses BinaryExpression even with single operand
        if matches!(operator, ast::UnaryOperator::Expand) {
            return elm::Expression::Expand(elm::BinaryExpression {
                element,
                operand: vec![operand],
                signature: Vec::new(),
            });
        }

        let unary = elm::UnaryExpression {
            element: element.clone(),
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        };

        match operator {
            ast::UnaryOperator::Not => elm::Expression::Not(unary),
            ast::UnaryOperator::Negate => elm::Expression::Negate(unary),
            ast::UnaryOperator::Exists => elm::Expression::Exists(unary),
            ast::UnaryOperator::IsNull => elm::Expression::IsNull(unary),
            ast::UnaryOperator::IsTrue => elm::Expression::IsTrue(unary),
            ast::UnaryOperator::IsFalse => elm::Expression::IsFalse(unary),
            ast::UnaryOperator::Predecessor => elm::Expression::Predecessor(unary),
            ast::UnaryOperator::Successor => elm::Expression::Successor(unary),
            ast::UnaryOperator::Distinct => elm::Expression::Distinct(unary),
            ast::UnaryOperator::Flatten => elm::Expression::Flatten(unary),
            ast::UnaryOperator::Start => elm::Expression::Start(unary),
            ast::UnaryOperator::End => elm::Expression::End(unary),
            ast::UnaryOperator::Width => elm::Expression::Width(unary),
            ast::UnaryOperator::PointFrom => elm::Expression::PointFrom(unary),
            ast::UnaryOperator::Collapse => elm::Expression::Collapse(unary),
            ast::UnaryOperator::Expand => unreachable!("Handled above"),
            ast::UnaryOperator::Singleton => elm::Expression::SingletonFrom(unary),
            ast::UnaryOperator::ToBoolean => elm::Expression::ToBoolean(unary),
            ast::UnaryOperator::ToInteger => elm::Expression::ToInteger(unary),
            ast::UnaryOperator::ToLong => elm::Expression::ToLong(unary),
            ast::UnaryOperator::ToDecimal => elm::Expression::ToDecimal(unary),
            ast::UnaryOperator::ToString => elm::Expression::ToStringExpr(unary),
            ast::UnaryOperator::ToDate => elm::Expression::ToDate(unary),
            ast::UnaryOperator::ToDateTime => elm::Expression::ToDateTime(unary),
            ast::UnaryOperator::ToTime => elm::Expression::ToTime(unary),
            ast::UnaryOperator::ToQuantity => elm::Expression::ToQuantity(unary),
            ast::UnaryOperator::ToConcept => elm::Expression::ToConcept(unary),
        }
    }

    /// Translate an AST binary expression to an ELM expression.
    ///
    /// This translates both operands and wraps them in the appropriate ELM operator.
    pub fn translate_binary_operator(
        &mut self,
        operator: ast::BinaryOperator,
        left: elm::Expression,
        right: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let binary = elm::BinaryExpression {
            element: element.clone(),
            operand: vec![left.clone(), right.clone()],
            signature: Vec::new(),
        };

        let time_binary = || elm::TimeBinaryExpression {
            element: element.clone(),
            operand: vec![left.clone(), right.clone()],
            signature: Vec::new(),
            precision: None,
        };

        match operator {
            // Arithmetic
            ast::BinaryOperator::Add => elm::Expression::Add(binary),
            ast::BinaryOperator::Subtract => elm::Expression::Subtract(binary),
            ast::BinaryOperator::Multiply => elm::Expression::Multiply(binary),
            ast::BinaryOperator::Divide => elm::Expression::Divide(binary),
            ast::BinaryOperator::TruncatedDivide => elm::Expression::TruncatedDivide(binary),
            ast::BinaryOperator::Modulo => elm::Expression::Modulo(binary),
            ast::BinaryOperator::Power => elm::Expression::Power(binary),
            ast::BinaryOperator::Log => elm::Expression::Log(binary),

            // Comparison
            ast::BinaryOperator::Equal => elm::Expression::Equal(binary),
            ast::BinaryOperator::NotEqual => elm::Expression::NotEqual(binary),
            ast::BinaryOperator::Equivalent => elm::Expression::Equivalent(binary),
            ast::BinaryOperator::NotEquivalent => {
                // Not equivalent is not(equivalent)
                let equiv = elm::Expression::Equivalent(binary);
                elm::Expression::Not(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(equiv)),
                    signature: Vec::new(),
                })
            }
            ast::BinaryOperator::Less => elm::Expression::Less(binary),
            ast::BinaryOperator::LessOrEqual => elm::Expression::LessOrEqual(binary),
            ast::BinaryOperator::Greater => elm::Expression::Greater(binary),
            ast::BinaryOperator::GreaterOrEqual => elm::Expression::GreaterOrEqual(binary),

            // Logical
            ast::BinaryOperator::And => elm::Expression::And(elm::NaryExpression {
                element,
                operand: vec![left, right],
                signature: Vec::new(),
            }),
            ast::BinaryOperator::Or => elm::Expression::Or(elm::NaryExpression {
                element,
                operand: vec![left, right],
                signature: Vec::new(),
            }),
            ast::BinaryOperator::Xor => elm::Expression::Xor(binary),
            ast::BinaryOperator::Implies => elm::Expression::Implies(binary),

            // String
            ast::BinaryOperator::Concatenate => elm::Expression::Concatenate(elm::NaryExpression {
                element,
                operand: vec![left, right],
                signature: Vec::new(),
            }),

            // Membership
            ast::BinaryOperator::In => elm::Expression::In(binary),
            ast::BinaryOperator::Contains => elm::Expression::Contains(binary),

            // Interval
            ast::BinaryOperator::Includes => elm::Expression::Includes(binary),
            ast::BinaryOperator::IncludedIn => elm::Expression::IncludedIn(binary),
            ast::BinaryOperator::ProperlyIncludes => elm::Expression::ProperIncludes(binary),
            ast::BinaryOperator::ProperlyIncludedIn => elm::Expression::ProperIncludedIn(binary),
            ast::BinaryOperator::Overlaps => elm::Expression::Overlaps(time_binary()),
            ast::BinaryOperator::OverlapsBefore => elm::Expression::OverlapsBefore(time_binary()),
            ast::BinaryOperator::OverlapsAfter => elm::Expression::OverlapsAfter(time_binary()),
            ast::BinaryOperator::Meets => elm::Expression::Meets(binary),
            ast::BinaryOperator::MeetsBefore => elm::Expression::MeetsBefore(binary),
            ast::BinaryOperator::MeetsAfter => elm::Expression::MeetsAfter(binary),
            ast::BinaryOperator::Starts => elm::Expression::Starts(time_binary()),
            ast::BinaryOperator::Ends => elm::Expression::Ends(time_binary()),
            ast::BinaryOperator::During => {
                // During is IncludedIn for intervals
                elm::Expression::IncludedIn(binary)
            }
            ast::BinaryOperator::Before => elm::Expression::Before(time_binary()),
            ast::BinaryOperator::After => elm::Expression::After(time_binary()),
            ast::BinaryOperator::SameAs => elm::Expression::SameAs(time_binary()),
            ast::BinaryOperator::SameOrBefore => elm::Expression::SameOrBefore(time_binary()),
            ast::BinaryOperator::SameOrAfter => elm::Expression::SameOrAfter(time_binary()),
            ast::BinaryOperator::Within => {
                // Within X of Y - this needs special handling with quantity
                // For now, translate as IncludedIn
                elm::Expression::IncludedIn(binary)
            }

            // List
            ast::BinaryOperator::Union => elm::Expression::Union(binary),
            ast::BinaryOperator::Intersect => elm::Expression::Intersect(binary),
            ast::BinaryOperator::Except => elm::Expression::Except(binary),
            ast::BinaryOperator::IndexOf => {
                // IndexOf(source, element) - ELM uses Indexer for [index] access
                // but IndexOf is a different operation
                elm::Expression::Indexer(binary)
            }
        }
    }

    /// Translate an AST ternary expression to an ELM expression.
    pub fn translate_ternary_operator(
        &mut self,
        operator: ast::TernaryOperator,
        first: elm::Expression,
        second: elm::Expression,
        third: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        match operator {
            ast::TernaryOperator::Between => {
                // X between Y and Z translates to: X >= Y and X <= Z
                // Create GreaterOrEqual and LessOrEqual, then And them
                let ge_element = self.element_fields();
                let le_element = self.element_fields();

                let ge = elm::Expression::GreaterOrEqual(elm::BinaryExpression {
                    element: ge_element,
                    operand: vec![first.clone(), second],
                    signature: Vec::new(),
                });

                let le = elm::Expression::LessOrEqual(elm::BinaryExpression {
                    element: le_element,
                    operand: vec![first, third],
                    signature: Vec::new(),
                });

                elm::Expression::And(elm::NaryExpression {
                    element,
                    operand: vec![ge, le],
                    signature: Vec::new(),
                })
            }
            ast::TernaryOperator::ReplaceMatches => {
                elm::Expression::ReplaceMatches(elm::TernaryExpression {
                    element,
                    operand: vec![first, second, third],
                    signature: Vec::new(),
                })
            }
        }
    }

    /// Translate an N-ary operator (operators that can take multiple operands).
    ///
    /// Examples: And, Or, Concatenate, Coalesce
    pub fn translate_nary_operator(
        &mut self,
        operator: NaryOperator,
        operands: Vec<elm::Expression>,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let nary = elm::NaryExpression {
            element,
            operand: operands,
            signature: Vec::new(),
        };

        match operator {
            NaryOperator::And => elm::Expression::And(nary),
            NaryOperator::Or => elm::Expression::Or(nary),
            NaryOperator::Concatenate => elm::Expression::Concatenate(nary),
            NaryOperator::Coalesce => elm::Expression::Coalesce(nary),
        }
    }

    /// Translate an AST unary expression using a callback to translate the operand.
    pub fn translate_ast_unary_expression(
        &mut self,
        expr: &ast::UnaryExpression,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let operand = translate_expr(self, &expr.operand);
        self.translate_unary_operator(expr.operator, operand, None)
    }

    /// Translate an AST binary expression using a callback to translate operands.
    pub fn translate_ast_binary_expression(
        &mut self,
        expr: &ast::BinaryExpression,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let left = translate_expr(self, &expr.left);
        let right = translate_expr(self, &expr.right);
        self.translate_binary_operator(expr.operator, left, right, None)
    }

    /// Translate an AST ternary expression using a callback to translate operands.
    pub fn translate_ast_ternary_expression(
        &mut self,
        expr: &ast::TernaryExpression,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let first = translate_expr(self, &expr.first);
        let second = translate_expr(self, &expr.second);
        let third = translate_expr(self, &expr.third);
        self.translate_ternary_operator(expr.operator, first, second, third, None)
    }
}

// ============================================================================
// N-ary Operator Enum
// ============================================================================

/// Operators that support multiple operands (n-ary).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaryOperator {
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
    /// String concatenation.
    Concatenate,
    /// Null coalescing.
    Coalesce,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a QName for a System type.
fn qname_system(name: &str) -> elm::QName {
    format!("{{urn:hl7-org:elm-types:r1}}{name}")
}

/// Create a QName with namespace and name.
fn qname_qualified(namespace: &str, name: &str) -> elm::QName {
    format!("{{{namespace}}}{name}")
}

/// Convert a DataType to a QName.
fn datatype_to_qname(dt: &DataType) -> elm::QName {
    match dt {
        DataType::System(sys) => qname_system(system_type_name(sys)),
        DataType::Model { namespace, name } => qname_qualified(namespace, name),
        DataType::List(elem) => {
            let elem_qname = datatype_to_qname(elem);
            let elem_name = extract_type_name(&elem_qname);
            qname_system(&format!("List<{elem_name}>"))
        }
        DataType::Interval(point) => {
            let point_qname = datatype_to_qname(point);
            let point_name = extract_type_name(&point_qname);
            qname_system(&format!("Interval<{point_name}>"))
        }
        DataType::Tuple(_) => qname_system("Tuple"),
        DataType::Choice(_) => qname_system("Any"),
        DataType::TypeParameter(name) => qname_system(name),
        DataType::Unknown => qname_system("Any"),
    }
}

/// Extract the type name from a QName (the part after the closing brace).
fn extract_type_name(qname: &str) -> &str {
    if let Some(pos) = qname.rfind('}') {
        &qname[pos + 1..]
    } else {
        qname
    }
}

/// Get the name of a system type.
fn system_type_name(sys: &SystemType) -> &'static str {
    match sys {
        SystemType::Any => "Any",
        SystemType::Boolean => "Boolean",
        SystemType::Integer => "Integer",
        SystemType::Long => "Long",
        SystemType::Decimal => "Decimal",
        SystemType::String => "String",
        SystemType::Date => "Date",
        SystemType::DateTime => "DateTime",
        SystemType::Time => "Time",
        SystemType::Quantity => "Quantity",
        SystemType::Ratio => "Ratio",
        SystemType::Code => "Code",
        SystemType::Concept => "Concept",
        SystemType::Vocabulary => "Vocabulary",
    }
}

/// Format a decimal value for ELM.
fn format_decimal(value: f64) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_null() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Null);
        assert!(matches!(result, elm::Expression::Null(_)));
    }

    #[test]
    fn test_translate_boolean_true() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Boolean(true));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("true".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("Boolean")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_boolean_false() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Boolean(false));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("false".to_string()));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_integer() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Integer(42));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("42".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("Integer")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_integer_negative() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Integer(-123));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("-123".to_string()));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_long() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Long(9999999999));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("9999999999L".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("Long")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_decimal() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Decimal(3.5));
        if let elm::Expression::Literal(lit) = result {
            assert!(lit.value.as_ref().unwrap().starts_with("3.5"));
            assert_eq!(lit.value_type, Some(qname_system("Decimal")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_decimal_whole_number() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Decimal(42.0));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("42.0".to_string()));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_string() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::String("hello".to_string()));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("hello".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("String")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_string_empty() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::String(String::new()));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some(String::new()));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_date() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Date("2024-01-15".to_string()));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("2024-01-15".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("Date")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_datetime() {
        let mut translator = ExpressionTranslator::new();
        let result = translator
            .translate_literal(&ast::Literal::DateTime("2024-01-15T10:30:00".to_string()));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("2024-01-15T10:30:00".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("DateTime")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_time() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Time("10:30:00".to_string()));
        if let elm::Expression::Literal(lit) = result {
            assert_eq!(lit.value, Some("10:30:00".to_string()));
            assert_eq!(lit.value_type, Some(qname_system("Time")));
        } else {
            panic!("Expected Literal");
        }
    }

    #[test]
    fn test_translate_quantity() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Quantity {
            value: 100.0,
            unit: "mg".to_string(),
        });
        if let elm::Expression::Quantity(q) = result {
            assert_eq!(q.value, Some(100.0));
            assert_eq!(q.unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity");
        }
    }

    #[test]
    fn test_translate_ratio() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Ratio {
            numerator: Box::new(ast::Literal::Quantity {
                value: 1.0,
                unit: "mg".to_string(),
            }),
            denominator: Box::new(ast::Literal::Quantity {
                value: 1.0,
                unit: "mL".to_string(),
            }),
        });
        if let elm::Expression::Ratio(r) = result {
            assert!(r.numerator.is_some());
            assert!(r.denominator.is_some());
        } else {
            panic!("Expected Ratio");
        }
    }

    #[test]
    fn test_translate_code() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Code {
            code: "12345".to_string(),
            system: Some("http://snomed.info/sct".to_string()),
            display: Some("Test Code".to_string()),
        });
        if let elm::Expression::Code(c) = result {
            assert_eq!(c.code, Some("12345".to_string()));
            assert!(c.system.is_some());
            assert_eq!(c.display, Some("Test Code".to_string()));
        } else {
            panic!("Expected Code");
        }
    }

    #[test]
    fn test_translate_code_no_system() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Code {
            code: "ABC".to_string(),
            system: None,
            display: None,
        });
        if let elm::Expression::Code(c) = result {
            assert_eq!(c.code, Some("ABC".to_string()));
            assert!(c.system.is_none());
            assert!(c.display.is_none());
        } else {
            panic!("Expected Code");
        }
    }

    #[test]
    fn test_literal_type_inference() {
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Null),
            DataType::any()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Boolean(true)),
            DataType::boolean()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Integer(1)),
            DataType::integer()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Long(1)),
            DataType::long()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Decimal(1.0)),
            DataType::decimal()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::String(String::new())),
            DataType::string()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Date(String::new())),
            DataType::date()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::DateTime(String::new())),
            DataType::date_time()
        );
        assert_eq!(
            ExpressionTranslator::literal_type(&ast::Literal::Time(String::new())),
            DataType::time()
        );
    }

    #[test]
    fn test_local_id_generation() {
        let mut translator = ExpressionTranslator::new().with_local_ids();
        let result1 = translator.translate_literal(&ast::Literal::Integer(1));
        let result2 = translator.translate_literal(&ast::Literal::Integer(2));

        if let elm::Expression::Literal(lit1) = result1 {
            assert_eq!(lit1.element.local_id, Some("1".to_string()));
        }
        if let elm::Expression::Literal(lit2) = result2 {
            assert_eq!(lit2.element.local_id, Some("2".to_string()));
        }
    }

    #[test]
    fn test_no_local_id_by_default() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Integer(42));
        if let elm::Expression::Literal(lit) = result {
            assert!(lit.element.local_id.is_none());
        }
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(3.5), "3.5");
        assert_eq!(format_decimal(42.0), "42.0");
        assert_eq!(format_decimal(0.0), "0.0");
        assert_eq!(format_decimal(-1.5), "-1.5");
    }

    #[test]
    fn test_datatype_to_qname() {
        let qname = datatype_to_qname(&DataType::integer());
        assert!(qname.contains("Integer"));
        assert!(qname.contains("urn:hl7-org:elm-types:r1"));

        let qname = datatype_to_qname(&DataType::list(DataType::string()));
        assert!(qname.contains("List"));
    }

    #[test]
    fn test_qname_format() {
        let qname = qname_system("Integer");
        assert_eq!(qname, "{urn:hl7-org:elm-types:r1}Integer");

        let qname = qname_qualified("http://example.org", "Patient");
        assert_eq!(qname, "{http://example.org}Patient");
    }

    #[test]
    fn test_extract_type_name() {
        assert_eq!(
            extract_type_name("{urn:hl7-org:elm-types:r1}Integer"),
            "Integer"
        );
        assert_eq!(extract_type_name("{http://example.org}Patient"), "Patient");
        assert_eq!(extract_type_name("NoNamespace"), "NoNamespace");
    }

    // ========================================================================
    // Reference Translation Tests
    // ========================================================================

    #[test]
    fn test_translate_expression_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref(
            "MyExpression",
            ResolvedRefKind::Expression,
            Some(&DataType::integer()),
        );
        if let elm::Expression::ExpressionRef(expr_ref) = result {
            assert_eq!(expr_ref.name, Some("MyExpression".to_string()));
            assert!(expr_ref.library_name.is_none());
            assert!(expr_ref.element.result_type_name.is_some());
        } else {
            panic!("Expected ExpressionRef");
        }
    }

    #[test]
    fn test_translate_parameter_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref(
            "MeasurementPeriod",
            ResolvedRefKind::Parameter,
            None,
        );
        if let elm::Expression::ParameterRef(param_ref) = result {
            assert_eq!(param_ref.name, Some("MeasurementPeriod".to_string()));
            assert!(param_ref.library_name.is_none());
        } else {
            panic!("Expected ParameterRef");
        }
    }

    #[test]
    fn test_translate_operand_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref("value", ResolvedRefKind::Operand, None);
        if let elm::Expression::OperandRef(op_ref) = result {
            assert_eq!(op_ref.name, Some("value".to_string()));
        } else {
            panic!("Expected OperandRef");
        }
    }

    #[test]
    fn test_translate_query_alias_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref("E", ResolvedRefKind::QueryAlias, None);
        if let elm::Expression::AliasRef(alias_ref) = result {
            assert_eq!(alias_ref.name, Some("E".to_string()));
        } else {
            panic!("Expected AliasRef");
        }
    }

    #[test]
    fn test_translate_let_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref("x", ResolvedRefKind::Let, None);
        if let elm::Expression::AliasRef(alias_ref) = result {
            assert_eq!(alias_ref.name, Some("x".to_string()));
        } else {
            panic!("Expected AliasRef for Let");
        }
    }

    #[test]
    fn test_translate_codesystem_ref() {
        let mut translator = ExpressionTranslator::new();
        let result =
            translator.translate_identifier_ref("LOINC", ResolvedRefKind::CodeSystem, None);
        if let elm::Expression::CodeSystemRef(cs_ref) = result {
            assert_eq!(cs_ref.name, Some("LOINC".to_string()));
            assert!(cs_ref.library_name.is_none());
        } else {
            panic!("Expected CodeSystemRef");
        }
    }

    #[test]
    fn test_translate_valueset_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref(
            "Diabetes Conditions",
            ResolvedRefKind::ValueSet,
            None,
        );
        if let elm::Expression::ValueSetRef(vs_ref) = result {
            assert_eq!(vs_ref.name, Some("Diabetes Conditions".to_string()));
            assert!(vs_ref.library_name.is_none());
        } else {
            panic!("Expected ValueSetRef");
        }
    }

    #[test]
    fn test_translate_code_ref() {
        let mut translator = ExpressionTranslator::new();
        let result =
            translator.translate_identifier_ref("DiabetesCode", ResolvedRefKind::Code, None);
        if let elm::Expression::CodeRef(code_ref) = result {
            assert_eq!(code_ref.name, Some("DiabetesCode".to_string()));
        } else {
            panic!("Expected CodeRef");
        }
    }

    #[test]
    fn test_translate_concept_ref() {
        let mut translator = ExpressionTranslator::new();
        let result =
            translator.translate_identifier_ref("DiabetesConcept", ResolvedRefKind::Concept, None);
        if let elm::Expression::ConceptRef(concept_ref) = result {
            assert_eq!(concept_ref.name, Some("DiabetesConcept".to_string()));
        } else {
            panic!("Expected ConceptRef");
        }
    }

    #[test]
    fn test_translate_unknown_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref("Unknown", ResolvedRefKind::Unknown, None);
        if let elm::Expression::IdentifierRef(id_ref) = result {
            assert_eq!(id_ref.name, Some("Unknown".to_string()));
        } else {
            panic!("Expected IdentifierRef");
        }
    }

    #[test]
    fn test_translate_qualified_library_expression() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_qualified_ref(
            "Common",
            "MyExpression",
            QualifiedRefKind::LibraryExpression,
            None,
        );
        if let elm::Expression::ExpressionRef(expr_ref) = result {
            assert_eq!(expr_ref.name, Some("MyExpression".to_string()));
            assert_eq!(expr_ref.library_name, Some("Common".to_string()));
        } else {
            panic!("Expected ExpressionRef with library");
        }
    }

    #[test]
    fn test_translate_qualified_library_parameter() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_qualified_ref(
            "Common",
            "MeasurementPeriod",
            QualifiedRefKind::LibraryParameter,
            None,
        );
        if let elm::Expression::ParameterRef(param_ref) = result {
            assert_eq!(param_ref.name, Some("MeasurementPeriod".to_string()));
            assert_eq!(param_ref.library_name, Some("Common".to_string()));
        } else {
            panic!("Expected ParameterRef with library");
        }
    }

    #[test]
    fn test_translate_qualified_library_valueset() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_qualified_ref(
            "Terminology",
            "DiabetesConditions",
            QualifiedRefKind::LibraryValueSet,
            None,
        );
        if let elm::Expression::ValueSetRef(vs_ref) = result {
            assert_eq!(vs_ref.name, Some("DiabetesConditions".to_string()));
            assert_eq!(vs_ref.library_name, Some("Terminology".to_string()));
        } else {
            panic!("Expected ValueSetRef with library");
        }
    }

    #[test]
    fn test_translate_qualified_property() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_qualified_ref(
            "E",
            "status",
            QualifiedRefKind::Property { source: None },
            Some(&DataType::string()),
        );
        if let elm::Expression::Property(prop) = result {
            assert_eq!(prop.path, Some("status".to_string()));
            assert_eq!(prop.scope, Some("E".to_string()));
            assert!(prop.source.is_none());
        } else {
            panic!("Expected Property");
        }
    }

    #[test]
    fn test_translate_function_ref() {
        let mut translator = ExpressionTranslator::new();
        let arg1 = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: Some(qname_system("Integer")),
            value: Some("1".to_string()),
        });
        let arg2 = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: Some(qname_system("Integer")),
            value: Some("2".to_string()),
        });

        let result = translator.translate_function_ref(
            "Add",
            None,
            vec![arg1, arg2],
            vec![],
            Some(&DataType::integer()),
        );
        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("Add".to_string()));
            assert!(func_ref.library_name.is_none());
            assert_eq!(func_ref.operand.len(), 2);
        } else {
            panic!("Expected FunctionRef");
        }
    }

    #[test]
    fn test_translate_function_ref_with_library() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_function_ref(
            "CustomFunction",
            Some("MyLib"),
            vec![],
            vec![],
            None,
        );
        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("CustomFunction".to_string()));
            assert_eq!(func_ref.library_name, Some("MyLib".to_string()));
        } else {
            panic!("Expected FunctionRef with library");
        }
    }

    #[test]
    fn test_translate_ast_identifier_ref() {
        let mut translator = ExpressionTranslator::new();
        let ast_ref = ast::IdentifierRef {
            name: "MyVar".to_string(),
            location: None,
        };
        let result = translator.translate_ast_identifier_ref(&ast_ref);
        if let elm::Expression::IdentifierRef(id_ref) = result {
            assert_eq!(id_ref.name, Some("MyVar".to_string()));
            assert!(id_ref.library_name.is_none());
        } else {
            panic!("Expected IdentifierRef");
        }
    }

    #[test]
    fn test_translate_ast_qualified_ref() {
        let mut translator = ExpressionTranslator::new();
        let ast_ref = ast::QualifiedIdentifierRef {
            qualifier: "Common".to_string(),
            name: "MyExpression".to_string(),
            location: None,
        };
        let result = translator.translate_ast_qualified_ref(&ast_ref);
        if let elm::Expression::IdentifierRef(id_ref) = result {
            assert_eq!(id_ref.name, Some("MyExpression".to_string()));
            assert_eq!(id_ref.library_name, Some("Common".to_string()));
        } else {
            panic!("Expected IdentifierRef with library");
        }
    }

    #[test]
    fn test_translate_ast_function_invocation() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: Some("Common".to_string()),
            name: "Add".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::Integer(1)),
                ast::Expression::Literal(ast::Literal::Integer(2)),
            ],
            location: None,
        };

        let result = translator.translate_ast_function_invocation(&func, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal in test");
            }
        });

        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("Add".to_string()));
            assert_eq!(func_ref.library_name, Some("Common".to_string()));
            assert_eq!(func_ref.operand.len(), 2);
            assert!(func_ref.signature.is_empty());
        } else {
            panic!("Expected FunctionRef");
        }
    }

    #[test]
    fn test_ref_with_local_id() {
        let mut translator = ExpressionTranslator::new().with_local_ids();
        let result =
            translator.translate_identifier_ref("MyExpression", ResolvedRefKind::Expression, None);
        if let elm::Expression::ExpressionRef(expr_ref) = result {
            assert_eq!(expr_ref.element.local_id, Some("1".to_string()));
        } else {
            panic!("Expected ExpressionRef");
        }
    }

    #[test]
    fn test_context_ref() {
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref("Patient", ResolvedRefKind::Context, None);
        if let elm::Expression::ExpressionRef(expr_ref) = result {
            assert_eq!(expr_ref.name, Some("Patient".to_string()));
        } else {
            panic!("Expected ExpressionRef for Context");
        }
    }

    // ========================================================================
    // Phase 4.5c: Operator Translation Tests
    // ========================================================================

    #[test]
    fn test_translate_unary_not() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Boolean(true));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::Not,
            operand,
            Some(&DataType::boolean()),
        );
        if let elm::Expression::Not(unary) = result {
            assert!(unary.operand.is_some());
            assert!(matches!(
                *unary.operand.unwrap(),
                elm::Expression::Literal(_)
            ));
        } else {
            panic!("Expected Not expression");
        }
    }

    #[test]
    fn test_translate_unary_negate() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Integer(42));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::Negate,
            operand,
            Some(&DataType::integer()),
        );
        if let elm::Expression::Negate(unary) = result {
            assert!(unary.operand.is_some());
        } else {
            panic!("Expected Negate expression");
        }
    }

    #[test]
    fn test_translate_unary_exists() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::Exists,
            operand,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Exists(_)));
    }

    #[test]
    fn test_translate_unary_is_null() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::IsNull,
            operand,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::IsNull(_)));
    }

    #[test]
    fn test_translate_unary_is_true() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Boolean(true));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::IsTrue,
            operand,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::IsTrue(_)));
    }

    #[test]
    fn test_translate_unary_to_integer() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::String("42".to_string()));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::ToInteger,
            operand,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::ToInteger(_)));
    }

    #[test]
    fn test_translate_unary_to_string() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Integer(42));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::ToString,
            operand,
            Some(&DataType::string()),
        );
        assert!(matches!(result, elm::Expression::ToStringExpr(_)));
    }

    #[test]
    fn test_translate_unary_expand() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_unary_operator(ast::UnaryOperator::Expand, operand, None);
        // Expand uses BinaryExpression even with single operand
        if let elm::Expression::Expand(binary) = result {
            assert_eq!(binary.operand.len(), 1);
        } else {
            panic!("Expected Expand expression");
        }
    }

    #[test]
    fn test_translate_unary_flatten() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Null);
        let result =
            translator.translate_unary_operator(ast::UnaryOperator::Flatten, operand, None);
        assert!(matches!(result, elm::Expression::Flatten(_)));
    }

    #[test]
    fn test_translate_unary_distinct() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Null);
        let result =
            translator.translate_unary_operator(ast::UnaryOperator::Distinct, operand, None);
        assert!(matches!(result, elm::Expression::Distinct(_)));
    }

    #[test]
    fn test_translate_binary_add() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(1));
        let right = translator.translate_literal(&ast::Literal::Integer(2));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Add,
            left,
            right,
            Some(&DataType::integer()),
        );
        if let elm::Expression::Add(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Add expression");
        }
    }

    #[test]
    fn test_translate_binary_subtract() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(10));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Subtract,
            left,
            right,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Subtract(_)));
    }

    #[test]
    fn test_translate_binary_multiply() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(4));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Multiply,
            left,
            right,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Multiply(_)));
    }

    #[test]
    fn test_translate_binary_divide() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Decimal(10.0));
        let right = translator.translate_literal(&ast::Literal::Decimal(2.0));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Divide,
            left,
            right,
            Some(&DataType::decimal()),
        );
        assert!(matches!(result, elm::Expression::Divide(_)));
    }

    #[test]
    fn test_translate_binary_modulo() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(10));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Modulo,
            left,
            right,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Modulo(_)));
    }

    #[test]
    fn test_translate_binary_equal() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Equal,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Equal(_)));
    }

    #[test]
    fn test_translate_binary_not_equal() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::NotEqual,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::NotEqual(_)));
    }

    #[test]
    fn test_translate_binary_less() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(3));
        let right = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Less,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Less(_)));
    }

    #[test]
    fn test_translate_binary_greater() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Greater,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Greater(_)));
    }

    #[test]
    fn test_translate_binary_less_or_equal() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::LessOrEqual,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::LessOrEqual(_)));
    }

    #[test]
    fn test_translate_binary_greater_or_equal() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::GreaterOrEqual,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::GreaterOrEqual(_)));
    }

    #[test]
    fn test_translate_binary_and() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Boolean(true));
        let right = translator.translate_literal(&ast::Literal::Boolean(false));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::And,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::And(_)));
    }

    #[test]
    fn test_translate_binary_or() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Boolean(true));
        let right = translator.translate_literal(&ast::Literal::Boolean(false));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Or,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Or(_)));
    }

    #[test]
    fn test_translate_binary_xor() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Boolean(true));
        let right = translator.translate_literal(&ast::Literal::Boolean(false));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Xor,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Xor(_)));
    }

    #[test]
    fn test_translate_binary_implies() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Boolean(true));
        let right = translator.translate_literal(&ast::Literal::Boolean(true));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Implies,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Implies(_)));
    }

    #[test]
    fn test_translate_binary_concatenate() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::String("hello".to_string()));
        let right = translator.translate_literal(&ast::Literal::String(" world".to_string()));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Concatenate,
            left,
            right,
            Some(&DataType::string()),
        );
        assert!(matches!(result, elm::Expression::Concatenate(_)));
    }

    #[test]
    fn test_translate_binary_union() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result =
            translator.translate_binary_operator(ast::BinaryOperator::Union, left, right, None);
        assert!(matches!(result, elm::Expression::Union(_)));
    }

    #[test]
    fn test_translate_binary_intersect() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result =
            translator.translate_binary_operator(ast::BinaryOperator::Intersect, left, right, None);
        assert!(matches!(result, elm::Expression::Intersect(_)));
    }

    #[test]
    fn test_translate_binary_except() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result =
            translator.translate_binary_operator(ast::BinaryOperator::Except, left, right, None);
        assert!(matches!(result, elm::Expression::Except(_)));
    }

    #[test]
    fn test_translate_binary_contains() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Integer(1));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Contains,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Contains(_)));
    }

    #[test]
    fn test_translate_binary_in() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(1));
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::In,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::In(_)));
    }

    #[test]
    fn test_translate_binary_includes() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Includes,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Includes(_)));
    }

    #[test]
    fn test_translate_binary_included_in() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::IncludedIn,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::IncludedIn(_)));
    }

    #[test]
    fn test_translate_binary_properly_includes() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::ProperlyIncludes,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::ProperIncludes(_)));
    }

    #[test]
    fn test_translate_binary_properly_included_in() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::ProperlyIncludedIn,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::ProperIncludedIn(_)));
    }

    #[test]
    fn test_translate_binary_overlaps() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Overlaps,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Overlaps(_)));
    }

    #[test]
    fn test_translate_binary_meets() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Meets,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Meets(_)));
    }

    #[test]
    fn test_translate_ternary_between() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Integer(5));
        let low = translator.translate_literal(&ast::Literal::Integer(1));
        let high = translator.translate_literal(&ast::Literal::Integer(10));
        let result = translator.translate_ternary_operator(
            ast::TernaryOperator::Between,
            operand,
            low,
            high,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::And(_)));
    }

    #[test]
    fn test_translate_ternary_replace_matches() {
        let mut translator = ExpressionTranslator::new();
        let input = translator.translate_literal(&ast::Literal::String("hello world".to_string()));
        let pattern = translator.translate_literal(&ast::Literal::String("world".to_string()));
        let replacement =
            translator.translate_literal(&ast::Literal::String("universe".to_string()));
        let result = translator.translate_ternary_operator(
            ast::TernaryOperator::ReplaceMatches,
            input,
            pattern,
            replacement,
            Some(&DataType::string()),
        );
        if let elm::Expression::ReplaceMatches(ternary) = result {
            assert_eq!(ternary.operand.len(), 3);
        } else {
            panic!("Expected ReplaceMatches expression");
        }
    }

    #[test]
    fn test_translate_ast_unary_expression() {
        let mut translator = ExpressionTranslator::new();
        let ast_unary = ast::UnaryExpression {
            operator: ast::UnaryOperator::Not,
            operand: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            location: None,
        };
        let result = translator.translate_ast_unary_expression(&ast_unary, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal in test");
            }
        });
        assert!(matches!(result, elm::Expression::Not(_)));
    }

    #[test]
    fn test_translate_ast_binary_expression() {
        let mut translator = ExpressionTranslator::new();
        let ast_binary = ast::BinaryExpression {
            operator: ast::BinaryOperator::Add,
            left: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            right: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
            location: None,
        };
        let result = translator.translate_ast_binary_expression(&ast_binary, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal in test");
            }
        });
        if let elm::Expression::Add(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Add expression");
        }
    }

    #[test]
    fn test_translate_ast_ternary_expression() {
        let mut translator = ExpressionTranslator::new();
        let ast_ternary = ast::TernaryExpression {
            operator: ast::TernaryOperator::Between,
            first: Box::new(ast::Expression::Literal(ast::Literal::Integer(5))),
            second: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            third: Box::new(ast::Expression::Literal(ast::Literal::Integer(10))),
            location: None,
        };
        let result = translator.translate_ast_ternary_expression(&ast_ternary, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal in test");
            }
        });
        // Between translates to And(GreaterOrEqual, LessOrEqual)
        assert!(matches!(result, elm::Expression::And(_)));
    }

    #[test]
    fn test_translate_binary_power() {
        let mut translator = ExpressionTranslator::new();
        let base = translator.translate_literal(&ast::Literal::Integer(2));
        let exponent = translator.translate_literal(&ast::Literal::Integer(8));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Power,
            base,
            exponent,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Power(_)));
    }

    #[test]
    fn test_translate_binary_truncated_divide() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(10));
        let right = translator.translate_literal(&ast::Literal::Integer(3));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::TruncatedDivide,
            left,
            right,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::TruncatedDivide(_)));
    }

    #[test]
    fn test_translate_binary_equivalent() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(5));
        let right = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Equivalent,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Equivalent(_)));
    }

    #[test]
    fn test_translate_binary_starts() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Starts,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Starts(_)));
    }

    #[test]
    fn test_translate_binary_ends() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Ends,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Ends(_)));
    }

    #[test]
    fn test_translate_binary_same_as() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::SameAs,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::SameAs(_)));
    }

    #[test]
    fn test_translate_binary_before() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Before,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::Before(_)));
    }

    #[test]
    fn test_translate_binary_after() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::After,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::After(_)));
    }

    #[test]
    fn test_translate_binary_same_or_before() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::SameOrBefore,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::SameOrBefore(_)));
    }

    #[test]
    fn test_translate_binary_same_or_after() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::SameOrAfter,
            left,
            right,
            Some(&DataType::boolean()),
        );
        assert!(matches!(result, elm::Expression::SameOrAfter(_)));
    }

    #[test]
    fn test_translate_unary_predecessor() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::Predecessor,
            operand,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Predecessor(_)));
    }

    #[test]
    fn test_translate_unary_successor() {
        let mut translator = ExpressionTranslator::new();
        let operand = translator.translate_literal(&ast::Literal::Integer(5));
        let result = translator.translate_unary_operator(
            ast::UnaryOperator::Successor,
            operand,
            Some(&DataType::integer()),
        );
        assert!(matches!(result, elm::Expression::Successor(_)));
    }

    #[test]
    fn test_translate_binary_log() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Integer(100));
        let right = translator.translate_literal(&ast::Literal::Integer(10));
        let result =
            translator.translate_binary_operator(ast::BinaryOperator::Log, left, right, None);
        assert!(matches!(result, elm::Expression::Log(_)));
    }

    #[test]
    fn test_translate_binary_index_of() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Integer(1));
        let result =
            translator.translate_binary_operator(ast::BinaryOperator::IndexOf, left, right, None);
        assert!(matches!(result, elm::Expression::Indexer(_)));
    }
}
