//! CQL operator-precedence expression parsers.
//!
//! Implements the full binary/unary precedence chain (levels 1-10), timing
//! operators, type expressions, invocation, and the `parse_term` dispatch.
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

use crate::parser::ast::*;
use crate::parser::lexer::{
    any_identifier, decimal_literal,
    integer_literal, keyword,
    skip_ws_and_comments, ws,
};
use crate::parser::span::{SourceLocation, Span};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::char,
    combinator::{map, not, opt, peek, value},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use super::literals::*;
use super::selectors::{
    parse_interval_selector, parse_list_selector, parse_inline_tuple_selector,
    parse_bare_list_selector, parse_tuple_selector, parse_instance_selector,
    parse_if_then_else, parse_case,
};
use super::query::{
    parse_query, parse_parenthesized_source_query, parse_identifier_source_query,
    parse_single_source_query, parse_unquoted_identifier_source_query,
};
use super::retrieve::{parse_retrieve, parse_type_specifier};
use super::expression;

// ============================================================================
// Shared Precedence Helpers
// ============================================================================

fn binary_expr(op: BinaryOperator, left: Expression, right: Expression) -> Expression {
    Expression::BinaryExpression(BinaryExpression {
        operator: op,
        left: Box::new(left),
        right: Box::new(right),
        precision: None,
        location: None,
    })
}

/// Fold a sequence of `(operator, rhs)` pairs into a left-associative binary
/// expression tree.  All parsers that respect left-to-right associativity use
/// this helper so the folding logic lives in exactly one place.
fn fold_left_assoc(first: Expression, rest: Vec<(BinaryOperator, Expression)>) -> Expression {
    rest.into_iter()
        .fold(first, |acc, (op, rhs)| binary_expr(op, acc, rhs))
}


// ============================================================================
// Precedence Level 1: OR / XOR (lowest precedence)
// ============================================================================

pub(crate) fn parse_or_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_and_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            value(BinaryOperator::Or, keyword("or")),
            value(BinaryOperator::Xor, keyword("xor")),
        ))),
        parse_and_expression,
    )))(input)?;
    Ok((input, fold_left_assoc(first, rest)))
}

// ============================================================================
// Precedence Level 2: AND
// ============================================================================

fn parse_and_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_implies_expression(input)?;
    let (input, rest) = many0(preceded(ws(keyword("and")), parse_implies_expression))(input)?;
    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, rhs| binary_expr(BinaryOperator::And, acc, rhs)),
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
        rest.into_iter()
            .fold(first, |acc, rhs| binary_expr(BinaryOperator::Implies, acc, rhs)),
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
    Ok((input, fold_left_assoc(first, rest)))
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
    Ok((input, fold_left_assoc(first, rest)))
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
    /// includes/included in with optional precision and right boundary
    /// e.g., "includes day of start B" or "includes start B"
    WithPrecisionAndRightBoundary {
        operator: BinaryOperator,
        precision: Option<DateTimePrecision>,
        right_boundary: Option<UnaryOperator>, // Start or End applied to right operand
    },
    /// Timing phrase (complex relative timing)
    Timing(TimingPhrase),
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
            IntervalOp::WithPrecisionAndRightBoundary {
                operator,
                precision,
                right_boundary,
            } => {
                // "X includes start Y" => Includes(X, Start(Y))
                let right_expr = match right_boundary {
                    Some(boundary) => Expression::UnaryExpression(UnaryExpression {
                        operator: boundary,
                        operand: Box::new(expr),
                        location: None,
                    }),
                    None => expr,
                };
                Expression::BinaryExpression(BinaryExpression {
                    operator,
                    left: Box::new(acc),
                    right: Box::new(right_expr),
                    precision,
                    location: None,
                })
            }
            IntervalOp::Timing(timing) => Expression::TimingExpression(TimingExpression {
                left: Box::new(acc),
                right: Box::new(expr),
                timing,
                location: None,
            }),
        }),
    ))
}

