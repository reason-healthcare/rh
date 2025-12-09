//! Operator resolution for CQL semantic analysis.
//!
//! This module provides the infrastructure for resolving CQL operators to their
//! appropriate signatures based on operand types. It handles:
//!
//! 1. **Operator Signatures**: Type signatures for all CQL system operators
//! 2. **Overload Resolution**: Finding the best matching signature for given operands
//! 3. **Implicit Conversions**: Applying type promotions when needed
//!
//! # Overview
//!
//! CQL operators are polymorphic - the same operator symbol (like `+`) can work
//! on different types (Integer+Integer, Decimal+Decimal, String+String, etc.).
//! The operator resolver determines which overload to use based on operand types.
//!
//! # Example
//!
//! ```
//! use rh_cql::operators::{OperatorResolver, OperatorSignature};
//! use rh_cql::datatype::DataType;
//! use rh_cql::parser::ast::BinaryOperator;
//!
//! let resolver = OperatorResolver::new();
//!
//! // Resolve Add operator for Integer + Integer
//! let result = resolver.resolve_binary(
//!     BinaryOperator::Add,
//!     &DataType::integer(),
//!     &DataType::integer(),
//! );
//! assert!(result.is_ok());
//! let resolved = result.unwrap();
//! assert_eq!(resolved.result_type, DataType::integer());
//!
//! // Resolve Add operator for Integer + Decimal (implicit conversion)
//! let result = resolver.resolve_binary(
//!     BinaryOperator::Add,
//!     &DataType::integer(),
//!     &DataType::decimal(),
//! );
//! assert!(result.is_ok());
//! let resolved = result.unwrap();
//! assert_eq!(resolved.result_type, DataType::decimal());
//! ```

use crate::datatype::{DataType, SystemType};
use crate::parser::ast::{BinaryOperator, TernaryOperator, UnaryOperator};
use std::collections::HashMap;

// ============================================================================
// Operator Errors
// ============================================================================

/// Errors that can occur during operator resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorError {
    /// No matching signature found for the given operand types.
    NoMatchingSignature {
        operator: String,
        operand_types: Vec<DataType>,
    },
    /// Ambiguous resolution - multiple signatures match equally well.
    AmbiguousResolution {
        operator: String,
        operand_types: Vec<DataType>,
        candidates: Vec<OperatorSignature>,
    },
    /// Invalid operand count for operator.
    InvalidOperandCount {
        operator: String,
        expected: usize,
        found: usize,
    },
    /// Unsupported operator.
    UnsupportedOperator { operator: String },
}

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorError::NoMatchingSignature {
                operator,
                operand_types,
            } => {
                let types: Vec<_> = operand_types.iter().map(|t| format!("{t:?}")).collect();
                write!(
                    f,
                    "No matching signature for operator '{operator}' with operand types: {}",
                    types.join(", ")
                )
            }
            OperatorError::AmbiguousResolution {
                operator,
                operand_types,
                ..
            } => {
                let types: Vec<_> = operand_types.iter().map(|t| format!("{t:?}")).collect();
                write!(
                    f,
                    "Ambiguous resolution for operator '{operator}' with operand types: {}",
                    types.join(", ")
                )
            }
            OperatorError::InvalidOperandCount {
                operator,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Invalid operand count for '{operator}': expected {expected}, found {found}"
                )
            }
            OperatorError::UnsupportedOperator { operator } => {
                write!(f, "Unsupported operator: '{operator}'")
            }
        }
    }
}

impl std::error::Error for OperatorError {}

/// Result type for operator operations.
pub type OperatorResult<T> = Result<T, OperatorError>;

// ============================================================================
// Operator Signature
// ============================================================================

/// The kind of operator (unary, binary, ternary, n-ary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    Unary,
    Binary,
    Ternary,
    Nary,
}

/// A signature for an operator overload.
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorSignature {
    /// The name of the operator (e.g., "Add", "Subtract").
    pub name: String,
    /// The kind of operator.
    pub kind: OperatorKind,
    /// The operand types (in order).
    pub operand_types: Vec<DataType>,
    /// The result type.
    pub result_type: DataType,
    /// Whether this signature uses generic type parameter T.
    pub is_generic: bool,
}

impl OperatorSignature {
    /// Create a new operator signature.
    pub fn new(
        name: impl Into<String>,
        kind: OperatorKind,
        operand_types: Vec<DataType>,
        result_type: DataType,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            operand_types,
            result_type,
            is_generic: false,
        }
    }

    /// Create a unary operator signature.
    pub fn unary(name: impl Into<String>, operand: DataType, result: DataType) -> Self {
        Self::new(name, OperatorKind::Unary, vec![operand], result)
    }

    /// Create a binary operator signature.
    pub fn binary(
        name: impl Into<String>,
        left: DataType,
        right: DataType,
        result: DataType,
    ) -> Self {
        Self::new(name, OperatorKind::Binary, vec![left, right], result)
    }

    /// Create a ternary operator signature.
    pub fn ternary(
        name: impl Into<String>,
        first: DataType,
        second: DataType,
        third: DataType,
        result: DataType,
    ) -> Self {
        Self::new(
            name,
            OperatorKind::Ternary,
            vec![first, second, third],
            result,
        )
    }

    /// Mark this signature as generic.
    pub fn generic(mut self) -> Self {
        self.is_generic = true;
        self
    }
}

