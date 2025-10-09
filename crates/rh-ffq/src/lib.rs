use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::{cut, map, opt, recognize, value},
    error::{Error, ErrorKind},
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/* =========================
AST
========================= */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Start {
    pub headers: Vec<Header>,
    pub expr: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Header {
    Alias { name: String, target: AliasTarget },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AliasTarget {
    System(SystemRef),
    SystemWithVersion(SystemRef, String), // system, version
    ValueSetUrl(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Minus(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Clause(Clause),
    Group(Box<Expr>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clause {
    pub system: SystemRef,
    pub version: Option<String>,
    pub inner: InnerExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemRef {
    Uri(String),
    Alias(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InnerExpr {
    Minus(Box<InnerExpr>, Box<InnerExpr>),
    Or(Box<InnerExpr>, Box<InnerExpr>),
    And(Box<InnerExpr>, Box<InnerExpr>),
    Not(Box<InnerExpr>),
    Group(Box<InnerExpr>),
    Term(Term),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Term {
    Hierarchy(HierarchyOp, CodeRef),
    PropertyEq(String, Value),
    PropertyIn(String, Vec<Value>),
    PropertyRegex(String, String), // prop, regex pattern (sans /.../)
    MembershipValueSet(String),    // vs(URL)
    MembershipAlias(String),       // in #alias
    Exists(String),                // has prop
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HierarchyOp {
    DescOrSelf, // <<
    DescOnly,   // <
    Isa,        // isa
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeRef {
    Code(String),
    Quoted(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Str(String),
    Code(String),
    Uri(String),
    Hierarchy(HierarchyOp, CodeRef),
}

/* =========================
AST to FHIR Translation
========================= */

use std::collections::HashMap;

pub fn translate_to_fhir(ast: &Start) -> ValueSetCompose {
    // Create alias lookup table
    let mut aliases = HashMap::new();
    for header in &ast.headers {
        match header {
            Header::Alias { name, target } => {
                aliases.insert(name.clone(), target.clone());
            }
        }
    }

    // Translate the expression
    let (include, exclude) = translate_expression(&ast.expr, &aliases);

    ValueSetCompose {
        inactive: None,
        include,
        exclude,
    }
}

fn translate_expression(
    expr: &Expr,
    aliases: &HashMap<String, AliasTarget>,
) -> (Vec<ConceptSetComponent>, Vec<ConceptSetComponent>) {
    match expr {
        Expr::Clause(clause) => translate_clause_with_excludes(clause, aliases),
        Expr::Minus(left, right) => {
            let (mut include, mut exclude) = translate_expression(left, aliases);
            let (right_include, right_exclude) = translate_expression(right, aliases);

            // In FHIR, A - B means include A and exclude B
            exclude.extend(right_include);
            include.extend(right_exclude); // Double negative becomes include

            (include, exclude)
        }
        Expr::Or(left, right) => {
            // OR means multiple include components
            let (mut left_include, mut left_exclude) = translate_expression(left, aliases);
            let (right_include, right_exclude) = translate_expression(right, aliases);

            left_include.extend(right_include);
            left_exclude.extend(right_exclude);

            (left_include, left_exclude)
        }
        Expr::And(left, right) => {
            // AND means we need to merge filters within components where possible
            // For now, we'll create separate components and let FHIR handle the intersection
            let (mut left_include, mut left_exclude) = translate_expression(left, aliases);
            let (right_include, right_exclude) = translate_expression(right, aliases);

            left_include.extend(right_include);
            left_exclude.extend(right_exclude);

            (left_include, left_exclude)
        }
        Expr::Not(inner) => {
            // NOT flips include/exclude
            let (include, exclude) = translate_expression(inner, aliases);
            (exclude, include)
        }
        Expr::Group(inner) => translate_expression(inner, aliases),
    }
}

fn translate_clause_with_excludes(
    clause: &Clause,
    aliases: &HashMap<String, AliasTarget>,
) -> (Vec<ConceptSetComponent>, Vec<ConceptSetComponent>) {
    let (system, alias_version) = resolve_system_and_version(&clause.system, aliases);
    // Clause version takes precedence over alias version
    let version = clause.version.clone().or(alias_version);

    // Check if this clause contains a minus operation
    if let InnerExpr::Minus(left, right) = &clause.inner {
        // Handle minus operation: left side becomes includes, right side becomes excludes
        let left_components = translate_inner_expr_with_expansion(left, aliases);
        let right_components = translate_inner_expr_with_expansion(right, aliases);

        let mut includes = vec![];
        let mut excludes = vec![];

        // Process left side as includes
        for (concept, filter, mut value_set) in left_components {
            value_set = resolve_value_set_aliases(value_set, aliases);
            includes.push(ConceptSetComponent {
                system: system.clone(),
                version: version.clone(),
                concept,
                filter,
                value_set,
            });
        }

        // Process right side as excludes
        for (concept, filter, mut value_set) in right_components {
            value_set = resolve_value_set_aliases(value_set, aliases);
            excludes.push(ConceptSetComponent {
                system: system.clone(),
                version: version.clone(),
                concept,
                filter,
                value_set,
            });
        }

        (includes, excludes)
    } else if contains_not_operation(&clause.inner) {
        // Handle AND operations with NOT - create separate includes and excludes
        let (includes, excludes) =
            extract_includes_excludes(&clause.inner, &system, &version, aliases);
        (includes, excludes)
    } else {
        // Normal case without minus or NOT+AND operations
        let components = translate_inner_expr_with_expansion(&clause.inner, aliases);

        let includes = components
            .into_iter()
            .map(|(concept, filter, mut value_set)| {
                value_set = resolve_value_set_aliases(value_set, aliases);
                ConceptSetComponent {
                    system: system.clone(),
                    version: version.clone(),
                    concept,
                    filter,
                    value_set,
                }
            })
            .collect();

        (includes, vec![])
    }
}

fn contains_not_operation(expr: &InnerExpr) -> bool {
    match expr {
        InnerExpr::Not(_) => true,
        InnerExpr::And(left, right) => {
            contains_not_operation(left) || contains_not_operation(right)
        }
        InnerExpr::Or(left, right) => contains_not_operation(left) || contains_not_operation(right),
        InnerExpr::Group(inner) => contains_not_operation(inner),
        _ => false,
    }
}

fn extract_includes_excludes(
    expr: &InnerExpr,
    system: &Option<String>,
    version: &Option<String>,
    aliases: &HashMap<String, AliasTarget>,
) -> (Vec<ConceptSetComponent>, Vec<ConceptSetComponent>) {
    match expr {
        InnerExpr::Not(inner) => {
            // NOT expressions become excludes
            let components = translate_inner_expr_with_expansion(inner, aliases);
            let excludes = components
                .into_iter()
                .map(|(concept, filter, mut value_set)| {
                    value_set = resolve_value_set_aliases(value_set, aliases);
                    ConceptSetComponent {
                        system: system.clone(),
                        version: version.clone(),
                        concept,
                        filter,
                        value_set,
                    }
                })
                .collect();
            (vec![], excludes)
        }
        InnerExpr::And(left, right) => {
            // Handle AND operations with potential NOT inside
            let (left_includes, left_excludes) =
                extract_includes_excludes(left, system, version, aliases);
            let (right_includes, right_excludes) =
                extract_includes_excludes(right, system, version, aliases);

            let mut all_includes = vec![];
            let mut all_excludes = vec![];

            // If both sides have includes, create Cartesian product
            if !left_includes.is_empty() && !right_includes.is_empty() {
                for left_inc in &left_includes {
                    for right_inc in &right_includes {
                        let mut combined_filters = left_inc.filter.clone();
                        combined_filters.extend(right_inc.filter.clone());

                        let mut combined_concepts = left_inc.concept.clone();
                        combined_concepts.extend(right_inc.concept.clone());

                        let mut combined_value_set = left_inc.value_set.clone();
                        combined_value_set.extend(right_inc.value_set.clone());

                        all_includes.push(ConceptSetComponent {
                            system: system.clone(),
                            version: version.clone(),
                            concept: combined_concepts,
                            filter: combined_filters,
                            value_set: combined_value_set,
                        });
                    }
                }
            } else {
                // If only one side has includes, use those
                all_includes.extend(left_includes);
                all_includes.extend(right_includes);
            }

            // Combine all excludes
            all_excludes.extend(left_excludes);
            all_excludes.extend(right_excludes);

            (all_includes, all_excludes)
        }
        _ => {
            // Regular expressions become includes
            let components = translate_inner_expr_with_expansion(expr, aliases);
            let includes = components
                .into_iter()
                .map(|(concept, filter, mut value_set)| {
                    value_set = resolve_value_set_aliases(value_set, aliases);
                    ConceptSetComponent {
                        system: system.clone(),
                        version: version.clone(),
                        concept,
                        filter,
                        value_set,
                    }
                })
                .collect();
            (includes, vec![])
        }
    }
}

fn resolve_value_set_aliases(
    value_set: Vec<String>,
    aliases: &HashMap<String, AliasTarget>,
) -> Vec<String> {
    value_set
        .into_iter()
        .map(|vs| {
            if let Some(alias_name) = vs.strip_prefix('#') {
                // Remove the '#' prefix
                match aliases.get(alias_name) {
                    Some(AliasTarget::ValueSetUrl(url)) => url.clone(),
                    _ => vs, // Keep original if can't resolve
                }
            } else {
                vs
            }
        })
        .collect()
}

fn resolve_system_and_version(
    system_ref: &SystemRef,
    aliases: &HashMap<String, AliasTarget>,
) -> (Option<String>, Option<String>) {
    match system_ref {
        SystemRef::Uri(uri) => (Some(uri.clone()), None),
        SystemRef::Alias(alias_name) => {
            match aliases.get(alias_name) {
                Some(AliasTarget::System(SystemRef::Uri(uri))) => (Some(uri.clone()), None),
                Some(AliasTarget::SystemWithVersion(SystemRef::Uri(uri), version)) => {
                    (Some(uri.clone()), Some(version.clone()))
                }
                Some(AliasTarget::System(SystemRef::Alias(nested_alias))) => {
                    // Handle nested aliases (though unlikely in practice)
                    resolve_system_and_version(&SystemRef::Alias(nested_alias.clone()), aliases)
                }
                Some(AliasTarget::SystemWithVersion(SystemRef::Alias(nested_alias), version)) => {
                    // Handle nested aliases with version - use the version from the alias
                    let (sys, _) = resolve_system_and_version(
                        &SystemRef::Alias(nested_alias.clone()),
                        aliases,
                    );
                    (sys, Some(version.clone()))
                }
                Some(AliasTarget::ValueSetUrl(_)) => (None, None), // This alias points to a ValueSet, not a system
                _ => (None, None),                                 // Unresolved alias
            }
        }
    }
}

fn translate_inner_expr_with_expansion(
    inner: &InnerExpr,
    aliases: &HashMap<String, AliasTarget>,
) -> Vec<(
    Vec<ConceptReferenceComponent>,
    Vec<ConceptSetFilter>,
    Vec<String>,
)> {
    match inner {
        InnerExpr::Term(term) => {
            if let Term::PropertyIn(property, values) = term {
                // Expand PropertyIn into multiple components, one per value
                values
                    .iter()
                    .map(|value| {
                        let filter = ConceptSetFilter {
                            property: property.clone(),
                            op: FilterOperator::Equals,
                            value: translate_value_to_string(value),
                        };
                        (vec![], vec![filter], vec![])
                    })
                    .collect()
            } else {
                vec![translate_term_with_aliases(term, aliases)]
            }
        }
        InnerExpr::And(left, right) => {
            let left_components = translate_inner_expr_with_expansion(left, aliases);
            let right_components = translate_inner_expr_with_expansion(right, aliases);

            // Cartesian product: combine each left component with each right component
            let mut result = Vec::new();
            for (left_concepts, left_filters, left_value_sets) in left_components {
                for (right_concepts, right_filters, right_value_sets) in &right_components {
                    let mut concepts = left_concepts.clone();
                    concepts.extend(right_concepts.iter().cloned());

                    let mut filters = left_filters.clone();
                    filters.extend(right_filters.iter().cloned());

                    let mut value_sets = left_value_sets.clone();
                    value_sets.extend(right_value_sets.iter().cloned());

                    result.push((concepts, filters, value_sets));
                }
            }
            result
        }
        InnerExpr::Or(left, right) => {
            let mut left_components = translate_inner_expr_with_expansion(left, aliases);
            let right_components = translate_inner_expr_with_expansion(right, aliases);
            left_components.extend(right_components);
            left_components
        }
        InnerExpr::Minus(_left, _right) => {
            // Minus operations within clauses are not directly supported
            // They should be handled at the expression level
            vec![(vec![], vec![], vec![])]
        }
        InnerExpr::Not(_inner) => {
            // NOT operations within clauses need special handling at clause level
            vec![(vec![], vec![], vec![])]
        }
        InnerExpr::Group(inner) => translate_inner_expr_with_expansion(inner, aliases),
    }
}

fn translate_term_with_aliases(
    term: &Term,
    aliases: &HashMap<String, AliasTarget>,
) -> (
    Vec<ConceptReferenceComponent>,
    Vec<ConceptSetFilter>,
    Vec<String>,
) {
    match term {
        Term::MembershipAlias(alias) => {
            match aliases.get(alias) {
                Some(AliasTarget::ValueSetUrl(url)) => (vec![], vec![], vec![url.clone()]),
                _ => (vec![], vec![], vec![]), // Unresolved alias
            }
        }
        _ => translate_term(term), // For non-alias terms, use the original function
    }
}

// Keep the old function for non-alias terms
fn translate_term(
    term: &Term,
) -> (
    Vec<ConceptReferenceComponent>,
    Vec<ConceptSetFilter>,
    Vec<String>,
) {
    match term {
        Term::Hierarchy(op, code_ref) => {
            let code = extract_code_from_ref(code_ref);
            let filter_op = match op {
                HierarchyOp::DescOrSelf => FilterOperator::IsA,
                HierarchyOp::DescOnly => FilterOperator::DescendentOf,
                HierarchyOp::Isa => FilterOperator::IsA,
            };

            let filter = ConceptSetFilter {
                property: "concept".to_string(),
                op: filter_op,
                value: code,
            };

            (vec![], vec![filter], vec![])
        }
        Term::PropertyEq(property, value) => {
            let (op, val) = match value {
                Value::Hierarchy(HierarchyOp::DescOrSelf, code_ref) => {
                    (FilterOperator::IsA, extract_code_from_ref(code_ref))
                }
                Value::Hierarchy(HierarchyOp::DescOnly, code_ref) => (
                    FilterOperator::DescendentOf,
                    extract_code_from_ref(code_ref),
                ),
                _ => (FilterOperator::Equals, translate_value_to_string(value)),
            };

            let filter = ConceptSetFilter {
                property: property.clone(),
                op,
                value: val,
            };

            (vec![], vec![filter], vec![])
        }
        Term::PropertyIn(property, values) => {
            let value_str = values
                .iter()
                .map(translate_value_to_string)
                .collect::<Vec<_>>()
                .join(",");

            let filter = ConceptSetFilter {
                property: property.clone(),
                op: FilterOperator::In,
                value: value_str,
            };

            (vec![], vec![filter], vec![])
        }
        Term::PropertyRegex(property, regex) => {
            let filter = ConceptSetFilter {
                property: property.clone(),
                op: FilterOperator::Regex,
                value: regex.clone(),
            };

            (vec![], vec![filter], vec![])
        }
        Term::MembershipValueSet(url) => (vec![], vec![], vec![url.clone()]),
        Term::MembershipAlias(_alias) => {
            // This should only be called from translate_term_with_aliases now
            // If we get here, something is wrong, so return empty
            (vec![], vec![], vec![])
        }
        Term::Exists(property) => {
            let filter = ConceptSetFilter {
                property: property.clone(),
                op: FilterOperator::Exists,
                value: "true".to_string(),
            };

            (vec![], vec![filter], vec![])
        }
    }
}

fn extract_code_from_ref(code_ref: &CodeRef) -> String {
    match code_ref {
        CodeRef::Code(code) => code.clone(),
        CodeRef::Quoted(quoted) => quoted.clone(),
    }
}

fn translate_value_to_string(value: &Value) -> String {
    match value {
        Value::Str(s) => s.clone(),
        Value::Code(c) => c.clone(),
        Value::Uri(u) => u.clone(),
        Value::Hierarchy(op, code_ref) => {
            let op_str = match op {
                HierarchyOp::DescOrSelf => "<<",
                HierarchyOp::DescOnly => "<",
                HierarchyOp::Isa => "isa",
            };
            format!("{} {}", op_str, extract_code_from_ref(code_ref))
        }
    }
}

/* =========================
Parsing utilities
========================= */

fn line_comment(i: &str) -> IResult<&str, ()> {
    value((), pair(tag("//"), take_while(|c| c != '\n')))(i)
}

fn block_comment(i: &str) -> IResult<&str, ()> {
    value((), delimited(tag("/*"), take_while(|_| true), tag("*/")))(i)
}

fn ws0(i: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((value((), multispace1), line_comment, block_comment))),
    )(i)
}

fn ws1(i: &str) -> IResult<&str, ()> {
    value((), alt((multispace1,)))(i)
}

fn sp<'a, O, F>(mut inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    move |i| {
        let (i, _) = ws0(i)?;
        let (i, out) = inner(i)?;
        let (i, _) = ws0(i)?;
        Ok((i, out))
    }
}

// keyword with word boundary (so "in" won't match "instrument")
fn kw<'a>(word: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |i: &str| {
        let (i2, m) = tag_no_case(word)(i)?;
        let next = i2.chars().next();
        if matches!(next, Some(c) if c.is_alphanumeric() || c == '_' ) {
            Err(nom::Err::Error(Error::new(i, ErrorKind::Tag)))
        } else {
            Ok((i2, m))
        }
    }
}

fn parse_ident(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
        )),
        |s: &str| s.to_string(),
    )(i)
}

fn is_code_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | ':' | '-')
}

fn parse_code(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_ascii_alphanumeric()),
            take_while(is_code_char),
        )),
        |s: &str| s.to_string(),
    )(i)
}

