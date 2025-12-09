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
use super::lexer::{
    any_identifier, date_literal, datetime_literal, decimal_literal, integer_literal, keyword,
    long_literal, quantity_literal, quoted_identifier, skip_ws_and_comments, string_literal,
    time_literal, ws,
};
use super::span::{SourceLocation, Span};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, multispace1},
    combinator::{map, not, opt, peek, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

// ============================================================================
// Main Expression Parser
// ============================================================================

/// Parse any CQL expression
pub fn expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    parse_or_expression(input)
}

// ============================================================================
// Precedence Level 1: OR / XOR (lowest precedence)
// ============================================================================

fn parse_or_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_and_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            value(BinaryOperator::Or, keyword("or")),
            value(BinaryOperator::Xor, keyword("xor")),
        ))),
        parse_and_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 2: AND
// ============================================================================

fn parse_and_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_implies_expression(input)?;
    let (input, rest) = many0(preceded(ws(keyword("and")), parse_implies_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| {
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::And,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 3: IMPLIES
// ============================================================================

fn parse_implies_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_equality_expression(input)?;
    let (input, rest) = many0(preceded(ws(keyword("implies")), parse_equality_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| {
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::Implies,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 4: Equality and Membership
// ============================================================================

fn parse_equality_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_comparison_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            // Multi-char operators first
            value(BinaryOperator::NotEqual, tag_no_case("!=")),
            value(BinaryOperator::NotEquivalent, tag_no_case("!~")),
            // Single-char operators
            value(BinaryOperator::Equal, char('=')),
            value(BinaryOperator::Equivalent, char('~')),
            // Keyword operators
            value(BinaryOperator::In, keyword("in")),
            value(BinaryOperator::Contains, keyword("contains")),
        ))),
        parse_comparison_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 5: Comparison (<, >, <=, >=) and Interval Operators
// ============================================================================

fn parse_comparison_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_between_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            // Multi-char operators first
            value(BinaryOperator::LessOrEqual, tag_no_case("<=")),
            value(BinaryOperator::GreaterOrEqual, tag_no_case(">=")),
            // Single-char operators
            value(BinaryOperator::Less, char('<')),
            value(BinaryOperator::Greater, char('>')),
        ))),
        parse_between_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Between Expression (ternary)
// ============================================================================

fn parse_between_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_interval_operator_expression(input)?;

    // Try to parse "between X and Y"
    let result = opt(tuple((
        ws(keyword("between")),
        parse_interval_operator_expression,
        ws(keyword("and")),
        parse_interval_operator_expression,
    )))(input)?;

    match result {
        (input, Some((_, low, _, high))) => Ok((
            input,
            Expression::TernaryExpression(TernaryExpression {
                operator: TernaryOperator::Between,
                first: Box::new(first),
                second: Box::new(low),
                third: Box::new(high),
                location: None,
            }),
        )),
        (input, None) => Ok((input, first)),
    }
}

// ============================================================================
// Interval Operators
// ============================================================================

/// Parse datetime precision specifier: year, month, week, day, hour, minute, second, millisecond
fn parse_precision(input: Span<'_>) -> IResult<Span<'_>, DateTimePrecision> {
    ws(alt((
        value(DateTimePrecision::Year, keyword("year")),
        value(DateTimePrecision::Month, keyword("month")),
        value(DateTimePrecision::Week, keyword("week")),
        value(DateTimePrecision::Day, keyword("day")),
        value(DateTimePrecision::Hour, keyword("hour")),
        value(DateTimePrecision::Minute, keyword("minute")),
        value(DateTimePrecision::Second, keyword("second")),
        value(DateTimePrecision::Millisecond, keyword("millisecond")),
    )))(input)
}

/// Parse optional precision specifier: [precision] "of"
fn parse_optional_precision_of(input: Span<'_>) -> IResult<Span<'_>, Option<DateTimePrecision>> {
    opt(map(
        tuple((parse_precision, ws(keyword("of")))),
        |(prec, _)| prec,
    ))(input)
}

/// Represents a parsed interval operator with optional modifier
#[derive(Debug, Clone)]
enum IntervalOp {
    /// Simple binary operator
    Simple(BinaryOperator),
    /// starts/ends followed by during/before/after with optional precision
    /// (interval_boundary, relationship, precision)
    Compound {
        boundary: UnaryOperator, // Start or End
        operator: BinaryOperator,
        precision: Option<DateTimePrecision>,
    },
    /// during/before/after with precision
    WithPrecision(BinaryOperator, DateTimePrecision),
}

fn parse_interval_operator_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_union_expression(input)?;

    // Parse interval relationship operators
    let (input, rest) = many0(parse_interval_op_with_operand)(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| match op {
            IntervalOp::Simple(binary_op) => Expression::BinaryExpression(BinaryExpression {
                operator: binary_op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            }),
            IntervalOp::Compound {
                boundary,
                operator,
                precision,
            } => {
                // "X ends during Y" => BinaryOp(End(X), Y) with `during` becoming `In`
                let boundary_expr = Expression::UnaryExpression(UnaryExpression {
                    operator: boundary,
                    operand: Box::new(acc),
                    location: None,
                });
                // Map "during" to "In" when used with point (from ends/starts)
                let mapped_op = match operator {
                    BinaryOperator::During => BinaryOperator::In,
                    other => other,
                };
                Expression::BinaryExpression(BinaryExpression {
                    operator: mapped_op,
                    left: Box::new(boundary_expr),
                    right: Box::new(expr),
                    precision,
                    location: None,
                })
            }
            IntervalOp::WithPrecision(binary_op, prec) => {
                Expression::BinaryExpression(BinaryExpression {
                    operator: binary_op,
                    left: Box::new(acc),
                    right: Box::new(expr),
                    precision: Some(prec),
                    location: None,
                })
            }
        }),
    ))
}

