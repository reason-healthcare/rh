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

    // ========================================================================
    // Query Translation (Phase 4.5d)
    // ========================================================================

    /// Translate a CQL AST query to an ELM Query expression.
    ///
    /// A CQL query has the form:
    /// ```cql
    /// from [Source] A
    /// let B = <expr>
    /// with [RelatedSource] R such that <condition>
    /// where <condition>
    /// return <expr>
    /// sort by <items>
    /// ```
    pub fn translate_query(
        &mut self,
        query: &ast::Query,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Translate sources
        let sources: Vec<elm::AliasedQuerySource> = query
            .sources
            .iter()
            .map(|s| self.translate_query_source(s, &translate_expr))
            .collect();

        // Translate let clauses
        let let_clauses: Vec<elm::LetClause> = query
            .let_clauses
            .iter()
            .map(|lc| self.translate_let_clause(lc, &translate_expr))
            .collect();

        // Translate relationship clauses
        let relationships: Vec<elm::RelationshipClause> = query
            .relationships
            .iter()
            .map(|r| self.translate_relationship_clause(r, &translate_expr))
            .collect();

        // Translate where clause
        let where_clause = query
            .where_clause
            .as_ref()
            .map(|w| Box::new(translate_expr(self, w)));

        // Translate return clause
        let return_clause = query
            .return_clause
            .as_ref()
            .map(|r| self.translate_return_clause(r, &translate_expr));

        // Translate sort clause
        let sort = query
            .sort_clause
            .as_ref()
            .map(|s| self.translate_sort_clause(s, &translate_expr));

        elm::Expression::Query(elm::Query {
            element,
            source: sources,
            let_clause: let_clauses,
            relationship: relationships,
            where_clause,
            return_clause,
            aggregate: None,
            sort,
        })
    }

    /// Translate a query source (aliased expression).
    pub fn translate_query_source(
        &mut self,
        source: &ast::QuerySource,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::AliasedQuerySource {
        elm::AliasedQuerySource {
            alias: Some(source.alias.clone()),
            expression: Some(Box::new(translate_expr(self, &source.expression))),
        }
    }

    /// Translate a let clause.
    pub fn translate_let_clause(
        &mut self,
        let_clause: &ast::LetClause,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::LetClause {
        elm::LetClause {
            identifier: Some(let_clause.identifier.clone()),
            expression: Some(Box::new(translate_expr(self, &let_clause.expression))),
        }
    }

    /// Translate a relationship clause (with/without).
    pub fn translate_relationship_clause(
        &mut self,
        rel: &ast::RelationshipClause,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::RelationshipClause {
        let relationship_type = match rel.kind {
            ast::RelationshipKind::With => Some("With".to_string()),
            ast::RelationshipKind::Without => Some("Without".to_string()),
        };

        elm::RelationshipClause {
            relationship_type,
            alias: Some(rel.source.alias.clone()),
            expression: Some(Box::new(translate_expr(self, &rel.source.expression))),
            such_that: rel
                .such_that
                .as_ref()
                .map(|st| Box::new(translate_expr(self, st))),
        }
    }

    /// Translate a return clause.
    pub fn translate_return_clause(
        &mut self,
        ret: &ast::ReturnClause,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::ReturnClause {
        elm::ReturnClause {
            distinct: if ret.distinct { Some(true) } else { None },
            expression: Some(Box::new(translate_expr(self, &ret.expression))),
        }
    }

    /// Translate a sort clause.
    pub fn translate_sort_clause(
        &mut self,
        sort: &ast::SortClause,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::SortClause {
        let by: Vec<elm::SortByItem> = sort
            .items
            .iter()
            .map(|item| self.translate_sort_item(item, &translate_expr))
            .collect();

        elm::SortClause { by }
    }

    /// Translate a sort item.
    pub fn translate_sort_item(
        &mut self,
        item: &ast::SortItem,
        _translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::SortByItem {
        // In ELM, sort items are typically ByExpression, ByColumn, or ByDirection
        // For simple property paths, we use ByColumn with a path
        // For complex expressions, we'd use ByExpression

        let direction = match item.direction {
            ast::SortDirection::Ascending => Some(elm::SortDirection::Asc),
            ast::SortDirection::Descending => Some(elm::SortDirection::Desc),
        };

        // Try to extract a simple path from the expression
        let path = extract_sort_path(&item.expression);

        elm::SortByItem { direction, path }
    }
}

/// Extract a simple path from an expression for sorting.
/// Returns the path string for simple identifier/property references.
fn extract_sort_path(expr: &ast::Expression) -> Option<String> {
    match expr {
        ast::Expression::IdentifierRef(id_ref) => Some(id_ref.name.clone()),
        ast::Expression::QualifiedIdentifierRef(qref) => {
            Some(format!("{}.{}", qref.qualifier, qref.name))
        }
        // For more complex expressions, we'd need ByExpression
        _ => None,
    }
}

// ============================================================================
// Retrieve Translation (Phase 4.5e)
// ============================================================================

impl ExpressionTranslator {
    /// Translate a CQL AST retrieve expression to an ELM Retrieve expression.
    ///
    /// A CQL retrieve has the form:
    /// ```cql
    /// [Condition: "Diabetes"]
    /// [Encounter: type in "Inpatient Encounters"]
    /// [Observation: code in "Blood Pressure Codes" during MeasurementPeriod]
    /// ```
    pub fn translate_retrieve(
        &mut self,
        retrieve: &ast::Retrieve,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Convert the data type specifier to a QName
        let data_type = type_specifier_to_qname(&retrieve.data_type);

        // Translate context expression if present
        let context = retrieve
            .context
            .as_ref()
            .map(|c| Box::new(translate_expr(self, c)));

        // Translate codes expression if present
        let codes = retrieve
            .codes
            .as_ref()
            .map(|c| Box::new(translate_expr(self, c)));

        // Translate date range expression if present
        let date_range = retrieve
            .date_range
            .as_ref()
            .map(|d| Box::new(translate_expr(self, d)));

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(data_type),
            template_id: None, // Set by model info resolution
            context,
            code_property: retrieve.code_path.clone(),
            code_comparator: None, // Default is "in"
            codes,
            date_property: retrieve.date_path.clone(),
            date_range,
        })
    }

    /// Translate a simple retrieve (just data type, no filters).
    pub fn translate_simple_retrieve(
        &mut self,
        data_type: &ast::TypeSpecifier,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let qname = type_specifier_to_qname(data_type);

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(qname),
            template_id: None,
            context: None,
            code_property: None,
            code_comparator: None,
            codes: None,
            date_property: None,
            date_range: None,
        })
    }

    /// Translate a retrieve with code filter.
    pub fn translate_retrieve_with_codes(
        &mut self,
        data_type: &ast::TypeSpecifier,
        code_path: &str,
        codes: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let qname = type_specifier_to_qname(data_type);

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(qname),
            template_id: None,
            context: None,
            code_property: Some(code_path.to_string()),
            code_comparator: None,
            codes: Some(Box::new(codes)),
            date_property: None,
            date_range: None,
        })
    }

    /// Translate a retrieve with date range filter.
    pub fn translate_retrieve_with_date_range(
        &mut self,
        data_type: &ast::TypeSpecifier,
        date_path: &str,
        date_range: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let qname = type_specifier_to_qname(data_type);

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(qname),
            template_id: None,
            context: None,
            code_property: None,
            code_comparator: None,
            codes: None,
            date_property: Some(date_path.to_string()),
            date_range: Some(Box::new(date_range)),
        })
    }

    /// Translate a retrieve with both code and date range filters.
    pub fn translate_retrieve_with_codes_and_date(
        &mut self,
        data_type: &ast::TypeSpecifier,
        code_path: &str,
        codes: elm::Expression,
        date_path: &str,
        date_range: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let qname = type_specifier_to_qname(data_type);

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(qname),
            template_id: None,
            context: None,
            code_property: Some(code_path.to_string()),
            code_comparator: None,
            codes: Some(Box::new(codes)),
            date_property: Some(date_path.to_string()),
            date_range: Some(Box::new(date_range)),
        })
    }

    /// Translate a retrieve with context.
    pub fn translate_retrieve_with_context(
        &mut self,
        data_type: &ast::TypeSpecifier,
        context: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let qname = type_specifier_to_qname(data_type);

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(qname),
            template_id: None,
            context: Some(Box::new(context)),
            code_property: None,
            code_comparator: None,
            codes: None,
            date_property: None,
            date_range: None,
        })
    }
}

