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

use crate::datatype::{DataType, SystemType, TupleElement};
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
    /// Compiler options controlling translation behavior.
    options: std::sync::Arc<crate::options::CompilerOptions>,
    /// Stack of query alias scopes for tracking in-scope query aliases.
    /// Each level represents a query context; inner queries push new scopes.
    /// Maps alias name to optional type name (e.g., "ER" -> Some("Observation")).
    query_alias_scopes: Vec<std::collections::HashMap<String, Option<String>>>,
}

impl Default for ExpressionTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionTranslator {
    /// Create a new expression translator with default options.
    pub fn new() -> Self {
        Self {
            local_id_counter: 0,
            options: std::sync::Arc::new(crate::options::CompilerOptions::default()),
            query_alias_scopes: Vec::new(),
        }
    }

    /// Create a new expression translator with specific options.
    pub fn with_options(options: crate::options::CompilerOptions) -> Self {
        Self {
            local_id_counter: 0,
            options: std::sync::Arc::new(options),
            query_alias_scopes: Vec::new(),
        }
    }

    /// Create a new expression translator with shared options.
    pub fn with_shared_options(options: std::sync::Arc<crate::options::CompilerOptions>) -> Self {
        Self {
            local_id_counter: 0,
            options,
            query_alias_scopes: Vec::new(),
        }
    }

    /// Enable local ID generation (for backwards compatibility).
    ///
    /// This is equivalent to enabling `EnableAnnotations` in options.
    pub fn with_local_ids(mut self) -> Self {
        // Create new options with annotations enabled
        let mut new_options = (*self.options).clone();
        new_options
            .options
            .insert(crate::options::CompilerOption::EnableAnnotations);
        self.options = std::sync::Arc::new(new_options);
        self
    }

    /// Get the current compiler options.
    pub fn options(&self) -> &crate::options::CompilerOptions {
        &self.options
    }

    // ========================================================================
    // Query Scope Management
    // ========================================================================

    /// Push a new query scope with the given aliases (without type info).
    ///
    /// Called when entering a query context to track which identifiers
    /// should resolve to query aliases (using `scope` attribute) vs other references.
    pub fn push_query_scope(&mut self, aliases: impl IntoIterator<Item = String>) {
        self.query_alias_scopes
            .push(aliases.into_iter().map(|a| (a, None)).collect());
    }

    /// Push a new query scope with the given aliases and their types.
    ///
    /// Called when entering a query context where we know the types of the aliases.
    /// The type name is the simple type name (e.g., "Observation" not "FHIR.Observation").
    pub fn push_query_scope_typed(
        &mut self,
        aliases: impl IntoIterator<Item = (String, Option<String>)>,
    ) {
        self.query_alias_scopes.push(aliases.into_iter().collect());
    }

    /// Pop the current query scope.
    ///
    /// Called when exiting a query context.
    pub fn pop_query_scope(&mut self) {
        self.query_alias_scopes.pop();
    }

    /// Check if an identifier is a query alias in the current scope.
    ///
    /// Returns true if the identifier matches any alias in any active query scope.
    pub fn is_query_alias(&self, name: &str) -> bool {
        self.query_alias_scopes
            .iter()
            .any(|scope| scope.contains_key(name))
    }

    /// Get the type of a query alias in the current scope.
    ///
    /// Returns the type name if the alias exists and has a known type.
    pub fn get_query_alias_type(&self, name: &str) -> Option<&str> {
        for scope in self.query_alias_scopes.iter().rev() {
            if let Some(type_opt) = scope.get(name) {
                return type_opt.as_deref();
            }
        }
        None
    }

    // ========================================================================
    // Local ID and Element Fields
    // ========================================================================

    /// Generate a new local ID if annotations are enabled.
    pub fn next_local_id(&mut self) -> Option<String> {
        if self.options.annotations_enabled() {
            self.local_id_counter += 1;
            Some(self.local_id_counter.to_string())
        } else {
            None
        }
    }

    /// Format a source location as a locator string.
    ///
    /// The format is "line:column" for a single point, matching ELM spec.
    pub fn format_locator(loc: &crate::parser::span::SourceLocation) -> String {
        format!("{}:{}", loc.line, loc.column)
    }