/// Parse a single interval operator followed by its operand
fn parse_interval_op_with_operand(input: Span<'_>) -> IResult<Span<'_>, (IntervalOp, Expression)> {
    // Try timing phrases first (most complex)
    if let Ok((remaining, (timing, expr))) = parse_timing_phrase_with_operand(input) {
        return Ok((remaining, (IntervalOp::Timing(timing), expr)));
    }

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
            // Try "includes" family operators: [properly] includes [precision of] [start|end]
            let includes_result = opt(tuple((
                ws(alt((
                    value(
                        BinaryOperator::ProperlyIncludes,
                        tuple((keyword("properly"), ws(keyword("includes")))),
                    ),
                    value(BinaryOperator::Includes, keyword("includes")),
                ))),
                opt(tuple((parse_precision, ws(keyword("of"))))),
                opt(ws(alt((
                    value(UnaryOperator::Start, keyword("start")),
                    value(UnaryOperator::End, keyword("end")),
                )))),
            )))(input)?;

            if let (remaining, Some((op, precision_opt, boundary))) = includes_result {
                let precision = precision_opt.map(|(p, _)| p);
                // Only use this variant if we have precision or boundary
                if precision.is_some() || boundary.is_some() {
                    let (remaining, expr) = parse_union_expression(remaining)?;
                    return Ok((
                        remaining,
                        (
                            IntervalOp::WithPrecisionAndRightBoundary {
                                operator: op,
                                precision,
                                right_boundary: boundary,
                            },
                            expr,
                        ),
                    ));
                }
                // Fall through to simple parsing if no precision or boundary
            }

            // Try temporal operators with precision: "operator precision of"
            // This includes: during, before, after, included in, properly included in
            let temporal_with_precision = opt(tuple((
                ws(alt((
                    // Longer matches first
                    value(
                        BinaryOperator::ProperlyIncludedIn,
                        tuple((
                            keyword("properly"),
                            ws(keyword("included")),
                            ws(keyword("in")),
                        )),
                    ),
                    value(
                        BinaryOperator::IncludedIn,
                        tuple((keyword("included"), ws(keyword("in")))),
                    ),
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

/// Parse timing phrase with its right operand
///
/// Handles:
/// - `[starts|ends|occurs] [offset [qualifier]] direction [precision of] [boundary] operand`
/// - `[starts|ends|occurs] [properly] within offset of [boundary] operand`
/// - `[starts|ends|occurs] same [precision] (as|or before|or after) [boundary] operand`
fn parse_timing_phrase_with_operand(
    input: Span<'_>,
) -> IResult<Span<'_>, (TimingPhrase, Expression)> {
    // Try "same" timing first
    if let Ok(result) = parse_same_timing(input) {
        return Ok(result);
    }

    // Try "within" timing
    if let Ok(result) = parse_within_timing(input) {
        return Ok(result);
    }

    // Try relative timing (before/after variants)
    parse_relative_timing(input)
}

/// Parse "same" timing phrase
/// Patterns: `[starts|ends|occurs] same [precision] (as|or before|or after) [start|end] operand`
fn parse_same_timing(input: Span<'_>) -> IResult<Span<'_>, (TimingPhrase, Expression)> {
    // Optional left boundary: starts/ends/occurs
    let (input, left_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("starts")),
        value(IntervalBoundary::End, keyword("ends")),
        value(IntervalBoundary::Start, keyword("occurs")),
    ))))(input)?;

    // "same" keyword is required
    let (input, _) = ws(keyword("same"))(input)?;

    // Optional precision: day, month, year, etc.
    let (input, precision) = opt(ws(parse_precision))(input)?;

    // Direction: "as", "or before", "or after"
    let (input, direction) = ws(alt((
        value(
            SameDirection::OrBefore,
            tuple((keyword("or"), ws(keyword("before")))),
        ),
        value(
            SameDirection::OrAfter,
            tuple((keyword("or"), ws(keyword("after")))),
        ),
        value(SameDirection::As, keyword("as")),
    )))(input)?;

    // Optional right boundary: start/end
    let (input, right_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("start")),
        value(IntervalBoundary::End, keyword("end")),
    ))))(input)?;

    let (input, expr) = parse_union_expression(input)?;

    Ok((
        input,
        (
            TimingPhrase::SameTiming {
                left_boundary,
                precision,
                direction,
                right_boundary,
            },
            expr,
        ),
    ))
}