/// Convert an AST TypeSpecifier to an ELM QName.
fn type_specifier_to_qname(ts: &ast::TypeSpecifier) -> elm::QName {
    match ts {
        ast::TypeSpecifier::Named(named) => {
            if let Some(ns) = &named.namespace {
                // Qualified name like FHIR.Patient
                format!("{{{ns}}}{}", named.name)
            } else {
                // Simple name - assume FHIR namespace for common types
                format!("{{http://hl7.org/fhir}}{}", named.name)
            }
        }
        ast::TypeSpecifier::List(list) => {
            let elem = type_specifier_to_qname(&list.element_type);
            format!("{{urn:hl7-org:elm-types:r1}}List<{elem}>")
        }
        ast::TypeSpecifier::Interval(interval) => {
            let point = type_specifier_to_qname(&interval.point_type);
            format!("{{urn:hl7-org:elm-types:r1}}Interval<{point}>")
        }
        ast::TypeSpecifier::Tuple(_) => {
            // Tuple types are anonymous
            "{urn:hl7-org:elm-types:r1}Tuple".to_string()
        }
        ast::TypeSpecifier::Choice(choice) => {
            // Choice types list alternatives
            let types: Vec<String> = choice.types.iter().map(type_specifier_to_qname).collect();
            format!("{{urn:hl7-org:elm-types:r1}}Choice<{}>", types.join(","))
        }
    }
}