fn is_uri_char(c: char) -> bool {
    // exclude whitespace and separators we use: | ( ) , #
    !(c.is_whitespace() || matches!(c, '|' | '(' | ')' | ',' | '#'))
}

fn parse_uri(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((
                tag_no_case("http://"),
                tag_no_case("https://"),
                tag_no_case("urn:"),
            )),
            take_while1(is_uri_char),
        )),
        |s: &str| s.to_string(),
    )(i)
}

fn parse_string(i: &str) -> IResult<&str, String> {
    // JSON-like "..." with backslash escapes kept as-is (you can unescape if you want)
    map(
        delimited(
            char('"'),
            map(
                many0(alt((
                    map(tag(r#"\""#), |_| r#"\""#),
                    map(tag(r#"\\"#), |_| r#"\\"#),
                    map(is_not("\"\n\r"), |s: &str| s),
                ))),
                |parts: Vec<&str>| parts.concat(),
            ),
            char('"'),
        ),
        |s| s.replace(r#"\""#, "\"").replace(r#"\\"#, "\\"),
    )(i)
}

fn parse_regex_body(i: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('/'),
            map(
                many0(alt((
                    map(tag(r#"\/"#), |_| "/".to_string()),
                    map(is_not("/\n\r"), |s: &str| s.to_string()),
                ))),
                |v| v.concat(),
            ),
            char('/'),
        ),
        |s| s,
    )(i)
}

/* =========================
Grammar: start / headers / query
========================= */

pub fn parse_start(i: &str) -> IResult<&str, Start> {
    let (i, _) = ws0(i)?;
    let (i, headers) = many0(sp(parse_header))(i)?;
    let (i, expr) = sp(parse_expr)(i)?;
    Ok((i, Start { headers, expr }))
}

fn parse_header(i: &str) -> IResult<&str, Header> {
    let (i, _) = char('@')(i)?;
    let (i, _) = kw("alias")(i)?;
    let (i, _) = ws1(i)?;
    let (i, name) = parse_ident(i)?;
    let (i, _) = sp(char('='))(i)?;
    let (i, target) = alt((
        map(parse_value_set_ref, AliasTarget::ValueSetUrl),
        // Parse system reference with optional version (like in parse_clause)
        map(
            pair(
                parse_system_ref,
                opt(preceded(char('|'), parse_version_code)),
            ),
            |(system, version)| match version {
                Some(ver) => AliasTarget::SystemWithVersion(system, ver),
                _ => AliasTarget::System(system),
            },
        ),
    ))(i)?;
    Ok((i, Header::Alias { name, target }))
}

/* =========================
Expressions (outer)
precedence: ! > & > | > -
========================= */

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    parse_minus_expr(i)
}

fn parse_minus_expr(i: &str) -> IResult<&str, Expr> {
    let (i, init) = parse_or_expr(i)?;
    fold_many0(
        preceded(sp(char('-')), parse_or_expr),
        move || init.clone(),
        |acc, rhs| Expr::Minus(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_or_expr(i: &str) -> IResult<&str, Expr> {
    let (i, init) = parse_and_expr(i)?;
    fold_many0(
        preceded(sp(char('|')), parse_and_expr),
        move || init.clone(),
        |acc, rhs| Expr::Or(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_and_expr(i: &str) -> IResult<&str, Expr> {
    let (i, init) = parse_unary_expr(i)?;
    fold_many0(
        preceded(sp(char('&')), parse_unary_expr),
        move || init.clone(),
        |acc, rhs| Expr::And(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_unary_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(sp(char('!')), parse_unary_expr), |e| {
            Expr::Not(Box::new(e))
        }),
        map(sp(parse_clause), Expr::Clause),
        map(delimited(sp(char('(')), parse_expr, sp(char(')'))), |e| {
            Expr::Group(Box::new(e))
        }),
    ))(i)
}

/* =========================
Clause & innerExpr (same precedence stack)
========================= */

fn parse_clause(i: &str) -> IResult<&str, Clause> {
    let (i, system) = parse_system_ref(i)?;
    let (i, version) = opt(preceded(char('|'), parse_version_code))(i)?;
    let (i, _) = sp(char(':'))(i)?;
    let (i, inner) = parse_inner_expr(i)?;
    Ok((
        i,
        Clause {
            system,
            version,
            inner,
        },
    ))
}

// Parse version code (doesn't allow : as separator)
fn parse_version_code(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_ascii_alphanumeric()),
            take_while(|c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')),
        )),
        |s: &str| s.to_string(),
    )(i)
}

fn parse_inner_expr(i: &str) -> IResult<&str, InnerExpr> {
    parse_inner_minus(i)
}

fn parse_inner_minus(i: &str) -> IResult<&str, InnerExpr> {
    let (i, init) = parse_inner_or(i)?;
    fold_many0(
        preceded(sp(char('-')), parse_inner_or),
        move || init.clone(),
        |acc, rhs| InnerExpr::Minus(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_inner_or(i: &str) -> IResult<&str, InnerExpr> {
    let (i, init) = parse_inner_and(i)?;
    fold_many0(
        preceded(sp(char('|')), parse_inner_and),
        move || init.clone(),
        |acc, rhs| InnerExpr::Or(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_inner_and(i: &str) -> IResult<&str, InnerExpr> {
    let (i, init) = parse_inner_unary(i)?;
    fold_many0(
        preceded(sp(char('&')), parse_inner_unary),
        move || init.clone(),
        |acc, rhs| InnerExpr::And(Box::new(acc), Box::new(rhs)),
    )(i)
}

fn parse_inner_unary(i: &str) -> IResult<&str, InnerExpr> {
    alt((
        map(preceded(sp(char('!')), parse_inner_unary), |e| {
            InnerExpr::Not(Box::new(e))
        }),
        map(
            delimited(sp(char('(')), parse_inner_expr, sp(char(')'))),
            |e| InnerExpr::Group(Box::new(e)),
        ),
        map(sp(parse_term), InnerExpr::Term),
    ))(i)
}

/* =========================
Terms
========================= */

fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((
        parse_hierarchy_term,
        parse_property_term,
        parse_membership_term,
        parse_exists_term,
    ))(i)
}

fn parse_hierarchy_term(i: &str) -> IResult<&str, Term> {
    let (i, op) = alt((
        value(HierarchyOp::DescOrSelf, tag("<<")),
        value(HierarchyOp::DescOnly, tag("<")),
        map(kw("isa"), |_| HierarchyOp::Isa),
    ))(i)?;
    let (i, _) = ws0(i)?;
    let (i, code) = parse_code_ref(i)?;
    Ok((i, Term::Hierarchy(op, code)))
}

fn parse_property_term(i: &str) -> IResult<&str, Term> {
    let (i, prop) = parse_prop_ref(i)?;
    let (i, _) = ws0(i)?;
    let prop1 = prop.clone();
    let prop2 = prop.clone();
    let prop3 = prop;
    alt((
        // prop = value
        map(preceded(sp(char('=')), parse_value), move |v| {
            Term::PropertyEq(prop1.clone(), v)
        }),
        // prop in (v1, v2, ...)
        map(
            preceded(
                tuple((ws0, kw("in"), ws0, char('('), ws0)),
                cut(terminated(
                    separated_list0(sp(char(',')), parse_value),
                    cut(tuple((ws0, char(')')))),
                )),
            ),
            move |list| Term::PropertyIn(prop2.clone(), list),
        ),
        // prop ~ /regex/
        map(
            preceded(tuple((ws0, tag("~"), ws0)), parse_regex_body),
            move |re| Term::PropertyRegex(prop3.clone(), re),
        ),
    ))(i)
}

fn parse_membership_term(i: &str) -> IResult<&str, Term> {
    let (i, _) = kw("in")(i)?;
    let (i, _) = ws0(i)?;
    alt((
        map(parse_value_set_ref, Term::MembershipValueSet),
        map(preceded(char('#'), parse_ident), Term::MembershipAlias),
    ))(i)
}

fn parse_exists_term(i: &str) -> IResult<&str, Term> {
    let (i, _) = kw("has")(i)?;
    let (i, _) = ws1(i)?;
    map(parse_prop_ref, Term::Exists)(i)
}

fn parse_prop_ref(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(parse_ident, many0(preceded(char('.'), parse_ident)))),
        |s: &str| s.to_string(),
    )(i)
}

fn parse_code_ref(i: &str) -> IResult<&str, CodeRef> {
    alt((
        map(parse_string, CodeRef::Quoted),
        map(parse_code, CodeRef::Code),
    ))(i)
}

fn parse_value(i: &str) -> IResult<&str, Value> {
    alt((
        map(parse_string, Value::Str),
        map(parse_uri, Value::Uri),
        // Parse hierarchy expressions like << 49755003
        map(
            pair(
                alt((
                    value(HierarchyOp::DescOrSelf, tag("<<")),
                    value(HierarchyOp::DescOnly, tag("<")),
                    map(kw("isa"), |_| HierarchyOp::Isa),
                )),
                preceded(ws0, parse_code_ref),
            ),
            |(op, code)| Value::Hierarchy(op, code),
        ),
        map(parse_code, Value::Code),
    ))(i)
}

fn parse_system_ref(i: &str) -> IResult<&str, SystemRef> {
    alt((
        map(parse_uri_for_system, SystemRef::Uri),
        map(parse_ident, SystemRef::Alias),
    ))(i)
}

// Parse URI but stop at | or : when followed by whitespace (indicating separators)
fn parse_uri_for_system(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((
                tag_no_case("http://"),
                tag_no_case("https://"),
                tag_no_case("urn:"),
            )),
            take_while1(|c| is_uri_char(c) && !(c == '|' || c == ':')),
        )),
        |s: &str| s.to_string(),
    )(i)
}

fn parse_value_set_ref(i: &str) -> IResult<&str, String> {
    delimited(
        tag_no_case("vs"),
        delimited(sp(char('(')), parse_uri, sp(char(')'))),
        ws0,
    )(i)
}

/* =========================
Debug pretty
========================= */

impl fmt::Display for SystemRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemRef::Uri(u) => write!(f, "{u}"),
            SystemRef::Alias(a) => write!(f, "{a}"),
        }
    }
}

/* =========================
FHIR ValueSet.compose structures
========================= */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetCompose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ConceptSetComponent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<ConceptSetComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSetComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<ConceptReferenceComponent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<ConceptSetFilter>,
    #[serde(rename = "valueSet", skip_serializing_if = "Vec::is_empty")]
    pub value_set: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptReferenceComponent {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSetFilter {
    pub property: String,
    pub op: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilterOperator {
    #[serde(rename = "=")]
    Equals,
    #[serde(rename = "is-a")]
    IsA,
    #[serde(rename = "descendent-of")]
    DescendentOf,
    #[serde(rename = "is-not-a")]
    IsNotA,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "not-in")]
    NotIn,
    #[serde(rename = "generalizes")]
    Generalizes,
    #[serde(rename = "exists")]
    Exists,
}

/* =========================
Tests
========================= */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_basic_hierarchy() {
        let input = "http://snomed.info/sct: < 22298006";
        let (_r, ast) = parse_start(input).unwrap();
        match ast.expr {
            Expr::Clause(Clause { inner, .. }) => match inner {
                InnerExpr::Term(Term::Hierarchy(HierarchyOp::DescOnly, CodeRef::Code(c))) => {
                    assert_eq!(c, "22298006");
                }
                _ => panic!("unexpected inner AST"),
            },
            _ => panic!("not a clause"),
        }
    }

    #[test]
    fn t_props_and_in_list() {
        let input = r#"http://loinc.org: component = "Glucose" & method in ("A","B")"#;
        let (_r, _ast) = parse_start(input).unwrap();
    }

    #[test]
    fn t_alias_and_membership() {
        let input = r#"
          @alias sct = http://snomed.info/sct|20250131
          @alias dm  = vs(https://example.org/fhir/ValueSet/diabetes)
          sct: << 73211009 | sct: in #dm - sct: << 44054006
        "#;
        let (_r, _ast) = parse_start(input).unwrap();
    }

    #[test]
    fn t_sample2_with_version() {
        let input =
            "http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003";
        let (_r, ast) = parse_start(input).unwrap();

        // Verify it parses as expected
        match ast.expr {
            Expr::Clause(Clause {
                system,
                version,
                inner,
            }) => {
                assert!(matches!(system, SystemRef::Uri(_)));
                assert_eq!(version, Some("20250731".to_string()));
                assert!(matches!(inner, InnerExpr::And(_, _)));
            }
            _ => panic!("Expected clause with version"),
        }
    }

    #[test]
    fn t_fhir_translation_basic_hierarchy() {
        let input = "http://snomed.info/sct: < 22298006";
        let (_r, ast) = parse_start(input).unwrap();
        let compose = translate_to_fhir(&ast);

        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 0);

        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.version, None);
        assert_eq!(include.filter.len(), 1);

        let filter = &include.filter[0];
        assert_eq!(filter.property, "concept");
        assert!(matches!(filter.op, FilterOperator::DescendentOf));
        assert_eq!(filter.value, "22298006");
    }

    #[test]
    fn t_fhir_translation_minus_operation() {
        // Test: sct: << 22298006 - << 1755008
        // Should create one include and one exclude block
        let input = "sct: << 22298006 - << 1755008";
        let (_r, ast) = parse_start(input).unwrap();
        let fhir = translate_to_fhir(&ast);

        // Should have 1 include and 1 exclude
        assert_eq!(fhir.include.len(), 1);
        assert_eq!(fhir.exclude.len(), 1);

        // Check include block
        let include = &fhir.include[0];
        assert_eq!(include.system, None); // sct alias not resolved
        assert_eq!(include.filter.len(), 1);
        assert_eq!(include.filter[0].property, "concept");
        assert!(matches!(include.filter[0].op, FilterOperator::IsA));
        assert_eq!(include.filter[0].value, "22298006");

        // Check exclude block
        let exclude = &fhir.exclude[0];
        assert_eq!(exclude.system, None); // sct alias not resolved
        assert_eq!(exclude.filter.len(), 1);
        assert_eq!(exclude.filter[0].property, "concept");
        assert!(matches!(exclude.filter[0].op, FilterOperator::IsA));
        assert_eq!(exclude.filter[0].value, "1755008");
    }

    #[test]
    fn t_fhir_translation_not_operation() {
        // Test: sct: << 404684003 & ! (associatedMorphology = << 49755003)
        // Should create one include and one exclude block
        let input = "sct: << 404684003 & ! (associatedMorphology = << 49755003)";
        let (_r, ast) = parse_start(input).unwrap();
        let fhir = translate_to_fhir(&ast);

        // Should have 1 include and 1 exclude
        assert_eq!(fhir.include.len(), 1);
        assert_eq!(fhir.exclude.len(), 1);

        // Check include block
        let include = &fhir.include[0];
        assert_eq!(include.system, None); // sct alias not resolved
        assert_eq!(include.filter.len(), 1);
        assert_eq!(include.filter[0].property, "concept");
        assert!(matches!(include.filter[0].op, FilterOperator::IsA));
        assert_eq!(include.filter[0].value, "404684003");

        // Check exclude block
        let exclude = &fhir.exclude[0];
        assert_eq!(exclude.system, None); // sct alias not resolved
        assert_eq!(exclude.filter.len(), 1);
        assert_eq!(exclude.filter[0].property, "associatedMorphology");
        assert!(matches!(exclude.filter[0].op, FilterOperator::IsA));
        assert_eq!(exclude.filter[0].value, "49755003");
    }

    #[test]
    fn t_fhir_translation_versioned_alias() {
        // Test versioned alias: @alias sct = http://snomed.info/sct|20250131
        let input = r#"
          @alias sct = http://snomed.info/sct|20250131
          sct: < 22298006
        "#;
        let (_r, ast) = parse_start(input).unwrap();
        let compose = translate_to_fhir(&ast);

        assert_eq!(compose.include.len(), 1);

        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.version, Some("20250131".to_string()));
        assert_eq!(include.filter.len(), 1);
        assert_eq!(include.filter[0].property, "concept");
        assert!(matches!(include.filter[0].op, FilterOperator::DescendentOf));
        assert_eq!(include.filter[0].value, "22298006");
    }

    #[test]
    fn t_fhir_translation_with_aliases() {
        let input = r#"
          @alias sct = http://snomed.info/sct|20250131
          @alias dm  = vs(https://example.org/fhir/ValueSet/diabetes)
          sct: << 73211009 | sct: in #dm - sct: << 44054006
        "#;
        let (_r, ast) = parse_start(input).unwrap();
        let compose = translate_to_fhir(&ast);

        // Should have includes and excludes
        assert!(!compose.include.is_empty());
        assert!(!compose.exclude.is_empty());

        // Check that aliases are resolved
        let has_system = compose
            .include
            .iter()
            .any(|inc| inc.system == Some("http://snomed.info/sct".to_string()));
        assert!(has_system, "System alias should be resolved");

        let has_valueset = compose.include.iter().any(|inc| {
            inc.value_set
                .contains(&"https://example.org/fhir/ValueSet/diabetes".to_string())
        });
        assert!(has_valueset, "ValueSet alias should be resolved");
    }

    #[test]
    fn t_fhir_translation_property_filters() {
        let input = r#"http://loinc.org: component = "Glucose" & method in ("Automated count","Manual count")"#;
        let (_r, ast) = parse_start(input).unwrap();
        let compose = translate_to_fhir(&ast);

        // With the new expansion logic, PropertyIn creates multiple include blocks
        assert_eq!(compose.include.len(), 2);

        // Each include block should have the component filter plus one method value
        for include in &compose.include {
            assert_eq!(include.system, Some("http://loinc.org".to_string()));
            assert_eq!(include.filter.len(), 2);

            // Check for component = "Glucose" filter
            let component_filter = include.filter.iter().find(|f| f.property == "component");
            assert!(component_filter.is_some());
            let component_filter = component_filter.unwrap();
            assert!(matches!(component_filter.op, FilterOperator::Equals));
            assert_eq!(component_filter.value, "Glucose");

            // Check for method filter (should be equals, not in)
            let method_filter = include.filter.iter().find(|f| f.property == "method");
            assert!(method_filter.is_some());
            let method_filter = method_filter.unwrap();
            assert!(matches!(method_filter.op, FilterOperator::Equals));
            assert!(
                method_filter.value == "Automated count" || method_filter.value == "Manual count"
            );
        }

        // Verify we have both method values across the two include blocks
        let method_values: Vec<String> = compose
            .include
            .iter()
            .flat_map(|inc| inc.filter.iter())
            .filter(|f| f.property == "method")
            .map(|f| f.value.clone())
            .collect();
        assert_eq!(method_values.len(), 2);
        assert!(method_values.contains(&"Automated count".to_string()));
        assert!(method_values.contains(&"Manual count".to_string()));
    }
}