/// Parse a single interval operator followed by its operand
fn parse_interval_op_with_operand(input: Span<'_>) -> IResult<Span<'_>, (IntervalOp, Expression)> {
    // First, try to parse compound operators: "starts/ends during/before/after [precision of]"
    let compound_result = opt(tuple((
        ws(alt((
            value(UnaryOperator::Start, keyword("starts")),
            value(UnaryOperator::End, keyword("ends")),
        ))),
        ws(alt((
            value(BinaryOperator::During, keyword("during")),
            value(BinaryOperator::Before, keyword("before")),
            value(BinaryOperator::After, keyword("after")),
        ))),
        parse_optional_precision_of,
    )))(input)?;

    match compound_result {
        (remaining, Some((boundary, op, precision))) => {
            let (remaining, expr) = parse_union_expression(remaining)?;
            Ok((
                remaining,
                (
                    IntervalOp::Compound {
                        boundary,
                        operator: op,
                        precision,
                    },
                    expr,
                ),
            ))
        }
        (_, None) => {
            // Try temporal operators with precision: "during/before/after [precision of]"
            let temporal_with_precision = opt(tuple((
                ws(alt((
                    value(BinaryOperator::During, keyword("during")),
                    value(BinaryOperator::Before, keyword("before")),
                    value(BinaryOperator::After, keyword("after")),
                ))),
                parse_precision,
                ws(keyword("of")),
            )))(input)?;

            match temporal_with_precision {
                (remaining, Some((op, precision, _))) => {
                    let (remaining, expr) = parse_union_expression(remaining)?;
                    Ok((remaining, (IntervalOp::WithPrecision(op, precision), expr)))
                }
                (_, None) => {
                    // Fall back to simple operators
                    let (remaining, op) = ws(alt((
                        // "properly includes" / "properly included in" first (longer match)
                        value(
                            BinaryOperator::ProperlyIncludes,
                            tuple((keyword("properly"), ws(keyword("includes")))),
                        ),
                        value(
                            BinaryOperator::ProperlyIncludedIn,
                            tuple((
                                keyword("properly"),
                                ws(keyword("included")),
                                ws(keyword("in")),
                            )),
                        ),
                        // Regular includes/included in
                        value(BinaryOperator::Includes, keyword("includes")),
                        value(
                            BinaryOperator::IncludedIn,
                            tuple((keyword("included"), ws(keyword("in")))),
                        ),
                        // Other interval operators
                        value(BinaryOperator::During, keyword("during")),
                        value(BinaryOperator::Overlaps, keyword("overlaps")),
                        value(
                            BinaryOperator::OverlapsBefore,
                            tuple((keyword("overlaps"), ws(keyword("before")))),
                        ),
                        value(
                            BinaryOperator::OverlapsAfter,
                            tuple((keyword("overlaps"), ws(keyword("after")))),
                        ),
                        value(BinaryOperator::Meets, keyword("meets")),
                        value(
                            BinaryOperator::MeetsBefore,
                            tuple((keyword("meets"), ws(keyword("before")))),
                        ),
                        value(
                            BinaryOperator::MeetsAfter,
                            tuple((keyword("meets"), ws(keyword("after")))),
                        ),
                        value(BinaryOperator::Starts, keyword("starts")),
                        value(BinaryOperator::Ends, keyword("ends")),
                        value(BinaryOperator::Before, keyword("before")),
                        value(BinaryOperator::After, keyword("after")),
                        value(
                            BinaryOperator::SameAs,
                            tuple((keyword("same"), ws(keyword("as")))),
                        ),
                        value(
                            BinaryOperator::SameOrBefore,
                            tuple((keyword("same"), ws(keyword("or")), ws(keyword("before")))),
                        ),
                        value(
                            BinaryOperator::SameOrAfter,
                            tuple((keyword("same"), ws(keyword("or")), ws(keyword("after")))),
                        ),
                        // Within
                        value(BinaryOperator::Within, keyword("within")),
                    )))(input)?;

                    let (remaining, expr) = parse_union_expression(remaining)?;
                    Ok((remaining, (IntervalOp::Simple(op), expr)))
                }
            }
        }
    }
}

// ============================================================================
// Precedence Level 6: Union, Except, Intersect
// ============================================================================

fn parse_union_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_additive_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            value(BinaryOperator::Union, char('|')),
            value(BinaryOperator::Union, keyword("union")),
            value(BinaryOperator::Except, keyword("except")),
            value(BinaryOperator::Intersect, keyword("intersect")),
        ))),
        parse_additive_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 7: Additive (+, -, &)