// ============================================================================
// Conditional Translation (Phase 4.5f)
// ============================================================================

impl ExpressionTranslator {
    /// Translate a CQL if-then-else expression to an ELM If expression.
    ///
    /// CQL syntax:
    /// ```cql
    /// if condition then thenExpr else elseExpr
    /// ```
    pub fn translate_if_then_else(
        &mut self,
        if_expr: &ast::IfThenElse,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let condition = translate_expr(self, &if_expr.condition);
        let then_expr = translate_expr(self, &if_expr.then_expr);
        let else_expr = translate_expr(self, &if_expr.else_expr);

        elm::Expression::If(elm::IfExpr {
            element,
            condition: Some(Box::new(condition)),
            then_expr: Some(Box::new(then_expr)),
            else_expr: Some(Box::new(else_expr)),
        })
    }

    /// Translate a CQL case expression to an ELM Case expression.
    ///
    /// CQL supports two forms:
    /// 1. Simple case (with comparand):
    /// ```cql
    /// case status
    ///   when 'active' then 1
    ///   when 'inactive' then 0
    ///   else -1
    /// end
    /// ```
    ///
    /// 2. Searched case (without comparand):
    /// ```cql
    /// case
    ///   when age < 18 then 'minor'
    ///   when age < 65 then 'adult'
    ///   else 'senior'
    /// end
    /// ```
    pub fn translate_case(
        &mut self,
        case_expr: &ast::Case,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Translate comparand if present (simple case)
        let comparand = case_expr
            .comparand
            .as_ref()
            .map(|c| Box::new(translate_expr(self, c)));

        // Translate case items
        let case_items: Vec<elm::CaseItem> = case_expr
            .items
            .iter()
            .map(|item| self.translate_case_item(item, &translate_expr))
            .collect();

        // Translate else expression
        let else_expr = Some(Box::new(translate_expr(self, &case_expr.else_expr)));

        elm::Expression::Case(elm::Case {
            element,
            comparand,
            case_item: case_items,
            else_expr,
        })
    }