    /// Create element fields with optional local ID.
    pub fn element_fields(&mut self) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            ..Default::default()
        }
    }

    /// Create element fields with a locator.
    pub fn element_fields_with_locator(
        &mut self,
        location: Option<&crate::parser::span::SourceLocation>,
    ) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            locator: if self.options.locators_enabled() {
                location.map(Self::format_locator)
            } else {
                None
            },
            ..Default::default()
        }
    }

    /// Create element fields with a result type.
    pub fn element_fields_typed(&mut self, result_type: &DataType) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            result_type_name: if self.options.result_types_enabled() {
                Some(datatype_to_qname(result_type))
            } else {
                None
            },
            ..Default::default()
        }
    }

    /// Create element fields with a result type and locator.
    pub fn element_fields_typed_with_locator(
        &mut self,
        result_type: &DataType,
        location: Option<&crate::parser::span::SourceLocation>,
    ) -> elm::ElementFields {
        elm::ElementFields {
            local_id: self.next_local_id(),
            locator: if self.options.locators_enabled() {
                location.map(Self::format_locator)
            } else {
                None
            },
            result_type_name: if self.options.result_types_enabled() {
                Some(datatype_to_qname(result_type))
            } else {
                None
            },
            ..Default::default()
        }
    }

    // ========================================================================
    // Unified Expression Translation (Phase 6.4a)
    // ========================================================================

    /// Translate any CQL AST expression to an ELM expression.
    ///
    /// This is the main entry point for expression translation. It dispatches
    /// to the appropriate translation method based on the expression type.
    ///
    /// # Example
    ///
    /// ```
    /// use rh_cql::translator::ExpressionTranslator;
    /// use rh_cql::parser::CqlParser;
    ///
    /// let parser = CqlParser::new();
    /// let ast_expr = parser.parse_expression("42 + 1").unwrap();
    ///
    /// let mut translator = ExpressionTranslator::new();
    /// let elm_expr = translator.translate_expression(&ast_expr);
    /// ```
    pub fn translate_expression(&mut self, expr: &ast::Expression) -> elm::Expression {
        match expr {
            // Literals
            ast::Expression::Literal(lit) => self.translate_literal(lit),

            // References
            ast::Expression::IdentifierRef(id_ref) => self.translate_ast_identifier_ref(id_ref),
            ast::Expression::QualifiedIdentifierRef(qid_ref) => {
                self.translate_ast_qualified_ref(qid_ref)
            }

            // Operators
            ast::Expression::UnaryExpression(unary) => {
                self.translate_ast_unary_expression(unary, |s, e| s.translate_expression(e))
            }
            ast::Expression::BinaryExpression(binary) => {
                self.translate_ast_binary_expression(binary, |s, e| s.translate_expression(e))
            }
            ast::Expression::TernaryExpression(ternary) => {
                self.translate_ast_ternary_expression(ternary, |s, e| s.translate_expression(e))
            }
            ast::Expression::DateTimeComponentFrom(dtc) => {
                self.translate_datetime_component_from(dtc, |s, e| s.translate_expression(e))
            }

            // Type operations
            ast::Expression::TypeExpression(type_expr) => {
                self.translate_type_expression(type_expr, |s, e| s.translate_expression(e))
            }

            // Function/invocation
            ast::Expression::FunctionInvocation(func) => {
                self.translate_ast_function_invocation(func, |s, e| s.translate_expression(e))
            }
            ast::Expression::MemberInvocation(member) => self.translate_member_invocation(member),
            ast::Expression::IndexInvocation(index) => self.translate_index_invocation(index),

            // Query
            ast::Expression::Query(query) => {
                self.translate_query(query, |s, e| s.translate_expression(e), None)
            }
            ast::Expression::Retrieve(retrieve) => {
                self.translate_retrieve(retrieve, |s, e| s.translate_expression(e), None)
            }

            // Conditionals
            ast::Expression::IfThenElse(if_expr) => {
                self.translate_if_then_else(if_expr, |s, e| s.translate_expression(e), None)
            }
            ast::Expression::Case(case_expr) => {
                self.translate_case(case_expr, |s, e| s.translate_expression(e), None)
            }

            // Interval/List/Tuple constructors
            ast::Expression::IntervalExpression(interval) => {
                self.translate_interval_expression(interval)
            }
            ast::Expression::ListExpression(list) => self.translate_list_expression(list),
            ast::Expression::TupleExpression(tuple) => self.translate_tuple_expression(tuple),

            // Instance
            ast::Expression::Instance(instance) => self.translate_instance_expression(instance),

            // Let
            ast::Expression::Let(let_clause) => self.translate_let_expression(let_clause),

            // Parenthesized - unwrap and translate inner expression
            ast::Expression::Parenthesized(inner) => self.translate_expression(inner),
        }
    }

    // ========================================================================
    // Interval/List/Tuple/Instance Translation (Phase 6.4a)
    // ========================================================================

    /// Translate an interval expression.
    ///
    /// CQL syntax: `Interval[low, high]` or `Interval(low, high)`
    fn translate_interval_expression(
        &mut self,
        interval: &ast::IntervalExpression,
    ) -> elm::Expression {
        let element = self.element_fields();

        let low = interval
            .low
            .as_ref()
            .map(|e| Box::new(self.translate_expression(e)));
        let high = interval
            .high
            .as_ref()
            .map(|e| Box::new(self.translate_expression(e)));

        elm::Expression::Interval(elm::IntervalExpr {
            element,
            low,
            high,
            low_closed: Some(interval.low_closed),
            high_closed: Some(interval.high_closed),
            low_closed_expression: None,
            high_closed_expression: None,
        })
    }

    /// Translate a list expression.
    ///
    /// CQL syntax: `{ expr1, expr2, ... }` or `List<Type> { ... }`
    fn translate_list_expression(&mut self, list: &ast::ListExpression) -> elm::Expression {
        let element = self.element_fields();

        let elements: Vec<elm::Expression> = list
            .elements
            .iter()
            .map(|e| self.translate_expression(e))
            .collect();

        let type_specifier = list
            .type_specifier
            .as_ref()
            .map(|ts| self.type_specifier_to_elm(ts));

        elm::Expression::List(elm::ListExpr {
            element,
            type_specifier,
            elements,
        })
    }

    /// Translate a tuple expression.
    ///
    /// CQL syntax: `Tuple { name: value, ... }`
    fn translate_tuple_expression(&mut self, tuple: &ast::TupleExpression) -> elm::Expression {
        let element = self.element_fields();

        let elements: Vec<elm::TupleElement> = tuple
            .elements
            .iter()
            .map(|e| elm::TupleElement {
                name: Some(e.name.clone()),
                value: Some(Box::new(self.translate_expression(&e.value))),
            })
            .collect();

        elm::Expression::Tuple(elm::TupleExpr { element, elements })
    }

    /// Translate an instance expression (type instantiation).
    ///
    /// CQL syntax: `TypeName { element: value, ... }`
    fn translate_instance_expression(&mut self, instance: &ast::Instance) -> elm::Expression {
        let element = self.element_fields();

        let class_type = Some(type_specifier_to_qname(&instance.class_type));

        let elements: Vec<elm::InstanceElement> = instance
            .elements
            .iter()
            .map(|e| elm::InstanceElement {
                name: Some(e.name.clone()),
                value: Some(Box::new(self.translate_expression(&e.value))),
            })
            .collect();

        elm::Expression::Instance(elm::Instance {
            element,
            class_type,
            elements,
        })
    }

    /// Translate a let expression.
    ///
    /// Note: Let expressions in CQL are typically part of queries.
    /// Standalone let expressions translate to the inner expression
    /// after binding the identifier in scope.
    fn translate_let_expression(&mut self, let_clause: &ast::LetClause) -> elm::Expression {
        // For standalone let expressions, we translate the expression
        // The actual let binding would be handled by query translation
        self.translate_expression(&let_clause.expression)
    }

    // ========================================================================
    // Member/Index Invocation Translation (Phase 6.4a)
    // ========================================================================

    /// Translate a member invocation (property access).
    ///
    /// CQL syntax: `expr.member` → Property expression
    ///
    /// When the source is a simple identifier that matches a query alias,
    /// uses the `scope` attribute instead of a nested `source` expression.
    /// This produces more compact ELM that matches the reference translator.
    fn translate_member_invocation(&mut self, member: &ast::MemberInvocation) -> elm::Expression {
        // Check if source is a simple identifier that's a query alias
        if let ast::Expression::IdentifierRef(ident_ref) = &*member.source {
            if self.is_query_alias(&ident_ref.name) {
                // Use scope attribute for query alias references
                return elm::Expression::Property(elm::Property {
                    element: self.element_fields(),
                    source: None,
                    path: Some(member.name.clone()),
                    scope: Some(ident_ref.name.clone()),
                });
            }
        }

        // Default: translate source expression normally
        let source = self.translate_expression(&member.source);

        // Create Property expression for member access
        elm::Expression::Property(elm::Property {
            element: self.element_fields(),
            source: Some(Box::new(source)),
            path: Some(member.name.clone()),
            scope: None,
        })
    }

    /// Translate an index invocation (indexer access).
    ///
    /// CQL syntax: `expr[index]`
    fn translate_index_invocation(&mut self, index: &ast::IndexInvocation) -> elm::Expression {
        let source = self.translate_expression(&index.source);
        let index_expr = self.translate_expression(&index.index);

        elm::Expression::Indexer(elm::BinaryExpression {
            element: self.element_fields(),
            operand: vec![source, index_expr],
            signature: Vec::new(),
        })
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
            ast::UnaryOperator::DateFrom => elm::Expression::DateFrom(unary),
            ast::UnaryOperator::TimeFrom => elm::Expression::TimeFrom(unary),
            ast::UnaryOperator::TimezoneOffsetFrom => elm::Expression::TimezoneOffsetFrom(unary),
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

    /// Wrap a CodeRef expression in ToConcept for type coercion.
    ///
    /// This is used to convert Code to Concept for Equivalent comparisons.
    /// Unlike FHIRHelpers.ToConcept (which converts FHIR types), this is
    /// the built-in CQL conversion from Code to Concept.
    fn maybe_wrap_code_in_to_concept(&mut self, expr: elm::Expression) -> elm::Expression {
        if matches!(expr, elm::Expression::CodeRef(_)) {
            let local_id = self.next_local_id();
            elm::Expression::ToConcept(elm::UnaryExpression {
                element: elm::ElementFields {
                    local_id,
                    locator: None,
                    result_type_name: None,
                    result_type_specifier: None,
                },
                operand: Some(Box::new(expr)),
                signature: Vec::new(),
            })
        } else {
            expr
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
        precision: Option<&ast::DateTimePrecision>,
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

        let precision_str = precision.map(|p| match p {
            ast::DateTimePrecision::Year => "Year".to_string(),
            ast::DateTimePrecision::Month => "Month".to_string(),
            ast::DateTimePrecision::Week => "Week".to_string(),
            ast::DateTimePrecision::Day => "Day".to_string(),
            ast::DateTimePrecision::Hour => "Hour".to_string(),
            ast::DateTimePrecision::Minute => "Minute".to_string(),
            ast::DateTimePrecision::Second => "Second".to_string(),
            ast::DateTimePrecision::Millisecond => "Millisecond".to_string(),
        });

        let time_binary = || elm::TimeBinaryExpression {
            element: element.clone(),
            operand: vec![left.clone(), right.clone()],
            signature: Vec::new(),
            precision: precision_str.clone(),
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
            ast::BinaryOperator::Equivalent => {
                // For Equivalent comparisons, wrap CodeRef operands in ToConcept
                // to convert Code to Concept for proper comparison
                let left_wrapped = self.maybe_wrap_code_in_to_concept(left);
                let right_wrapped = self.maybe_wrap_code_in_to_concept(right);
                elm::Expression::Equivalent(elm::BinaryExpression {
                    element,
                    operand: vec![left_wrapped, right_wrapped],
                    signature: Vec::new(),
                })
            }
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
            ast::BinaryOperator::In => elm::Expression::In(time_binary()),
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
        self.translate_binary_operator(expr.operator, left, right, None, expr.precision.as_ref())
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

    /// Translate a DateTime component extraction (year from, month from, etc.)
    pub fn translate_datetime_component_from(
        &mut self,
        expr: &ast::DateTimeComponentFromExpr,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let operand = translate_expr(self, &expr.operand);
        let precision_str = match expr.precision {
            ast::DateTimePrecision::Year => "Year",
            ast::DateTimePrecision::Month => "Month",
            ast::DateTimePrecision::Day => "Day",
            ast::DateTimePrecision::Hour => "Hour",
            ast::DateTimePrecision::Minute => "Minute",
            ast::DateTimePrecision::Second => "Second",
            ast::DateTimePrecision::Millisecond => "Millisecond",
            ast::DateTimePrecision::Week => "Week",
        };
        elm::Expression::DateTimeComponentFrom(elm::DateTimeComponentFrom {
            element: self.element_fields(),
            operand: Some(Box::new(operand)),
            precision: Some(precision_str.to_string()),
        })
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
        // Call the typed version with empty type map
        self.translate_query_with_types(
            query,
            translate_expr,
            result_type,
            &std::collections::HashMap::new(),
        )
    }

    /// Translate a CQL AST query to an ELM Query expression with alias type info.
    ///
    /// The `alias_types` parameter maps alias names to their FHIR type names
    /// (e.g., "ER" -> "Observation"). This allows property access on aliases
    /// to have type information for implicit conversions.
    pub fn translate_query_with_types(
        &mut self,
        query: &ast::Query,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
        alias_types: &std::collections::HashMap<String, String>,
    ) -> elm::Expression {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Collect all aliases from sources and let clauses for scope tracking
        // Include type info if available from alias_types
        let mut typed_aliases: Vec<(String, Option<String>)> = query
            .sources
            .iter()
            .map(|s| {
                let type_name = alias_types.get(&s.alias).cloned();
                (s.alias.clone(), type_name)
            })
            .collect();

        for lc in &query.let_clauses {
            typed_aliases.push((lc.identifier.clone(), None));
        }

        // Push query scope with typed aliases before translating query body
        self.push_query_scope_typed(typed_aliases);

        // Translate sources (note: source expressions are evaluated BEFORE entering scope)
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

        // Pop query scope after translating
        self.pop_query_scope();

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
    ///
    /// The relationship alias is added to the query scope for the `such_that` condition.
    pub fn translate_relationship_clause(
        &mut self,
        rel: &ast::RelationshipClause,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::RelationshipClause {
        let relationship_type = match rel.kind {
            ast::RelationshipKind::With => Some("With".to_string()),
            ast::RelationshipKind::Without => Some("Without".to_string()),
        };

        // Translate the source expression (before alias is in scope)
        let expression = translate_expr(self, &rel.source.expression);

        // Push the relationship alias for the such_that condition
        self.push_query_scope(std::iter::once(rel.source.alias.clone()));

        let such_that = rel
            .such_that
            .as_ref()
            .map(|st| Box::new(translate_expr(self, st)));

        // Pop the relationship alias
        self.pop_query_scope();

        elm::RelationshipClause {
            relationship_type,
            alias: Some(rel.source.alias.clone()),
            expression: Some(Box::new(expression)),
            such_that,
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

        // Determine the type based on the expression
        // ByColumn is used for simple path references
        // ByDirection would be used for sort by direction only (no path)
        // ByExpression would be used for complex expressions
        let sort_by_type = if path.is_some() {
            Some("ByColumn".to_string())
        } else {
            // For complex expressions without a simple path, we'd use ByExpression
            // For now, default to ByColumn to match reference implementation
            Some("ByColumn".to_string())
        };

        elm::SortByItem {
            sort_by_type,
            direction,
            path,
        }
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
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
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
        })
    }
}

/// Convert an AST TypeSpecifier to an ELM QName.
pub fn type_specifier_to_qname(ts: &ast::TypeSpecifier) -> elm::QName {
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

    // ========================================================================
    // Type Operator Translation (Phase 4.5g)
    // ========================================================================

    /// Translate a CQL type expression (is, as, convert) to ELM.
    ///
    /// CQL supports three type operators:
    /// - `is`: Check if expression is of a type (returns Boolean)
    /// - `as`: Cast expression to a type (returns null if not compatible)
    /// - `convert`: Convert expression to a type (throws error if not convertible)
    ///
    /// # Example
    /// ```cql
    /// value is Integer
    /// value as Integer
    /// convert value to String
    /// ```
    pub fn translate_type_expression(
        &mut self,
        type_expr: &ast::TypeExpression,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::Expression {
        let operand = translate_expr(self, &type_expr.operand);
        let type_spec = self.type_specifier_to_elm(&type_expr.type_specifier);

        match type_expr.operator {
            ast::TypeOperator::Is => self.translate_is(operand, type_spec),
            ast::TypeOperator::As => self.translate_as(operand, type_spec, false),
            ast::TypeOperator::Convert => self.translate_convert(operand, type_spec),
        }
    }

    /// Create an ELM Is expression (type test).
    ///
    /// The `is` operator returns true if the operand is of the specified type.
    ///
    /// # Example
    /// ```cql
    /// value is Integer  // returns true if value is an Integer
    /// ```
    pub fn translate_is(
        &mut self,
        operand: elm::Expression,
        type_specifier: elm::TypeSpecifier,
    ) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Boolean));

        elm::Expression::Is(elm::IsExpr {
            element,
            operand: Some(Box::new(operand)),
            is_type_specifier: Some(type_specifier),
            is_type: None,
        })
    }

    /// Create an ELM Is expression using a QName.
    pub fn translate_is_qname(
        &mut self,
        operand: elm::Expression,
        is_type: elm::QName,
    ) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Boolean));

        elm::Expression::Is(elm::IsExpr {
            element,
            operand: Some(Box::new(operand)),
            is_type_specifier: None,
            is_type: Some(is_type),
        })
    }

    /// Create an ELM As expression (type cast).
    ///
    /// The `as` operator casts the operand to the specified type.
    /// If strict is false (default), returns null if the cast fails.
    /// If strict is true, the cast must succeed.
    ///
    /// # Example
    /// ```cql
    /// value as Integer  // returns Integer or null if not compatible
    /// value as strict Integer  // strict cast
    /// ```
    pub fn translate_as(
        &mut self,
        operand: elm::Expression,
        type_specifier: elm::TypeSpecifier,
        strict: bool,
    ) -> elm::Expression {
        // Result type is the target type
        let result_type = self.type_specifier_to_datatype(&type_specifier);
        let element = match &result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        elm::Expression::As(elm::AsExpr {
            element,
            operand: Some(Box::new(operand)),
            as_type_specifier: Some(type_specifier),
            as_type: None,
            strict: if strict { Some(true) } else { None },
        })
    }

    /// Create an ELM As expression using a QName.
    pub fn translate_as_qname(
        &mut self,
        operand: elm::Expression,
        as_type: elm::QName,
        strict: bool,
    ) -> elm::Expression {
        elm::Expression::As(elm::AsExpr {
            element: self.element_fields(),
            operand: Some(Box::new(operand)),
            as_type_specifier: None,
            as_type: Some(as_type),
            strict: if strict { Some(true) } else { None },
        })
    }

    /// Create an ELM Convert expression (type conversion).
    ///
    /// The `convert` operator converts the operand to the specified type.
    /// Unlike `as`, convert can invoke conversion functions.
    ///
    /// # Example
    /// ```cql
    /// convert value to String
    /// ```
    pub fn translate_convert(
        &mut self,
        operand: elm::Expression,
        type_specifier: elm::TypeSpecifier,
    ) -> elm::Expression {
        // Result type is the target type
        let result_type = self.type_specifier_to_datatype(&type_specifier);
        let element = match &result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        elm::Expression::Convert(elm::ConvertExpr {
            element,
            operand: Some(Box::new(operand)),
            to_type_specifier: Some(type_specifier),
            to_type: None,
        })
    }

    /// Create an ELM Convert expression using a QName.
    pub fn translate_convert_qname(
        &mut self,
        operand: elm::Expression,
        to_type: elm::QName,
    ) -> elm::Expression {
        elm::Expression::Convert(elm::ConvertExpr {
            element: self.element_fields(),
            operand: Some(Box::new(operand)),
            to_type_specifier: None,
            to_type: Some(to_type),
        })
    }

    /// Create an ELM CanConvert expression.
    ///
    /// The `CanConvert` expression tests if a conversion is possible.
    pub fn translate_can_convert(
        &mut self,
        operand: elm::Expression,
        type_specifier: elm::TypeSpecifier,
    ) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Boolean));

        elm::Expression::CanConvert(elm::CanConvert {
            element,
            operand: Some(Box::new(operand)),
            to_type_specifier: Some(type_specifier),
            to_type: None,
        })
    }

    /// Create an ELM ToInteger conversion.
    pub fn translate_to_integer(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Integer));
        elm::Expression::ToInteger(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToLong conversion.
    pub fn translate_to_long(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Long));
        elm::Expression::ToLong(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToDecimal conversion.
    pub fn translate_to_decimal(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Decimal));
        elm::Expression::ToDecimal(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToBoolean conversion.
    pub fn translate_to_boolean(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Boolean));
        elm::Expression::ToBoolean(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToString conversion.
    pub fn translate_to_string(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::String));
        elm::Expression::ToStringExpr(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToDate conversion.
    pub fn translate_to_date(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Date));
        elm::Expression::ToDate(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToDateTime conversion.
    pub fn translate_to_datetime(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::DateTime));
        elm::Expression::ToDateTime(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToTime conversion.
    pub fn translate_to_time(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Time));
        elm::Expression::ToTime(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToQuantity conversion.
    pub fn translate_to_quantity(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Quantity));
        elm::Expression::ToQuantity(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToRatio conversion.
    pub fn translate_to_ratio(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Ratio));
        elm::Expression::ToRatio(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToConcept conversion.
    pub fn translate_to_concept(&mut self, operand: elm::Expression) -> elm::Expression {
        let element = self.element_fields_typed(&DataType::System(SystemType::Concept));
        elm::Expression::ToConcept(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToList conversion.
    pub fn translate_to_list(
        &mut self,
        operand: elm::Expression,
        element_type: Option<&DataType>,
    ) -> elm::Expression {
        let result_type = element_type.map(|et| DataType::List(Box::new(et.clone())));
        let element = match &result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };
        elm::Expression::ToList(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Create an ELM ToChars conversion (String to List<String>).
    pub fn translate_to_chars(&mut self, operand: elm::Expression) -> elm::Expression {
        let result_type = DataType::List(Box::new(DataType::System(SystemType::String)));
        let element = self.element_fields_typed(&result_type);
        elm::Expression::ToChars(elm::UnaryExpression {
            element,
            operand: Some(Box::new(operand)),
            signature: Vec::new(),
        })
    }

    /// Convert an AST TypeSpecifier to an ELM TypeSpecifier.
    #[allow(clippy::only_used_in_recursion)]
    pub fn type_specifier_to_elm(&self, ts: &ast::TypeSpecifier) -> elm::TypeSpecifier {
        match ts {
            ast::TypeSpecifier::Named(named) => {
                // Convert AST NamedTypeSpecifier to ELM QName format
                let qname = match &named.namespace {
                    Some(ns) => format!("{{{ns}}}{}", named.name),
                    None => named.name.clone(),
                };
                elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
                    name: qname,
                    ..Default::default()
                })
            }
            ast::TypeSpecifier::List(list_spec) => {
                elm::TypeSpecifier::List(elm::ListTypeSpecifier {
                    element_type: Some(Box::new(
                        self.type_specifier_to_elm(&list_spec.element_type),
                    )),
                    ..Default::default()
                })
            }
            ast::TypeSpecifier::Interval(interval_spec) => {
                elm::TypeSpecifier::Interval(elm::IntervalTypeSpecifier {
                    point_type: Some(Box::new(
                        self.type_specifier_to_elm(&interval_spec.point_type),
                    )),
                    ..Default::default()
                })
            }
            ast::TypeSpecifier::Tuple(tuple_spec) => {
                let elm_elements: Vec<elm::TupleElementDefinition> = tuple_spec
                    .elements
                    .iter()
                    .map(|elem| elm::TupleElementDefinition {
                        name: elem.name.clone(),
                        element_type: Some(Box::new(
                            self.type_specifier_to_elm(&elem.element_type),
                        )),
                        ..Default::default()
                    })
                    .collect();
                elm::TypeSpecifier::Tuple(elm::TupleTypeSpecifier {
                    element: elm_elements,
                    ..Default::default()
                })
            }
            ast::TypeSpecifier::Choice(choice_spec) => {
                let elm_choices: Vec<elm::TypeSpecifier> = choice_spec
                    .types
                    .iter()
                    .map(|c| self.type_specifier_to_elm(c))
                    .collect();
                elm::TypeSpecifier::Choice(elm::ChoiceTypeSpecifier {
                    choice: elm_choices,
                    ..Default::default()
                })
            }
        }
    }

    /// Convert an AST TypeSpecifier to an ELM QName string.
    pub fn type_specifier_to_qname(&self, ts: &ast::TypeSpecifier) -> elm::QName {
        type_specifier_to_qname(ts)
    }

    /// Convert an ELM TypeSpecifier back to a DataType (for result typing).
    #[allow(clippy::only_used_in_recursion)]
    fn type_specifier_to_datatype(&self, ts: &elm::TypeSpecifier) -> Option<DataType> {
        match ts {
            elm::TypeSpecifier::Named(named) => {
                // Parse the QName to extract namespace and name
                let name = &named.name;
                if name.starts_with("{urn:hl7-org:elm-types:r1}") {
                    let type_name = name
                        .strip_prefix("{urn:hl7-org:elm-types:r1}")
                        .unwrap_or(name);
                    Some(DataType::System(string_to_system_type(type_name)))
                } else if let Some(pos) = name.rfind('}') {
                    let namespace = &name[1..pos];
                    let type_name = &name[pos + 1..];
                    Some(DataType::Model {
                        namespace: namespace.to_string(),
                        name: type_name.to_string(),
                    })
                } else {
                    // Simple name, assume System type
                    Some(DataType::System(string_to_system_type(name)))
                }
            }
            elm::TypeSpecifier::List(list) => {
                let elem_type = list
                    .element_type
                    .as_ref()
                    .and_then(|et| self.type_specifier_to_datatype(et))?;
                Some(DataType::List(Box::new(elem_type)))
            }
            elm::TypeSpecifier::Interval(interval) => {
                let point_type = interval
                    .point_type
                    .as_ref()
                    .and_then(|pt| self.type_specifier_to_datatype(pt))?;
                Some(DataType::Interval(Box::new(point_type)))
            }
            elm::TypeSpecifier::Tuple(tuple) => {
                let elements: Vec<TupleElement> = tuple
                    .element
                    .iter()
                    .filter_map(|e| {
                        let name = e.name.clone();
                        let element_type = e
                            .element_type
                            .as_ref()
                            .and_then(|et| self.type_specifier_to_datatype(et))?;
                        Some(TupleElement {
                            name,
                            element_type: Box::new(element_type),
                        })
                    })
                    .collect();
                Some(DataType::Tuple(elements))
            }
            elm::TypeSpecifier::Choice(choice) => {
                let types: Vec<DataType> = choice
                    .choice
                    .iter()
                    .filter_map(|c| self.type_specifier_to_datatype(c))
                    .collect();
                Some(DataType::Choice(types))
            }
            elm::TypeSpecifier::Parameter(param) => {
                // Type parameters become DataType::TypeParameter
                Some(DataType::TypeParameter(
                    param.parameter_name.clone().unwrap_or_default(),
                ))
            }
        }
    }

    // ========================================================================
    // Function Resolution (Phase 4.6)
    // ========================================================================

    /// Resolve a function invocation to the appropriate ELM expression.
    ///
    /// This method determines whether a function call should be translated to:
    /// - A system operator (e.g., `Sum`, `Count`, `First`, `Now`)
    /// - A user-defined function reference
    ///
    /// # System Functions
    ///
    /// CQL provides many built-in functions that translate directly to ELM operators:
    ///
    /// - **Aggregate**: `Sum`, `Count`, `Min`, `Max`, `Avg`, `Median`, `Mode`, etc.
    /// - **List**: `First`, `Last`, `Flatten`, `Distinct`, `Exists`, `SingletonFrom`
    /// - **Nullological**: `Coalesce`, `IsNull`, `IsTrue`, `IsFalse`
    /// - **Date/Time**: `Now`, `Today`, `TimeOfDay`
    /// - **String**: `Length`, `Upper`, `Lower`, `Substring`, `StartsWith`, etc.
    /// - **Arithmetic**: `Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`
    ///
    /// # Example
    /// ```cql
    /// Sum({1, 2, 3})           -- Translates to Sum aggregate expression
    /// Count(Encounters)         -- Translates to Count aggregate expression
    /// First(SortedList)         -- Translates to First unary expression
    /// Now()                     -- Translates to Now nullary expression
    /// MyLib.CustomFunc(x)       -- Translates to FunctionRef
    /// ```
    pub fn resolve_function_invocation(
        &mut self,
        func: &ast::FunctionInvocation,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        // If there's a library qualifier, it's a user-defined or library function
        if func.library.is_some() {
            return self.translate_ast_function_invocation(func, translate_expr);
        }

        // Try to resolve as a system function
        if let Some(expr) = self.try_resolve_system_function(
            &func.name,
            &func.arguments,
            &translate_expr,
            result_type,
        ) {
            return expr;
        }

        // Fall back to FunctionRef for user-defined functions
        self.translate_ast_function_invocation(func, translate_expr)
    }

    /// Try to resolve a function name as a system function.
    ///
    /// Returns `None` if the function is not a system function.
    #[allow(clippy::only_used_in_recursion)]
    fn try_resolve_system_function(
        &mut self,
        name: &str,
        args: &[ast::Expression],
        translate_expr: &impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> Option<elm::Expression> {
        let element = match result_type {
            Some(dt) => self.element_fields_typed(dt),
            None => self.element_fields(),
        };

        // Nullary functions (0 arguments)
        if args.is_empty() {
            return match name {
                "Now" => Some(elm::Expression::Now(elm::NullaryExpression { element })),
                "Today" => Some(elm::Expression::Today(elm::NullaryExpression { element })),
                "TimeOfDay" => Some(elm::Expression::TimeOfDay(elm::NullaryExpression {
                    element,
                })),
                _ => None,
            };
        }

        // Single argument functions
        if args.len() == 1 {
            let operand = translate_expr(self, &args[0]);

            return match name {
                // Aggregate functions
                "Sum" => Some(elm::Expression::Sum(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Count" => Some(elm::Expression::Count(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Min" => Some(elm::Expression::Min(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Max" => Some(elm::Expression::Max(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Avg" => Some(elm::Expression::Avg(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Median" => Some(elm::Expression::Median(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Mode" => Some(elm::Expression::Mode(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "Variance" => Some(elm::Expression::Variance(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "StdDev" => Some(elm::Expression::StdDev(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "PopulationVariance" => Some(elm::Expression::PopulationVariance(
                    elm::AggregateExpression {
                        element,
                        source: Some(Box::new(operand)),
                        path: None,
                        signature: Vec::new(),
                    },
                )),
                "PopulationStdDev" => Some(elm::Expression::PopulationStdDev(
                    elm::AggregateExpression {
                        element,
                        source: Some(Box::new(operand)),
                        path: None,
                        signature: Vec::new(),
                    },
                )),
                "AllTrue" => Some(elm::Expression::AllTrue(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),
                "AnyTrue" => Some(elm::Expression::AnyTrue(elm::AggregateExpression {
                    element,
                    source: Some(Box::new(operand)),
                    path: None,
                    signature: Vec::new(),
                })),

                // List functions
                "First" => Some(elm::Expression::First(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Last" => Some(elm::Expression::Last(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Flatten" => Some(elm::Expression::Flatten(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Distinct" => Some(elm::Expression::Distinct(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Exists" => Some(elm::Expression::Exists(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "SingletonFrom" => Some(elm::Expression::SingletonFrom(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToList" => Some(elm::Expression::ToList(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Collapse" => Some(elm::Expression::Collapse(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Nullological functions
                "IsNull" => Some(elm::Expression::IsNull(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "IsTrue" => Some(elm::Expression::IsTrue(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "IsFalse" => Some(elm::Expression::IsFalse(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Arithmetic functions
                "Abs" => Some(elm::Expression::Abs(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Ceiling" => Some(elm::Expression::Ceiling(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Floor" => Some(elm::Expression::Floor(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Truncate" => Some(elm::Expression::Truncate(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Round" => Some(elm::Expression::Round(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Ln" => Some(elm::Expression::Ln(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Exp" => Some(elm::Expression::Exp(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Negate" => Some(elm::Expression::Negate(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Successor" => Some(elm::Expression::Successor(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Predecessor" => Some(elm::Expression::Predecessor(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // String functions
                "Length" => Some(elm::Expression::Length(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Upper" => Some(elm::Expression::Upper(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Lower" => Some(elm::Expression::Lower(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToChars" => Some(elm::Expression::ToChars(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Interval functions
                "Start" => Some(elm::Expression::Start(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "End" => Some(elm::Expression::End(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Width" => Some(elm::Expression::Width(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "Size" => Some(elm::Expression::Size(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "PointFrom" => Some(elm::Expression::PointFrom(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ExpandValueSet" => Some(elm::Expression::ExpandValueSet(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Clinical functions
                "CalculateAge" => Some(elm::Expression::CalculateAge(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Type conversion functions (handled in Phase 4.5g but included for completeness)
                "ToBoolean" => Some(elm::Expression::ToBoolean(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToInteger" => Some(elm::Expression::ToInteger(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToLong" => Some(elm::Expression::ToLong(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToDecimal" => Some(elm::Expression::ToDecimal(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToString" => Some(elm::Expression::ToStringExpr(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToDate" => Some(elm::Expression::ToDate(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToDateTime" => Some(elm::Expression::ToDateTime(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToTime" => Some(elm::Expression::ToTime(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToQuantity" => Some(elm::Expression::ToQuantity(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToRatio" => Some(elm::Expression::ToRatio(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),
                "ToConcept" => Some(elm::Expression::ToConcept(elm::UnaryExpression {
                    element,
                    operand: Some(Box::new(operand)),
                    signature: Vec::new(),
                })),

                // Not a system unary function
                _ => None,
            };
        }

        // Two argument functions
        if args.len() == 2 {
            let left = translate_expr(self, &args[0]);
            let right = translate_expr(self, &args[1]);

            return match name {
                // Arithmetic
                "Log" => Some(elm::Expression::Log(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Power" => Some(elm::Expression::Power(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),

                // String
                "StartsWith" => Some(elm::Expression::StartsWith(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "EndsWith" => Some(elm::Expression::EndsWith(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Matches" => Some(elm::Expression::Matches(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),

                // List/Interval
                "Contains" => Some(elm::Expression::Contains(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "In" => Some(elm::Expression::In(elm::TimeBinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                    precision: None,
                })),
                "Includes" => Some(elm::Expression::Includes(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "IncludedIn" => Some(elm::Expression::IncludedIn(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "ProperContains" => Some(elm::Expression::ProperContains(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "ProperIn" => Some(elm::Expression::ProperIn(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "ProperIncludes" => Some(elm::Expression::ProperIncludes(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "ProperIncludedIn" => {
                    Some(elm::Expression::ProperIncludedIn(elm::BinaryExpression {
                        element,
                        operand: vec![left, right],
                        signature: Vec::new(),
                    }))
                }
                "Meets" => Some(elm::Expression::Meets(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "MeetsBefore" => Some(elm::Expression::MeetsBefore(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "MeetsAfter" => Some(elm::Expression::MeetsAfter(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Union" => Some(elm::Expression::Union(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Intersect" => Some(elm::Expression::Intersect(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Except" => Some(elm::Expression::Except(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Indexer" => Some(elm::Expression::Indexer(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Times" => Some(elm::Expression::Times(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "Expand" => Some(elm::Expression::Expand(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),

                // Clinical
                "CalculateAgeAt" => Some(elm::Expression::CalculateAgeAt(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "ConvertQuantity" => {
                    Some(elm::Expression::ConvertQuantity(elm::BinaryExpression {
                        element,
                        operand: vec![left, right],
                        signature: Vec::new(),
                    }))
                }
                "CanConvertQuantity" => {
                    Some(elm::Expression::CanConvertQuantity(elm::BinaryExpression {
                        element,
                        operand: vec![left, right],
                        signature: Vec::new(),
                    }))
                }

                // Terminology
                "Subsumes" => Some(elm::Expression::Subsumes(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),
                "SubsumedBy" => Some(elm::Expression::SubsumedBy(elm::BinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                })),

                // Not a system binary function
                _ => None,
            };
        }

        // Variable argument functions (Coalesce, Concatenate)
        if !args.is_empty() {
            let operands: Vec<elm::Expression> =
                args.iter().map(|a| translate_expr(self, a)).collect();

            return match name {
                "Coalesce" => Some(elm::Expression::Coalesce(elm::NaryExpression {
                    element,
                    operand: operands,
                    signature: Vec::new(),
                })),
                "Concatenate" => Some(elm::Expression::Concatenate(elm::NaryExpression {
                    element,
                    operand: operands,
                    signature: Vec::new(),
                })),
                _ => None,
            };
        }

        None
    }

    /// Resolve a fluent function invocation.
    ///
    /// Fluent functions are called using the syntax `value.function(args)`, where
    /// the value becomes the first argument to the function.
    ///
    /// # Example
    /// ```cql
    /// list.First()           -- Translates to First(list)
    /// encounters.Count()     -- Translates to Count(encounters)
    /// value.MyCustomFunc(x)  -- Translates to FunctionRef with value as first arg
    /// ```
    pub fn resolve_fluent_function(
        &mut self,
        source: &ast::Expression,
        name: &str,
        args: &[ast::Expression],
        library: Option<&str>,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
        result_type: Option<&DataType>,
    ) -> elm::Expression {
        // Build combined arguments with source as first argument
        let mut all_args = vec![source.clone()];
        all_args.extend(args.iter().cloned());

        // Create a synthetic function invocation
        let func = ast::FunctionInvocation {
            library: library.map(String::from),
            name: name.to_string(),
            arguments: all_args,
            location: None,
        };

        // Resolve as regular function
        self.resolve_function_invocation(&func, translate_expr, result_type)
    }

    /// Translate a user-defined function definition to ELM.
    ///
    /// This translates function definitions, including:
    /// - Regular functions with body
    /// - External functions (no body)
    /// - Fluent functions
    ///
    /// # Example
    /// ```cql
    /// define function Double(x Integer) returns Integer: x * 2
    /// define fluent function IsAdult(p Patient) returns Boolean: AgeInYears(p) >= 18
    /// ```
    pub fn translate_function_def(
        &mut self,
        func: &ast::FunctionDef,
        translate_expr: impl Fn(&mut Self, &ast::Expression) -> elm::Expression,
    ) -> elm::FunctionDef {
        // Translate operands (parameters)
        let operand: Vec<elm::OperandDef> = func
            .parameters
            .iter()
            .map(|p| elm::OperandDef {
                name: Some(p.name.clone()),
                operand_type_name: p.type_specifier.as_ref().map(|ts| {
                    let elm_ts = self.type_specifier_to_elm(ts);
                    type_specifier_to_qname_string(&elm_ts)
                }),
                operand_type_specifier: p
                    .type_specifier
                    .as_ref()
                    .map(|ts| self.type_specifier_to_elm(ts)),
            })
            .collect();

        // Translate return type
        let result_type_specifier = func
            .return_type
            .as_ref()
            .map(|ts| self.type_specifier_to_elm(ts));
        let result_type_name = result_type_specifier
            .as_ref()
            .map(type_specifier_to_qname_string);

        // Translate body expression
        let expression = func
            .body
            .as_ref()
            .map(|b| Box::new(translate_expr(self, b)));

        // Map access modifier
        let access_level = match func.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        elm::FunctionDef {
            local_id: self.next_local_id(),
            locator: None,
            name: Some(func.name.clone()),
            context: None,
            access_level,
            result_type_name,
            result_type_specifier,
            operand,
            expression,
            external: if func.external { Some(true) } else { None },
            fluent: if func.fluent { Some(true) } else { None },
            annotation: Vec::new(),
        }
    }

    // ========================================================================
    // Statement Translation (Phase 6.4b)
    // ========================================================================

    /// Translate an AST expression definition to ELM.
    ///
    /// This translates CQL `define` statements to ELM ExpressionDef.
    ///
    /// # Example
    /// ```cql
    /// define "Active Encounters":
    ///   [Encounter] E where E.status = 'active'
    /// ```
    pub fn translate_expression_def(
        &mut self,
        expr_def: &ast::ExpressionDef,
        context: Option<&str>,
    ) -> elm::ExpressionDef {
        // Translate the expression body
        let expression = Some(Box::new(self.translate_expression(&expr_def.expression)));

        // Map access modifier
        let access_level = match expr_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        elm::ExpressionDef {
            local_id: self.next_local_id(),
            locator: None,
            name: Some(expr_def.name.clone()),
            context: context.map(String::from),
            access_level,
            result_type_name: None,
            result_type_specifier: None,
            expression,
            annotation: Vec::new(),
        }
    }

    /// Translate an AST parameter definition to ELM.
    ///
    /// This translates CQL `parameter` declarations to ELM ParameterDef.
    ///
    /// # Example
    /// ```cql
    /// parameter "Measurement Period" Interval<DateTime>
    /// parameter "Max Count" Integer default 10
    /// ```
    pub fn translate_parameter_def(&mut self, param_def: &ast::ParameterDef) -> elm::ParameterDef {
        // Translate the type specifier
        let parameter_type_specifier = param_def
            .type_specifier
            .as_ref()
            .map(|ts| self.type_specifier_to_elm(ts));

        let parameter_type_name = parameter_type_specifier
            .as_ref()
            .map(type_specifier_to_qname_string);

        // Translate the default value
        let default_value = param_def
            .default
            .as_ref()
            .map(|d| Box::new(self.translate_expression(d)));

        // Map access modifier
        let access_level = match param_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        elm::ParameterDef {
            name: Some(param_def.name.clone()),
            access_level,
            parameter_type_name,
            parameter_type_specifier,
            default_value,
        }
    }

    /// Translate a statement (expression def or function def) to ELM.
    ///
    /// This is a convenience method that dispatches to the appropriate
    /// translation method based on the statement type.
    pub fn translate_statement(
        &mut self,
        stmt: &ast::Statement,
        context: Option<&str>,
    ) -> StatementTranslation {
        match stmt {
            ast::Statement::ExpressionDef(expr_def) => {
                StatementTranslation::Expression(self.translate_expression_def(expr_def, context))
            }
            ast::Statement::FunctionDef(func_def) => StatementTranslation::Function(
                self.translate_function_def(func_def, |t, e| t.translate_expression(e)),
            ),
        }
    }

    // ========================================================================
    // Terminology Translation (Phase 6.4c)
    // ========================================================================

    /// Translate a using definition (data model reference) to ELM.
    ///
    /// # Example
    /// ```cql
    /// using FHIR version '4.0.1'
    /// ```
    pub fn translate_using_def(&self, using_def: &ast::UsingDef) -> elm::UsingDef {
        // Map model name to URI
        let uri = model_name_to_uri(&using_def.model_name);

        elm::UsingDef {
            local_identifier: Some(using_def.model_name.clone()),
            uri: Some(uri),
            version: using_def.version.clone(),
        }
    }

    /// Translate an include definition (library reference) to ELM.
    ///
    /// # Example
    /// ```cql
    /// include FHIRHelpers version '4.0.1' called Helpers
    /// ```
    pub fn translate_include_def(&self, include_def: &ast::IncludeDef) -> elm::IncludeDef {
        elm::IncludeDef {
            local_identifier: include_def
                .alias
                .clone()
                .or_else(|| Some(include_def.path.clone())),
            path: Some(include_def.path.clone()),
            version: include_def.version.clone(),
        }
    }

    /// Translate a code system definition to ELM.
    ///
    /// # Example
    /// ```cql
    /// codesystem "LOINC": 'http://loinc.org'
    /// ```
    pub fn translate_codesystem_def(&self, cs_def: &ast::CodeSystemDef) -> elm::CodeSystemDef {
        let access_level = match cs_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        elm::CodeSystemDef {
            name: Some(cs_def.name.clone()),
            id: Some(cs_def.id.clone()),
            version: cs_def.version.clone(),
            access_level,
        }
    }

    /// Translate a value set definition to ELM.
    ///
    /// # Example
    /// ```cql
    /// valueset "Active Conditions": 'http://example.org/fhir/ValueSet/active-conditions'
    /// ```
    pub fn translate_valueset_def(&self, vs_def: &ast::ValueSetDef) -> elm::ValueSetDef {
        let access_level = match vs_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        // Convert code system references
        let code_system: Vec<elm::CodeSystemDefRef> = vs_def
            .codesystems
            .iter()
            .map(|cs_name| elm::CodeSystemDefRef {
                name: Some(cs_name.clone()),
                library_name: None,
            })
            .collect();

        elm::ValueSetDef {
            name: Some(vs_def.name.clone()),
            id: Some(vs_def.id.clone()),
            version: vs_def.version.clone(),
            access_level,
            code_system,
        }
    }

    /// Translate a code definition to ELM.
    ///
    /// # Example
    /// ```cql
    /// code "Blood Pressure": '85354-9' from "LOINC" display 'Blood pressure panel'
    /// ```
    pub fn translate_code_def(&self, code_def: &ast::CodeDef) -> elm::CodeDef {
        let access_level = match code_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        elm::CodeDef {
            name: Some(code_def.name.clone()),
            id: Some(code_def.code.clone()),
            display: code_def.display.clone(),
            access_level,
            code_system: Some(elm::CodeSystemDefRef {
                name: Some(code_def.codesystem.clone()),
                library_name: None,
            }),
        }
    }

    /// Translate a concept definition to ELM.
    ///
    /// # Example
    /// ```cql
    /// concept "Blood Pressure Codes": { "Systolic BP", "Diastolic BP" } display 'Blood Pressure'
    /// ```
    pub fn translate_concept_def(&self, concept_def: &ast::ConceptDef) -> elm::ConceptDef {
        let access_level = match concept_def.access {
            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
        };

        // Convert code references
        let code: Vec<elm::CodeDefRef> = concept_def
            .codes
            .iter()
            .map(|code_name| elm::CodeDefRef {
                name: Some(code_name.clone()),
                library_name: None,
            })
            .collect();

        elm::ConceptDef {
            name: Some(concept_def.name.clone()),
            display: concept_def.display.clone(),
            access_level,
            code,
        }
    }

    /// Translate a context definition to ELM.
    ///
    /// # Example
    /// ```cql
    /// context Patient
    /// ```
    pub fn translate_context_def(&self, ctx_def: &ast::ContextDef) -> elm::ContextDef {
        elm::ContextDef {
            name: Some(ctx_def.name.clone()),
        }
    }
}

/// Map a model name to its URI.
fn model_name_to_uri(model_name: &str) -> String {
    match model_name {
        "FHIR" => "http://hl7.org/fhir".to_string(),
        "QDM" => "urn:healthit-gov:qdm:v5_6".to_string(),
        "QICore" => "http://hl7.org/fhir/us/qicore".to_string(),
        "USCore" => "http://hl7.org/fhir/us/core".to_string(),
        "System" => "urn:hl7-org:elm-types:r1".to_string(),
        _ => format!("http://unknown/{model_name}"),
    }
}

/// Result of translating a statement.
#[derive(Debug, Clone)]
pub enum StatementTranslation {
    /// An expression definition.
    Expression(elm::ExpressionDef),
    /// A function definition.
    Function(elm::FunctionDef),
}

/// Convert an ELM TypeSpecifier to a QName string.
fn type_specifier_to_qname_string(ts: &elm::TypeSpecifier) -> String {
    match ts {
        elm::TypeSpecifier::Named(named) => named.name.clone(),
        elm::TypeSpecifier::List(list) => {
            if let Some(elem) = &list.element_type {
                format!("List<{}>", type_specifier_to_qname_string(elem))
            } else {
                "List".to_string()
            }
        }
        elm::TypeSpecifier::Interval(interval) => {
            if let Some(point) = &interval.point_type {
                format!("Interval<{}>", type_specifier_to_qname_string(point))
            } else {
                "Interval".to_string()
            }
        }
        elm::TypeSpecifier::Tuple(_) => "Tuple".to_string(),
        elm::TypeSpecifier::Choice(_) => "Choice".to_string(),
        elm::TypeSpecifier::Parameter(param) => param.parameter_name.clone().unwrap_or_default(),
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

/// Convert a string name to a SystemType.
fn string_to_system_type(name: &str) -> SystemType {
    match name {
        "Any" => SystemType::Any,
        "Boolean" => SystemType::Boolean,
        "Integer" => SystemType::Integer,
        "Long" => SystemType::Long,
        "Decimal" => SystemType::Decimal,
        "String" => SystemType::String,
        "Date" => SystemType::Date,
        "DateTime" => SystemType::DateTime,
        "Time" => SystemType::Time,
        "Quantity" => SystemType::Quantity,
        "Ratio" => SystemType::Ratio,
        "Code" => SystemType::Code,
        "Concept" => SystemType::Concept,
        "Vocabulary" => SystemType::Vocabulary,
        _ => SystemType::Any, // Default to Any for unknown types
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
        // Use CompilerOptions::new() which has no options enabled
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::new());
        let result = translator.translate_literal(&ast::Literal::Integer(42));
        if let elm::Expression::Literal(lit) = result {
            assert!(lit.element.local_id.is_none());
        }
    }

    #[test]
    fn test_local_id_with_annotations_enabled() {
        // Default options have annotations enabled
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_literal(&ast::Literal::Integer(42));
        if let elm::Expression::Literal(lit) = result {
            assert!(lit.element.local_id.is_some());
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
        // Use debug options which enable result types
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
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
    fn test_translate_expression_ref_no_result_type() {
        // Default options do not include EnableResultTypes
        let mut translator = ExpressionTranslator::new();
        let result = translator.translate_identifier_ref(
            "MyExpression",
            ResolvedRefKind::Expression,
            Some(&DataType::integer()),
        );
        if let elm::Expression::ExpressionRef(expr_ref) = result {
            assert_eq!(expr_ref.name, Some("MyExpression".to_string()));
            // Result type not included without EnableResultTypes
            assert!(expr_ref.element.result_type_name.is_none());
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
        );
        assert!(matches!(result, elm::Expression::Concatenate(_)));
    }

    #[test]
    fn test_translate_binary_union() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Union,
            left,
            right,
            None,
            None,
        );
        assert!(matches!(result, elm::Expression::Union(_)));
    }

    #[test]
    fn test_translate_binary_intersect() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Intersect,
            left,
            right,
            None,
            None,
        );
        assert!(matches!(result, elm::Expression::Intersect(_)));
    }

    #[test]
    fn test_translate_binary_except() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Null);
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::Except,
            left,
            right,
            None,
            None,
        );
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            precision: None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            translator.translate_binary_operator(ast::BinaryOperator::Log, left, right, None, None);
        assert!(matches!(result, elm::Expression::Log(_)));
    }

    #[test]
    fn test_translate_binary_index_of() {
        let mut translator = ExpressionTranslator::new();
        let left = translator.translate_literal(&ast::Literal::Null);
        let right = translator.translate_literal(&ast::Literal::Integer(1));
        let result = translator.translate_binary_operator(
            ast::BinaryOperator::IndexOf,
            left,
            right,
            None,
            None,
        );
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
        assert_eq!(
            result.by[0].sort_by_type,
            Some("ByColumn".to_string()),
            "sort by type should be ByColumn for path-based sort"
        );
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
        assert_eq!(
            result.by[0].sort_by_type,
            Some("ByColumn".to_string()),
            "sort by type should be ByColumn"
        );
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
        // Use debug options which enable result types
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
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

    // ========================================================================
    // Type Operator Translation Tests (Phase 4.5g)
    // ========================================================================

    #[test]
    fn test_translate_is_expression() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("Integer")),
            ..Default::default()
        });
        let type_spec = elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}Integer".to_string(),
            ..Default::default()
        });

        let result = translator.translate_is(operand, type_spec);

        if let elm::Expression::Is(is_expr) = result {
            assert!(is_expr.operand.is_some());
            assert!(is_expr.is_type_specifier.is_some());
            assert!(is_expr.is_type.is_none());
        } else {
            panic!("Expected Is expression");
        }
    }

    #[test]
    fn test_translate_is_qname() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Null(elm::Null::default());

        let result =
            translator.translate_is_qname(operand, "{urn:hl7-org:elm-types:r1}String".to_string());

        if let elm::Expression::Is(is_expr) = result {
            assert!(is_expr.operand.is_some());
            assert!(is_expr.is_type_specifier.is_none());
            assert_eq!(
                is_expr.is_type,
                Some("{urn:hl7-org:elm-types:r1}String".to_string())
            );
        } else {
            panic!("Expected Is expression");
        }
    }

    #[test]
    fn test_translate_as_expression() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("test".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });
        let type_spec = elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}String".to_string(),
            ..Default::default()
        });

        let result = translator.translate_as(operand, type_spec, false);

        if let elm::Expression::As(as_expr) = result {
            assert!(as_expr.operand.is_some());
            assert!(as_expr.as_type_specifier.is_some());
            assert!(as_expr.strict.is_none());
        } else {
            panic!("Expected As expression");
        }
    }

    #[test]
    fn test_translate_as_strict() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Null(elm::Null::default());
        let type_spec = elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: "Integer".to_string(),
            ..Default::default()
        });

        let result = translator.translate_as(operand, type_spec, true);

        if let elm::Expression::As(as_expr) = result {
            assert_eq!(as_expr.strict, Some(true));
        } else {
            panic!("Expected As expression");
        }
    }

    #[test]
    fn test_translate_convert_expression() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });
        let type_spec = elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: "{urn:hl7-org:elm-types:r1}Integer".to_string(),
            ..Default::default()
        });

        let result = translator.translate_convert(operand, type_spec);

        if let elm::Expression::Convert(convert_expr) = result {
            assert!(convert_expr.operand.is_some());
            assert!(convert_expr.to_type_specifier.is_some());
        } else {
            panic!("Expected Convert expression");
        }
    }

    #[test]
    fn test_translate_can_convert() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            ..Default::default()
        });
        let type_spec = elm::TypeSpecifier::Named(elm::NamedTypeSpecifier {
            name: "Integer".to_string(),
            ..Default::default()
        });

        let result = translator.translate_can_convert(operand, type_spec);

        if let elm::Expression::CanConvert(can_convert) = result {
            assert!(can_convert.operand.is_some());
            assert!(can_convert.to_type_specifier.is_some());
        } else {
            panic!("Expected CanConvert expression");
        }
    }

    #[test]
    fn test_translate_to_integer() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });

        let result = translator.translate_to_integer(operand);

        if let elm::Expression::ToInteger(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(
                unary.element.result_type_name,
                Some(qname_system("Integer"))
            );
        } else {
            panic!("Expected ToInteger expression");
        }
    }

    #[test]
    fn test_translate_to_string() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("Integer")),
            ..Default::default()
        });

        let result = translator.translate_to_string(operand);

        if let elm::Expression::ToStringExpr(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(unary.element.result_type_name, Some(qname_system("String")));
        } else {
            panic!("Expected ToString expression");
        }
    }

    #[test]
    fn test_translate_to_decimal() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("Integer")),
            ..Default::default()
        });

        let result = translator.translate_to_decimal(operand);

        if let elm::Expression::ToDecimal(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(
                unary.element.result_type_name,
                Some(qname_system("Decimal"))
            );
        } else {
            panic!("Expected ToDecimal expression");
        }
    }

    #[test]
    fn test_translate_to_boolean() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("true".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });

        let result = translator.translate_to_boolean(operand);

        if let elm::Expression::ToBoolean(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(
                unary.element.result_type_name,
                Some(qname_system("Boolean"))
            );
        } else {
            panic!("Expected ToBoolean expression");
        }
    }

    #[test]
    fn test_translate_to_date() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("2024-01-15".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });

        let result = translator.translate_to_date(operand);

        if let elm::Expression::ToDate(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(unary.element.result_type_name, Some(qname_system("Date")));
        } else {
            panic!("Expected ToDate expression");
        }
    }

    #[test]
    fn test_translate_to_datetime() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("2024-01-15T10:30:00".to_string()),
            ..Default::default()
        });

        let result = translator.translate_to_datetime(operand);

        if let elm::Expression::ToDateTime(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(
                unary.element.result_type_name,
                Some(qname_system("DateTime"))
            );
        } else {
            panic!("Expected ToDateTime expression");
        }
    }

    #[test]
    fn test_translate_to_time() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("10:30:00".to_string()),
            ..Default::default()
        });

        let result = translator.translate_to_time(operand);

        if let elm::Expression::ToTime(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(unary.element.result_type_name, Some(qname_system("Time")));
        } else {
            panic!("Expected ToTime expression");
        }
    }

    #[test]
    fn test_translate_to_quantity() {
        let mut translator =
            ExpressionTranslator::with_options(crate::options::CompilerOptions::debug());
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42 'kg'".to_string()),
            ..Default::default()
        });

        let result = translator.translate_to_quantity(operand);

        if let elm::Expression::ToQuantity(unary) = result {
            assert!(unary.operand.is_some());
            assert_eq!(
                unary.element.result_type_name,
                Some(qname_system("Quantity"))
            );
        } else {
            panic!("Expected ToQuantity expression");
        }
    }

    #[test]
    fn test_translate_to_list() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("42".to_string()),
            value_type: Some(qname_system("Integer")),
            ..Default::default()
        });

        let result =
            translator.translate_to_list(operand, Some(&DataType::System(SystemType::Integer)));

        if let elm::Expression::ToList(unary) = result {
            assert!(unary.operand.is_some());
        } else {
            panic!("Expected ToList expression");
        }
    }

    #[test]
    fn test_translate_to_chars() {
        let mut translator = ExpressionTranslator::new();
        let operand = elm::Expression::Literal(elm::Literal {
            value: Some("hello".to_string()),
            value_type: Some(qname_system("String")),
            ..Default::default()
        });

        let result = translator.translate_to_chars(operand);

        if let elm::Expression::ToChars(unary) = result {
            assert!(unary.operand.is_some());
        } else {
            panic!("Expected ToChars expression");
        }
    }

    #[test]
    fn test_translate_type_expression_is() {
        let mut translator = ExpressionTranslator::new();
        let type_expr = ast::TypeExpression {
            operator: ast::TypeOperator::Is,
            operand: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
            type_specifier: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            }),
            location: None,
        };

        let result = translator.translate_type_expression(&type_expr, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert!(matches!(result, elm::Expression::Is(_)));
    }

    #[test]
    fn test_translate_type_expression_as() {
        let mut translator = ExpressionTranslator::new();
        let type_expr = ast::TypeExpression {
            operator: ast::TypeOperator::As,
            operand: Box::new(ast::Expression::Literal(ast::Literal::String(
                "42".to_string(),
            ))),
            type_specifier: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: Some("urn:hl7-org:elm-types:r1".to_string()),
                name: "Integer".to_string(),
            }),
            location: None,
        };

        let result = translator.translate_type_expression(&type_expr, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert!(matches!(result, elm::Expression::As(_)));
    }

    #[test]
    fn test_translate_type_expression_convert() {
        let mut translator = ExpressionTranslator::new();
        let type_expr = ast::TypeExpression {
            operator: ast::TypeOperator::Convert,
            operand: Box::new(ast::Expression::Literal(ast::Literal::Integer(42))),
            type_specifier: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "String".to_string(),
            }),
            location: None,
        };

        let result = translator.translate_type_expression(&type_expr, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected literal");
            }
        });

        assert!(matches!(result, elm::Expression::Convert(_)));
    }

    #[test]
    fn test_type_specifier_to_elm_named() {
        let translator = ExpressionTranslator::new();
        let ast_ts = ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
            namespace: Some("http://hl7.org/fhir".to_string()),
            name: "Patient".to_string(),
        });

        let result = translator.type_specifier_to_elm(&ast_ts);

        if let elm::TypeSpecifier::Named(named) = result {
            assert_eq!(named.name, "{http://hl7.org/fhir}Patient");
        } else {
            panic!("Expected Named type specifier");
        }
    }

    #[test]
    fn test_type_specifier_to_elm_list() {
        let translator = ExpressionTranslator::new();
        let ast_ts = ast::TypeSpecifier::List(ast::ListTypeSpecifier {
            element_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
        });

        let result = translator.type_specifier_to_elm(&ast_ts);

        if let elm::TypeSpecifier::List(list) = result {
            assert!(list.element_type.is_some());
        } else {
            panic!("Expected List type specifier");
        }
    }

    #[test]
    fn test_type_specifier_to_elm_interval() {
        let translator = ExpressionTranslator::new();
        let ast_ts = ast::TypeSpecifier::Interval(ast::IntervalTypeSpecifier {
            point_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Date".to_string(),
            })),
        });

        let result = translator.type_specifier_to_elm(&ast_ts);

        if let elm::TypeSpecifier::Interval(interval) = result {
            assert!(interval.point_type.is_some());
        } else {
            panic!("Expected Interval type specifier");
        }
    }

    #[test]
    fn test_type_specifier_to_elm_tuple() {
        let translator = ExpressionTranslator::new();
        let ast_ts = ast::TypeSpecifier::Tuple(ast::TupleTypeSpecifier {
            elements: vec![ast::TupleElementDef {
                name: "value".to_string(),
                element_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "Integer".to_string(),
                }),
            }],
        });

        let result = translator.type_specifier_to_elm(&ast_ts);

        if let elm::TypeSpecifier::Tuple(tuple) = result {
            assert_eq!(tuple.element.len(), 1);
            assert_eq!(tuple.element[0].name, "value");
        } else {
            panic!("Expected Tuple type specifier");
        }
    }

    // ========================================================================
    // Function Resolution Tests (Phase 4.6)
    // ========================================================================

    // -- Nullary System Functions --

    #[test]
    fn test_resolve_now() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Now".to_string(),
            arguments: vec![],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |_, _| panic!("Should not translate args"),
            None,
        );

        assert!(matches!(result, elm::Expression::Now(_)));
    }

    #[test]
    fn test_resolve_today() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Today".to_string(),
            arguments: vec![],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |_, _| panic!("Should not translate args"),
            None,
        );

        assert!(matches!(result, elm::Expression::Today(_)));
    }

    #[test]
    fn test_resolve_time_of_day() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "TimeOfDay".to_string(),
            arguments: vec![],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |_, _| panic!("Should not translate args"),
            None,
        );

        assert!(matches!(result, elm::Expression::TimeOfDay(_)));
    }

    // -- Aggregate Functions --

    #[test]
    fn test_resolve_sum() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Sum".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "values".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        if let elm::Expression::Sum(agg) = result {
            assert!(agg.source.is_some());
        } else {
            panic!("Expected Sum expression");
        }
    }

    #[test]
    fn test_resolve_count() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Count".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "items".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Count(_)));
    }

    #[test]
    fn test_resolve_min() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Min".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "values".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Min(_)));
    }

    #[test]
    fn test_resolve_max() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Max".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "values".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Max(_)));
    }

    #[test]
    fn test_resolve_avg() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Avg".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "values".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Avg(_)));
    }

    #[test]
    fn test_resolve_all_true() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "AllTrue".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "flags".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::AllTrue(_)));
    }

    #[test]
    fn test_resolve_any_true() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "AnyTrue".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "flags".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::AnyTrue(_)));
    }

    // -- List Functions --

    #[test]
    fn test_resolve_first() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "First".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "list".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        if let elm::Expression::First(unary) = result {
            assert!(unary.operand.is_some());
        } else {
            panic!("Expected First expression");
        }
    }

    #[test]
    fn test_resolve_last() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Last".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "list".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Last(_)));
    }

    #[test]
    fn test_resolve_flatten() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Flatten".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "nested".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Flatten(_)));
    }

    #[test]
    fn test_resolve_distinct() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Distinct".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "list".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Distinct(_)));
    }

    #[test]
    fn test_resolve_exists() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Exists".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "list".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Exists(_)));
    }

    // -- Nullological Functions --

    #[test]
    fn test_resolve_is_null() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "IsNull".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "value".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::IsNull(_)));
    }

    #[test]
    fn test_resolve_coalesce() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Coalesce".to_string(),
            arguments: vec![
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "a".to_string(),
                    location: None,
                }),
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "b".to_string(),
                    location: None,
                }),
                ast::Expression::Literal(ast::Literal::Integer(0)),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression"),
            },
            None,
        );

        if let elm::Expression::Coalesce(nary) = result {
            assert_eq!(nary.operand.len(), 3);
        } else {
            panic!("Expected Coalesce expression");
        }
    }

    // -- Arithmetic Functions --

    #[test]
    fn test_resolve_abs() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Abs".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::Integer(-5))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Abs(_)));
    }

    #[test]
    fn test_resolve_ceiling() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Ceiling".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::Decimal(3.7))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Ceiling(_)));
    }

    #[test]
    fn test_resolve_floor() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Floor".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::Decimal(3.7))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Floor(_)));
    }

    #[test]
    fn test_resolve_log_binary() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Log".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::Integer(100)),
                ast::Expression::Literal(ast::Literal::Integer(10)),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        if let elm::Expression::Log(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Log expression");
        }
    }

    #[test]
    fn test_resolve_power() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Power".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::Integer(2)),
                ast::Expression::Literal(ast::Literal::Integer(8)),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        if let elm::Expression::Power(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Power expression");
        }
    }

    // -- String Functions --

    #[test]
    fn test_resolve_length() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Length".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::String(
                "hello".to_string(),
            ))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Length(_)));
    }

    #[test]
    fn test_resolve_upper() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Upper".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::String(
                "hello".to_string(),
            ))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Upper(_)));
    }

    #[test]
    fn test_resolve_lower() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Lower".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::String(
                "HELLO".to_string(),
            ))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Lower(_)));
    }

    #[test]
    fn test_resolve_starts_with() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "StartsWith".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::String("hello".to_string())),
                ast::Expression::Literal(ast::Literal::String("he".to_string())),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::StartsWith(_)));
    }

    #[test]
    fn test_resolve_ends_with() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "EndsWith".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::String("hello".to_string())),
                ast::Expression::Literal(ast::Literal::String("lo".to_string())),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::EndsWith(_)));
    }

    #[test]
    fn test_resolve_concatenate() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Concatenate".to_string(),
            arguments: vec![
                ast::Expression::Literal(ast::Literal::String("Hello".to_string())),
                ast::Expression::Literal(ast::Literal::String(" ".to_string())),
                ast::Expression::Literal(ast::Literal::String("World".to_string())),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        if let elm::Expression::Concatenate(nary) = result {
            assert_eq!(nary.operand.len(), 3);
        } else {
            panic!("Expected Concatenate expression");
        }
    }

    // -- Interval Functions --

    #[test]
    fn test_resolve_start() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Start".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "interval".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Start(_)));
    }

    #[test]
    fn test_resolve_end() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "End".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "interval".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::End(_)));
    }

    #[test]
    fn test_resolve_width() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Width".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "interval".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::Width(_)));
    }

    #[test]
    fn test_resolve_union() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Union".to_string(),
            arguments: vec![
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list1".to_string(),
                    location: None,
                }),
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list2".to_string(),
                    location: None,
                }),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        if let elm::Expression::Union(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Union expression");
        }
    }

    #[test]
    fn test_resolve_intersect() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Intersect".to_string(),
            arguments: vec![
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list1".to_string(),
                    location: None,
                }),
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list2".to_string(),
                    location: None,
                }),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        if let elm::Expression::Intersect(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Intersect expression");
        }
    }

    #[test]
    fn test_resolve_except() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "Except".to_string(),
            arguments: vec![
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list1".to_string(),
                    location: None,
                }),
                ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "list2".to_string(),
                    location: None,
                }),
            ],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        if let elm::Expression::Except(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected Except expression");
        }
    }

    // -- User-Defined Functions --

    #[test]
    fn test_resolve_user_defined_function() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "MyCustomFunction".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::Integer(42))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        // Should fall back to FunctionRef for unknown functions
        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("MyCustomFunction".to_string()));
            assert_eq!(func_ref.operand.len(), 1);
        } else {
            panic!("Expected FunctionRef for user-defined function");
        }
    }

    #[test]
    fn test_resolve_library_qualified_function() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: Some("FHIRHelpers".to_string()),
            name: "ToQuantity".to_string(),
            arguments: vec![ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "value".to_string(),
                location: None,
            })],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        // Library-qualified functions always become FunctionRef
        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("ToQuantity".to_string()));
            assert_eq!(func_ref.library_name, Some("FHIRHelpers".to_string()));
        } else {
            panic!("Expected FunctionRef for library-qualified function");
        }
    }

    // -- Fluent Function Syntax --

    #[test]
    fn test_resolve_fluent_first() {
        let mut translator = ExpressionTranslator::new();
        let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "myList".to_string(),
            location: None,
        });

        let result = translator.resolve_fluent_function(
            &source,
            "First",
            &[], // No additional args
            None,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        // Should resolve to First(myList)
        if let elm::Expression::First(unary) = result {
            assert!(unary.operand.is_some());
        } else {
            panic!("Expected First expression for fluent call");
        }
    }

    #[test]
    fn test_resolve_fluent_count() {
        let mut translator = ExpressionTranslator::new();
        let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "encounters".to_string(),
            location: None,
        });

        let result = translator.resolve_fluent_function(
            &source,
            "Count",
            &[],
            None,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        // Should resolve to Count(encounters)
        if let elm::Expression::Count(agg) = result {
            assert!(agg.source.is_some());
        } else {
            panic!("Expected Count expression for fluent call");
        }
    }

    #[test]
    fn test_resolve_fluent_user_function() {
        let mut translator = ExpressionTranslator::new();
        let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "patient".to_string(),
            location: None,
        });

        let result = translator.resolve_fluent_function(
            &source,
            "IsAdult",
            &[],
            None,
            |t, expr| {
                if let ast::Expression::IdentifierRef(id_ref) = expr {
                    t.translate_ast_identifier_ref(id_ref)
                } else {
                    panic!("Expected IdentifierRef");
                }
            },
            None,
        );

        // User-defined fluent function becomes FunctionRef with source as first arg
        if let elm::Expression::FunctionRef(func_ref) = result {
            assert_eq!(func_ref.name, Some("IsAdult".to_string()));
            assert_eq!(func_ref.operand.len(), 1); // patient is first arg
        } else {
            panic!("Expected FunctionRef for fluent user function");
        }
    }

    #[test]
    fn test_resolve_fluent_with_args() {
        let mut translator = ExpressionTranslator::new();
        let source = ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "str".to_string(),
            location: None,
        });

        let result = translator.resolve_fluent_function(
            &source,
            "StartsWith",
            &[ast::Expression::Literal(ast::Literal::String(
                "prefix".to_string(),
            ))],
            None,
            |t, expr| match expr {
                ast::Expression::IdentifierRef(id_ref) => t.translate_ast_identifier_ref(id_ref),
                ast::Expression::Literal(lit) => t.translate_literal(lit),
                _ => panic!("Unexpected expression"),
            },
            None,
        );

        // str.StartsWith("prefix") -> StartsWith(str, "prefix")
        if let elm::Expression::StartsWith(binary) = result {
            assert_eq!(binary.operand.len(), 2);
        } else {
            panic!("Expected StartsWith expression");
        }
    }

    // -- Function Definition Translation --

    #[test]
    fn test_translate_function_def_basic() {
        let mut translator = ExpressionTranslator::new();
        let func_def = ast::FunctionDef {
            name: "Double".to_string(),
            parameters: vec![ast::FunctionParameter {
                name: "x".to_string(),
                type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "Integer".to_string(),
                })),
            }],
            return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
            body: Some(ast::Expression::Literal(ast::Literal::Integer(42))),
            fluent: false,
            external: false,
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_function_def(&func_def, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected Literal");
            }
        });

        assert_eq!(result.name, Some("Double".to_string()));
        assert_eq!(result.operand.len(), 1);
        assert_eq!(result.operand[0].name, Some("x".to_string()));
        assert!(result.expression.is_some());
        assert_eq!(result.fluent, None); // Not fluent
        assert_eq!(result.external, None); // Not external
    }

    #[test]
    fn test_translate_function_def_fluent() {
        let mut translator = ExpressionTranslator::new();
        let func_def = ast::FunctionDef {
            name: "IsAdult".to_string(),
            parameters: vec![ast::FunctionParameter {
                name: "p".to_string(),
                type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: Some("FHIR".to_string()),
                    name: "Patient".to_string(),
                })),
            }],
            return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Boolean".to_string(),
            })),
            body: Some(ast::Expression::Literal(ast::Literal::Boolean(true))),
            fluent: true,
            external: false,
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_function_def(&func_def, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected Literal");
            }
        });

        assert_eq!(result.name, Some("IsAdult".to_string()));
        assert_eq!(result.fluent, Some(true));
    }

    #[test]
    fn test_translate_function_def_external() {
        let mut translator = ExpressionTranslator::new();
        let func_def = ast::FunctionDef {
            name: "ExternalFunc".to_string(),
            parameters: vec![],
            return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "String".to_string(),
            })),
            body: None, // External functions have no body
            fluent: false,
            external: true,
            access: ast::AccessModifier::Private,
            location: None,
        };

        let result = translator.translate_function_def(&func_def, |_, _| {
            panic!("Should not translate body");
        });

        assert_eq!(result.name, Some("ExternalFunc".to_string()));
        assert_eq!(result.external, Some(true));
        assert!(result.expression.is_none());
        assert_eq!(result.access_level, Some(elm::AccessModifier::Private));
    }

    #[test]
    fn test_translate_function_def_multiple_params() {
        let mut translator = ExpressionTranslator::new();
        let func_def = ast::FunctionDef {
            name: "Add".to_string(),
            parameters: vec![
                ast::FunctionParameter {
                    name: "a".to_string(),
                    type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                        namespace: None,
                        name: "Integer".to_string(),
                    })),
                },
                ast::FunctionParameter {
                    name: "b".to_string(),
                    type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                        namespace: None,
                        name: "Integer".to_string(),
                    })),
                },
            ],
            return_type: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
            body: Some(ast::Expression::Literal(ast::Literal::Integer(0))),
            fluent: false,
            external: false,
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_function_def(&func_def, |t, expr| {
            if let ast::Expression::Literal(lit) = expr {
                t.translate_literal(lit)
            } else {
                panic!("Expected Literal");
            }
        });

        assert_eq!(result.operand.len(), 2);
        assert_eq!(result.operand[0].name, Some("a".to_string()));
        assert_eq!(result.operand[1].name, Some("b".to_string()));
    }

    // -- Type Conversion Functions (via function invocation) --

    #[test]
    fn test_resolve_to_integer_function() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "ToInteger".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::String(
                "42".to_string(),
            ))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::ToInteger(_)));
    }

    #[test]
    fn test_resolve_to_string_function() {
        let mut translator = ExpressionTranslator::new();
        let func = ast::FunctionInvocation {
            library: None,
            name: "ToString".to_string(),
            arguments: vec![ast::Expression::Literal(ast::Literal::Integer(42))],
            location: None,
        };

        let result = translator.resolve_function_invocation(
            &func,
            |t, expr| {
                if let ast::Expression::Literal(lit) = expr {
                    t.translate_literal(lit)
                } else {
                    panic!("Expected Literal");
                }
            },
            None,
        );

        assert!(matches!(result, elm::Expression::ToStringExpr(_)));
    }

    // ========================================================================
    // translate_expression Tests (Phase 6.4a)
    // ========================================================================

    #[test]
    fn test_translate_expression_literal() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::Literal(ast::Literal::Integer(42));
        let result = translator.translate_expression(&expr);
        assert!(matches!(result, elm::Expression::Literal(_)));
    }

    #[test]
    fn test_translate_expression_identifier_ref() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::IdentifierRef(ast::IdentifierRef {
            name: "foo".to_string(),
            location: None,
        });
        let result = translator.translate_expression(&expr);
        assert!(matches!(result, elm::Expression::IdentifierRef(_)));
    }

    #[test]
    fn test_translate_expression_binary_add() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::BinaryExpression(ast::BinaryExpression {
            operator: ast::BinaryOperator::Add,
            left: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            right: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
            precision: None,
            location: None,
        });
        let result = translator.translate_expression(&expr);
        assert!(matches!(result, elm::Expression::Add(_)));
    }

    #[test]
    fn test_translate_expression_parenthesized() {
        let mut translator = ExpressionTranslator::new();
        // Parenthesized expressions should unwrap to their inner expression
        let expr = ast::Expression::Parenthesized(Box::new(ast::Expression::Literal(
            ast::Literal::Boolean(true),
        )));
        let result = translator.translate_expression(&expr);
        assert!(matches!(result, elm::Expression::Literal(_)));
    }

    #[test]
    fn test_translate_expression_interval() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::IntervalExpression(ast::IntervalExpression {
            low: Some(Box::new(ast::Expression::Literal(ast::Literal::Integer(1)))),
            high: Some(Box::new(ast::Expression::Literal(ast::Literal::Integer(
                10,
            )))),
            low_closed: true,
            high_closed: true,
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::Interval(interval) = result {
            assert!(interval.low.is_some());
            assert!(interval.high.is_some());
            assert_eq!(interval.low_closed, Some(true));
            assert_eq!(interval.high_closed, Some(true));
        } else {
            panic!("Expected Interval expression");
        }
    }

    #[test]
    fn test_translate_expression_list() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::ListExpression(ast::ListExpression {
            elements: vec![
                ast::Expression::Literal(ast::Literal::Integer(1)),
                ast::Expression::Literal(ast::Literal::Integer(2)),
                ast::Expression::Literal(ast::Literal::Integer(3)),
            ],
            type_specifier: None,
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::List(list) = result {
            assert_eq!(list.elements.len(), 3);
        } else {
            panic!("Expected List expression");
        }
    }

    #[test]
    fn test_translate_expression_tuple() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::TupleExpression(ast::TupleExpression {
            elements: vec![
                ast::TupleElement {
                    name: "x".to_string(),
                    value: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                },
                ast::TupleElement {
                    name: "y".to_string(),
                    value: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
                },
            ],
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::Tuple(tuple) = result {
            assert_eq!(tuple.elements.len(), 2);
            assert_eq!(tuple.elements[0].name, Some("x".to_string()));
            assert_eq!(tuple.elements[1].name, Some("y".to_string()));
        } else {
            panic!("Expected Tuple expression");
        }
    }

    #[test]
    fn test_translate_expression_instance() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::Instance(ast::Instance {
            class_type: ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: Some("FHIR".to_string()),
                name: "Quantity".to_string(),
            }),
            elements: vec![ast::InstanceElement {
                name: "value".to_string(),
                value: Box::new(ast::Expression::Literal(ast::Literal::Decimal(5.0))),
            }],
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::Instance(instance) = result {
            assert!(instance.class_type.is_some());
            assert_eq!(instance.elements.len(), 1);
            assert_eq!(instance.elements[0].name, Some("value".to_string()));
        } else {
            panic!("Expected Instance expression");
        }
    }

    #[test]
    fn test_translate_expression_member_invocation() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::MemberInvocation(ast::MemberInvocation {
            source: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "patient".to_string(),
                location: None,
            })),
            name: "birthDate".to_string(),
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::Property(prop) = result {
            assert_eq!(prop.path, Some("birthDate".to_string()));
            assert!(prop.source.is_some());
        } else {
            panic!("Expected Property expression");
        }
    }

    #[test]
    fn test_translate_expression_index_invocation() {
        let mut translator = ExpressionTranslator::new();
        let expr = ast::Expression::IndexInvocation(ast::IndexInvocation {
            source: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "list".to_string(),
                location: None,
            })),
            index: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
            location: None,
        });
        let result = translator.translate_expression(&expr);
        if let elm::Expression::Indexer(indexer) = result {
            assert_eq!(indexer.operand.len(), 2);
        } else {
            panic!("Expected Indexer expression");
        }
    }

    // ========================================================================
    // Statement Translation Tests (Phase 6.4b)
    // ========================================================================

    #[test]
    fn test_translate_expression_def_basic() {
        let mut translator = ExpressionTranslator::new();
        let expr_def = ast::ExpressionDef {
            name: "ActiveEncounters".to_string(),
            expression: ast::Expression::Literal(ast::Literal::Integer(42)),
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_expression_def(&expr_def, Some("Patient"));

        assert_eq!(result.name, Some("ActiveEncounters".to_string()));
        assert_eq!(result.context, Some("Patient".to_string()));
        assert_eq!(result.access_level, Some(elm::AccessModifier::Public));
        assert!(result.expression.is_some());
    }

    #[test]
    fn test_translate_expression_def_private() {
        let mut translator = ExpressionTranslator::new();
        let expr_def = ast::ExpressionDef {
            name: "PrivateHelper".to_string(),
            expression: ast::Expression::Literal(ast::Literal::Boolean(true)),
            access: ast::AccessModifier::Private,
            location: None,
        };

        let result = translator.translate_expression_def(&expr_def, None);

        assert_eq!(result.name, Some("PrivateHelper".to_string()));
        assert_eq!(result.context, None);
        assert_eq!(result.access_level, Some(elm::AccessModifier::Private));
    }

    #[test]
    fn test_translate_expression_def_with_complex_body() {
        let mut translator = ExpressionTranslator::new();
        let expr_def = ast::ExpressionDef {
            name: "Sum".to_string(),
            expression: ast::Expression::BinaryExpression(ast::BinaryExpression {
                operator: ast::BinaryOperator::Add,
                left: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
                right: Box::new(ast::Expression::Literal(ast::Literal::Integer(2))),
                precision: None,
                location: None,
            }),
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_expression_def(&expr_def, Some("Practitioner"));

        assert_eq!(result.name, Some("Sum".to_string()));
        assert!(result.expression.is_some());
        if let Some(expr) = result.expression {
            assert!(matches!(*expr, elm::Expression::Add(_)));
        }
    }

    #[test]
    fn test_translate_parameter_def_basic() {
        let mut translator = ExpressionTranslator::new();
        let param_def = ast::ParameterDef {
            name: "MeasurementPeriod".to_string(),
            type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "DateTime".to_string(),
            })),
            default: None,
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_parameter_def(&param_def);

        assert_eq!(result.name, Some("MeasurementPeriod".to_string()));
        assert!(result.parameter_type_specifier.is_some());
        assert!(result.parameter_type_name.is_some());
        assert!(result.default_value.is_none());
        assert_eq!(result.access_level, Some(elm::AccessModifier::Public));
    }

    #[test]
    fn test_translate_parameter_def_with_default() {
        let mut translator = ExpressionTranslator::new();
        let param_def = ast::ParameterDef {
            name: "MaxCount".to_string(),
            type_specifier: Some(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                namespace: None,
                name: "Integer".to_string(),
            })),
            default: Some(ast::Expression::Literal(ast::Literal::Integer(10))),
            access: ast::AccessModifier::Private,
            location: None,
        };

        let result = translator.translate_parameter_def(&param_def);

        assert_eq!(result.name, Some("MaxCount".to_string()));
        assert!(result.default_value.is_some());
        assert_eq!(result.access_level, Some(elm::AccessModifier::Private));
    }

    #[test]
    fn test_translate_parameter_def_interval_type() {
        let mut translator = ExpressionTranslator::new();
        let param_def = ast::ParameterDef {
            name: "MeasurementPeriod".to_string(),
            type_specifier: Some(ast::TypeSpecifier::Interval(ast::IntervalTypeSpecifier {
                point_type: Box::new(ast::TypeSpecifier::Named(ast::NamedTypeSpecifier {
                    namespace: None,
                    name: "DateTime".to_string(),
                })),
            })),
            default: None,
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_parameter_def(&param_def);

        assert_eq!(result.name, Some("MeasurementPeriod".to_string()));
        assert!(result.parameter_type_specifier.is_some());
        if let Some(ts) = result.parameter_type_specifier {
            assert!(matches!(ts, elm::TypeSpecifier::Interval(_)));
        }
        assert_eq!(
            result.parameter_type_name,
            Some("Interval<DateTime>".to_string())
        );
    }

    #[test]
    fn test_translate_statement_expression() {
        let mut translator = ExpressionTranslator::new();
        let stmt = ast::Statement::ExpressionDef(ast::ExpressionDef {
            name: "TestExpr".to_string(),
            expression: ast::Expression::Literal(ast::Literal::String("hello".to_string())),
            access: ast::AccessModifier::Public,
            location: None,
        });

        let result = translator.translate_statement(&stmt, Some("Patient"));

        if let StatementTranslation::Expression(expr_def) = result {
            assert_eq!(expr_def.name, Some("TestExpr".to_string()));
            assert_eq!(expr_def.context, Some("Patient".to_string()));
        } else {
            panic!("Expected Expression translation");
        }
    }

    #[test]
    fn test_translate_statement_function() {
        let mut translator = ExpressionTranslator::new();
        let stmt = ast::Statement::FunctionDef(ast::FunctionDef {
            name: "TestFunc".to_string(),
            parameters: vec![],
            return_type: None,
            body: Some(ast::Expression::Literal(ast::Literal::Integer(42))),
            fluent: false,
            external: false,
            access: ast::AccessModifier::Public,
            location: None,
        });

        let result = translator.translate_statement(&stmt, None);

        if let StatementTranslation::Function(func_def) = result {
            assert_eq!(func_def.name, Some("TestFunc".to_string()));
        } else {
            panic!("Expected Function translation");
        }
    }

    // ========================================================================
    // Terminology Translation Tests (Phase 6.4c)
    // ========================================================================

    #[test]
    fn test_translate_using_def_fhir() {
        let translator = ExpressionTranslator::new();
        let using_def = ast::UsingDef {
            model_name: "FHIR".to_string(),
            version: Some("4.0.1".to_string()),
            location: None,
        };

        let result = translator.translate_using_def(&using_def);

        assert_eq!(result.local_identifier, Some("FHIR".to_string()));
        assert_eq!(result.uri, Some("http://hl7.org/fhir".to_string()));
        assert_eq!(result.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_translate_using_def_qicore() {
        let translator = ExpressionTranslator::new();
        let using_def = ast::UsingDef {
            model_name: "QICore".to_string(),
            version: None,
            location: None,
        };

        let result = translator.translate_using_def(&using_def);

        assert_eq!(result.local_identifier, Some("QICore".to_string()));
        assert_eq!(
            result.uri,
            Some("http://hl7.org/fhir/us/qicore".to_string())
        );
        assert_eq!(result.version, None);
    }

    #[test]
    fn test_translate_include_def_with_alias() {
        let translator = ExpressionTranslator::new();
        let include_def = ast::IncludeDef {
            path: "FHIRHelpers".to_string(),
            version: Some("4.0.1".to_string()),
            alias: Some("Helpers".to_string()),
            location: None,
        };

        let result = translator.translate_include_def(&include_def);

        assert_eq!(result.local_identifier, Some("Helpers".to_string()));
        assert_eq!(result.path, Some("FHIRHelpers".to_string()));
        assert_eq!(result.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_translate_include_def_no_alias() {
        let translator = ExpressionTranslator::new();
        let include_def = ast::IncludeDef {
            path: "CommonLibrary".to_string(),
            version: None,
            alias: None,
            location: None,
        };

        let result = translator.translate_include_def(&include_def);

        assert_eq!(result.local_identifier, Some("CommonLibrary".to_string()));
        assert_eq!(result.path, Some("CommonLibrary".to_string()));
    }

    #[test]
    fn test_translate_codesystem_def() {
        let translator = ExpressionTranslator::new();
        let cs_def = ast::CodeSystemDef {
            name: "LOINC".to_string(),
            id: "http://loinc.org".to_string(),
            version: Some("2.73".to_string()),
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_codesystem_def(&cs_def);

        assert_eq!(result.name, Some("LOINC".to_string()));
        assert_eq!(result.id, Some("http://loinc.org".to_string()));
        assert_eq!(result.version, Some("2.73".to_string()));
        assert_eq!(result.access_level, Some(elm::AccessModifier::Public));
    }

    #[test]
    fn test_translate_valueset_def() {
        let translator = ExpressionTranslator::new();
        let vs_def = ast::ValueSetDef {
            name: "Active Conditions".to_string(),
            id: "http://example.org/fhir/ValueSet/active-conditions".to_string(),
            version: None,
            codesystems: vec!["SNOMED".to_string(), "ICD10".to_string()],
            access: ast::AccessModifier::Private,
            location: None,
        };

        let result = translator.translate_valueset_def(&vs_def);

        assert_eq!(result.name, Some("Active Conditions".to_string()));
        assert_eq!(
            result.id,
            Some("http://example.org/fhir/ValueSet/active-conditions".to_string())
        );
        assert_eq!(result.access_level, Some(elm::AccessModifier::Private));
        assert_eq!(result.code_system.len(), 2);
        assert_eq!(result.code_system[0].name, Some("SNOMED".to_string()));
        assert_eq!(result.code_system[1].name, Some("ICD10".to_string()));
    }

    #[test]
    fn test_translate_code_def() {
        let translator = ExpressionTranslator::new();
        let code_def = ast::CodeDef {
            name: "Blood Pressure".to_string(),
            code: "85354-9".to_string(),
            codesystem: "LOINC".to_string(),
            display: Some("Blood pressure panel".to_string()),
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_code_def(&code_def);

        assert_eq!(result.name, Some("Blood Pressure".to_string()));
        assert_eq!(result.id, Some("85354-9".to_string()));
        assert_eq!(result.display, Some("Blood pressure panel".to_string()));
        assert!(result.code_system.is_some());
        assert_eq!(
            result.code_system.as_ref().unwrap().name,
            Some("LOINC".to_string())
        );
    }

    #[test]
    fn test_translate_concept_def() {
        let translator = ExpressionTranslator::new();
        let concept_def = ast::ConceptDef {
            name: "Blood Pressure Codes".to_string(),
            codes: vec!["Systolic BP".to_string(), "Diastolic BP".to_string()],
            display: Some("Blood Pressure".to_string()),
            access: ast::AccessModifier::Public,
            location: None,
        };

        let result = translator.translate_concept_def(&concept_def);

        assert_eq!(result.name, Some("Blood Pressure Codes".to_string()));
        assert_eq!(result.display, Some("Blood Pressure".to_string()));
        assert_eq!(result.code.len(), 2);
        assert_eq!(result.code[0].name, Some("Systolic BP".to_string()));
        assert_eq!(result.code[1].name, Some("Diastolic BP".to_string()));
    }

    #[test]
    fn test_translate_context_def() {
        let translator = ExpressionTranslator::new();
        let ctx_def = ast::ContextDef {
            name: "Patient".to_string(),
            location: None,
        };

        let result = translator.translate_context_def(&ctx_def);

        assert_eq!(result.name, Some("Patient".to_string()));
    }
}