// ============================================================================
// Resolved Operator
// ============================================================================

/// The result of resolving an operator with specific operand types.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedOperator {
    /// The matched signature.
    pub signature: OperatorSignature,
    /// The result type for this invocation.
    pub result_type: DataType,
    /// Conversions needed for each operand (None if no conversion needed).
    pub conversions: Vec<Option<DataType>>,
}

impl ResolvedOperator {
    /// Check if any conversions are needed.
    pub fn needs_conversion(&self) -> bool {
        self.conversions.iter().any(|c| c.is_some())
    }
}

// ============================================================================
// Operator Resolver
// ============================================================================

/// Resolves CQL operators to their appropriate signatures.
///
/// The resolver maintains a registry of all operator signatures and provides
/// methods to find the best matching signature for given operand types.
pub struct OperatorResolver {
    /// Signatures indexed by operator name.
    signatures: HashMap<String, Vec<OperatorSignature>>,
}

impl Default for OperatorResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OperatorResolver {
    /// Create a new operator resolver with all system operator signatures.
    pub fn new() -> Self {
        let mut resolver = Self {
            signatures: HashMap::new(),
        };
        resolver.register_arithmetic_operators();
        resolver.register_comparison_operators();
        resolver.register_logical_operators();
        resolver.register_string_operators();
        resolver.register_nullological_operators();
        resolver.register_type_operators();
        resolver.register_list_operators();
        resolver.register_interval_operators();
        resolver.register_datetime_operators();
        resolver
    }

    /// Register an operator signature.
    pub fn register(&mut self, signature: OperatorSignature) {
        self.signatures
            .entry(signature.name.clone())
            .or_default()
            .push(signature);
    }

    /// Get all signatures for an operator.
    pub fn get_signatures(&self, name: &str) -> Option<&[OperatorSignature]> {
        self.signatures.get(name).map(|v| v.as_slice())
    }

    /// Resolve a unary operator.
    pub fn resolve_unary(
        &self,
        operator: UnaryOperator,
        operand: &DataType,
    ) -> OperatorResult<ResolvedOperator> {
        let name = unary_operator_name(operator);
        self.resolve_with_operands(&name, &[operand.clone()])
    }

    /// Resolve a binary operator.
    pub fn resolve_binary(
        &self,
        operator: BinaryOperator,
        left: &DataType,
        right: &DataType,
    ) -> OperatorResult<ResolvedOperator> {
        let name = binary_operator_name(operator);
        self.resolve_with_operands(&name, &[left.clone(), right.clone()])
    }

    /// Resolve a ternary operator.
    pub fn resolve_ternary(
        &self,
        operator: TernaryOperator,
        first: &DataType,
        second: &DataType,
        third: &DataType,
    ) -> OperatorResult<ResolvedOperator> {
        let name = ternary_operator_name(operator);
        self.resolve_with_operands(&name, &[first.clone(), second.clone(), third.clone()])
    }

    /// Resolve an operator by name with given operand types.
    pub fn resolve_with_operands(
        &self,
        name: &str,
        operands: &[DataType],
    ) -> OperatorResult<ResolvedOperator> {
        let signatures =
            self.signatures
                .get(name)
                .ok_or_else(|| OperatorError::UnsupportedOperator {
                    operator: name.to_string(),
                })?;

        // Find all matching signatures
        let mut candidates: Vec<(OperatorSignature, Vec<Option<DataType>>, i32)> = Vec::new();

        for sig in signatures {
            if sig.operand_types.len() != operands.len() {
                continue;
            }

            if let Some((conversions, score)) = self.match_signature(sig, operands) {
                candidates.push((sig.clone(), conversions, score));
            }
        }

        if candidates.is_empty() {
            return Err(OperatorError::NoMatchingSignature {
                operator: name.to_string(),
                operand_types: operands.to_vec(),
            });
        }

        // Sort by score (lower is better - fewer conversions)
        candidates.sort_by_key(|(_, _, score)| *score);

        // Check for ambiguity
        if candidates.len() > 1 && candidates[0].2 == candidates[1].2 {
            return Err(OperatorError::AmbiguousResolution {
                operator: name.to_string(),
                operand_types: operands.to_vec(),
                candidates: candidates.iter().map(|(s, _, _)| s.clone()).collect(),
            });
        }

        let (signature, conversions, _) = candidates.remove(0);

        // Determine result type (may need substitution for generics)
        let result_type = if signature.is_generic {
            self.substitute_generic_result(&signature, operands, &conversions)
        } else {
            signature.result_type.clone()
        };

        Ok(ResolvedOperator {
            signature,
            result_type,
            conversions,
        })
    }