/// Parse "within" timing phrase
fn parse_within_timing(input: Span<'_>) -> IResult<Span<'_>, (TimingPhrase, Expression)> {
    let (input, left_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("starts")),
        value(IntervalBoundary::End, keyword("ends")),
        value(IntervalBoundary::Start, keyword("occurs")), // occurs uses start by convention
    ))))(input)?;

    let (input, properly) = opt(ws(keyword("properly")))(input)?;
    let (input, _) = ws(keyword("within"))(input)?;
    let (input, quantity) = ws(parse_duration_quantity)(input)?;
    let (input, _) = ws(keyword("of"))(input)?;

    let (input, right_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("start")),
        value(IntervalBoundary::End, keyword("end")),
    ))))(input)?;

    let (input, expr) = parse_union_expression(input)?;

    let offset = TimingOffset {
        quantity: QuantityValue {
            value: quantity.0,
            unit: quantity.1,
        },
        qualifier: None,
    };

    Ok((
        input,
        (
            TimingPhrase::WithinTiming {
                left_boundary,
                properly: properly.is_some(),
                offset,
                right_boundary,
            },
            expr,
        ),
    ))
}

/// Parse relative timing (before/after variants)
fn parse_relative_timing(input: Span<'_>) -> IResult<Span<'_>, (TimingPhrase, Expression)> {
    // Optional left boundary: starts/ends/occurs
    let (input, left_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("starts")),
        value(IntervalBoundary::End, keyword("ends")),
        value(IntervalBoundary::Start, keyword("occurs")),
    ))))(input)?;

    // Try to parse offset with optional qualifier
    // Patterns:
    // - <quantity> [or less|or more] before/after
    // - less than <quantity> before/after
    // - more than <quantity> before/after
    let (input, offset) = parse_timing_offset(input)?;

    // Parse direction - must have at least offset or boundary to trigger timing phrase parsing
    // If we have neither, this isn't a timing phrase
    if offset.is_none() && left_boundary.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // We need an offset for timing phrases (simple before/after without offset is handled elsewhere)
    let offset = match offset {
        Some(o) => o,
        None => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    let (input, direction) = ws(parse_timing_direction)(input)?;

    // Optional precision: day of, hour of, etc.
    let (input, precision) = parse_optional_precision_of(input)?;

    // Optional right boundary: start/end
    let (input, right_boundary) = opt(ws(alt((
        value(IntervalBoundary::Start, keyword("start")),
        value(IntervalBoundary::End, keyword("end")),
    ))))(input)?;

    let (input, expr) = parse_union_expression(input)?;

    Ok((
        input,
        (
            TimingPhrase::RelativeTiming {
                left_boundary,
                offset: Some(offset),
                direction,
                precision,
                right_boundary,
            },
            expr,
        ),
    ))
}

