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
}

