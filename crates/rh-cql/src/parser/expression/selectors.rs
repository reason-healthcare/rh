//! Collection constructor and conditional expression parsers.
//!
//! Covers: Interval, List, bare-list, Tuple, Instance selectors, and
//! `if/then/else` + `case` expressions.

use crate::parser::ast::*;
use crate::parser::lexer::{
    any_identifier, keyword,
    skip_ws_and_comments, ws,
};
use crate::parser::span::Span;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{opt, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use super::retrieve::parse_type_specifier;
use super::expression;

// ============================================================================
// Collection Constructors
// ============================================================================

pub(crate) fn parse_interval_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_list_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_bare_list_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_inline_tuple_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_tuple_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_instance_selector(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_if_then_else(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

pub(crate) fn parse_case(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