/// Parse timing offset with optional qualifier
fn parse_timing_offset(input: Span<'_>) -> IResult<Span<'_>, Option<TimingOffset>> {
    // Try "less than <quantity>" or "more than <quantity>" first
    let less_than = opt(tuple((
        ws(keyword("less")),
        ws(keyword("than")),
        ws(parse_duration_quantity),
    )))(input)?;

    if let (remaining, Some((_, _, quantity))) = less_than {
        return Ok((
            remaining,
            Some(TimingOffset {
                quantity: QuantityValue {
                    value: quantity.0,
                    unit: quantity.1,
                },
                qualifier: Some(TimingQualifier::LessThan),
            }),
        ));
    }

    let more_than = opt(tuple((
        ws(keyword("more")),
        ws(keyword("than")),
        ws(parse_duration_quantity),
    )))(input)?;

    if let (remaining, Some((_, _, quantity))) = more_than {
        return Ok((
            remaining,
            Some(TimingOffset {
                quantity: QuantityValue {
                    value: quantity.0,
                    unit: quantity.1,
                },
                qualifier: Some(TimingQualifier::MoreThan),
            }),
        ));
    }

    // Try "<quantity> [or less|or more]"
    let quantity_result = opt(ws(parse_duration_quantity))(input)?;

    match quantity_result {
        (remaining, Some(quantity)) => {
            // Check for "or less" / "or more"
            let (remaining, qualifier) = opt(ws(alt((
                value(
                    TimingQualifier::OrLess,
                    tuple((keyword("or"), ws(keyword("less")))),
                ),
                value(
                    TimingQualifier::OrMore,
                    tuple((keyword("or"), ws(keyword("more")))),
                ),
            ))))(remaining)?;

            Ok((
                remaining,
                Some(TimingOffset {
                    quantity: QuantityValue {
                        value: quantity.0,
                        unit: quantity.1,
                    },
                    qualifier,
                }),
            ))
        }
        (remaining, None) => Ok((remaining, None)),
    }
}

/// Parse a duration quantity (number with temporal unit keyword)
/// Handles: 3 days, 1 year, 2 weeks, etc.
fn parse_duration_quantity(input: Span<'_>) -> IResult<Span<'_>, (f64, String)> {
    let (input, value) = alt((decimal_literal, map(integer_literal, |i| i as f64)))(input)?;
    let (input, unit) = ws(parse_duration_unit)(input)?;
    Ok((input, (value, unit)))
}

/// Parse a temporal unit keyword
fn parse_duration_unit(input: Span<'_>) -> IResult<Span<'_>, String> {
    alt((
        // Singular and plural forms
        map(alt((keyword("years"), keyword("year"))), |_| {
            "years".to_string()
        }),
        map(alt((keyword("months"), keyword("month"))), |_| {
            "months".to_string()
        }),
        map(alt((keyword("weeks"), keyword("week"))), |_| {
            "weeks".to_string()
        }),
        map(alt((keyword("days"), keyword("day"))), |_| {
            "days".to_string()
        }),
        map(alt((keyword("hours"), keyword("hour"))), |_| {
            "hours".to_string()
        }),
        map(alt((keyword("minutes"), keyword("minute"))), |_| {
            "minutes".to_string()
        }),
        map(alt((keyword("seconds"), keyword("second"))), |_| {
            "seconds".to_string()
        }),
        map(
            alt((keyword("milliseconds"), keyword("millisecond"))),
            |_| "milliseconds".to_string(),
        ),
    ))(input)
}

/// Parse timing direction keyword
fn parse_timing_direction(input: Span<'_>) -> IResult<Span<'_>, TimingDirection> {
    alt((
        // Multi-word variants first (longer match)
        value(
            TimingDirection::OnOrBefore,
            tuple((keyword("on"), ws(keyword("or")), ws(keyword("before")))),
        ),
        value(
            TimingDirection::OnOrAfter,
            tuple((keyword("on"), ws(keyword("or")), ws(keyword("after")))),
        ),
        value(
            TimingDirection::BeforeOrOn,
            tuple((keyword("before"), ws(keyword("or")), ws(keyword("on")))),
        ),
        value(
            TimingDirection::AfterOrOn,
            tuple((keyword("after"), ws(keyword("or")), ws(keyword("on")))),
        ),
        // Simple variants
        value(TimingDirection::Before, keyword("before")),
        value(TimingDirection::After, keyword("after")),
    ))(input)
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
    Ok((input, fold_left_assoc(first, rest)))
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
    Ok((input, fold_left_assoc(first, rest)))
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
    Ok((input, fold_left_assoc(first, rest)))
}

// ============================================================================
// Precedence Level 8.5: Power (^) - Higher than multiplicative
// ============================================================================

fn parse_power_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_unary_expression(input)?;
    let (input, rest) = many0(preceded(ws(char('^')), parse_unary_expression))(input)?;
    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, rhs| binary_expr(BinaryOperator::Power, acc, rhs)),
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
        // "start of" expression (interval boundary)
        map(
            preceded(
                tuple((ws(keyword("start")), ws(keyword("of")))),
                parse_invocation_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::Start,
                    operand: Box::new(operand),
                    location: None,
                })
            },
        ),
        // "end of" expression (interval boundary)
        map(
            preceded(
                tuple((ws(keyword("end")), ws(keyword("of")))),
                parse_invocation_expression,
            ),
            |operand| {
                Expression::UnaryExpression(UnaryExpression {
                    operator: UnaryOperator::End,
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
        // "minimum <TypeSpecifier>" expression - returns minimum value for the type
        map(
            preceded(ws(keyword("minimum")), parse_type_specifier),
            Expression::MinValue,
        ),
        // "maximum <TypeSpecifier>" expression - returns maximum value for the type
        map(
            preceded(ws(keyword("maximum")), parse_type_specifier),
            Expression::MaxValue,
        ),
        // "cast X as Type" expression (prefix syntax)
        parse_cast_expression,
        // "convert X to Type" expression (prefix syntax)
        parse_convert_expression,
        // Type expression suffix handling
        parse_type_expression,
    ))(input)
}

// ============================================================================
// Type Expressions (is, as, cast, convert)
// ============================================================================

fn parse_cast_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("cast"))(input)?;
    let (input, operand) = parse_invocation_expression(input)?;
    let (input, _) = ws(keyword("as"))(input)?;
    let (input, type_spec) = parse_type_specifier(input)?;

    Ok((
        input,
        Expression::TypeExpression(TypeExpression {
            operator: TypeOperator::Cast,
            operand: Box::new(operand),
            type_specifier: type_spec,
            location: None,
        }),
    ))
}