    /// Translate a case item (when-then pair).
    fn translate_case_item(
        &mut self,
        item: &ast::CaseItem,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::CaseItem {
        elm::CaseItem {
            when_expr: Some(Box::new(translate_expr(self, &item.when))),
            then_expr: Some(Box::new(translate_expr(self, &item.then))),
        }
    }

    /// Create an If expression directly from ELM components.
    pub fn make_if(
        &mut self,
        condition: elm::Expression,
        then_expr: elm::Expression,
        else_expr: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        elm::Expression::If(elm::IfExpr {
            element,
            condition: Some(Box::new(condition)),
            then_expr: Some(Box::new(then_expr)),
            else_expr: Some(Box::new(else_expr)),
        })
    }

    /// Create a Case expression directly from ELM components.
    pub fn make_case(
        &mut self,
        comparand: Option<elm::Expression>,
        items: Vec<(elm::Expression, elm::Expression)>,
        else_expr: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        let case_items: Vec<elm::CaseItem> = items
            .into_iter()
            .map(|(when_expr, then_expr)| elm::CaseItem {
                when_expr: Some(Box::new(when_expr)),
                then_expr: Some(Box::new(then_expr)),
            })
            .collect();

        elm::Expression::Case(elm::Case {
            element,
            comparand: comparand.map(Box::new),
            case_item: case_items,
            else_expr: Some(Box::new(else_expr)),
        })
    }

    /// Create a searched case (no comparand) from ELM components.
    pub fn make_searched_case(
        &mut self,
        items: Vec<(elm::Expression, elm::Expression)>,
        else_expr: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        self.make_case(None, items, else_expr, result_type)
    }

    /// Create a simple case (with comparand) from ELM components.
    pub fn make_simple_case(
        &mut self,
        comparand: elm::Expression,
        items: Vec<(elm::Expression, elm::Expression)>,
        else_expr: elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        self.make_case(Some(comparand), items, else_expr, result_type)
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

    // ========================================================================
    // Phase 4.5d: Query Translation Tests
    // ========================================================================

    #[test]
    fn test_translate_query_source() {
        let mut translator = ExpressionTranslator::new();
        let source = ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Encounters".to_string(),
                location: None,
            })),
            alias: "E".to_string(),
            location: None,
        };

        let result = translator.translate_query_source(&source, |t, expr| {
            if let ast::Expression::IdentifierRef(id_ref) = expr {
                t.translate_ast_identifier_ref(id_ref)
            } else {
                panic!("Expected identifier ref");
            }
        });

        assert_eq!(result.alias, Some("E".to_string()));
        assert!(result.expression.is_some());
    }

    #[test]
    fn test_translate_let_clause() {
        let mut translator = ExpressionTranslator::new();
        let let_clause = ast::LetClause {
            identifier: "result".to_string(),
            expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
            location: None,
        };

        let result = translator.translate_let_clause(&let_clause, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert_eq!(result.identifier, Some("result".to_string()));
        assert!(result.expression.is_some());
    }

    #[test]
    fn test_translate_relationship_with() {
        let mut translator = ExpressionTranslator::new();
        let rel = ast::RelationshipClause {
            kind: ast::RelationshipKind::With,
            source: ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Observations".to_string(),
                    location: None,
                })),
                alias: "O".to_string(),
                location: None,
            },
            such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            location: None,
        };

        let result = translator.translate_relationship_clause(&rel, |t, expr| match expr {
            ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
            ast::Expression::Literal(lit) => t.translate_literal(lit),
            _ => panic!("Unexpected expression"),
        });

        assert_eq!(result.relationship_type, Some("With".to_string()));
        assert_eq!(result.alias, Some("O".to_string()));
        assert!(result.expression.is_some());
        assert!(result.such_that.is_some());
    }

    #[test]
    fn test_translate_relationship_without() {
        let mut translator = ExpressionTranslator::new();
        let rel = ast::RelationshipClause {
            kind: ast::RelationshipKind::Without,
            source: ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Allergies".to_string(),
                    location: None,
                })),
                alias: "A".to_string(),
                location: None,
            },
            such_that: None,
            location: None,
        };

        let result = translator.translate_relationship_clause(&rel, |t, expr| {
            if let ast::Expression::IdentifierRef(id_ref) = expr {
                t.translate_ast_identifier_ref(id_ref)
            } else {
                panic!("Expected identifier ref");
            }
        });

        assert_eq!(result.relationship_type, Some("Without".to_string()));
        assert_eq!(result.alias, Some("A".to_string()));
        assert!(result.such_that.is_none());
    }

    #[test]
    fn test_translate_return_clause() {
        let mut translator = ExpressionTranslator::new();
        let ret = ast::ReturnClause {
            distinct: true,
            all: false,
            expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            location: None,
        };

        let result = translator.translate_return_clause(&ret, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert_eq!(result.distinct, Some(true));
        assert!(result.expression.is_some());
    }

    #[test]
    fn test_translate_return_clause_no_distinct() {
        let mut translator = ExpressionTranslator::new();
        let ret = ast::ReturnClause {
            distinct: false,
            all: false,
            expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            location: None,
        };

        let result = translator.translate_return_clause(&ret, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert!(result.distinct.is_none());
        assert!(result.expression.is_some());
    }

    #[test]
    fn test_translate_sort_clause_ascending() {
        let mut translator = ExpressionTranslator::new();
        let sort = ast::SortClause {
            items: vec![ast::SortItem {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "date".to_string(),
                    location: None,
                })),
                direction: ast::SortDirection::Ascending,
            }],
            location: None,
        };

        let result = translator.translate_sort_clause(&sort, |t, expr| {
            if let ast::Expression::IdentifierRef(id_ref) = expr {
                t.translate_ast_identifier_ref(id_ref)
            } else {
                panic!("Expected identifier ref");
            }
        });

        assert_eq!(result.by.len(), 1);
        assert_eq!(result.by[0].direction, Some(elm::SortDirection::Asc));
        assert_eq!(result.by[0].path, Some("date".to_string()));
    }

    #[test]
    fn test_translate_sort_clause_descending() {
        let mut translator = ExpressionTranslator::new();
        let sort = ast::SortClause {
            items: vec![ast::SortItem {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "priority".to_string(),
                    location: None,
                })),
                direction: ast::SortDirection::Descending,
            }],
            location: None,
        };

        let result = translator.translate_sort_clause(&sort, |_t, _expr| {
            panic!("Should not be called for path extraction");
        });

        assert_eq!(result.by.len(), 1);
        assert_eq!(result.by[0].direction, Some(elm::SortDirection::Desc));
        assert_eq!(result.by[0].path, Some("priority".to_string()));
    }

    #[test]
    fn test_translate_sort_clause_multiple() {
        let mut translator = ExpressionTranslator::new();
        let sort = ast::SortClause {
            items: vec![
                ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "category".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Ascending,
                },
                ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "date".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Descending,
                },
            ],
            location: None,
        };

        let result = translator.translate_sort_clause(&sort, |_t, _expr| {
            panic!("Should not be called for path extraction");
        });

        assert_eq!(result.by.len(), 2);
        assert_eq!(result.by[0].path, Some("category".to_string()));
        assert_eq!(result.by[0].direction, Some(elm::SortDirection::Asc));
        assert_eq!(result.by[1].path, Some("date".to_string()));
        assert_eq!(result.by[1].direction, Some(elm::SortDirection::Desc));
    }

    #[test]
    fn test_translate_simple_query() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Encounters".to_string(),
                    location: None,
                })),
                alias: "E".to_string(),
                location: None,
            }],
            let_clauses: vec![],
            relationships: vec![],
            where_clause: None,
            return_clause: None,
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.source.len(), 1);
            assert_eq!(q.source[0].alias, Some("E".to_string()));
            assert!(q.let_clause.is_empty());
            assert!(q.relationship.is_empty());
            assert!(q.where_clause.is_none());
            assert!(q.return_clause.is_none());
            assert!(q.sort.is_none());
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_query_with_where() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Patients".to_string(),
                    location: None,
                })),
                alias: "P".to_string(),
                location: None,
            }],
            let_clauses: vec![],
            relationships: vec![],
            where_clause: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            return_clause: None,
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.source.len(), 1);
            assert!(q.where_clause.is_some());
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_query_with_return() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Conditions".to_string(),
                    location: None,
                })),
                alias: "C".to_string(),
                location: None,
            }],
            let_clauses: vec![],
            relationships: vec![],
            where_clause: None,
            return_clause: Some(ast::ReturnClause {
                distinct: true,
                all: false,
                expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                location: None,
            }),
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert!(q.return_clause.is_some());
            let ret = q.return_clause.unwrap();
            assert_eq!(ret.distinct, Some(true));
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_query_with_let() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Observations".to_string(),
                    location: None,
                })),
                alias: "O".to_string(),
                location: None,
            }],
            let_clauses: vec![ast::LetClause {
                identifier: "value".to_string(),
                expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
                location: None,
            }],
            relationships: vec![],
            where_clause: None,
            return_clause: None,
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.let_clause.len(), 1);
            assert_eq!(q.let_clause[0].identifier, Some("value".to_string()));
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_query_with_relationship() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Encounters".to_string(),
                    location: None,
                })),
                alias: "E".to_string(),
                location: None,
            }],
            let_clauses: vec![],
            relationships: vec![ast::RelationshipClause {
                kind: ast::RelationshipKind::With,
                source: ast::QuerySource {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "Diagnoses".to_string(),
                        location: None,
                    })),
                    alias: "D".to_string(),
                    location: None,
                },
                such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                    true,
                )))),
                location: None,
            }],
            where_clause: None,
            return_clause: None,
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.relationship.len(), 1);
            assert_eq!(
                q.relationship[0].relationship_type,
                Some("With".to_string())
            );
            assert_eq!(q.relationship[0].alias, Some("D".to_string()));
            assert!(q.relationship[0].such_that.is_some());
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_query_with_sort() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Events".to_string(),
                    location: None,
                })),
                alias: "Ev".to_string(),
                location: None,
            }],
            let_clauses: vec![],
            relationships: vec![],
            where_clause: None,
            return_clause: None,
            sort_clause: Some(ast::SortClause {
                items: vec![ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "timestamp".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Descending,
                }],
                location: None,
            }),
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert!(q.sort.is_some());
            let sort = q.sort.unwrap();
            assert_eq!(sort.by.len(), 1);
            assert_eq!(sort.by[0].path, Some("timestamp".to_string()));
            assert_eq!(sort.by[0].direction, Some(elm::SortDirection::Desc));
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_complex_query() {
        let mut translator = ExpressionTranslator::new();
        // from Encounters E
        // let Duration = ...
        // with Diagnoses D such that ...
        // where ...
        // return distinct ...
        // sort by date desc
        let query = ast::Query {
            sources: vec![ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Encounters".to_string(),
                    location: None,
                })),
                alias: "E".to_string(),
                location: None,
            }],
            let_clauses: vec![ast::LetClause {
                identifier: "Duration".to_string(),
                expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(30))),
                location: None,
            }],
            relationships: vec![ast::RelationshipClause {
                kind: ast::RelationshipKind::With,
                source: ast::QuerySource {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "Diagnoses".to_string(),
                        location: None,
                    })),
                    alias: "D".to_string(),
                    location: None,
                },
                such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                    true,
                )))),
                location: None,
            }],
            where_clause: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            return_clause: Some(ast::ReturnClause {
                distinct: true,
                all: false,
                expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                location: None,
            }),
            sort_clause: Some(ast::SortClause {
                items: vec![ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "date".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Descending,
                }],
                location: None,
            }),
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.source.len(), 1);
            assert_eq!(q.source[0].alias, Some("E".to_string()));
            assert_eq!(q.let_clause.len(), 1);
            assert_eq!(q.relationship.len(), 1);
            assert!(q.where_clause.is_some());
            assert!(q.return_clause.is_some());
            assert!(q.sort.is_some());
        } else {
            panic!("Expected Query expression");
        }
    }

    #[test]
    fn test_translate_multi_source_query() {
        let mut translator = ExpressionTranslator::new();
        let query = ast::Query {
            sources: vec![
                ast::QuerySource {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "Encounters".to_string(),
                        location: None,
                    })),
                    alias: "E".to_string(),
                    location: None,
                },
                ast::QuerySource {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "Conditions".to_string(),
                        location: None,
                    })),
                    alias: "C".to_string(),
                    location: None,
                },
            ],
            let_clauses: vec![],
            relationships: vec![],
            where_clause: None,
            return_clause: None,
            sort_clause: None,
            location: None,
        };

        let result = translator.translate_query(
            &query,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression type"),
            },
            None,
        );

        if let elm::Expression::Query(q) = result {
            assert_eq!(q.source.len(), 2);
            assert_eq!(q.source[0].alias, Some("E".to_string()));
            assert_eq!(q.source[1].alias, Some("C".to_string()));
        } else {
            panic!("Expected Query expression");
        }
    }

    // ========================================================================
    // Phase 4.5e: Retrieve Translation Tests
    // ========================================================================

    fn make_named_type(name: &str) -> ast::TypeSpecifier {
        ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: None,
            name: name.to_string(),
        })
    }

    fn make_qualified_type(ns: &str, name: &str) -> ast::TypeSpecifier {
        ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: Some(ns.to_string()),
            name: name.to_string(),
        })
    }

    #[test]
    fn test_translate_simple_retrieve() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_named_type("Condition");

        let result = translator.translate_simple_retrieve(&data_type, None);

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert!(r.data_type.unwrap().contains("Condition"));
            assert!(r.codes.is_none());
            assert!(r.date_range.is_none());
            assert!(r.context.is_none());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_qualified_type() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_qualified_type("FHIR", "Patient");

        let result = translator.translate_simple_retrieve(&data_type, None);

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            let dt = r.data_type.unwrap();
            assert!(dt.contains("FHIR"));
            assert!(dt.contains("Patient"));
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_with_codes() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_named_type("Condition");
        let codes =
            translator.translate_identifier_ref("DiabetesCodes", ResolvedRefKind::ValueSet, None);

        let result = translator.translate_retrieve_with_codes(&data_type, "code", codes, None);

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert_eq!(r.code_property, Some("code".to_string()));
            assert!(r.codes.is_some());
            assert!(r.date_range.is_none());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_with_date_range() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_named_type("Encounter");
        let date_range = translator.translate_identifier_ref(
            "MeasurementPeriod",
            ResolvedRefKind::Parameter,
            None,
        );

        let result =
            translator.translate_retrieve_with_date_range(&data_type, "period", date_range, None);

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert_eq!(r.date_property, Some("period".to_string()));
            assert!(r.date_range.is_some());
            assert!(r.codes.is_none());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_with_codes_and_date() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_named_type("Observation");
        let codes = translator.translate_identifier_ref(
            "BloodPressureCodes",
            ResolvedRefKind::ValueSet,
            None,
        );
        let date_range = translator.translate_identifier_ref(
            "MeasurementPeriod",
            ResolvedRefKind::Parameter,
            None,
        );

        let result = translator.translate_retrieve_with_codes_and_date(
            &data_type,
            "code",
            codes,
            "effective",
            date_range,
            None,
        );

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert_eq!(r.code_property, Some("code".to_string()));
            assert!(r.codes.is_some());
            assert_eq!(r.date_property, Some("effective".to_string()));
            assert!(r.date_range.is_some());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_with_context() {
        let mut translator = ExpressionTranslator::new();
        let data_type = make_named_type("Condition");
        let context =
            translator.translate_identifier_ref("Patient", ResolvedRefKind::Context, None);

        let result = translator.translate_retrieve_with_context(&data_type, context, None);

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert!(r.context.is_some());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_full_retrieve() {
        let mut translator = ExpressionTranslator::new();
        let retrieve = ast::Retrieve {
            data_type: make_named_type("Condition"),
            context: None,
            code_path: Some("code".to_string()),
            codes: Some(Box::new(ast::Expression::IdentifierRef(
                ast::IdentifierRef {
                    name: "DiabetesCodes".to_string(),
                    location: None,
                },
            ))),
            date_path: Some("onset".to_string()),
            date_range: Some(Box::new(ast::Expression::IdentifierRef(
                ast::IdentifierRef {
                    name: "MeasurementPeriod".to_string(),
                    location: None,
                },
            ))),
            location: None,
        };

        let result = translator.translate_retrieve(
            &retrieve,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected identifier ref");
                }
            },
            None,
        );

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert_eq!(r.code_property, Some("code".to_string()));
            assert!(r.codes.is_some());
            assert_eq!(r.date_property, Some("onset".to_string()));
            assert!(r.date_range.is_some());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_no_filters() {
        let mut translator = ExpressionTranslator::new();
        let retrieve = ast::Retrieve {
            data_type: make_named_type("Patient"),
            context: None,
            code_path: None,
            codes: None,
            date_path: None,
            date_range: None,
            location: None,
        };

        let result = translator.translate_retrieve(
            &retrieve,
            |_t, _expr| {
                panic!("Should not be called for empty retrieve");
            },
            None,
        );

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert!(r.data_type.unwrap().contains("Patient"));
            assert!(r.codes.is_none());
            assert!(r.date_range.is_none());
            assert!(r.context.is_none());
            assert!(r.code_property.is_none());
            assert!(r.date_property.is_none());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_translate_retrieve_with_context_full() {
        let mut translator = ExpressionTranslator::new();
        let retrieve = ast::Retrieve {
            data_type: make_named_type("Observation"),
            context: Some(Box::new(ast::Expression::IdentifierRef(
                ast::IdentifierRef {
                    name: "Patient".to_string(),
                    location: None,
                },
            ))),
            code_path: None,
            codes: None,
            date_path: None,
            date_range: None,
            location: None,
        };

        let result = translator.translate_retrieve(
            &retrieve,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected identifier ref");
                }
            },
            None,
        );

        if let elm::Expression::Retrieve(r) = result {
            assert!(r.data_type.is_some());
            assert!(r.context.is_some());
        } else {
            panic!("Expected Retrieve expression");
        }
    }

    #[test]
    fn test_type_specifier_to_qname_named() {
        let ts = make_named_type("Condition");
        let qname = super::type_specifier_to_qname(&ts);
        assert!(qname.contains("Condition"));
        assert!(qname.contains("http://hl7.org/fhir"));
    }

    #[test]
    fn test_type_specifier_to_qname_qualified() {
        let ts = make_qualified_type("FHIR", "Patient");
        let qname = super::type_specifier_to_qname(&ts);
        assert!(qname.contains("Patient"));
        assert!(qname.contains("FHIR"));
    }

    #[test]
    fn test_type_specifier_to_qname_list() {
        let ts = ast::TypeSpecifier::List(ast::ListTypeSpecifier {
            element_type: Box::new(make_named_type("Condition")),
        });
        let qname = super::type_specifier_to_qname(&ts);
        assert!(qname.contains("List"));
    }

    #[test]
    fn test_type_specifier_to_qname_interval() {
        let ts = ast::TypeSpecifier::Interval(ast::IntervalTypeSpecifier {
            point_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: Some("urn:hl7-org:elm-types:r1".to_string()),
                name: "DateTime".to_string(),
            })),
        });
        let qname = super::type_specifier_to_qname(&ts);
        assert!(qname.contains("Interval"));
    }

    // ========================================================================
    // Phase 4.5f: Conditional Translation Tests
    // ========================================================================

    #[test]
    fn test_translate_if_then_else() {
        let mut translator = ExpressionTranslator::new();
        let if_expr = ast::IfThenElse {
            condition: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            then_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
            location: None,
        };

        let result = translator.translate_if_then_else(
            &if_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            None,
        );

        if let elm::Expression::If(if_result) = result {
            assert!(if_result.condition.is_some());
            assert!(if_result.then_expr.is_some());
            assert!(if_result.else_expr.is_some());
        } else {
            panic!("Expected If expression");
        }
    }

    #[test]
    fn test_translate_if_then_else_with_type() {
        let mut translator = ExpressionTranslator::new();
        let if_expr = ast::IfThenElse {
            condition: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            then_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
                "yes".to_string(),
            ))),
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
                "no".to_string(),
            ))),
            location: None,
        };

        let result = translator.translate_if_then_else(
            &if_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            Some(&DataType::string()),
        );

        if let elm::Expression::If(if_result) = result {
            assert!(if_result.element.result_type_name.is_some());
        } else {
            panic!("Expected If expression");
        }
    }

    #[test]
    fn test_translate_searched_case() {
        let mut translator = ExpressionTranslator::new();
        let case_expr = ast::Case {
            comparand: None,
            items: vec![
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(false))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
                },
            ],
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
            location: None,
        };

        let result = translator.translate_case(
            &case_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            None,
        );

        if let elm::Expression::Case(case_result) = result {
            assert!(case_result.comparand.is_none());
            assert_eq!(case_result.case_item.len(), 2);
            assert!(case_result.else_expr.is_some());
        } else {
            panic!("Expected Case expression");
        }
    }

    #[test]
    fn test_translate_simple_case() {
        let mut translator = ExpressionTranslator::new();
        let case_expr = ast::Case {
            comparand: Some(Box::new(ast::Expression::Literal(ast::Literal::String(
                "status".to_string(),
            )))),
            items: vec![
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::String(
                        "active".to_string(),
                    ))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::String(
                        "inactive".to_string(),
                    ))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
                },
            ],
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(-1))),
            location: None,
        };

        let result = translator.translate_case(
            &case_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            None,
        );

        if let elm::Expression::Case(case_result) = result {
            assert!(case_result.comparand.is_some());
            assert_eq!(case_result.case_item.len(), 2);
            assert!(case_result.else_expr.is_some());
        } else {
            panic!("Expected Case expression");
        }
    }

    #[test]
    fn test_translate_case_single_item() {
        let mut translator = ExpressionTranslator::new();
        let case_expr = ast::Case {
            comparand: None,
            items: vec![ast::CaseItem {
                when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                then: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
            }],
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
            location: None,
        };

        let result = translator.translate_case(
            &case_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            None,
        );

        if let elm::Expression::Case(case_result) = result {
            assert_eq!(case_result.case_item.len(), 1);
        } else {
            panic!("Expected Case expression");
        }
    }

    #[test]
    fn test_make_if() {
        let mut translator = ExpressionTranslator::new();
        let condition = translator.translate_literal(&ast::Literal::Boolean(true));
        let then_expr = translator.translate_literal(&ast::Literal::Integer(1));
        let else_expr = translator.translate_literal(&ast::Literal::Integer(0));

        let result = translator.make_if(condition, then_expr, else_expr, None);

        if let elm::Expression::If(if_result) = result {
            assert!(if_result.condition.is_some());
            assert!(if_result.then_expr.is_some());
            assert!(if_result.else_expr.is_some());
        } else {
            panic!("Expected If expression");
        }
    }

    #[test]
    fn test_make_searched_case() {
        let mut translator = ExpressionTranslator::new();
        let items = vec![
            (
                translator.translate_literal(&ast::Literal::Boolean(true)),
                translator.translate_literal(&ast::Literal::Integer(1)),
            ),
            (
                translator.translate_literal(&ast::Literal::Boolean(false)),
                translator.translate_literal(&ast::Literal::Integer(2)),
            ),
        ];
        let else_expr = translator.translate_literal(&ast::Literal::Integer(0));

        let result = translator.make_searched_case(items, else_expr, None);

        if let elm::Expression::Case(case_result) = result {
            assert!(case_result.comparand.is_none());
            assert_eq!(case_result.case_item.len(), 2);
        } else {
            panic!("Expected Case expression");
        }
    }

    #[test]
    fn test_make_simple_case() {
        let mut translator = ExpressionTranslator::new();
        let comparand = translator.translate_literal(&ast::Literal::String("x".to_string()));
        let items = vec![
            (
                translator.translate_literal(&ast::Literal::String("a".to_string())),
                translator.translate_literal(&ast::Literal::Integer(1)),
            ),
            (
                translator.translate_literal(&ast::Literal::String("b".to_string())),
                translator.translate_literal(&ast::Literal::Integer(2)),
            ),
        ];
        let else_expr = translator.translate_literal(&ast::Literal::Integer(0));

        let result = translator.make_simple_case(comparand, items, else_expr, None);

        if let elm::Expression::Case(case_result) = result {
            assert!(case_result.comparand.is_some());
            assert_eq!(case_result.case_item.len(), 2);
        } else {
            panic!("Expected Case expression");
        }
    }

    #[test]
    fn test_case_item_translation() {
        let mut translator = ExpressionTranslator::new();
        let item = ast::CaseItem {
            when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            then: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
        };

        let result = translator.translate_case_item(&item, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert!(result.when_expr.is_some());
        assert!(result.then_expr.is_some());
    }

    #[test]
    fn test_nested_if_then_else() {
        let mut translator = ExpressionTranslator::new();
        // if true then (if false then 1 else 2) else 3
        let inner_if = ast::IfThenElse {
            condition: Box::new(ast::Expression::Literal(ast::Literal::Boolean(false))),
            then_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
            location: None,
        };

        let outer_if = ast::IfThenElse {
            condition: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            then_expr: Box::new(ast::Expression::IfThenElse(inner_if)),
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(3))),
            location: None,
        };

        fn translate_expr_recursive(
            t: &mut ExpressionTranslator,
            expr: &ast::Expression,
        ) -> elm::Expression {
            match expr {
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                ast::Expression::IfThenElse(if_expr) => {
                    t.translate_if_then_else(if_expr, translate_expr_recursive, None)
                }
                _ => panic!("Unexpected expression"),
            }
        }

        let result = translator.translate_if_then_else(&outer_if, translate_expr_recursive, None);

        if let elm::Expression::If(outer) = result {
            assert!(outer.condition.is_some());
            // Check that then_expr is also an If
            if let Some(then_box) = outer.then_expr {
                assert!(matches!(*then_box, elm::Expression::If(_)));
            } else {
                panic!("Expected then expression");
            }
        } else {
            panic!("Expected If expression");
        }
    }

    #[test]
    fn test_case_with_many_items() {
        let mut translator = ExpressionTranslator::new();
        let case_expr = ast::Case {
            comparand: None,
            items: vec![
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(3))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(4))),
                },
                ast::CaseItem {
                    when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
                    then: Box::new(ast::Expression::Literal(ast::Literal::Integer(5))),
                },
            ],
            else_expr: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
            location: None,
        };

        let result = translator.translate_case(
            &case_expr,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected literal");
                }
            },
            None,
        );

        if let elm::Expression::Case(case_result) = result {
            assert_eq!(case_result.case_item.len(), 5);
        } else {
            panic!("Expected Case expression");
        }
    }
}