// ============================================================================

fn parse_additive_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_multiplicative_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            value(BinaryOperator::Add, char('+')),
            value(BinaryOperator::Subtract, char('-')),
            value(BinaryOperator::Concatenate, char('&')),
        ))),
        parse_multiplicative_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 8: Multiplicative (*, /, div, mod)
// ============================================================================

fn parse_multiplicative_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_power_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            value(BinaryOperator::Multiply, char('*')),
            value(BinaryOperator::Divide, char('/')),
            value(BinaryOperator::TruncatedDivide, keyword("div")),
            value(BinaryOperator::Modulo, keyword("mod")),
        ))),
        parse_power_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
            Expression::BinaryExpression(BinaryExpression {
                operator: op,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 8.5: Power (^) - Higher than multiplicative
// ============================================================================

fn parse_power_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_unary_expression(input)?;
    let (input, rest) = many0(preceded(ws(char('^')), parse_unary_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| {
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::Power,
                left: Box::new(acc),
                right: Box::new(expr),
                precision: None,
                location: None,
            })
        }),
    ))
}

// ============================================================================
// Precedence Level 9: Unary Expressions
// ============================================================================

fn parse_unary_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    alt((
        // "not" expression
        map(
            preceded(ws(keyword("not")), parse_unary_expression),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Not,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // Negation (but not if followed by a number - that's a negative literal)
        map(
            preceded(
                tuple((ws(char('-')), peek(not(nom::character::complete::digit1)))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Negate,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "exists" expression
        map(
            preceded(ws(keyword("exists")), parse_unary_expression),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Exists,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "distinct" expression
        map(
            preceded(ws(keyword("distinct")), parse_unary_expression),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Distinct,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "flatten" expression
        map(
            preceded(ws(keyword("flatten")), parse_unary_expression),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Flatten,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "collapse" expression
        map(
            preceded(ws(keyword("collapse")), parse_unary_expression),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Collapse,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "singleton from" expression
        map(
            preceded(
                tuple((ws(keyword("singleton")), ws(keyword("from")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Singleton,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "predecessor of" expression
        map(
            preceded(
                tuple((ws(keyword("predecessor")), ws(keyword("of")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Predecessor,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "successor of" expression
        map(
            preceded(
                tuple((ws(keyword("successor")), ws(keyword("of")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Successor,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "date from" expression
        map(
            preceded(
                tuple((ws(keyword("date")), ws(keyword("from")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::DateFrom,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "time from" expression
        map(
            preceded(
                tuple((ws(keyword("time")), ws(keyword("from")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::TimeFrom,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "timezoneoffset from" expression
        map(
            preceded(
                tuple((ws(keyword("timezoneoffset")), ws(keyword("from")))),
                parse_unary_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::TimezoneOffsetFrom,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // DateTime component extraction: year/month/day/hour/minute/second/millisecond from
        map(
            tuple((
                ws(alt((
                    value(DateTimePrecision::Year, keyword("year")),
                    value(DateTimePrecision::Month, keyword("month")),
                    value(DateTimePrecision::Day, keyword("day")),
                    value(DateTimePrecision::Hour, keyword("hour")),
                    value(DateTimePrecision::Minute, keyword("minute")),
                    value(DateTimePrecision::Second, keyword("second")),
                    value(DateTimePrecision::Millisecond, keyword("millisecond")),
                ))),
                preceded(ws(keyword("from")), parse_unary_expression),
            )),
            |(precision, operand)| {
                Expression::DateTimeComponentFrom(DateTimeComponentFromExpr {
                    precision,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // Type expression suffix handling
        parse_type_expression,
    ))(input)
}

// ============================================================================
// Type Expressions (is, as, convert)
// ============================================================================

fn parse_type_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_invocation_expression(input)?;

    // Check for type operator suffix
    let (input, type_op) = opt(tuple((
        ws(alt((
            value(TypeOperator::Is, keyword("is")),
            value(TypeOperator::As, keyword("as")),
        ))),
        parse_type_specifier,
    )))(input)?;

    match type_op {
        Some((op, type_spec)) => Ok((
            input,
            Expression::TypeExpression(TypeExpression {
                operator: op,
                operand: Box::new(first),
                type_specifier: type_spec,
                location: None,
            }),
        )),
        None => Ok((input, first)),
    }
}

// ============================================================================
// Precedence Level 10: Invocation (highest precedence)
// ============================================================================

fn parse_invocation_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_term(input)?;
    let (input, suffixes) = many0(alt((
        // Member invocation: .name or .name()
        parse_member_suffix,
        // Index invocation: [expr]
        parse_index_suffix,
    )))(input)?;

    Ok((
        input,
        suffixes
            .into_iter()
            .fold(first, |acc, suffix| match suffix {
                InvocationSuffix::Member {
                    name,
                    args,
                    location,
                } => {
                    if let Some(arguments) = args {
                        // Check if acc is a simple identifier - if so, it's a library qualifier
                        if let Expression::IdentifierRef(IdentifierRef { name: lib_name, .. }) =
                            &acc
                        {
                            Expression::FunctionInvocation(FunctionInvocation {
                                library: Some(lib_name.clone()),
                                name,
                                arguments,
                                location,
                            })
                        } else {
                            // Otherwise it's a fluent function call
                            Expression::FunctionInvocation(FunctionInvocation {
                                library: None,
                                name,
                                arguments,
                                location,
                            })
                        }
                    } else {
                        Expression::MemberInvocation(MemberInvocation {
                            source: Box::new(acc),
                            name,
                            location,
                        })
                    }
                }
                InvocationSuffix::Index { index, location } => {
                    Expression::IndexInvocation(IndexInvocation {
                        source: Box::new(acc),
                        index: Box::new(index),
                        location,
                    })
                }
            }),
    ))
}

enum InvocationSuffix {
    Member {
        name: String,
        args: Option<Vec<Expression>>,
        location: Option<SourceLocation>,
    },
    Index {
        index: Expression,
        location: Option<SourceLocation>,
    },
}

fn parse_member_suffix(input: Span<'_>) -> IResult<Span<'_>, InvocationSuffix> {
    let (input, _) = ws(char('.'))(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, args) = opt(delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), expression),
        ws(char(')')),
    ))(input)?;

    Ok((
        input,
        InvocationSuffix::Member {
            name,
            args,
            location: None,
        },
    ))
}

fn parse_index_suffix(input: Span<'_>) -> IResult<Span<'_>, InvocationSuffix> {
    let (input, _) = ws(char('['))(input)?;
    let (input, index) = expression(input)?;
    let (input, _) = ws(char(']'))(input)?;

    Ok((
        input,
        InvocationSuffix::Index {
            index,
            location: None,
        },
    ))
}

// ============================================================================
// Terms (Primary Expressions)
// ============================================================================

fn parse_term(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = skip_ws_and_comments(input)?;

    alt((
        // Group 1: Literals
        alt((
            parse_null_literal,
            parse_boolean_literal,
            parse_datetime_literal_expr,
            parse_time_literal_expr,
            parse_date_literal_expr,
            parse_duration_quantity_expr, // 30 days style
            parse_quantity_literal_expr,  // 5 'mg' style
            parse_long_literal_expr,
            parse_decimal_literal_expr,
            parse_integer_literal_expr,
            parse_string_literal_expr, // Single-quoted CQL strings
        )),
        // Group 2: Selectors (inline tuple must come before list to handle { name: value } correctly)
        alt((
            parse_interval_selector,
            parse_inline_tuple_selector,
            parse_list_selector,
            parse_tuple_selector,
            parse_instance_selector,
        )),
        // Group 3: Control flow
        alt((parse_if_then_else, parse_case)),
        // Group 4: Queries and retrieves
        alt((
            parse_query,
            parse_parenthesized_source_query,
            parse_identifier_source_query,
            parse_single_source_query,
            parse_retrieve,
        )),
        // Group 5: Others
        alt((parse_parenthesized, parse_function_or_identifier)),
    ))(input)
}

// ============================================================================
// Literal Parsers
// ============================================================================

fn parse_null_literal(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(keyword("null"), |_| Expression::Literal(Literal::Null))(input)
}

fn parse_boolean_literal(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    alt((
        map(keyword("true"), |_| {
            Expression::Literal(Literal::Boolean(true))
        }),
        map(keyword("false"), |_| {
            Expression::Literal(Literal::Boolean(false))
        }),
    ))(input)
}

fn parse_string_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(string_literal, |s| Expression::Literal(Literal::String(s)))(input)
}

fn parse_integer_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(integer_literal, |n| {
        Expression::Literal(Literal::Integer(n))
    })(input)
}

fn parse_long_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(long_literal, |n| Expression::Literal(Literal::Long(n)))(input)
}

fn parse_decimal_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(decimal_literal, |n| {
        Expression::Literal(Literal::Decimal(n))
    })(input)
}

fn parse_quantity_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(quantity_literal, |(value, unit)| {
        Expression::Literal(Literal::Quantity { value, unit })
    })(input)
}

fn parse_duration_quantity_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, qty_value) = alt((decimal_literal, map(integer_literal, |i| i as f64)))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, unit) = alt((
        value("year", keyword("year")),
        value("year", keyword("years")),
        value("month", keyword("month")),
        value("month", keyword("months")),
        value("week", keyword("week")),
        value("week", keyword("weeks")),
        value("day", keyword("day")),
        value("day", keyword("days")),
        value("hour", keyword("hour")),
        value("hour", keyword("hours")),
        value("minute", keyword("minute")),
        value("minute", keyword("minutes")),
        value("second", keyword("second")),
        value("second", keyword("seconds")),
        value("millisecond", keyword("millisecond")),
        value("millisecond", keyword("milliseconds")),
    ))(input)?;

    Ok((
        input,
        Expression::Literal(Literal::Quantity {
            value: qty_value,
            unit: unit.to_string(),
        }),
    ))
}

fn parse_date_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(date_literal, |s| Expression::Literal(Literal::Date(s)))(input)
}

fn parse_time_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(time_literal, |s| Expression::Literal(Literal::Time(s)))(input)
}

fn parse_datetime_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(datetime_literal, |s| {
        Expression::Literal(Literal::DateTime(s))
    })(input)
}

// ============================================================================
// Collection Constructors
// ============================================================================

fn parse_interval_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("Interval"))(input)?;
    let (input, low_closed) = alt((value(true, char('[')), value(false, char('('))))(input)?;
    let (input, _) = skip_ws_and_comments(input)?;

    // Parse low bound (null for open)
    let (input, low) = opt(expression)(input)?;
    let (input, _) = ws(char(','))(input)?;

    // Parse high bound (null for open)
    let (input, high) = opt(expression)(input)?;
    let (input, _) = skip_ws_and_comments(input)?;

    let (input, high_closed) = alt((value(true, char(']')), value(false, char(')'))))(input)?;

    Ok((
        input,
        Expression::IntervalExpression(IntervalExpression {
            low: low.map(Box::new),
            high: high.map(Box::new),
            low_closed,
            high_closed,
            location: None,
        }),
    ))
}

fn parse_list_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Optional type specifier: List<Type> or just { ... }
    let (input, type_spec) = opt(preceded(
        ws(keyword("List")),
        delimited(ws(char('<')), parse_type_specifier, ws(char('>'))),
    ))(input)?;

    let (input, _) = ws(char('{'))(input)?;
    let (input, elements) = separated_list0(ws(char(',')), expression)(input)?;
    let (input, _) = ws(char('}'))(input)?;

    Ok((
        input,
        Expression::ListExpression(ListExpression {
            elements,
            type_specifier: type_spec,
            location: None,
        }),
    ))
}

fn parse_inline_tuple_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Inline tuple: { name: value, ... } without the Tuple keyword
    // Must have at least one element to distinguish from empty list {}
    let (input, _) = ws(char('{'))(input)?;
    let (input, elements) = separated_list1(ws(char(',')), parse_tuple_element)(input)?;
    let (input, _) = ws(char('}'))(input)?;

    Ok((
        input,
        Expression::TupleExpression(TupleExpression {
            elements,
            location: None,
        }),
    ))
}

fn parse_tuple_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("Tuple"))(input)?;
    let (input, _) = ws(char('{'))(input)?;
    let (input, elements) = separated_list0(ws(char(',')), parse_tuple_element)(input)?;
    let (input, _) = ws(char('}'))(input)?;

    Ok((
        input,
        Expression::TupleExpression(TupleExpression {
            elements,
            location: None,
        }),
    ))
}

fn parse_tuple_element(input: Span<'_>) -> IResult<Span<'_>, TupleElement> {
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, value) = expression(input)?;

    Ok((
        input,
        TupleElement {
            name,
            value: Box::new(value),
        },
    ))
}

fn parse_instance_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Type { element: value, ... }
    let (input, class_type) = parse_type_specifier(input)?;
    let (input, _) = ws(char('{'))(input)?;
    let (input, elements) = separated_list0(ws(char(',')), parse_instance_element)(input)?;
    let (input, _) = ws(char('}'))(input)?;

    Ok((
        input,
        Expression::Instance(Instance {
            class_type,
            elements,
            location: None,
        }),
    ))
}

fn parse_instance_element(input: Span<'_>) -> IResult<Span<'_>, InstanceElement> {
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, value) = expression(input)?;

    Ok((
        input,
        InstanceElement {
            name,
            value: Box::new(value),
        },
    ))
}

// ============================================================================
// Conditionals
// ============================================================================

fn parse_if_then_else(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("if"))(input)?;
    let (input, condition) = expression(input)?;
    let (input, _) = ws(keyword("then"))(input)?;
    let (input, then_expr) = expression(input)?;
    let (input, _) = ws(keyword("else"))(input)?;
    let (input, else_expr) = expression(input)?;

    Ok((
        input,
        Expression::IfThenElse(IfThenElse {
            condition: Box::new(condition),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
            location: None,
        }),
    ))
}

fn parse_case(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("case"))(input)?;

    // Optional comparand for simple case
    let (input, comparand) = opt(terminated(expression, peek(ws(keyword("when")))))(input)?;

    let (input, items) = many0(parse_case_item)(input)?;
    let (input, _) = ws(keyword("else"))(input)?;
    let (input, else_expr) = expression(input)?;
    let (input, _) = ws(keyword("end"))(input)?;

    Ok((
        input,
        Expression::Case(Case {
            comparand: comparand.map(Box::new),
            items,
            else_expr: Box::new(else_expr),
            location: None,
        }),
    ))
}

fn parse_case_item(input: Span<'_>) -> IResult<Span<'_>, CaseItem> {
    let (input, _) = ws(keyword("when"))(input)?;
    let (input, when_expr) = expression(input)?;
    let (input, _) = ws(keyword("then"))(input)?;
    let (input, then_expr) = expression(input)?;

    Ok((
        input,
        CaseItem {
            when: Box::new(when_expr),
            then: Box::new(then_expr),
        },
    ))
}

// ============================================================================
// Query
// ============================================================================

/// Parse a full query with explicit `from` keyword
fn parse_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("from"))(input)?;
    let (input, sources) = separated_list1(ws(char(',')), parse_query_source)(input)?;
    let (input, let_clauses) = many0(parse_let_clause)(input)?;
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    Ok((
        input,
        Expression::Query(Query {
            sources,
            let_clauses,
            relationships,
            where_clause: where_clause.map(Box::new),
            return_clause,
            sort_clause,
            location: None,
        }),
    ))
}

/// Parse a single-source query with a retrieve as source
/// This handles the implicit from syntax: `[Type] alias where ...`
fn parse_single_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // First parse the retrieve
    let (input, retrieve) = parse_retrieve(input)?;

    // Then look for an alias - this is what distinguishes a query from a plain retrieve
    let (input, alias) = any_identifier(input)?;

    // Now we must have at least one query clause (where, return, sort, let, with, without)
    // Otherwise this would just be a retrieve with an erroneous identifier after it
    let (input, let_clauses) = many0(parse_let_clause)(input)?;
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && sort_clause.is_none()
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let source = QuerySource {
        expression: Box::new(retrieve),
        alias,
        location: None,
    };

    Ok((
        input,
        Expression::Query(Query {
            sources: vec![source],
            let_clauses,
            relationships,
            where_clause: where_clause.map(Box::new),
            return_clause,
            sort_clause,
            location: None,
        }),
    ))
}

/// Parse a single-source query with a parenthesized expression as source
/// This handles: `({ 1, 2, 3 }) X sort desc`
fn parse_parenthesized_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Parse a parenthesized expression
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, source_expr) = delimited(ws(char('(')), expression, ws(char(')')))(input)?;

    // Then look for an alias (with whitespace handling)
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, alias) = any_identifier(input)?;

    // Now parse query clauses
    let (input, let_clauses) = many0(parse_let_clause)(input)?;
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && sort_clause.is_none()
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let source = QuerySource {
        expression: Box::new(source_expr),
        alias,
        location: None,
    };

    Ok((
        input,
        Expression::Query(Query {
            sources: vec![source],
            let_clauses,
            relationships,
            where_clause: where_clause.map(Box::new),
            return_clause,
            sort_clause,
            location: None,
        }),
    ))
}

/// Parse a single-source query with a quoted identifier as source
/// This handles: `"DefinitionName" alias sort by ...`
fn parse_identifier_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Parse a quoted identifier (definition reference) - uses double quotes or backticks
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = quoted_identifier(input)?;

    let mut source_expr = Expression::IdentifierRef(IdentifierRef {
        name,
        location: None,
    });

    // Check for property access chain (.property)
    let (input, properties) = many0(preceded(ws(char('.')), any_identifier))(input)?;

    // Build property access chain
    for prop in properties {
        source_expr = Expression::MemberInvocation(MemberInvocation {
            source: Box::new(source_expr),
            name: prop,
            location: None,
        });
    }

    // Then look for an alias (with whitespace handling)
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, alias) = any_identifier(input)?;

    // Now parse query clauses
    let (input, let_clauses) = many0(parse_let_clause)(input)?;
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && sort_clause.is_none()
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let source = QuerySource {
        expression: Box::new(source_expr),
        alias,
        location: None,
    };

    Ok((
        input,
        Expression::Query(Query {
            sources: vec![source],
            let_clauses,
            relationships,
            where_clause: where_clause.map(Box::new),
            return_clause,
            sort_clause,
            location: None,
        }),
    ))
}

fn parse_query_source(input: Span<'_>) -> IResult<Span<'_>, QuerySource> {
    let (input, expr) = expression(input)?;
    let (input, alias) = any_identifier(input)?;

    Ok((
        input,
        QuerySource {
            expression: Box::new(expr),
            alias,
            location: None,
        },
    ))
}

fn parse_let_clause(input: Span<'_>) -> IResult<Span<'_>, LetClause> {
    let (input, _) = ws(keyword("let"))(input)?;
    let (input, identifier) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, expr) = expression(input)?;

    Ok((
        input,
        LetClause {
            identifier,
            expression: Box::new(expr),
            location: None,
        },
    ))
}

fn parse_relationship_clause(input: Span<'_>) -> IResult<Span<'_>, RelationshipClause> {
    let (input, kind) = ws(alt((
        value(RelationshipKind::With, keyword("with")),
        value(RelationshipKind::Without, keyword("without")),
    )))(input)?;

    let (input, source) = parse_query_source(input)?;
    let (input, such_that) = opt(preceded(
        tuple((ws(keyword("such")), ws(keyword("that")))),
        expression,
    ))(input)?;

    Ok((
        input,
        RelationshipClause {
            kind,
            source,
            such_that: such_that.map(Box::new),
            location: None,
        },
    ))
}

fn parse_return_clause(input: Span<'_>) -> IResult<Span<'_>, ReturnClause> {
    let (input, _) = ws(keyword("return"))(input)?;
    let (input, distinct) = opt(keyword("distinct"))(input)?;
    let (input, all) = opt(keyword("all"))(input)?;
    let (input, expr) = expression(input)?;

    Ok((
        input,
        ReturnClause {
            distinct: distinct.is_some(),
            all: all.is_some(),
            expression: Box::new(expr),
            location: None,
        },
    ))
}

fn parse_sort_clause(input: Span<'_>) -> IResult<Span<'_>, SortClause> {
    let (input, _) = ws(keyword("sort"))(input)?;
    let (input, _) = opt(keyword("by"))(input)?;
    let (input, items) = separated_list1(ws(char(',')), parse_sort_item)(input)?;

    Ok((
        input,
        SortClause {
            items,
            location: None,
        },
    ))
}

fn parse_sort_item(input: Span<'_>) -> IResult<Span<'_>, SortItem> {
    let (input, expr) = expression(input)?;
    let (input, direction) = opt(ws(alt((
        value(
            SortDirection::Ascending,
            alt((keyword("asc"), keyword("ascending"))),
        ),
        value(
            SortDirection::Descending,
            alt((keyword("desc"), keyword("descending"))),
        ),
    ))))(input)?;

    Ok((
        input,
        SortItem {
            expression: Box::new(expr),
            direction: direction.unwrap_or_default(),
        },
    ))
}

// ============================================================================
// Retrieve
// ============================================================================

fn parse_retrieve(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(char('['))(input)?;

    // Context expression (optional)
    let (input, context) = opt(terminated(expression, ws(tag_no_case("->"))))(input)?;

    // Data type
    let (input, data_type) = parse_type_specifier(input)?;

    // Code path and codes (optional)
    // Supports three forms:
    // 1. [Type] - no codes
    // 2. [Type: codesExpression] - codes with default code path
    // 3. [Type: codePath in codesExpression] - explicit code path
    let (input, code_info) = opt(preceded(
        ws(char(':')),
        alt((
            // Form 3: codePath in codesExpression
            map(
                tuple((any_identifier, ws(keyword("in")), expression)),
                |(path, _, codes_expr)| (Some(path), codes_expr),
            ),
            // Form 2: just codesExpression (default code path)
            map(expression, |codes_expr| (None, codes_expr)),
        )),
    ))(input)?;

    let (input, _) = ws(char(']'))(input)?;

    let (code_path, codes) = match code_info {
        Some((path, codes_expr)) => (path, Some(Box::new(codes_expr))),
        None => (None, None),
    };

    Ok((
        input,
        Expression::Retrieve(Retrieve {
            data_type,
            context: context.map(Box::new),
            code_path,
            codes,
            date_path: None,
            date_range: None,
            location: None,
        }),
    ))
}

// ============================================================================
// Type Specifier
// ============================================================================

pub fn parse_type_specifier(input: Span<'_>) -> IResult<Span<'_>, TypeSpecifier> {
    let (input, _) = skip_ws_and_comments(input)?;

    alt((
        // List<Type>
        map(
            tuple((
                keyword("List"),
                ws(char('<')),
                parse_type_specifier,
                ws(char('>')),
            )),
            |(_, _, element_type, _)| {
                TypeSpecifier::List(ListTypeSpecifier {
                    element_type: Box::new(element_type),
                })
            },
        ),
        // Interval<Type>
        map(
            tuple((
                keyword("Interval"),
                ws(char('<')),
                parse_type_specifier,
                ws(char('>')),
            )),
            |(_, _, point_type, _)| {
                TypeSpecifier::Interval(IntervalTypeSpecifier {
                    point_type: Box::new(point_type),
                })
            },
        ),
        // Tuple { name Type, ... }
        map(
            tuple((
                keyword("Tuple"),
                ws(char('{')),
                separated_list0(ws(char(',')), parse_tuple_type_element),
                ws(char('}')),
            )),
            |(_, _, elements, _)| TypeSpecifier::Tuple(TupleTypeSpecifier { elements }),
        ),
        // Choice<Type, Type, ...>
        map(
            tuple((
                keyword("Choice"),
                ws(char('<')),
                separated_list1(ws(char(',')), parse_type_specifier),
                ws(char('>')),
            )),
            |(_, _, types, _)| TypeSpecifier::Choice(ChoiceTypeSpecifier { types }),
        ),
        // Named type (possibly qualified: Namespace.Type)
        map(parse_qualified_type_name, |(namespace, name)| {
            TypeSpecifier::Named(NamedTypeSpecifier { namespace, name })
        }),
    ))(input)
}

fn parse_tuple_type_element(input: Span<'_>) -> IResult<Span<'_>, TupleElementDef> {
    let (input, name) = any_identifier(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, element_type) = parse_type_specifier(input)?;

    Ok((input, TupleElementDef { name, element_type }))
}

fn parse_qualified_type_name(input: Span<'_>) -> IResult<Span<'_>, (Option<String>, String)> {
    let (input, parts) = separated_list1(char('.'), any_identifier)(input)?;

    if parts.len() == 1 {
        Ok((input, (None, parts.into_iter().next().unwrap())))
    } else {
        let mut parts = parts;
        let name = parts.pop().unwrap();
        let namespace = parts.join(".");
        Ok((input, (Some(namespace), name)))
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn parse_parenthesized(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(
        delimited(ws(char('(')), expression, ws(char(')'))),
        |expr| Expression::Parenthesized(Box::new(expr)),
    )(input)
}

fn parse_function_or_identifier(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = skip_ws_and_comments(input)?;

    // Parse a simple identifier
    let (input, name) = any_identifier(input)?;

    // Check for function call
    let (input, args) = opt(delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), expression),
        ws(char(')')),
    ))(input)?;

    match args {
        // Function call: Name(args)
        Some(arguments) => Ok((
            input,
            Expression::FunctionInvocation(FunctionInvocation {
                library: None,
                name,
                arguments,
                location: None,
            }),
        )),
        // Simple identifier reference
        None => Ok((
            input,
            Expression::IdentifierRef(IdentifierRef {
                name,
                location: None,
            }),
        )),
    }
}

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
}
