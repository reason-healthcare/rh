//! CQL Expression Parser
//!
//! Parses CQL expressions following operator precedence rules.
//! Based on CQL v1.5.3 grammar specification.
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. `implies`
//! 2. `or`, `xor`
//! 3. `and`
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
    precedence::parse_implies_expression(input)
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

    fn parse_complete_expr(s: &str) -> Expression {
        crate::parser::CqlParser::new().parse_expression(s).unwrap()
    }

    fn medication_request_body(library: &Library) -> &Expression {
        match library.statements.as_slice() {
            [Statement::FunctionDef(FunctionDef {
                name,
                body: Some(body),
                ..
            })] if name == "Is Current Medication Request" => body,
            other => panic!("expected medication function, got {other:?}"),
        }
    }

    fn ast_without_locations(expression: &Expression) -> serde_json::Value {
        fn remove_locations(value: &mut serde_json::Value) {
            match value {
                serde_json::Value::Object(object) => {
                    object.remove("location");
                    for child in object.values_mut() {
                        remove_locations(child);
                    }
                }
                serde_json::Value::Array(items) => {
                    for child in items {
                        remove_locations(child);
                    }
                }
                _ => {}
            }
        }

        let mut value = serde_json::to_value(expression).expect("AST should serialize");
        remove_locations(&mut value);
        value
    }

    #[derive(Clone, Copy)]
    enum NestedSide {
        Left,
        Right,
    }

    fn assert_binary_shape(
        expression: &Expression,
        description: &str,
        root_operator: BinaryOperator,
        nested_operator: BinaryOperator,
        nested_side: NestedSide,
    ) {
        let Expression::BinaryExpression(root) = expression else {
            panic!("expected binary root for {description}, got {expression:?}");
        };
        assert_eq!(
            root.operator, root_operator,
            "unexpected root for {description}"
        );
        let nested = match nested_side {
            NestedSide::Left => &root.left,
            NestedSide::Right => &root.right,
        };
        assert!(
            matches!(
                nested.as_ref(),
                Expression::BinaryExpression(BinaryExpression { operator, .. })
                    if *operator == nested_operator
            ),
            "unexpected nested grouping for {description}: {nested:?}"
        );
    }

    fn assert_mixed_logical_shape(
        source: &str,
        root_operator: BinaryOperator,
        nested_operator: BinaryOperator,
        nested_side: NestedSide,
    ) {
        assert_binary_shape(
            &parse_complete_expr(source),
            source,
            root_operator,
            nested_operator,
            nested_side,
        );
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

    #[test]
    fn test_code_selector_literal() {
        let expr = parse_expr("Code 'ABC' from \"FAKECS\" display 'ABC'");
        assert!(matches!(
            expr,
            Expression::Literal(Literal::Code {
                code,
                system: Some(system),
                display: Some(display),
            }) if code == "ABC" && system == "FAKECS" && display == "ABC"
        ));
    }

    #[test]
    fn test_code_selector_literal_with_qualified_codesystem() {
        let expr = parse_expr(
            "Code '307469008' from QICoreCommon.\"SNOMEDCT\" display 'Every eight hours'",
        );
        assert!(matches!(
            expr,
            Expression::Literal(Literal::Code {
                code,
                system: Some(system),
                display: Some(display),
            }) if code == "307469008"
                && system == "QICoreCommon.SNOMEDCT"
                && display == "Every eight hours"
        ));
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

    #[test]
    fn test_all_mixed_logical_operator_pairings() {
        use BinaryOperator::{And, Implies, Or, Xor};
        use NestedSide::{Left, Right};

        for (source, root, nested, nested_side) in [
            ("false and false or true", Or, And, Left),
            ("true or false and false", Or, And, Right),
            ("false and false xor true", Xor, And, Left),
            ("true xor true and false", Xor, And, Right),
            ("true or false xor true", Xor, Or, Left),
            ("true xor false or true", Or, Xor, Left),
            ("false and true implies false", Implies, And, Left),
            ("false implies true and false", Implies, And, Right),
            ("true or false implies false", Implies, Or, Left),
            ("false implies false or false", Implies, Or, Right),
            ("true xor false implies true", Implies, Xor, Left),
            ("false implies false xor true", Implies, Xor, Right),
        ] {
            assert_mixed_logical_shape(source, root, nested, nested_side);
        }
    }

    #[test]
    fn test_logical_operator_associativity() {
        use BinaryOperator::{And, Implies, Or, Xor};

        for (source, operator) in [
            ("true and false and true", And),
            ("true or false or true", Or),
            ("true xor false xor true", Xor),
            // CQL 1.5.3's direct-left-recursive grammar declares no
            // right-associative override, matching the reference translator.
            ("false implies true implies false", Implies),
        ] {
            assert_mixed_logical_shape(source, operator, operator, NestedSide::Left);
        }
    }

    #[test]
    fn test_parenthesized_function_and_query_logical_grouping() {
        let parenthesized = parse_complete_expr("(false implies true) and false");
        assert!(matches!(
            parenthesized,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                left,
                ..
            }) if matches!(
                left.as_ref(),
                Expression::Parenthesized(inner) if matches!(
                    inner.as_ref(),
                    Expression::BinaryExpression(BinaryExpression {
                        operator: BinaryOperator::Implies,
                        ..
                    })
                )
            )
        ));

        let parser = crate::parser::CqlParser::new();
        let library = parser
            .parse("library T define function F(): false implies true and false")
            .expect("function should parse");
        let function_body = match library.statements.as_slice() {
            [Statement::FunctionDef(FunctionDef {
                body: Some(body), ..
            })] => body,
            other => panic!("expected one function definition, got {other:?}"),
        };
        assert_binary_shape(
            function_body,
            "function body",
            BinaryOperator::Implies,
            BinaryOperator::And,
            NestedSide::Right,
        );

        let query = parse_complete_expr("from { 1 } N where false implies true and false return N");
        let Expression::Query(Query {
            where_clause: Some(where_clause),
            ..
        }) = query
        else {
            panic!("expected query with where clause, got {query:?}");
        };
        assert_binary_shape(
            &where_clause,
            "query where clause",
            BinaryOperator::Implies,
            BinaryOperator::And,
            NestedSide::Right,
        );
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
    fn test_literal_is_expressions_use_canonical_unary_operators() {
        for (source, expected) in [
            ("x is null", UnaryOperator::IsNull),
            ("x is true", UnaryOperator::IsTrue),
            ("x is false", UnaryOperator::IsFalse),
        ] {
            let expr = parse_complete_expr(source);
            assert!(
                matches!(
                    expr,
                    Expression::UnaryExpression(UnaryExpression { operator, .. })
                        if operator == expected
                ),
                "unexpected AST for {source}: {expr:?}"
            );
        }
    }

    #[test]
    fn test_negated_literal_is_expressions_wrap_canonical_operator() {
        for (source, expected) in [
            ("x is not null", UnaryOperator::IsNull),
            ("x is not true", UnaryOperator::IsTrue),
            ("x is not false", UnaryOperator::IsFalse),
        ] {
            let expr = parse_complete_expr(source);
            assert!(
                matches!(
                    &expr,
                    Expression::UnaryExpression(UnaryExpression {
                        operator: UnaryOperator::Not,
                        operand,
                        ..
                    }) if matches!(
                        operand.as_ref(),
                        Expression::UnaryExpression(UnaryExpression { operator, .. })
                            if *operator == expected
                    )
                ),
                "unexpected AST for {source}: {expr:?}"
            );
        }
    }

    #[test]
    fn test_literal_is_keywords_do_not_shadow_qualified_type_tests() {
        for source in ["value is FHIR.Quantity", "value is not FHIR.Quantity"] {
            let expr = parse_complete_expr(source);
            let type_expression = match expr {
                Expression::TypeExpression(type_expression) => type_expression,
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Not,
                    operand,
                    ..
                }) => match *operand {
                    Expression::TypeExpression(type_expression) => type_expression,
                    other => panic!("expected type expression for {source}, got {other:?}"),
                },
                other => panic!("expected type test for {source}, got {other:?}"),
            };

            assert_eq!(type_expression.operator, TypeOperator::Is);
            assert!(matches!(
                type_expression.type_specifier,
                TypeSpecifier::Named(NamedTypeSpecifier {
                    namespace: Some(ref namespace),
                    ref name,
                }) if namespace == "FHIR" && name == "Quantity"
            ));
        }
    }

    #[test]
    fn test_nested_medication_request_null_test_preserves_member_operand() {
        let expr = parse_complete_expr("medicationRequest.dispenseRequest.validityPeriod is null");
        let operand = match expr {
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::IsNull,
                operand,
                ..
            }) => operand,
            other => panic!("expected IsNull expression, got {other:?}"),
        };

        let validity_period = match *operand {
            Expression::MemberInvocation(member) => member,
            other => panic!("expected validityPeriod member access, got {other:?}"),
        };
        assert_eq!(validity_period.name, "validityPeriod");

        let dispense_request = match *validity_period.source {
            Expression::MemberInvocation(member) => member,
            other => panic!("expected dispenseRequest member access, got {other:?}"),
        };
        assert_eq!(dispense_request.name, "dispenseRequest");
        assert!(matches!(
            dispense_request.source.as_ref(),
            Expression::IdentifierRef(IdentifierRef { name, .. })
                if name == "medicationRequest"
        ));
    }

    #[test]
    fn test_medication_request_neighboring_expression_controls() {
        let repeated = parse_complete_expr("A and B and C");
        assert!(matches!(
            repeated,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                left,
                ..
            }) if matches!(
                left.as_ref(),
                Expression::BinaryExpression(BinaryExpression {
                    operator: BinaryOperator::And,
                    ..
                })
            )
        ));

        let parenthesized = parse_complete_expr("A and (B or C)");
        assert!(matches!(
            parenthesized,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                right,
                ..
            }) if matches!(
                right.as_ref(),
                Expression::Parenthesized(inner) if matches!(
                    inner.as_ref(),
                    Expression::BinaryExpression(BinaryExpression {
                        operator: BinaryOperator::Or,
                        ..
                    })
                )
            )
        ));

        let qualified = parse_complete_expr(
            "end of FHIRHelpers.ToInterval(request.dispenseRequest.validityPeriod)",
        );
        assert!(matches!(
            qualified,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::End,
                operand,
                ..
            }) if matches!(
                operand.as_ref(),
                Expression::FunctionInvocation(FunctionInvocation {
                    library: Some(library),
                    name,
                    arguments,
                    ..
                }) if library == "FHIRHelpers" && name == "ToInterval" && arguments.len() == 1
            )
        ));
    }

    #[test]
    fn test_zero_offset_temporal_relationship_spellings() {
        let cases = [
            (
                "@2024-01-01 on before @2024-02-01",
                TimingDirection::Before,
                None,
            ),
            (
                "Interval[@2024-01-01, @2024-02-01] on or before month of @2024-03-01",
                TimingDirection::OnOrBefore,
                Some(DateTimePrecision::Month),
            ),
            (
                "@T10:00 on after hour of Interval[@T08:00, @T09:00]",
                TimingDirection::After,
                Some(DateTimePrecision::Hour),
            ),
            (
                "Interval[1, 5] on or after Interval[1, 3]",
                TimingDirection::OnOrAfter,
                None,
            ),
            (
                "@2024-01-01 before @2024-02-01",
                TimingDirection::Before,
                None,
            ),
            (
                "Interval[@2024-01-01, @2024-02-01] before or on month of @2024-03-01",
                TimingDirection::BeforeOrOn,
                Some(DateTimePrecision::Month),
            ),
            (
                "@T10:00 after hour of Interval[@T08:00, @T09:00]",
                TimingDirection::After,
                Some(DateTimePrecision::Hour),
            ),
            (
                "Interval[1, 5] after or on Interval[1, 3]",
                TimingDirection::AfterOrOn,
                None,
            ),
        ];

        for (source, expected_direction, expected_precision) in cases {
            let expr = parse_complete_expr(source);
            assert!(
                matches!(
                    expr,
                    Expression::TimingExpression(TimingExpression {
                        timing: TimingPhrase::RelativeTiming {
                            offset: None,
                            direction,
                            precision,
                            ..
                        },
                        ..
                    }) if direction == expected_direction && precision == expected_precision
                ),
                "unexpected timing AST for {source}: {expr:?}"
            );
        }
    }

    #[test]
    fn test_current_medication_request_multiline_and_single_line_ast_match() {
        const MULTILINE: &str = r#"
library MedicationTest version '1.0.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1' called FHIRHelpers

define function "Is Current Medication Request"(
  medicationRequest FHIR.MedicationRequest
):
  medicationRequest.status in { 'active', 'on-hold' }
    and medicationRequest.intent in { 'order', 'plan' }
    and (
      medicationRequest.dispenseRequest.validityPeriod is null
        or end of FHIRHelpers.ToInterval(
          medicationRequest.dispenseRequest.validityPeriod
        ) on or after Today()
    )
"#;
        const SINGLE_LINE: &str = "library MedicationTest version '1.0.0' using FHIR version \
'4.0.1' include FHIRHelpers version '4.0.1' called FHIRHelpers define function \
\"Is Current Medication Request\"(medicationRequest FHIR.MedicationRequest): \
medicationRequest.status in { 'active', 'on-hold' } and medicationRequest.intent in \
{ 'order', 'plan' } and (medicationRequest.dispenseRequest.validityPeriod is null or end of \
FHIRHelpers.ToInterval(medicationRequest.dispenseRequest.validityPeriod) on or after Today())";

        let parser = crate::parser::CqlParser::new();
        let multiline = parser
            .parse(MULTILINE)
            .expect("multiline function should parse");
        let single_line = parser
            .parse(SINGLE_LINE)
            .expect("single-line function should parse");
        assert_eq!(
            ast_without_locations(medication_request_body(&multiline)),
            ast_without_locations(medication_request_body(&single_line))
        );
        let outer = medication_request_body(&multiline);
        assert!(matches!(
            outer,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                left,
                right,
                ..
            }) if matches!(
                left.as_ref(),
                Expression::BinaryExpression(BinaryExpression {
                    operator: BinaryOperator::And,
                    ..
                })
            ) && matches!(
                right.as_ref(),
                Expression::Parenthesized(inner) if matches!(
                    inner.as_ref(),
                    Expression::BinaryExpression(BinaryExpression {
                        operator: BinaryOperator::Or,
                        right,
                        ..
                    }) if matches!(right.as_ref(), Expression::TimingExpression(_))
                )
            )
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

    #[test]
    fn test_same_or_after_hour_precision_time() {
        let expr = parse_expr("@T10:30 same or after @T10");
        assert!(matches!(
            expr,
            Expression::TimingExpression(TimingExpression {
                timing: TimingPhrase::SameTiming {
                    direction: SameDirection::OrAfter,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn test_overlaps_before_prefers_compound_operator() {
        let expr = parse_expr("Interval[1, 5] overlaps before Interval[5, 10]");
        assert!(matches!(
            expr,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::OverlapsBefore,
                ..
            })
        ));
    }

    #[test]
    fn test_meets_after_prefers_compound_operator() {
        let expr = parse_expr("Interval[5, 10] meets after Interval[1, 5)");
        assert!(matches!(
            expr,
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::MeetsAfter,
                ..
            })
        ));
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