    /// Try to match a signature against operand types.
    /// Returns conversions needed and a score (0 = exact match, higher = more conversions).
    fn match_signature(
        &self,
        sig: &OperatorSignature,
        operands: &[DataType],
    ) -> Option<(Vec<Option<DataType>>, i32)> {
        let mut conversions = Vec::with_capacity(operands.len());
        let mut score = 0;

        for (expected, actual) in sig.operand_types.iter().zip(operands.iter()) {
            if expected == actual {
                // Exact match
                conversions.push(None);
            } else if Self::matches_generic(expected, actual) {
                // Generic type match (including parameterized types)
                conversions.push(None);
            } else if Self::can_implicitly_convert(actual, expected) {
                // Implicit conversion needed
                conversions.push(Some(expected.clone()));
                score += 1;
            } else if Self::is_subtype(actual, expected) {
                // Subtype relationship
                conversions.push(None);
            } else {
                // No match
                return None;
            }
        }

        Some((conversions, score))
    }

    /// Check if an expected type (possibly generic) matches an actual type.
    fn matches_generic(expected: &DataType, actual: &DataType) -> bool {
        match (expected, actual) {
            // Direct type parameter match
            (DataType::TypeParameter(_), _) => true,

            // List<T> matches List<Concrete>
            (DataType::List(exp_elem), DataType::List(act_elem)) => {
                Self::matches_generic(exp_elem, act_elem)
            }

            // Interval<T> matches Interval<Concrete>
            (DataType::Interval(exp_point), DataType::Interval(act_point)) => {
                Self::matches_generic(exp_point, act_point)
            }

            // No match
            _ => false,
        }
    }

    /// Check if `from` can be implicitly converted to `to`.
    fn can_implicitly_convert(from: &DataType, to: &DataType) -> bool {
        use SystemType::*;

        match (from, to) {
            // Numeric promotions
            (DataType::System(Integer), DataType::System(Long | Decimal)) => true,
            (DataType::System(Long), DataType::System(Decimal)) => true,

            // Date to DateTime
            (DataType::System(Date), DataType::System(DateTime)) => true,

            // Code to Concept
            (DataType::System(Code), DataType::System(Concept)) => true,

            // List covariance
            (DataType::List(from_elem), DataType::List(to_elem)) => {
                Self::can_implicitly_convert(from_elem, to_elem)
            }

            // Interval covariance
            (DataType::Interval(from_point), DataType::Interval(to_point)) => {
                Self::can_implicitly_convert(from_point, to_point)
            }

            _ => false,
        }
    }

    /// Check if `sub` is a subtype of `super`.
    fn is_subtype(sub: &DataType, sup: &DataType) -> bool {
        // Any is the supertype of everything
        if sup.is_any() {
            return true;
        }

        // Same type
        if sub == sup {
            return true;
        }

        // List subtyping
        if let (DataType::List(sub_elem), DataType::List(sup_elem)) = (sub, sup) {
            return Self::is_subtype(sub_elem, sup_elem);
        }

        false
    }

    /// Substitute type parameters in the result type based on actual operands.
    fn substitute_generic_result(
        &self,
        sig: &OperatorSignature,
        operands: &[DataType],
        _conversions: &[Option<DataType>],
    ) -> DataType {
        // Build a mapping from type parameter names to actual types
        let mut type_bindings: std::collections::HashMap<String, DataType> =
            std::collections::HashMap::new();

        for (expected, actual) in sig.operand_types.iter().zip(operands.iter()) {
            Self::collect_type_bindings(expected, actual, &mut type_bindings);
        }

        // Substitute type parameters in the result type
        Self::substitute_type_params(&sig.result_type, &type_bindings)
    }

    /// Collect type parameter bindings by matching expected type to actual type.
    fn collect_type_bindings(
        expected: &DataType,
        actual: &DataType,
        bindings: &mut std::collections::HashMap<String, DataType>,
    ) {
        match (expected, actual) {
            (DataType::TypeParameter(name), _) => {
                bindings.insert(name.clone(), actual.clone());
            }
            (DataType::List(exp_elem), DataType::List(act_elem)) => {
                Self::collect_type_bindings(exp_elem, act_elem, bindings);
            }
            (DataType::Interval(exp_point), DataType::Interval(act_point)) => {
                Self::collect_type_bindings(exp_point, act_point, bindings);
            }
            _ => {}
        }
    }

    /// Substitute type parameters in a type using the bindings.
    fn substitute_type_params(
        dt: &DataType,
        bindings: &std::collections::HashMap<String, DataType>,
    ) -> DataType {
        match dt {
            DataType::TypeParameter(name) => {
                bindings.get(name).cloned().unwrap_or_else(|| dt.clone())
            }
            DataType::List(elem) => {
                DataType::List(Box::new(Self::substitute_type_params(elem, bindings)))
            }
            DataType::Interval(point) => {
                DataType::Interval(Box::new(Self::substitute_type_params(point, bindings)))
            }
            _ => dt.clone(),
        }
    }

    // =========================================================================
    // Operator Registration
    // =========================================================================