fn parse_convert_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("convert"))(input)?;
    let (input, operand) = parse_invocation_expression(input)?;
    let (input, _) = ws(keyword("to"))(input)?;
    let (input, type_spec) = parse_type_specifier(input)?;

    Ok((
        input,
        Expression::TypeExpression(TypeExpression {
            operator: TypeOperator::Convert,
            operand: Box::new(operand),
            type_specifier: type_spec,
            location: None,
        }),
    ))
}

fn parse_type_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, first) = parse_invocation_expression(input)?;

    // Check for 'is not' (negated type check)
    if let Ok((input2, _)) = tuple((ws(keyword("is")), ws(keyword("not"))))(input) {
        let (input, type_spec) = parse_type_specifier(input2)?;
        return Ok((
            input,
            Expression::UnaryExpression(UnaryExpression {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::TypeExpression(TypeExpression {
                    operator: TypeOperator::Is,
                    operand: Box::new(first),
                    type_specifier: type_spec,
                    location: None,
                })),
                location: None,
            }),
        ));
    }

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
                        index,
                        location,
                    })
                }
            }),
    ))
}

pub(crate) enum InvocationSuffix {
    Member {
        name: String,
        args: Option<Vec<Expression>>,
        location: Option<SourceLocation>,
    },
    Index {
        index: Box<Expression>,
        location: Option<SourceLocation>,
    },
}

pub(crate) fn parse_member_suffix(input: Span<'_>) -> IResult<Span<'_>, InvocationSuffix> {
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

pub(crate) fn parse_index_suffix(input: Span<'_>) -> IResult<Span<'_>, InvocationSuffix> {
    let (input, _) = ws(char('['))(input)?;
    let (input, index) = expression(input)?;
    let (input, _) = ws(char(']'))(input)?;

    Ok((
        input,
        InvocationSuffix::Index {
            index: Box::new(index),
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
        // Group 2: Selectors (inline tuple must come before bare list to handle { name: value } correctly)
        alt((
            parse_interval_selector,
            parse_list_selector,
            parse_inline_tuple_selector,
            parse_bare_list_selector,
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
            parse_unquoted_identifier_source_query,
            parse_retrieve,
        )),
        // Group 5: Others
        alt((
            parse_parenthesized,
            parse_special_identifier,
            parse_function_or_identifier,
        )),
    ))(input)
}


// ============================================================================
// Helpers (parenthesized expressions, identifiers, function calls)
// ============================================================================

// ============================================================================
// Helpers
// ============================================================================

fn parse_parenthesized(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(
        delimited(ws(char('(')), expression, ws(char(')'))),
        |expr| Expression::Parenthesized(Box::new(expr)),
    )(input)
}

fn parse_special_identifier(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = char('$')(input)?;
    let (input, name) = any_identifier(input)?;

    Ok((
        input,
        Expression::IdentifierRef(IdentifierRef {
            name: format!("${name}"),
            location: None,
        }),
    ))
}

pub(crate) fn parse_function_or_identifier(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = skip_ws_and_comments(input)?;
    let start_loc = input.location();

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
                location: Some(start_loc),
            }),
        )),
        // Simple identifier reference
        None => Ok((
            input,
            Expression::IdentifierRef(IdentifierRef {
                name,
                location: Some(start_loc),
            }),
        )),
    }
}

