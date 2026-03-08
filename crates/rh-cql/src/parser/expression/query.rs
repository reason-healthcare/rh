//! Query expression parser.
//!
//! Covers: `from` queries, implicit-source queries, and all query tail
//! clauses (`let`, `with`/`without`, `where`, `return`, `aggregate`, `sort`).

use super::expression;
use super::precedence::{
    parse_function_or_identifier, parse_index_suffix, parse_member_suffix, InvocationSuffix,
};
use super::retrieve::parse_retrieve;
use super::selectors::{parse_bare_list_selector, parse_list_selector};
use crate::parser::ast::*;
use crate::parser::lexer::{
    any_identifier, identifier, keyword, quoted_identifier, skip_ws_and_comments, ws,
};
use crate::parser::span::Span;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt, value},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

// ============================================================================
// Query
// ============================================================================

/// All optional tail clauses shared by every query form
/// (`from`, single-source, parenthesized-source, identifier-source).
struct QueryTail {
    let_clauses: Vec<LetClause>,
    relationships: Vec<RelationshipClause>,
    where_clause: Option<Expression>,
    return_clause: Option<ReturnClause>,
    aggregate_clause: Option<AggregateClause>,
    sort_clause: Option<SortClause>,
}

impl QueryTail {
    fn is_empty(&self) -> bool {
        self.let_clauses.is_empty()
            && self.relationships.is_empty()
            && self.where_clause.is_none()
            && self.return_clause.is_none()
            && self.aggregate_clause.is_none()
            && self.sort_clause.is_none()
    }

    fn into_query(self, sources: Vec<QuerySource>) -> Expression {
        Expression::Query(Query {
            sources,
            let_clauses: self.let_clauses,
            relationships: self.relationships,
            where_clause: self.where_clause.map(Box::new),
            return_clause: self.return_clause,
            aggregate_clause: self.aggregate_clause,
            sort_clause: self.sort_clause,
            location: None,
        })
    }
}

/// Parse the tail clauses (`let…`, `with`/`without`, `where`, `return`,
/// `aggregate`, `sort`) that appear after the source alias in every query
/// form.  Using a single parser here avoids repeating the same 7-line block
/// in five different query variants.
fn parse_query_tail(input: Span<'_>) -> IResult<Span<'_>, QueryTail> {
    let (input, let_groups) = many0(parse_let_clause_group)(input)?;
    let let_clauses = let_groups.into_iter().flatten().collect::<Vec<_>>();
    let (input, relationships) = many0(parse_relationship_clause)(input)?;
    let (input, where_clause) = opt(preceded(ws(keyword("where")), expression))(input)?;
    let (input, return_clause) = opt(parse_return_clause)(input)?;
    let (input, aggregate_clause) = opt(parse_aggregate_clause)(input)?;
    let (input, sort_clause) = opt(parse_sort_clause)(input)?;
    Ok((
        input,
        QueryTail {
            let_clauses,
            relationships,
            where_clause,
            return_clause,
            aggregate_clause,
            sort_clause,
        },
    ))
}

/// Parse a full query with explicit `from` keyword
pub(crate) fn parse_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, _) = ws(keyword("from"))(input)?;
    let (input, sources) = separated_list1(ws(char(',')), parse_query_source)(input)?;
    let (input, tail) = parse_query_tail(input)?;
    Ok((input, tail.into_query(sources)))
}

/// Parse a single-source query with a retrieve as source
/// This handles the implicit from syntax: `[Type] alias where ...`
pub(crate) fn parse_single_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

    let (input, tail) = parse_query_tail(input)?;
    let source = QuerySource {
        expression: Box::new(retrieve),
        alias,
        location: None,
    };
    Ok((input, tail.into_query(vec![source])))
}

/// Parse a single-source query with a parenthesized expression as source
/// This handles: `({ 1, 2, 3 }) X sort desc`
pub(crate) fn parse_parenthesized_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    // Parse a parenthesized expression
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, source_expr) = delimited(ws(char('(')), expression, ws(char(')')))(input)?;

    // Then look for an alias (with whitespace handling)
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, alias) = any_identifier(input)?;

    let (input, tail) = parse_query_tail(input)?;

    // Must have at least one clause to be a query (distinguishes from a
    // parenthesized expression)
    if tail.is_empty() {
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
    Ok((input, tail.into_query(vec![source])))
}

/// Parse a single-source query with a quoted identifier as source
/// This handles: `"DefinitionName" alias sort by ...`
pub(crate) fn parse_identifier_source_query(input: Span<'_>) -> IResult<Span<'_>, Expression> {
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

    let (input, tail) = parse_query_tail(input)?;

    // Must have at least one clause to be a query
    if tail.is_empty() {
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
    Ok((input, tail.into_query(vec![source])))
}

/// Parse a single-source query with an unquoted identifier as source
/// This handles: `DefinitionName alias where ...` or `Src.property alias where ...`
pub(crate) fn parse_unquoted_identifier_source_query(
    input: Span<'_>,
) -> IResult<Span<'_>, Expression> {
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

    let (input, tail) = parse_query_tail(input)?;

    // Must have at least one clause to be a query
    if tail.is_empty() {
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
    Ok((input, tail.into_query(vec![source])))
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
