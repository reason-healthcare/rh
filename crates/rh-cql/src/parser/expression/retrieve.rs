//! Retrieve expression and type-specifier parsers.

use super::expression;
use crate::parser::ast::*;
use crate::parser::lexer::{any_identifier, keyword, skip_ws_and_comments, ws};
use crate::parser::span::Span;
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::char,
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub(crate) fn parse_retrieve(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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
