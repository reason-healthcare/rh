//! CQL Expression Parser
//!
//! Parses CQL expressions following operator precedence rules.
//! Based on CQL v1.5.3 grammar specification.
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. `or`, `xor`
//! 2. `and`
//! 3. `implies`
//! 4. `=`, `!=`, `~`, `!~`, `in`, `contains`
//! 5. `<`, `>`, `<=`, `>=`
//! 6. `|` (union)
//! 7. `+`, `-`
//! 8. `*`, `/`, `div`, `mod`
//! 9. Unary: `not`, `-`, `exists`, etc.
//! 10. Invocation: `.`, `[]`, `()`

use super::ast::*;
use super::span::Span;
use nom::IResult;

mod literals;
mod precedence;
mod query;
mod retrieve;
mod selectors;

/// Parse any CQL expression.
///
/// Entry point for the full CQL expression grammar.
pub fn expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    precedence::parse_or_expression(input)
}

/// Parse a type specifier (used by the statement parser for type annotations
/// and by retrieve expressions).
pub use retrieve::parse_type_specifier;

#[cfg(test)]
mod tests {
    use super::*;

    fn span(s: &str) -> Span<'_> {
        Span::new(s)
    }

    fn parse_expr(s: &str) -> Expression {
        expression(span(s)).unwrap().1
    }

    // ========================================================================
    // Literal Tests
    // ========================================================================

    #[test]
    fn test_null_literal() {
        let expr = parse_expr("null");
        assert!(matches!(expr, Expression::Literal(Literal::Null)));
    }

    #[test]
    fn test_boolean_literals() {
        let expr = parse_expr("true");
        assert!(matches!(expr, Expression::Literal(Literal::Boolean(true))));

        let expr = parse_expr("false");
        assert!(matches!(expr, Expression::Literal(Literal::Boolean(false))));
    }

    #[test]
    fn test_integer_literal() {
        let expr = parse_expr("42");
        assert!(matches!(expr, Expression::Literal(Literal::Integer(42))));
    }

    #[test]
    fn test_decimal_literal() {
        if let Expression::Literal(Literal::Decimal(n)) = parse_expr("3.25") {
            assert!((n - 3.25).abs() < 0.001);
        } else {
            panic!("Expected decimal literal");
        }
    }

    #[test]
    fn test_string_literal() {
        let expr = parse_expr("'hello'");
        assert!(matches!(expr, Expression::Literal(Literal::String(s)) if s == "hello"));
    }

    // ========================================================================
    // Operator Tests
    // ========================================================================

    #[test]
    fn test_binary_arithmetic() {
        let expr = parse_expr("1 + 2");
        assert!(matches!(
            expr,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::Add,
                ..
            })
        ));
    }

    #[test]
    fn test_binary_comparison() {
        let expr = parse_expr("x > 5");
        assert!(matches!(
            expr,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::Greater,
                ..
            })
        ));
    }

    #[test]
    fn test_binary_logical() {
        let expr = parse_expr("a and b");
        assert!(matches!(
            expr,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                ..
            })
        ));
    }

    #[test]
    fn test_operator_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let expr = parse_expr("1 + 2 * 3");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Add);
            if let Expression::BinaryExpression(right) = *bin.right {
                assert_eq!(right.operator, BinaryOperator::Multiply);
            } else {
                panic!("Expected multiplication on right");
            }
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_logical_precedence() {
        // a or b and c should parse as a or (b and c)
        let expr = parse_expr("a or b and c");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Or);
        } else {
            panic!("Expected binary expression");
        }
    }

    // ========================================================================
    // Unary Expression Tests
    // ========================================================================

    #[test]
    fn test_not_expression() {
        let expr = parse_expr("not true");
        assert!(matches!(
            expr,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::Not,
                ..
            })
        ));
    }

    #[test]
    fn test_exists_expression() {
        let expr = parse_expr("exists items");
        assert!(matches!(
            expr,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::Exists,
                ..
            })
        ));
    }

    // ========================================================================
    // Function and Identifier Tests
    // ========================================================================

    #[test]
    fn test_identifier_reference() {
        let expr = parse_expr("MyIdentifier");
        assert!(matches!(
            expr,
            Expression::IdentifierRef(IdentifierRef { name, .. }) if name == "MyIdentifier"
        ));
    }

    #[test]
    fn test_function_invocation() {
        let expr = parse_expr("Sum(values)");
        if let Expression::FunctionInvocation(func) = expr {
            assert_eq!(func.name, "Sum");
            assert_eq!(func.arguments.len(), 1);
        } else {
            panic!("Expected function invocation");
        }
    }

    #[test]
    fn test_qualified_function() {
        let expr = parse_expr("FHIRHelpers.ToQuantity(x)");
        if let Expression::FunctionInvocation(func) = expr {
            assert_eq!(func.library, Some("FHIRHelpers".to_string()));
            assert_eq!(func.name, "ToQuantity");
        } else {
            panic!("Expected function invocation");
        }
    }

    // ========================================================================
    // Conditional Tests
    // ========================================================================

    #[test]
    fn test_if_then_else() {
        let expr = parse_expr("if x > 0 then 'positive' else 'non-positive'");
        assert!(matches!(expr, Expression::IfThenElse(_)));
    }

    // ========================================================================
    // Collection Tests
    // ========================================================================

    #[test]
    fn test_list_literal() {
        let expr = parse_expr("{ 1, 2, 3 }");
        if let Expression::ListExpression(list) = expr {
            assert_eq!(list.elements.len(), 3);
        } else {
            panic!("Expected list expression");
        }
    }

    // ========================================================================
    // Type Expression Tests
    // ========================================================================

    #[test]
    fn test_is_expression() {
        let expr = parse_expr("x is Integer");
        assert!(matches!(
            expr,
            Expression::TypeExpression(TypeExpression {
                operator: TypeOperator::Is,
                ..
            })
        ));
    }

    #[test]
    fn test_as_expression() {
        let expr = parse_expr("x as FHIR.Patient");
        assert!(matches!(
            expr,
            Expression::TypeExpression(TypeExpression {
                operator: TypeOperator::As,
                ..
            })
        ));
    }

    // ========================================================================
    // Member Access Tests
    // ========================================================================

    #[test]
    fn test_member_access() {
        let expr = parse_expr("Patient.name");
        assert!(matches!(expr, Expression::MemberInvocation(_)));
    }

    #[test]
    fn test_index_access() {
        let expr = parse_expr("items[0]");
        assert!(matches!(expr, Expression::IndexInvocation(_)));
    }

    #[test]
    fn test_special_identifiers() {
        let expr = parse_expr("$this");
        match expr {
            Expression::IdentifierRef(ref id) => assert_eq!(id.name, "$this"),
            _ => panic!("Expected IdentifierRef for $this, got {expr:?}"),
        }

        let expr = parse_expr("$index");
        match expr {
            Expression::IdentifierRef(ref id) => assert_eq!(id.name, "$index"),
            _ => panic!("Expected IdentifierRef for $index, got {expr:?}"),
        }
    }

    #[test]
    fn test_sort_by_this() {
        // Test `$this * $this` expression
        let expr = parse_expr("$this * $this");
        assert!(
            matches!(expr, Expression::BinaryExpression(_)),
            "Expected BinaryExpression, got {expr:?}"
        );

        // Test full query with sort by $this
        let expr = parse_expr("({ 1, 2, 3, 4, 5 }) X sort by $this * $this");
        assert!(
            matches!(expr, Expression::Query(_)),
            "Expected Query, got {expr:?}"
        );
    }

    #[test]
    fn test_from_query() {
        // Test simple from query
        let expr = parse_expr("from X Y where Y = 1");
        assert!(
            matches!(expr, Expression::Query(_)),
            "Expected Query, got {expr:?}"
        );

        // Test multi-source from query
        let expr = parse_expr("from X A, Y B where A = B");
        assert!(
            matches!(expr, Expression::Query(_)),
            "Expected Query, got {expr:?}"
        );
    }

    #[test]
    fn test_includes_with_start() {
        // Test "A includes start B" - boundary without precision
        let expr = parse_expr("A includes start B");
        if let Expression::BinaryExpression(BinaryExpression {
            operator,
            right,
            precision,
            ..
        }) = expr
        {
            assert_eq!(operator, BinaryOperator::Includes);
            assert!(precision.is_none());
            // right should be Start(B)
            assert!(
                matches!(
                    *right,
                    Expression::UnaryExpression(UnaryExpression {
                        operator: UnaryOperator::Start,
                        ..
                    })
                ),
                "Expected Start(B), got {right:?}"
            );
        } else {
            panic!("Expected BinaryExpression with Includes");
        }
    }

    #[test]
    fn test_includes_with_precision_and_start() {
        // Test "A includes day of start B" - precision + boundary
        let expr = parse_expr("A includes day of start B");
        if let Expression::BinaryExpression(BinaryExpression {
            operator,
            right,
            precision,
            ..
        }) = expr
        {
            assert_eq!(operator, BinaryOperator::Includes);
            assert_eq!(precision, Some(DateTimePrecision::Day));
            // right should be Start(B)
            assert!(
                matches!(
                    *right,
                    Expression::UnaryExpression(UnaryExpression {
                        operator: UnaryOperator::Start,
                        ..
                    })
                ),
                "Expected Start(B), got {right:?}"
            );
        } else {
            panic!("Expected BinaryExpression with Includes");
        }
    }

    #[test]
    fn test_properly_includes_with_end() {
        // Test "A properly includes end B"
        let expr = parse_expr("A properly includes end B");
        if let Expression::BinaryExpression(BinaryExpression {
            operator,
            right,
            precision,
            ..
        }) = expr
        {
            assert_eq!(operator, BinaryOperator::ProperlyIncludes);
            assert!(precision.is_none());
            // right should be End(B)
            assert!(
                matches!(
                    *right,
                    Expression::UnaryExpression(UnaryExpression {
                        operator: UnaryOperator::End,
                        ..
                    })
                ),
                "Expected End(B), got {right:?}"
            );
        } else {
            panic!("Expected BinaryExpression with ProperlyIncludes");
        }
    }

    // ========================================================================
    // Section 6.9 Parser Conformance Tests
    // ========================================================================

    #[test]
    fn test_predecessor_of() {
        let expr = parse_expr("predecessor of 5");
        assert!(matches!(
            expr,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::Predecessor,
                ..
            })
        ));
    }

    #[test]
    fn test_successor_of() {
        let expr = parse_expr("successor of x");
        assert!(matches!(
            expr,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::Successor,
                ..
            })
        ));
    }

    #[test]
    fn test_power_operator() {
        let expr = parse_expr("2 ^ 10");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Power);
        } else {
            panic!("Expected BinaryExpression with Power operator");
        }
    }

    #[test]
    fn test_minimum_type_specifier() {
        let expr = parse_expr("minimum Integer");
        if let Expression::MinValue(ts) = expr {
            assert!(matches!(ts, TypeSpecifier::Named(_)));
        } else {
            panic!("Expected MinValue expression, got: {expr:?}");
        }
    }

    #[test]
    fn test_maximum_type_specifier() {
        let expr = parse_expr("maximum Date");
        if let Expression::MaxValue(ts) = expr {
            assert!(matches!(ts, TypeSpecifier::Named(_)));
        } else {
            panic!("Expected MaxValue expression, got: {expr:?}");
        }
    }

    #[test]
    fn test_query_with_aggregate_clause() {
        let expr = parse_expr("from X A\n  let b: A.val\n  aggregate total: Sum(b)");
        if let Expression::Query(q) = expr {
            assert!(!q.let_clauses.is_empty(), "Expected let clauses");
            assert!(q.aggregate_clause.is_some(), "Expected aggregate clause");
            let agg = q.aggregate_clause.unwrap();
            assert_eq!(agg.identifier, "total");
            assert!(!agg.distinct);
        } else {
            panic!("Expected Query expression, got: {expr:?}");
        }
    }

    #[test]
    fn test_query_with_multiple_let_items() {
        // Multiple let items in one let clause group
        let expr = parse_expr("from X A\n  let b: A.val, c: A.other\n  where b > 1");
        if let Expression::Query(q) = expr {
            assert_eq!(q.let_clauses.len(), 2, "Expected 2 let clause items");
            assert_eq!(q.let_clauses[0].identifier, "b");
            assert_eq!(q.let_clauses[1].identifier, "c");
        } else {
            panic!("Expected Query expression, got: {expr:?}");
        }
    }

    #[test]
    fn test_timing_starts() {
        let expr = parse_expr("A starts B");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Starts);
        } else {
            panic!("Expected BinaryExpression with Starts operator, got: {expr:?}");
        }
    }

    #[test]
    fn test_timing_ends() {
        let expr = parse_expr("A ends B");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Ends);
        } else {
            panic!("Expected BinaryExpression with Ends operator, got: {expr:?}");
        }
    }

    #[test]
    fn test_timing_occurs_during() {
        // "during" is the binary operator for inclusion
        let expr = parse_expr("A during B");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::During);
        } else {
            panic!("Expected BinaryExpression with During operator, got: {expr:?}");
        }
    }

    #[test]
    fn test_timing_properly_during() {
        // "properly included in" is the long form of properly during
        let expr = parse_expr("A properly included in B");
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::ProperlyIncludedIn);
        } else {
            panic!("Expected BinaryExpression with ProperlyIncludedIn operator, got: {expr:?}");
        }
    }
}
