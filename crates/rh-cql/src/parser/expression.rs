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
    any_identifier, date_literal, datetime_literal, decimal_literal, identifier, integer_literal,
    keyword, long_literal, quantity_literal, quoted_identifier, skip_ws_and_comments,
    string_literal, time_literal, ws,
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

enum InvocationSuffix {
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
    // Two forms:
    // 1. List { elements } or List<Type> { elements } - keyword-prefixed
    // 2. { elements } - bare braces (handled separately to avoid conflict with tuple)
    let (input, _) = ws(keyword("List"))(input)?;

    // Optional type specifier: <Type>
    let (input, type_spec) = opt(delimited(
        ws(char('<')),
        parse_type_specifier,
        ws(char('>')),
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

fn parse_bare_list_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Bare braces list: { elements } without List keyword
    // Must not start with `identifier :` to avoid conflict with inline tuple
    let (input, _) = ws(char('{'))(input)?;

    // Peek ahead - if we see `identifier :`, this is a tuple, not a list
    let result = opt(tuple((any_identifier, ws(char(':')))))(input)?;
    if result.1.is_some() {
        // This looks like a tuple element (name: value), not a list
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (input, elements) = separated_list0(ws(char(',')), expression)(input)?;
    let (input, _) = ws(char('}'))(input)?;

    Ok((
        input,
        Expression::ListExpression(ListExpression {
            elements,
            type_specifier: None,
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

    // Try to parse case items starting with 'when' first (searched case without comparand)
    // If that works, we have no comparand. Otherwise, parse comparand + when clauses.
    let (input, comparand, items) = if let Ok((input2, items)) = many0(parse_case_item)(input) {
        if !items.is_empty() {
            // Successfully parsed when clauses without comparand
            (input2, None, items)
        } else {
            // No when clauses found, try parsing comparand
            let (input, expr) = expression(input)?;
            let (input, items) = many0(parse_case_item)(input)?;
            (input, Some(expr), items)
        }
    } else {
        // Failed to parse when clauses, try comparand
        let (input, expr) = expression(input)?;
        let (input, items) = many0(parse_case_item)(input)?;
        (input, Some(expr), items)
    };

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
    let (input, let_clauses) = {
        let (i, groups) = many0(parse_let_clause_group)(input)?;
        (i, groups.into_iter().flatten().collect::<Vec<_>>())
    };
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    Ok((
        input,
        Expression::Query(Query {
            sources,
            let_clauses,
            relationships,
            where_clause: where_clause.map(Box::new),
            return_clause,
            aggregate_clause,
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

    // Try to parse an alias (this is what distinguishes a query from a plain retrieve)
    // Use opt to avoid consuming tokens if it fails
    let (input, alias_opt) = opt(any_identifier)(input)?;

    // If no alias, fail - this should be handled as a plain retrieve
    let alias = match alias_opt {
        Some(a) => {
            // Reject statement-level and query keywords as aliases
            let lower = a.to_lowercase();
            if matches!(
                lower.as_str(),
                "define"
                    | "context"
                    | "using"
                    | "include"
                    | "codesystem"
                    | "valueset"
                    | "code"
                    | "concept"
                    | "parameter"
                    | "library"
                    | "let"
                    | "where"
                    | "return"
                    | "sort"
                    | "with"
                    | "without"
                    | "from"
                    | "select"
                    | "distinct"
                    | "flatten"
            ) {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
            a
        }
        None => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    // Now parse optional query clauses
    let (input, let_clauses) = {
        let (i, groups) = many0(parse_let_clause_group)(input)?;
        (i, groups.into_iter().flatten().collect::<Vec<_>>())
    };
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

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
            aggregate_clause,
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
    let (input, let_clauses) = {
        let (i, groups) = many0(parse_let_clause_group)(input)?;
        (i, groups.into_iter().flatten().collect::<Vec<_>>())
    };
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && aggregate_clause.is_none()
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
            aggregate_clause,
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
    let start_loc = input.location();
    let (input, name) = quoted_identifier(input)?;

    let mut source_expr = Expression::IdentifierRef(IdentifierRef {
        name,
        location: Some(start_loc),
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
    let (input, let_clauses) = {
        let (i, groups) = many0(parse_let_clause_group)(input)?;
        (i, groups.into_iter().flatten().collect::<Vec<_>>())
    };
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && aggregate_clause.is_none()
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
            aggregate_clause,
            sort_clause,
            location: None,
        }),
    ))
}

/// Parse a single-source query with an unquoted identifier as source
/// This handles: `DefinitionName alias where ...` or `Src.property alias where ...`
fn parse_unquoted_identifier_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = skip_ws_and_comments(input)?;
    let start_loc = input.location();

    // Parse a regular (unquoted) identifier
    let (input, name) = identifier(input)?;

    let mut source_expr = Expression::IdentifierRef(IdentifierRef {
        name,
        location: Some(start_loc),
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
    let (input, let_clauses) = {
        let (i, groups) = many0(parse_let_clause_group)(input)?;
        (i, groups.into_iter().flatten().collect::<Vec<_>>())
    };
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;

    // Must have at least one clause to be a query
    if let_clauses.is_empty()
        && relationships.is_empty()
        && where_clause.is_none()
        && return_clause.is_none()
        && aggregate_clause.is_none()
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
            aggregate_clause,
            sort_clause,
            location: None,
        }),
    ))
}

fn parse_query_source(input: Span<'_>) -> IResult<Span<'_>, QuerySource> {
    // For query sources, we need to parse a simple expression followed by an alias
    // We avoid full expression parsing to prevent query-within-query issues
    let (input, expr) = parse_simple_source_expression(input)?;
    let (input, alias) = ws(any_identifier)(input)?;

    Ok((
        input,
        QuerySource {
            expression: Box::new(expr),
            alias,
            location: None,
        },
    ))
}

/// Parse a simple expression suitable for query sources.
/// This is a limited expression parser that doesn't include query syntax
/// to avoid infinite recursion in multi-source queries.
fn parse_simple_source_expression(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Query sources can be:
    // - Retrieves: [Type]
    // - Identifiers: MyList or "Quoted Identifier"
    // - Function calls: SomeFunction()
    // - Parenthesized expressions: (expr)
    // - Lists: { 1, 2, 3 } or List { ... }
    let (input, _) = skip_ws_and_comments(input)?;

    let (input, base) = alt((
        parse_retrieve,
        parse_list_selector,
        parse_bare_list_selector,
        map(delimited(ws(char('(')), expression, ws(char(')'))), |e| e),
        parse_function_or_identifier,
    ))(input)?;

    // Allow member access chains on the base
    let (input, suffixes) = many0(alt((parse_member_suffix, parse_index_suffix)))(input)?;

    Ok((
        input,
        suffixes.into_iter().fold(base, |acc, suffix| match suffix {
            InvocationSuffix::Member { name, .. } => {
                Expression::MemberInvocation(MemberInvocation {
                    source: Box::new(acc),
                    name,
                    location: None,
                })
            }
            InvocationSuffix::Index { index, .. } => Expression::IndexInvocation(IndexInvocation {
                source: Box::new(acc),
                index,
                location: None,
            }),
        }),
    ))
}

fn parse_let_clause_item(input: Span<'_>) -> IResult<Span<'_>, LetClause> {
    let (input, identifier) = ws(any_identifier)(input)?;
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

/// Parse a `let` group: `let <id>: <expr> [, <id>: <expr>]*`
fn parse_let_clause_group(input: Span<'_>) -> IResult<Span<'_>, Vec<LetClause>> {
    let (input, _) = ws(keyword("let"))(input)?;
    separated_list1(ws(char(',')), parse_let_clause_item)(input)
}

fn parse_aggregate_clause(input: Span<'_>) -> IResult<Span<'_>, AggregateClause> {
    let (input, _) = ws(keyword("aggregate"))(input)?;
    let (input, distinct) = opt(ws(alt((
        value(true, keyword("distinct")),
        value(false, keyword("all")),
    ))))(input)?;
    let (input, identifier) = ws(any_identifier)(input)?;
    let (input, starting) = opt(|input| {
        let (input, _) = ws(keyword("starting"))(input)?;
        let (input, distinct_start) = opt(ws(alt((
            value(true, keyword("distinct")),
            value(false, keyword("all")),
        ))))(input)?;
        let (input, expr) = expression(input)?;
        Ok((
            input,
            AggregateStarting {
                distinct: distinct_start.unwrap_or(false),
                expression: Box::new(expr),
            },
        ))
    })(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, expr) = expression(input)?;

    Ok((
        input,
        AggregateClause {
            distinct: distinct.unwrap_or(false),
            identifier,
            starting,
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

fn parse_function_or_identifier(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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