    fn register_arithmetic_operators(&mut self) {
        // Add: T, T -> T for numeric types
        for sys in [SystemType::Integer, SystemType::Long, SystemType::Decimal] {
            let t = DataType::System(sys);
            self.register(OperatorSignature::binary(
                "Add",
                t.clone(),
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Subtract",
                t.clone(),
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Multiply",
                t.clone(),
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Divide",
                t.clone(),
                t.clone(),
                DataType::decimal(),
            ));
            self.register(OperatorSignature::binary(
                "TruncatedDivide",
                t.clone(),
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Modulo",
                t.clone(),
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::unary("Negate", t.clone(), t.clone()));
            self.register(OperatorSignature::unary("Abs", t.clone(), t.clone()));
        }

        // Quantity arithmetic
        let q = DataType::quantity();
        self.register(OperatorSignature::binary(
            "Add",
            q.clone(),
            q.clone(),
            q.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Subtract",
            q.clone(),
            q.clone(),
            q.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Multiply",
            q.clone(),
            q.clone(),
            q.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Divide",
            q.clone(),
            q.clone(),
            q.clone(),
        ));
        self.register(OperatorSignature::unary("Negate", q.clone(), q.clone()));
        self.register(OperatorSignature::unary("Abs", q.clone(), q.clone()));

        // String concatenation
        let s = DataType::string();
        self.register(OperatorSignature::binary(
            "Add",
            s.clone(),
            s.clone(),
            s.clone(),
        ));

        // Decimal-only operations
        let d = DataType::decimal();
        self.register(OperatorSignature::binary(
            "Power",
            d.clone(),
            d.clone(),
            d.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Log",
            d.clone(),
            d.clone(),
            d.clone(),
        ));
        self.register(OperatorSignature::unary("Ln", d.clone(), d.clone()));
        self.register(OperatorSignature::unary("Exp", d.clone(), d.clone()));
        self.register(OperatorSignature::unary(
            "Ceiling",
            d.clone(),
            DataType::integer(),
        ));
        self.register(OperatorSignature::unary(
            "Floor",
            d.clone(),
            DataType::integer(),
        ));
        self.register(OperatorSignature::unary(
            "Truncate",
            d.clone(),
            DataType::integer(),
        ));
        self.register(OperatorSignature::unary("Round", d.clone(), d.clone()));

        // Predecessor/Successor for ordered types
        for sys in [
            SystemType::Integer,
            SystemType::Long,
            SystemType::Decimal,
            SystemType::Date,
            SystemType::DateTime,
            SystemType::Time,
            SystemType::Quantity,
        ] {
            let t = DataType::System(sys);
            self.register(OperatorSignature::unary(
                "Predecessor",
                t.clone(),
                t.clone(),
            ));
            self.register(OperatorSignature::unary("Successor", t.clone(), t.clone()));
        }
    }

    fn register_comparison_operators(&mut self) {
        // All types support equality/equivalence (using generic T)
        let t = DataType::type_parameter("T");
        self.register(
            OperatorSignature::binary("Equal", t.clone(), t.clone(), DataType::boolean()).generic(),
        );
        self.register(
            OperatorSignature::binary("Equivalent", t.clone(), t.clone(), DataType::boolean())
                .generic(),
        );

        // Ordered types support less/greater
        for sys in [
            SystemType::Integer,
            SystemType::Long,
            SystemType::Decimal,
            SystemType::String,
            SystemType::Date,
            SystemType::DateTime,
            SystemType::Time,
            SystemType::Quantity,
        ] {
            let t = DataType::System(sys);
            self.register(OperatorSignature::binary(
                "Less",
                t.clone(),
                t.clone(),
                DataType::boolean(),
            ));
            self.register(OperatorSignature::binary(
                "Greater",
                t.clone(),
                t.clone(),
                DataType::boolean(),
            ));
            self.register(OperatorSignature::binary(
                "LessOrEqual",
                t.clone(),
                t.clone(),
                DataType::boolean(),
            ));
            self.register(OperatorSignature::binary(
                "GreaterOrEqual",
                t.clone(),
                t.clone(),
                DataType::boolean(),
            ));

            // Between: T between T and T -> Boolean
            self.register(OperatorSignature::ternary(
                "Between",
                t.clone(),
                t.clone(),
                t.clone(),
                DataType::boolean(),
            ));
        }
    }

    fn register_logical_operators(&mut self) {
        let b = DataType::boolean();
        self.register(OperatorSignature::binary(
            "And",
            b.clone(),
            b.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Or",
            b.clone(),
            b.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Xor",
            b.clone(),
            b.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Implies",
            b.clone(),
            b.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::unary("Not", b.clone(), b.clone()));
    }

    fn register_string_operators(&mut self) {
        let s = DataType::string();
        let i = DataType::integer();
        let b = DataType::boolean();

        self.register(OperatorSignature::binary(
            "Concatenate",
            s.clone(),
            s.clone(),
            s.clone(),
        ));
        self.register(OperatorSignature::unary("Length", s.clone(), i.clone()));
        self.register(OperatorSignature::unary("Upper", s.clone(), s.clone()));
        self.register(OperatorSignature::unary("Lower", s.clone(), s.clone()));
        self.register(OperatorSignature::binary(
            "StartsWith",
            s.clone(),
            s.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::binary(
            "EndsWith",
            s.clone(),
            s.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Matches",
            s.clone(),
            s.clone(),
            b.clone(),
        ));
        self.register(OperatorSignature::ternary(
            "ReplaceMatches",
            s.clone(),
            s.clone(),
            s.clone(),
            s.clone(),
        ));
        self.register(OperatorSignature::binary(
            "IndexOf",
            s.clone(),
            s.clone(),
            i.clone(),
        ));
    }

    fn register_nullological_operators(&mut self) {
        let t = DataType::type_parameter("T");
        let b = DataType::boolean();

        self.register(OperatorSignature::unary("IsNull", t.clone(), b.clone()).generic());
        self.register(OperatorSignature::unary("IsTrue", b.clone(), b.clone()));
        self.register(OperatorSignature::unary("IsFalse", b.clone(), b.clone()));
    }

    fn register_type_operators(&mut self) {
        // Conversion operators - these return the target type
        // The actual type resolution happens during expression translation

        // Boolean conversions
        let b = DataType::boolean();
        self.register(OperatorSignature::unary(
            "ToBoolean",
            DataType::string(),
            b.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToBoolean",
            DataType::integer(),
            b.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToBoolean",
            DataType::decimal(),
            b.clone(),
        ));

        // Integer conversions
        let i = DataType::integer();
        self.register(OperatorSignature::unary(
            "ToInteger",
            DataType::string(),
            i.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToInteger",
            DataType::boolean(),
            i.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToInteger",
            DataType::long(),
            i.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToInteger",
            DataType::decimal(),
            i.clone(),
        ));

        // Long conversions
        let l = DataType::long();
        self.register(OperatorSignature::unary(
            "ToLong",
            DataType::string(),
            l.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToLong",
            DataType::integer(),
            l.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToLong",
            DataType::decimal(),
            l.clone(),
        ));

        // Decimal conversions
        let d = DataType::decimal();
        self.register(OperatorSignature::unary(
            "ToDecimal",
            DataType::string(),
            d.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToDecimal",
            DataType::integer(),
            d.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToDecimal",
            DataType::long(),
            d.clone(),
        ));
        self.register(OperatorSignature::unary(
            "ToDecimal",
            DataType::boolean(),
            d.clone(),
        ));

        // String conversions (almost any type to string)
        let s = DataType::string();
        for sys in [
            SystemType::Boolean,
            SystemType::Integer,
            SystemType::Long,
            SystemType::Decimal,
            SystemType::Date,
            SystemType::DateTime,
            SystemType::Time,
            SystemType::Quantity,
        ] {
            self.register(OperatorSignature::unary(
                "ToString",
                DataType::System(sys),
                s.clone(),
            ));
        }

        // DateTime/Date/Time conversions
        self.register(OperatorSignature::unary(
            "ToDateTime",
            DataType::string(),
            DataType::date_time(),
        ));
        self.register(OperatorSignature::unary(
            "ToDateTime",
            DataType::date(),
            DataType::date_time(),
        ));
        self.register(OperatorSignature::unary(
            "ToDate",
            DataType::string(),
            DataType::date(),
        ));
        self.register(OperatorSignature::unary(
            "ToDate",
            DataType::date_time(),
            DataType::date(),
        ));
        self.register(OperatorSignature::unary(
            "ToTime",
            DataType::string(),
            DataType::time(),
        ));

        // Quantity conversion
        self.register(OperatorSignature::unary(
            "ToQuantity",
            DataType::string(),
            DataType::quantity(),
        ));
        self.register(OperatorSignature::unary(
            "ToQuantity",
            DataType::integer(),
            DataType::quantity(),
        ));
        self.register(OperatorSignature::unary(
            "ToQuantity",
            DataType::decimal(),
            DataType::quantity(),
        ));

        // ToConcept
        self.register(OperatorSignature::unary(
            "ToConcept",
            DataType::code(),
            DataType::concept(),
        ));
    }

    fn register_list_operators(&mut self) {
        let t = DataType::type_parameter("T");
        let b = DataType::boolean();
        let i = DataType::integer();
        let list_t = DataType::list(t.clone());

        // Existence
        self.register(OperatorSignature::unary("Exists", list_t.clone(), b.clone()).generic());

        // Element access
        self.register(OperatorSignature::unary("First", list_t.clone(), t.clone()).generic());
        self.register(OperatorSignature::unary("Last", list_t.clone(), t.clone()).generic());
        self.register(
            OperatorSignature::binary("Indexer", list_t.clone(), i.clone(), t.clone()).generic(),
        );
        self.register(
            OperatorSignature::unary("SingletonFrom", list_t.clone(), t.clone()).generic(),
        );

        // List operations
        self.register(
            OperatorSignature::unary("Distinct", list_t.clone(), list_t.clone()).generic(),
        );
        self.register(
            OperatorSignature::unary("Flatten", list_t.clone(), list_t.clone()).generic(),
        );

        // Set operations
        self.register(
            OperatorSignature::binary("Union", list_t.clone(), list_t.clone(), list_t.clone())
                .generic(),
        );
        self.register(
            OperatorSignature::binary("Intersect", list_t.clone(), list_t.clone(), list_t.clone())
                .generic(),
        );
        self.register(
            OperatorSignature::binary("Except", list_t.clone(), list_t.clone(), list_t.clone())
                .generic(),
        );

        // Membership
        self.register(
            OperatorSignature::binary("In", t.clone(), list_t.clone(), b.clone()).generic(),
        );
        self.register(
            OperatorSignature::binary("Contains", list_t.clone(), t.clone(), b.clone()).generic(),
        );
        self.register(
            OperatorSignature::binary("Includes", list_t.clone(), list_t.clone(), b.clone())
                .generic(),
        );
        self.register(
            OperatorSignature::binary("IncludedIn", list_t.clone(), list_t.clone(), b.clone())
                .generic(),
        );
        self.register(
            OperatorSignature::binary(
                "ProperlyIncludes",
                list_t.clone(),
                list_t.clone(),
                b.clone(),
            )
            .generic(),
        );
        self.register(
            OperatorSignature::binary(
                "ProperlyIncludedIn",
                list_t.clone(),
                list_t.clone(),
                b.clone(),
            )
            .generic(),
        );

        // IndexOf for list
        self.register(
            OperatorSignature::binary("IndexOf", list_t.clone(), t.clone(), i.clone()).generic(),
        );
    }

    fn register_interval_operators(&mut self) {
        // Register interval operators for ordered point types
        for sys in [
            SystemType::Integer,
            SystemType::Long,
            SystemType::Decimal,
            SystemType::Date,
            SystemType::DateTime,
            SystemType::Time,
            SystemType::Quantity,
        ] {
            let point_t = DataType::System(sys);
            let interval_t = DataType::interval(point_t.clone());
            let b = DataType::boolean();

            // Interval accessors
            self.register(OperatorSignature::unary(
                "Start",
                interval_t.clone(),
                point_t.clone(),
            ));
            self.register(OperatorSignature::unary(
                "End",
                interval_t.clone(),
                point_t.clone(),
            ));
            self.register(OperatorSignature::unary(
                "Width",
                interval_t.clone(),
                point_t.clone(),
            ));
            self.register(OperatorSignature::unary(
                "PointFrom",
                interval_t.clone(),
                point_t.clone(),
            ));

            // Point in interval
            self.register(OperatorSignature::binary(
                "In",
                point_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Contains",
                interval_t.clone(),
                point_t.clone(),
                b.clone(),
            ));

            // Interval relationships
            self.register(OperatorSignature::binary(
                "Includes",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "IncludedIn",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "ProperlyIncludes",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "ProperlyIncludedIn",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Overlaps",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "OverlapsBefore",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "OverlapsAfter",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Meets",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "MeetsBefore",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "MeetsAfter",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Starts",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Ends",
                interval_t.clone(),
                interval_t.clone(),
                b.clone(),
            ));

            // Interval set operations
            self.register(OperatorSignature::binary(
                "Union",
                interval_t.clone(),
                interval_t.clone(),
                interval_t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Intersect",
                interval_t.clone(),
                interval_t.clone(),
                interval_t.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Except",
                interval_t.clone(),
                interval_t.clone(),
                interval_t.clone(),
            ));

            // Collapse
            let list_interval = DataType::list(interval_t.clone());
            self.register(OperatorSignature::unary(
                "Collapse",
                list_interval.clone(),
                list_interval.clone(),
            ));
        }
    }

    fn register_datetime_operators(&mut self) {
        // Date/Time comparison with precision
        for sys in [SystemType::Date, SystemType::DateTime, SystemType::Time] {
            let t = DataType::System(sys);
            let b = DataType::boolean();

            self.register(OperatorSignature::binary(
                "SameAs",
                t.clone(),
                t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "SameOrBefore",
                t.clone(),
                t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "SameOrAfter",
                t.clone(),
                t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "Before",
                t.clone(),
                t.clone(),
                b.clone(),
            ));
            self.register(OperatorSignature::binary(
                "After",
                t.clone(),
                t.clone(),
                b.clone(),
            ));
        }

        // DateTime arithmetic (add/subtract quantities)
        let dt = DataType::date_time();
        let d = DataType::date();
        let q = DataType::quantity();

        self.register(OperatorSignature::binary(
            "Add",
            dt.clone(),
            q.clone(),
            dt.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Subtract",
            dt.clone(),
            q.clone(),
            dt.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Add",
            d.clone(),
            q.clone(),
            d.clone(),
        ));
        self.register(OperatorSignature::binary(
            "Subtract",
            d.clone(),
            q.clone(),
            d.clone(),
        ));

        // Duration between
        self.register(OperatorSignature::binary(
            "DurationBetween",
            dt.clone(),
            dt.clone(),
            DataType::integer(),
        ));
        self.register(OperatorSignature::binary(
            "DurationBetween",
            d.clone(),
            d.clone(),
            DataType::integer(),
        ));
    }
}

// ============================================================================
// Operator Name Mapping
// ============================================================================

/// Get the ELM operator name for a unary operator.
pub fn unary_operator_name(op: UnaryOperator) -> String {
    match op {
        UnaryOperator::Not => "Not",
        UnaryOperator::Negate => "Negate",
        UnaryOperator::Exists => "Exists",
        UnaryOperator::IsNull => "IsNull",
        UnaryOperator::IsTrue => "IsTrue",
        UnaryOperator::IsFalse => "IsFalse",
        UnaryOperator::Predecessor => "Predecessor",
        UnaryOperator::Successor => "Successor",
        UnaryOperator::Distinct => "Distinct",
        UnaryOperator::Flatten => "Flatten",
        UnaryOperator::Start => "Start",
        UnaryOperator::End => "End",
        UnaryOperator::Width => "Width",
        UnaryOperator::PointFrom => "PointFrom",
        UnaryOperator::Collapse => "Collapse",
        UnaryOperator::Expand => "Expand",
        UnaryOperator::Singleton => "SingletonFrom",
        UnaryOperator::DateFrom => "DateFrom",
        UnaryOperator::TimeFrom => "TimeFrom",
        UnaryOperator::TimezoneOffsetFrom => "TimezoneOffsetFrom",
        UnaryOperator::ToBoolean => "ToBoolean",
        UnaryOperator::ToInteger => "ToInteger",
        UnaryOperator::ToLong => "ToLong",
        UnaryOperator::ToDecimal => "ToDecimal",
        UnaryOperator::ToString => "ToString",
        UnaryOperator::ToDate => "ToDate",
        UnaryOperator::ToDateTime => "ToDateTime",
        UnaryOperator::ToTime => "ToTime",
        UnaryOperator::ToQuantity => "ToQuantity",
        UnaryOperator::ToConcept => "ToConcept",
    }
    .to_string()
}

/// Get the ELM operator name for a binary operator.
pub fn binary_operator_name(op: BinaryOperator) -> String {
    match op {
        BinaryOperator::Add => "Add",
        BinaryOperator::Subtract => "Subtract",
        BinaryOperator::Multiply => "Multiply",
        BinaryOperator::Divide => "Divide",
        BinaryOperator::TruncatedDivide => "TruncatedDivide",
        BinaryOperator::Modulo => "Modulo",
        BinaryOperator::Power => "Power",
        BinaryOperator::Log => "Log",
        BinaryOperator::Equal => "Equal",
        BinaryOperator::NotEqual => "NotEqual",
        BinaryOperator::Equivalent => "Equivalent",
        BinaryOperator::NotEquivalent => "NotEquivalent",
        BinaryOperator::Less => "Less",
        BinaryOperator::LessOrEqual => "LessOrEqual",
        BinaryOperator::Greater => "Greater",
        BinaryOperator::GreaterOrEqual => "GreaterOrEqual",
        BinaryOperator::And => "And",
        BinaryOperator::Or => "Or",
        BinaryOperator::Xor => "Xor",
        BinaryOperator::Implies => "Implies",
        BinaryOperator::Concatenate => "Concatenate",
        BinaryOperator::In => "In",
        BinaryOperator::Contains => "Contains",
        BinaryOperator::Includes => "Includes",
        BinaryOperator::IncludedIn => "IncludedIn",
        BinaryOperator::ProperlyIncludes => "ProperlyIncludes",
        BinaryOperator::ProperlyIncludedIn => "ProperlyIncludedIn",
        BinaryOperator::Overlaps => "Overlaps",
        BinaryOperator::OverlapsBefore => "OverlapsBefore",
        BinaryOperator::OverlapsAfter => "OverlapsAfter",
        BinaryOperator::Meets => "Meets",
        BinaryOperator::MeetsBefore => "MeetsBefore",
        BinaryOperator::MeetsAfter => "MeetsAfter",
        BinaryOperator::Starts => "Starts",
        BinaryOperator::Ends => "Ends",
        BinaryOperator::During => "During",
        BinaryOperator::Before => "Before",
        BinaryOperator::After => "After",
        BinaryOperator::SameAs => "SameAs",
        BinaryOperator::SameOrBefore => "SameOrBefore",
        BinaryOperator::SameOrAfter => "SameOrAfter",
        BinaryOperator::Within => "Within",
        BinaryOperator::Union => "Union",
        BinaryOperator::Intersect => "Intersect",
        BinaryOperator::Except => "Except",
        BinaryOperator::IndexOf => "IndexOf",
    }
    .to_string()
}

/// Get the ELM operator name for a ternary operator.
pub fn ternary_operator_name(op: TernaryOperator) -> String {
    match op {
        TernaryOperator::Between => "Between",
        TernaryOperator::ReplaceMatches => "ReplaceMatches",
    }
    .to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_add_integers() {
        let resolver = OperatorResolver::new();
        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::integer(),
            &DataType::integer(),
        );
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.result_type, DataType::integer());
        assert!(!resolved.needs_conversion());
    }

    #[test]
    fn test_resolve_add_decimals() {
        let resolver = OperatorResolver::new();
        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::decimal(),
            &DataType::decimal(),
        );
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.result_type, DataType::decimal());
    }

    #[test]
    fn test_resolve_add_integer_decimal_promotion() {
        let resolver = OperatorResolver::new();
        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::integer(),
            &DataType::decimal(),
        );
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.result_type, DataType::decimal());
        assert!(resolved.needs_conversion());
    }

    #[test]
    fn test_resolve_add_strings() {
        let resolver = OperatorResolver::new();
        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::string(),
            &DataType::string(),
        );
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.result_type, DataType::string());
    }

    #[test]
    fn test_resolve_divide_returns_decimal() {
        let resolver = OperatorResolver::new();
        let result = resolver.resolve_binary(
            BinaryOperator::Divide,
            &DataType::integer(),
            &DataType::integer(),
        );
        assert!(result.is_ok());
        let resolved = result.unwrap();
        // Division always returns Decimal in CQL
        assert_eq!(resolved.result_type, DataType::decimal());
    }

    #[test]
    fn test_resolve_comparison_operators() {
        let resolver = OperatorResolver::new();

        // Equal works on any type
        let result = resolver.resolve_binary(
            BinaryOperator::Equal,
            &DataType::string(),
            &DataType::string(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());

        // Less works on ordered types
        let result = resolver.resolve_binary(
            BinaryOperator::Less,
            &DataType::integer(),
            &DataType::integer(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }

    #[test]
    fn test_resolve_logical_operators() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_binary(
            BinaryOperator::And,
            &DataType::boolean(),
            &DataType::boolean(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());

        let result = resolver.resolve_unary(UnaryOperator::Not, &DataType::boolean());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }

    #[test]
    fn test_resolve_string_operators() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_binary(
            BinaryOperator::Concatenate,
            &DataType::string(),
            &DataType::string(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::string());
    }

    #[test]
    fn test_resolve_unary_operators() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_unary(UnaryOperator::Negate, &DataType::integer());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::integer());

        let result = resolver.resolve_unary(UnaryOperator::IsNull, &DataType::string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }

    #[test]
    fn test_resolve_type_conversion() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_unary(UnaryOperator::ToString, &DataType::integer());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::string());

        let result = resolver.resolve_unary(UnaryOperator::ToInteger, &DataType::string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::integer());
    }

    #[test]
    fn test_resolve_list_operators() {
        let resolver = OperatorResolver::new();
        let list_int = DataType::list(DataType::integer());

        let result = resolver.resolve_unary(UnaryOperator::Exists, &list_int);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());

        let result = resolver.resolve_binary(BinaryOperator::Union, &list_int, &list_int);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_interval_operators() {
        let resolver = OperatorResolver::new();
        let interval_int = DataType::interval(DataType::integer());

        let result = resolver.resolve_unary(UnaryOperator::Start, &interval_int);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::integer());

        let result =
            resolver.resolve_binary(BinaryOperator::Overlaps, &interval_int, &interval_int);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }

    #[test]
    fn test_resolve_datetime_arithmetic() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::date_time(),
            &DataType::quantity(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::date_time());
    }

    #[test]
    fn test_no_matching_signature_error() {
        let resolver = OperatorResolver::new();

        // Can't add boolean and integer
        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::boolean(),
            &DataType::integer(),
        );
        assert!(matches!(
            result,
            Err(OperatorError::NoMatchingSignature { .. })
        ));
    }

    #[test]
    fn test_operator_error_display() {
        let err = OperatorError::NoMatchingSignature {
            operator: "Add".to_string(),
            operand_types: vec![DataType::boolean(), DataType::integer()],
        };
        let msg = err.to_string();
        assert!(msg.contains("Add"));
        assert!(msg.contains("No matching signature"));
    }

    #[test]
    fn test_ternary_operator_resolution() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_ternary(
            TernaryOperator::ReplaceMatches,
            &DataType::string(),
            &DataType::string(),
            &DataType::string(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::string());
    }

    #[test]
    fn test_integer_to_long_promotion() {
        let resolver = OperatorResolver::new();

        // Integer + Long should promote Integer to Long
        let result =
            resolver.resolve_binary(BinaryOperator::Add, &DataType::integer(), &DataType::long());
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.result_type, DataType::long());
    }

    #[test]
    fn test_quantity_arithmetic() {
        let resolver = OperatorResolver::new();

        let result = resolver.resolve_binary(
            BinaryOperator::Add,
            &DataType::quantity(),
            &DataType::quantity(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::quantity());
    }

    #[test]
    fn test_membership_operators() {
        let resolver = OperatorResolver::new();
        let list_int = DataType::list(DataType::integer());

        // Element in list
        let result = resolver.resolve_binary(BinaryOperator::In, &DataType::integer(), &list_int);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());

        // List contains element
        let result =
            resolver.resolve_binary(BinaryOperator::Contains, &list_int, &DataType::integer());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }

    #[test]
    fn test_point_in_interval() {
        let resolver = OperatorResolver::new();
        let interval_int = DataType::interval(DataType::integer());

        // Point in interval
        let result =
            resolver.resolve_binary(BinaryOperator::In, &DataType::integer(), &interval_int);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result_type, DataType::boolean());
    }
}
