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

fn main() {
    let samples = &[
        r#"http://snomed.info/sct: < 22298006"#,
        r#"http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003"#,
        r#"http://loinc.org: component = "Glucose" & method in ("Automated count","Manual count")"#,
        r#"
        @alias sct = http://snomed.info/sct|20250131
        @alias dm  = vs(https://example.org/fhir/ValueSet/diabetes)
        sct: << 73211009 | sct: in #dm - sct: << 44054006
        "#,
    ];

    for (i, s) in samples.iter().enumerate() {
        println!("\n=== SAMPLE {} ===\n{}", i + 1, s.trim());
        match parse_start(s) {
            Ok((rest, ast)) => {
                println!("\nAST:\n{:#?}", ast);
                if !rest.trim().is_empty() {
                    println!("\n[WARN] Unparsed tail: {:?}", rest);
                }
                
                // Translate to FHIR ValueSet.compose
                let compose = translate_to_fhir(&ast);
                println!("\nFHIR ValueSet.compose:");
                match serde_json::to_string_pretty(&compose) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("JSON serialization error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Parse error: {:?}", e);
            }
        }
    }
}

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

fn translate_expression(expr: &Expr, aliases: &HashMap<String, AliasTarget>) -> (Vec<ConceptSetComponent>, Vec<ConceptSetComponent>) {
    match expr {
        Expr::Clause(clause) => {
            let component = translate_clause(clause, aliases);
            (vec![component], vec![])
        }
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
        Expr::Group(inner) => {
            translate_expression(inner, aliases)
        }
    }
}

fn translate_clause(clause: &Clause, aliases: &HashMap<String, AliasTarget>) -> ConceptSetComponent {
    let system = resolve_system_ref(&clause.system, aliases);
    let version = clause.version.clone();
    
    let (concept, filter, mut value_set) = translate_inner_expr_with_aliases(&clause.inner, aliases);
    
    // Remove any unresolved alias markers and resolve them
    value_set = value_set.into_iter().map(|vs| {
        if vs.starts_with('#') {
            let alias_name = &vs[1..]; // Remove the '#' prefix
            match aliases.get(alias_name) {
                Some(AliasTarget::ValueSetUrl(url)) => url.clone(),
                _ => vs, // Keep original if can't resolve
            }
        } else {
            vs
        }
    }).collect();
    
    ConceptSetComponent {
        system,
        version,
        concept,
        filter,
        value_set,
    }
}

fn resolve_system_ref(system_ref: &SystemRef, aliases: &HashMap<String, AliasTarget>) -> Option<String> {
    match system_ref {
        SystemRef::Uri(uri) => Some(uri.clone()),
        SystemRef::Alias(alias_name) => {
            match aliases.get(alias_name) {
                Some(AliasTarget::System(SystemRef::Uri(uri))) => Some(uri.clone()),
                Some(AliasTarget::System(SystemRef::Alias(nested_alias))) => {
                    // Handle nested aliases (though unlikely in practice)
                    resolve_system_ref(&SystemRef::Alias(nested_alias.clone()), aliases)
                }
                Some(AliasTarget::ValueSetUrl(_)) => None, // This alias points to a ValueSet, not a system
                None => None, // Unresolved alias
            }
        }
    }
}

fn translate_inner_expr_with_aliases(inner: &InnerExpr, aliases: &HashMap<String, AliasTarget>) -> (Vec<ConceptReferenceComponent>, Vec<ConceptSetFilter>, Vec<String>) {
    match inner {
        InnerExpr::Term(term) => translate_term_with_aliases(term, aliases),
        InnerExpr::And(left, right) => {
            let (mut concepts, mut filters, mut value_sets) = translate_inner_expr_with_aliases(left, aliases);
            let (right_concepts, right_filters, right_value_sets) = translate_inner_expr_with_aliases(right, aliases);
            
            concepts.extend(right_concepts);
            filters.extend(right_filters);
            value_sets.extend(right_value_sets);
            
            (concepts, filters, value_sets)
        }
        InnerExpr::Or(left, right) => {
            let (mut concepts, mut filters, mut value_sets) = translate_inner_expr_with_aliases(left, aliases);
            let (right_concepts, right_filters, right_value_sets) = translate_inner_expr_with_aliases(right, aliases);
            
            concepts.extend(right_concepts);
            filters.extend(right_filters);
            value_sets.extend(right_value_sets);
            
            (concepts, filters, value_sets)
        }
        InnerExpr::Minus(_left, _right) => {
            (vec![], vec![], vec![])
        }
        InnerExpr::Not(_inner) => {
            (vec![], vec![], vec![])
        }
        InnerExpr::Group(inner) => translate_inner_expr_with_aliases(inner, aliases),
    }
}



fn translate_term_with_aliases(term: &Term, aliases: &HashMap<String, AliasTarget>) -> (Vec<ConceptReferenceComponent>, Vec<ConceptSetFilter>, Vec<String>) {
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
fn translate_term(term: &Term) -> (Vec<ConceptReferenceComponent>, Vec<ConceptSetFilter>, Vec<String>) {
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
            let filter = ConceptSetFilter {
                property: property.clone(),
                op: FilterOperator::Equals,
                value: translate_value_to_string(value),
            };
            
            (vec![], vec![filter], vec![])
        }
        Term::PropertyIn(property, values) => {
            let value_str = values.iter()
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
        Term::MembershipValueSet(url) => {
            (vec![], vec![], vec![url.clone()])
        }
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
    value((), many0(alt((
        value((), multispace1),
        line_comment,
        block_comment,
    ))))(i)
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
            alt((tag_no_case("http://"), tag_no_case("https://"), tag_no_case("urn:"))),
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

fn parse_start(i: &str) -> IResult<&str, Start> {
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
            pair(parse_system_ref, opt(preceded(char('|'), parse_version_code))),
            |(system, _version)| AliasTarget::System(system)
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
        |acc, rhs| Expr::Minus(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_or_expr(i: &str) -> IResult<&str, Expr> {
    let (i, init) = parse_and_expr(i)?;
    fold_many0(
        preceded(sp(char('|')), parse_and_expr),
        move || init.clone(),
        |acc, rhs| Expr::Or(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_and_expr(i: &str) -> IResult<&str, Expr> {
    let (i, init) = parse_unary_expr(i)?;
    fold_many0(
        preceded(sp(char('&')), parse_unary_expr),
        move || init.clone(),
        |acc, rhs| Expr::And(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_unary_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(sp(char('!')), parse_unary_expr), |e| Expr::Not(Box::new(e))),
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
    Ok((i, Clause { system, version, inner }))
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
        |acc, rhs| InnerExpr::Minus(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_inner_or(i: &str) -> IResult<&str, InnerExpr> {
    let (i, init) = parse_inner_and(i)?;
    fold_many0(
        preceded(sp(char('|')), parse_inner_and),
        move || init.clone(),
        |acc, rhs| InnerExpr::Or(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_inner_and(i: &str) -> IResult<&str, InnerExpr> {
    let (i, init) = parse_inner_unary(i)?;
    fold_many0(
        preceded(sp(char('&')), parse_inner_unary),
        move || init.clone(),
        |acc, rhs| InnerExpr::And(Box::new(acc), Box::new(rhs))
    )(i)
}

fn parse_inner_unary(i: &str) -> IResult<&str, InnerExpr> {
    alt((
        map(preceded(sp(char('!')), parse_inner_unary), |e| {
            InnerExpr::Not(Box::new(e))
        }),
        map(delimited(sp(char('(')), parse_inner_expr, sp(char(')'))), |e| {
            InnerExpr::Group(Box::new(e))
        }),
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
        map(preceded(tuple((ws0, tag("~"), ws0)), parse_regex_body), move |re| {
            Term::PropertyRegex(prop3.clone(), re)
        }),
    ))(i)
}

fn parse_membership_term(i: &str) -> IResult<&str, Term> {
    let (i, _) = kw("in")(i)?;
    let (i, _) = ws0(i)?;
    alt((
        map(parse_value_set_ref, |url| Term::MembershipValueSet(url)),
        map(preceded(char('#'), parse_ident), |a| Term::MembershipAlias(a)),
    ))(i)
}

fn parse_exists_term(i: &str) -> IResult<&str, Term> {
    let (i, _) = kw("has")(i)?;
    let (i, _) = ws1(i)?;
    map(parse_prop_ref, Term::Exists)(i)
}

fn parse_prop_ref(i: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            parse_ident,
            many0(preceded(char('.'), parse_ident)),
        )),
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
                preceded(ws0, parse_code_ref)
            ),
            |(op, code)| Value::Hierarchy(op, code)
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
            alt((tag_no_case("http://"), tag_no_case("https://"), tag_no_case("urn:"))),
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
            SystemRef::Uri(u) => write!(f, "{}", u),
            SystemRef::Alias(a) => write!(f, "{}", a),
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
            Expr::Clause(Clause { inner, .. }) => {
                match inner {
                    InnerExpr::Term(Term::Hierarchy(HierarchyOp::DescOnly, CodeRef::Code(c))) => {
                        assert_eq!(c, "22298006");
                    }
                    _ => panic!("unexpected inner AST"),
                }
            }
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
        let input = "http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003";
        let (_r, ast) = parse_start(input).unwrap();
        
        // Verify it parses as expected
        match ast.expr {
            Expr::Clause(Clause { system, version, inner }) => {
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
        let has_system = compose.include.iter().any(|inc| {
            inc.system == Some("http://snomed.info/sct".to_string())
        });
        assert!(has_system, "System alias should be resolved");
        
        let has_valueset = compose.include.iter().any(|inc| {
            inc.value_set.contains(&"https://example.org/fhir/ValueSet/diabetes".to_string())
        });
        assert!(has_valueset, "ValueSet alias should be resolved");
    }

    #[test]
    fn t_fhir_translation_property_filters() {
        let input = r#"http://loinc.org: component = "Glucose" & method in ("Automated count","Manual count")"#;
        let (_r, ast) = parse_start(input).unwrap();
        let compose = translate_to_fhir(&ast);
        
        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://loinc.org".to_string()));
        assert_eq!(include.filter.len(), 2);
        
        // Check for component = "Glucose" filter
        let component_filter = include.filter.iter().find(|f| f.property == "component");
        assert!(component_filter.is_some());
        let component_filter = component_filter.unwrap();
        assert!(matches!(component_filter.op, FilterOperator::Equals));
        assert_eq!(component_filter.value, "Glucose");
        
        // Check for method in (...) filter
        let method_filter = include.filter.iter().find(|f| f.property == "method");
        assert!(method_filter.is_some());
        let method_filter = method_filter.unwrap();
        assert!(matches!(method_filter.op, FilterOperator::In));
        assert_eq!(method_filter.value, "Automated count,Manual count");
    }
}

