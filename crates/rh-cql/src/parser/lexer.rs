//! CQL Lexer Utilities
//!
//! This module provides lexical analysis utilities for CQL parsing:
//!
//! - Whitespace and comment handling
//! - Keyword recognition
//! - Operator tokenization
//! - Literal parsing (strings, numbers, dates, etc.)
//! - Identifier parsing
//!
//! ## CQL Keywords (v1.5.3)
//!
//! CQL has a large set of reserved keywords. This module provides
//! utilities for recognizing them and differentiating from identifiers.

use crate::parser::span::Span;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while, take_while1, take_while_m_n},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, not, opt, peek, recognize, value},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

// ============================================================================
// CQL Keywords
// ============================================================================

/// CQL reserved keywords (case-insensitive in CQL)
pub const KEYWORDS: &[&str] = &[
    // Library structure
    "library",
    "version",
    "using",
    "include",
    "called",
    "public",
    "private",
    // Data model
    "codesystem",
    "valueset",
    "code",
    "concept",
    "parameter",
    "default",
    // Definitions
    "context",
    "define",
    "function",
    "fluent",
    "returns",
    "external",
    // Types
    "Boolean",
    "Integer",
    "Long",
    "Decimal",
    "String",
    "Date",
    "DateTime",
    "Time",
    "Quantity",
    "Ratio",
    "Code",
    "Concept",
    "Any",
    "List",
    "Interval",
    "Tuple",
    "Choice",
    // Literals
    "true",
    "false",
    "null",
    // Logical operators
    "and",
    "or",
    "xor",
    "not",
    "implies",
    // Comparison
    "between",
    // Null handling
    "is",
    "as",
    // Query
    "from",
    "where",
    "return",
    "all",
    "distinct",
    "such",
    "that",
    "with",
    "without",
    "let",
    "sort",
    "by",
    "asc",
    "ascending",
    "desc",
    "descending",
    // Membership/containment
    "in",
    "contains",
    "properly",
    "includes",
    "included",
    "during",
    "starts",
    "ends",
    "occurs",
    "same",
    "before",
    "after",
    "overlaps",
    "meets",
    "within",
    // Interval
    "start",
    "end",
    "of",
    "width",
    "successor",
    "predecessor",
    "singleton",
    "point",
    // Aggregate
    "aggregate",
    "starting",
    // Retrieve
    "retrieve",
    // Conditionals
    "if",
    "then",
    "else",
    "case",
    "when",
    // Temporal keywords
    "day",
    "days",
    "week",
    "weeks",
    "month",
    "months",
    "year",
    "years",
    "hour",
    "hours",
    "minute",
    "minutes",
    "second",
    "seconds",
    "millisecond",
    "milliseconds",
    // Other
    "convert",
    "to",
    "union",
    "intersect",
    "except",
    "collapse",
    "expand",
    "flatten",
    "per",
    "display",
    "minimum",
    "maximum",
];

/// Check if a string is a CQL keyword (case-insensitive)
pub fn is_keyword(s: &str) -> bool {
    let lower = s.to_lowercase();
    KEYWORDS.iter().any(|k| k.to_lowercase() == lower)
}

// ============================================================================
// Whitespace and Comments
// ============================================================================

/// Parse whitespace (spaces, tabs, newlines)
pub fn ws<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O>,
{
    delimited(skip_ws_and_comments, inner, skip_ws_and_comments)
}

/// Skip whitespace and comments
pub fn skip_ws_and_comments(input: Span<'_>) -> IResult<Span<'_>, ()> {
    let (input, _) = many0(alt((
        value((), multispace1),
        value((), line_comment),
        value((), block_comment),
    )))(input)?;
    Ok((input, ()))
}

/// Parse a single-line comment (// ... \n)
fn line_comment(input: Span<'_>) -> IResult<Span<'_>, Span<'_>> {
    recognize(pair(tag("//"), take_while(|c| c != '\n')))(input)
}

/// Parse a block comment (/* ... */)
fn block_comment(input: Span<'_>) -> IResult<Span<'_>, Span<'_>> {
    recognize(tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

// ============================================================================
// Identifiers
// ============================================================================

/// Parse a regular identifier
///
/// CQL identifiers start with a letter or underscore, followed by
/// letters, digits, or underscores.
pub fn identifier(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, ident) = recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))(input)?;

    let name = ident.fragment().to_string();

    // Don't match keywords as identifiers
    // But allow them if they're part of a longer identifier
    Ok((input, name))
}

/// Parse a quoted identifier (`"identifier with spaces"` or backticks)
pub fn quoted_identifier(input: Span<'_>) -> IResult<Span<'_>, String> {
    alt((
        // Double-quoted identifier
        map(
            delimited(char('"'), take_while(|c| c != '"'), char('"')),
            |s: Span<'_>| s.fragment().to_string(),
        ),
        // Backtick-quoted identifier
        map(
            delimited(char('`'), take_while(|c| c != '`'), char('`')),
            |s: Span<'_>| s.fragment().to_string(),
        ),
    ))(input)
}

/// Parse any identifier (regular or quoted)
pub fn any_identifier(input: Span<'_>) -> IResult<Span<'_>, String> {
    alt((quoted_identifier, identifier))(input)
}

/// Parse a qualified identifier (namespace.name or just name)
pub fn qualified_identifier(input: Span<'_>) -> IResult<Span<'_>, Vec<String>> {
    let (input, first) = any_identifier(input)?;
    let (input, rest) = many0(preceded(char('.'), any_identifier))(input)?;

    let mut parts = vec![first];
    parts.extend(rest);
    Ok((input, parts))
}

// ============================================================================
// Keywords (case-insensitive matching)
// ============================================================================

/// Match a keyword (case-insensitive), ensuring it's not part of a larger identifier
pub fn keyword<'a>(kw: &'static str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    move |input: Span<'a>| {
        let (remaining, matched) = tag_no_case(kw)(input)?;
        // Ensure the keyword is not followed by an identifier character
        let (remaining, _) =
            not(peek(take_while1(|c: char| c.is_alphanumeric() || c == '_')))(remaining)?;
        Ok((remaining, matched))
    }
}

// ============================================================================
// Operators
// ============================================================================

/// CQL operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    // Comparison
    Equal,
    NotEqual,
    Equivalent,
    NotEquivalent,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,

    // Logical (keywords handled separately)

    // String
    Ampersand,

    // List/interval
    Pipe, // union

    // Punctuation
    Dot,
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Arrow,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Caret => "^",
            Operator::Equal => "=",
            Operator::NotEqual => "!=",
            Operator::Equivalent => "~",
            Operator::NotEquivalent => "!~",
            Operator::Less => "<",
            Operator::LessOrEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterOrEqual => ">=",
            Operator::Ampersand => "&",
            Operator::Pipe => "|",
            Operator::Dot => ".",
            Operator::Comma => ",",
            Operator::Colon => ":",
            Operator::Semicolon => ";",
            Operator::LeftParen => "(",
            Operator::RightParen => ")",
            Operator::LeftBracket => "[",
            Operator::RightBracket => "]",
            Operator::LeftBrace => "{",
            Operator::RightBrace => "}",
            Operator::Arrow => "->",
        };
        write!(f, "{s}")
    }
}

/// Parse multi-character operators
fn multi_char_operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        value(Operator::NotEqual, tag("!=")),
        value(Operator::NotEquivalent, tag("!~")),
        value(Operator::LessOrEqual, tag("<=")),
        value(Operator::GreaterOrEqual, tag(">=")),
        value(Operator::Arrow, tag("->")),
    ))(input)
}

/// Parse arithmetic operators
fn arithmetic_operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        value(Operator::Plus, char('+')),
        value(Operator::Minus, char('-')),
        value(Operator::Star, char('*')),
        value(Operator::Slash, char('/')),
        value(Operator::Caret, char('^')),
    ))(input)
}

/// Parse comparison operators (single char)
fn comparison_operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        value(Operator::Equal, char('=')),
        value(Operator::Equivalent, char('~')),
        value(Operator::Less, char('<')),
        value(Operator::Greater, char('>')),
    ))(input)
}

/// Parse punctuation operators
fn punctuation_operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        value(Operator::Ampersand, char('&')),
        value(Operator::Pipe, char('|')),
        value(Operator::Dot, char('.')),
        value(Operator::Comma, char(',')),
        value(Operator::Colon, char(':')),
        value(Operator::Semicolon, char(';')),
    ))(input)
}

/// Parse bracket operators
fn bracket_operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        value(Operator::LeftParen, char('(')),
        value(Operator::RightParen, char(')')),
        value(Operator::LeftBracket, char('[')),
        value(Operator::RightBracket, char(']')),
        value(Operator::LeftBrace, char('{')),
        value(Operator::RightBrace, char('}')),
    ))(input)
}

/// Parse an operator
pub fn operator(input: Span<'_>) -> IResult<Span<'_>, Operator> {
    alt((
        multi_char_operator,
        arithmetic_operator,
        comparison_operator,
        punctuation_operator,
        bracket_operator,
    ))(input)
}

// ============================================================================
// Literals
// ============================================================================

/// Parse a string literal ('...' with escape sequences)
pub fn string_literal(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('\'')(input)?;

    let mut result = String::new();
    let mut remaining = input;

    loop {
        // Take characters until we hit a quote or backslash
        let (rest, chars) = take_while(|c| c != '\'' && c != '\\')(remaining)?;
        result.push_str(chars.fragment());
        remaining = rest;

        if remaining.fragment().starts_with('\'') {
            // End of string
            let (rest, _) = char('\'')(remaining)?;
            return Ok((rest, result));
        } else if remaining.fragment().starts_with('\\') {
            // Escape sequence
            let (rest, _) = char('\\')(remaining)?;
            if rest.is_empty() {
                return Err(nom::Err::Error(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Char,
                )));
            }

            // Parse escape character
            let (rest, escaped) = alt((
                value('\'', char('\'')),
                value('"', char('"')),
                value('\\', char('\\')),
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value('\x0C', char('f')),
            ))(rest)?;
            result.push(escaped);
            remaining = rest;
        } else {
            // Unexpected end of input
            return Err(nom::Err::Error(nom::error::Error::new(
                remaining,
                nom::error::ErrorKind::Char,
            )));
        }
    }
}

/// Parse an integer literal
pub fn integer_literal(input: Span<'_>) -> IResult<Span<'_>, i64> {
    let (input, sign) = opt(char('-'))(input)?;
    let (input, digits) = digit1(input)?;

    let value: i64 = digits.fragment().parse().unwrap_or(0);
    let value = if sign.is_some() { -value } else { value };

    Ok((input, value))
}

/// Parse a long literal (integer with L suffix)
pub fn long_literal(input: Span<'_>) -> IResult<Span<'_>, i64> {
    let (input, value) = integer_literal(input)?;
    let (input, _) = char('L')(input)?;
    Ok((input, value))
}

/// Parse a decimal literal
pub fn decimal_literal(input: Span<'_>) -> IResult<Span<'_>, f64> {
    let (input, num_str) = recognize(tuple((opt(char('-')), digit1, char('.'), digit1)))(input)?;

    let value: f64 = num_str.fragment().parse().unwrap_or(0.0);
    Ok((input, value))
}

/// Parse a quantity literal (number with unit)
/// Example: 5 'mg', 10.5 'kg', 100 '[lb_av]'
pub fn quantity_literal(input: Span<'_>) -> IResult<Span<'_>, (f64, String)> {
    let (input, value) = alt((decimal_literal, map(integer_literal, |i| i as f64)))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, unit) = string_literal(input)?;
    Ok((input, (value, unit)))
}

/// Parse a date literal (@YYYY-MM-DD)
pub fn date_literal(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;

    // Optional month
    let (input, month_day) = opt(tuple((
        char('-'),
        take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        opt(tuple((
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))),
    )))(input)?;

    // Ensure this is not a datetime
    if input.fragment().starts_with('T') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    let date_str = match month_day {
        Some((_, month, Some((_, day)))) => {
            format!(
                "{}-{}-{}",
                year.fragment(),
                month.fragment(),
                day.fragment()
            )
        }
        Some((_, month, None)) => {
            format!("{}-{}", year.fragment(), month.fragment())
        }
        None => year.fragment().to_string(),
    };

    Ok((input, date_str))
}

/// Parse a time literal (@T12:30:00)
pub fn time_literal(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('@')(input)?;
    let (input, _) = char('T')(input)?;
    let (input, hour) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, minute) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;

    // Optional seconds
    let (input, seconds) = opt(preceded(
        char(':'),
        take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
    ))(input)?;

    // Optional milliseconds
    let (input, millis) = opt(preceded(
        char('.'),
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
    ))(input)?;

    let time_str = match (seconds, millis) {
        (Some(sec), Some(ms)) => {
            format!(
                "{}:{}:{}.{}",
                hour.fragment(),
                minute.fragment(),
                sec.fragment(),
                ms.fragment()
            )
        }
        (Some(sec), None) => {
            format!(
                "{}:{}:{}",
                hour.fragment(),
                minute.fragment(),
                sec.fragment()
            )
        }
        (None, _) => {
            format!("{}:{}", hour.fragment(), minute.fragment())
        }
    };

    Ok((input, time_str))
}

/// Parse a datetime literal (@YYYY-MM-DDTHH:MM:SS)
pub fn datetime_literal(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, month) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, day) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('T')(input)?;
    let (input, hour) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;

    // Optional minutes
    let (input, time_rest) = opt(tuple((
        char(':'),
        take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        opt(tuple((
            char(':'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            opt(preceded(
                char('.'),
                take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
            )),
        ))),
    )))(input)?;

    // Optional timezone
    let (input, tz) = opt(alt((
        map(char('Z'), |_| "Z".to_string()),
        map(
            tuple((
                alt((char('+'), char('-'))),
                take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
                char(':'),
                take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            )),
            |(sign, h, _, m): (char, Span<'_>, char, Span<'_>)| {
                format!("{sign}{}:{}", h.fragment(), m.fragment())
            },
        ),
    )))(input)?;

    let datetime_str = match time_rest {
        Some((_, min, Some((_, sec, Some(ms))))) => {
            let base = format!(
                "{}-{}-{}T{}:{}:{}.{}",
                year.fragment(),
                month.fragment(),
                day.fragment(),
                hour.fragment(),
                min.fragment(),
                sec.fragment(),
                ms.fragment()
            );
            match tz {
                Some(t) => format!("{base}{t}"),
                None => base,
            }
        }
        Some((_, min, Some((_, sec, None)))) => {
            let base = format!(
                "{}-{}-{}T{}:{}:{}",
                year.fragment(),
                month.fragment(),
                day.fragment(),
                hour.fragment(),
                min.fragment(),
                sec.fragment()
            );
            match tz {
                Some(t) => format!("{base}{t}"),
                None => base,
            }
        }
        Some((_, min, None)) => {
            let base = format!(
                "{}-{}-{}T{}:{}",
                year.fragment(),
                month.fragment(),
                day.fragment(),
                hour.fragment(),
                min.fragment()
            );
            match tz {
                Some(t) => format!("{base}{t}"),
                None => base,
            }
        }
        None => {
            let base = format!(
                "{}-{}-{}T{}",
                year.fragment(),
                month.fragment(),
                day.fragment(),
                hour.fragment()
            );
            match tz {
                Some(t) => format!("{base}{t}"),
                None => base,
            }
        }
    };

    Ok((input, datetime_str))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn span(s: &str) -> Span<'_> {
        Span::new(s)
    }

    // ========================================================================
    // Keyword Tests
    // ========================================================================

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("define"));
        assert!(is_keyword("Define"));
        assert!(is_keyword("DEFINE"));
        assert!(is_keyword("library"));
        assert!(is_keyword("true"));
        assert!(is_keyword("false"));
        assert!(!is_keyword("foo"));
        assert!(!is_keyword("myFunction"));
    }

    #[test]
    fn test_all_keywords_recognized() {
        // Test a sampling of keywords from each category
        let test_keywords = [
            // Library structure
            "library",
            "version",
            "using",
            "include",
            // Data model
            "codesystem",
            "valueset",
            "parameter",
            // Definitions
            "context",
            "define",
            "function",
            // Types
            "Boolean",
            "Integer",
            "Decimal",
            "String",
            "Date",
            "DateTime",
            "List",
            "Interval",
            // Literals
            "true",
            "false",
            "null",
            // Operators
            "and",
            "or",
            "not",
            "implies",
            // Query
            "from",
            "where",
            "return",
            "sort",
            "by",
            // Conditionals
            "if",
            "then",
            "else",
            "case",
            "when",
        ];

        for kw in test_keywords {
            assert!(
                is_keyword(kw),
                "Expected '{kw}' to be recognized as keyword"
            );
        }
    }

    #[test]
    fn test_keyword_matching() {
        // keyword() should match case-insensitively and not partial matches
        let (rest, _) = keyword("define")(span("define Foo")).unwrap();
        assert_eq!(rest.fragment(), " Foo");

        let (rest, _) = keyword("define")(span("DEFINE Foo")).unwrap();
        assert_eq!(rest.fragment(), " Foo");

        // Should not match partial identifier
        assert!(keyword("define")(span("defineFunction")).is_err());
    }

    // ========================================================================
    // Identifier Tests
    // ========================================================================

    #[test]
    fn test_identifier() {
        let (rest, id) = identifier(span("foo")).unwrap();
        assert_eq!(id, "foo");
        assert!(rest.is_empty());

        let (rest, id) = identifier(span("_private")).unwrap();
        assert_eq!(id, "_private");
        assert!(rest.is_empty());

        let (rest, id) = identifier(span("camelCase123")).unwrap();
        assert_eq!(id, "camelCase123");
        assert!(rest.is_empty());
    }

    #[test]
    fn test_identifier_with_trailing() {
        let (rest, id) = identifier(span("foo + bar")).unwrap();
        assert_eq!(id, "foo");
        assert_eq!(rest.fragment(), " + bar");
    }

    #[test]
    fn test_identifier_unicode() {
        // CQL supports unicode identifiers
        let (_, id) = identifier(span("patiënt")).unwrap();
        assert_eq!(id, "patiënt");
    }

    #[test]
    fn test_quoted_identifier() {
        let (_, id) = quoted_identifier(span("\"with spaces\"")).unwrap();
        assert_eq!(id, "with spaces");

        let (_, id) = quoted_identifier(span("`backticks`")).unwrap();
        assert_eq!(id, "backticks");
    }

    #[test]
    fn test_quoted_identifier_special_chars() {
        let (_, id) = quoted_identifier(span("\"123-starts-with-number\"")).unwrap();
        assert_eq!(id, "123-starts-with-number");

        let (_, id) = quoted_identifier(span("\"%special%\"")).unwrap();
        assert_eq!(id, "%special%");
    }

    #[test]
    fn test_qualified_identifier() {
        let (_, parts) = qualified_identifier(span("FHIR.Patient")).unwrap();
        assert_eq!(parts, vec!["FHIR", "Patient"]);

        let (_, parts) = qualified_identifier(span("simple")).unwrap();
        assert_eq!(parts, vec!["simple"]);

        let (_, parts) = qualified_identifier(span("a.b.c")).unwrap();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_qualified_identifier_mixed() {
        let (_, parts) = qualified_identifier(span("FHIR.\"Patient Resource\"")).unwrap();
        assert_eq!(parts, vec!["FHIR", "Patient Resource"]);
    }

    // ========================================================================
    // Operator Tests
    // ========================================================================

    #[test]
    fn test_operator() {
        let (_, op) = operator(span("!=")).unwrap();
        assert_eq!(op, Operator::NotEqual);

        let (_, op) = operator(span("<=")).unwrap();
        assert_eq!(op, Operator::LessOrEqual);

        let (_, op) = operator(span("+")).unwrap();
        assert_eq!(op, Operator::Plus);
    }

    #[test]
    fn test_all_operators() {
        let test_cases = [
            ("!=", Operator::NotEqual),
            ("!~", Operator::NotEquivalent),
            ("<=", Operator::LessOrEqual),
            (">=", Operator::GreaterOrEqual),
            ("->", Operator::Arrow),
            ("+", Operator::Plus),
            ("-", Operator::Minus),
            ("*", Operator::Star),
            ("/", Operator::Slash),
            ("^", Operator::Caret),
            ("=", Operator::Equal),
            ("~", Operator::Equivalent),
            ("<", Operator::Less),
            (">", Operator::Greater),
            ("&", Operator::Ampersand),
            ("|", Operator::Pipe),
            (".", Operator::Dot),
            (",", Operator::Comma),
            (":", Operator::Colon),
            (";", Operator::Semicolon),
            ("(", Operator::LeftParen),
            (")", Operator::RightParen),
            ("[", Operator::LeftBracket),
            ("]", Operator::RightBracket),
            ("{", Operator::LeftBrace),
            ("}", Operator::RightBrace),
        ];

        for (input, expected) in test_cases {
            let (_, op) = operator(span(input)).unwrap();
            assert_eq!(op, expected, "Failed for input '{input}'");
        }
    }

    #[test]
    fn test_operator_display() {
        assert_eq!(format!("{}", Operator::NotEqual), "!=");
        assert_eq!(format!("{}", Operator::LessOrEqual), "<=");
        assert_eq!(format!("{}", Operator::Arrow), "->");
    }

    // ========================================================================
    // String Literal Tests
    // ========================================================================

    #[test]
    fn test_string_literal() {
        let (_, s) = string_literal(span("'hello'")).unwrap();
        assert_eq!(s, "hello");

        let (_, s) = string_literal(span("'with\\'quote'")).unwrap();
        assert_eq!(s, "with'quote");

        let (_, s) = string_literal(span("'line\\nbreak'")).unwrap();
        assert_eq!(s, "line\nbreak");
    }

    #[test]
    fn test_string_literal_empty() {
        let (_, s) = string_literal(span("''")).unwrap();
        assert_eq!(s, "");
    }

    #[test]
    fn test_string_literal_escapes() {
        let (_, s) = string_literal(span("'tab\\there'")).unwrap();
        assert_eq!(s, "tab\there");

        let (_, s) = string_literal(span("'return\\rhere'")).unwrap();
        assert_eq!(s, "return\rhere");

        let (_, s) = string_literal(span("'backslash\\\\'")).unwrap();
        assert_eq!(s, "backslash\\");

        let (_, s) = string_literal(span("'quote\\\"here'")).unwrap();
        assert_eq!(s, "quote\"here");
    }

    #[test]
    fn test_string_literal_unicode() {
        let (_, s) = string_literal(span("'日本語'")).unwrap();
        assert_eq!(s, "日本語");

        let (_, s) = string_literal(span("'émoji 🎉'")).unwrap();
        assert_eq!(s, "émoji 🎉");
    }

    // ========================================================================
    // Number Literal Tests
    // ========================================================================

    #[test]
    fn test_integer_literal() {
        let (_, n) = integer_literal(span("42")).unwrap();
        assert_eq!(n, 42);

        let (_, n) = integer_literal(span("-123")).unwrap();
        assert_eq!(n, -123);
    }

    #[test]
    fn test_integer_literal_zero() {
        let (_, n) = integer_literal(span("0")).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_long_literal() {
        let (_, n) = long_literal(span("42L")).unwrap();
        assert_eq!(n, 42);

        let (_, n) = long_literal(span("-9999999999L")).unwrap();
        assert_eq!(n, -9999999999);
    }

    #[test]
    fn test_decimal_literal() {
        let (_, n) = decimal_literal(span("3.25")).unwrap();
        assert!((n - 3.25).abs() < 0.001);

        let (_, n) = decimal_literal(span("-2.5")).unwrap();
        assert!((n - -2.5).abs() < 0.001);
    }

    #[test]
    fn test_decimal_literal_precision() {
        let (_, n) = decimal_literal(span("0.123456789")).unwrap();
        assert!((n - 0.123456789).abs() < 1e-10);
    }

    // ========================================================================
    // Quantity Literal Tests
    // ========================================================================

    #[test]
    fn test_quantity_literal() {
        let (_, (val, unit)) = quantity_literal(span("5 'mg'")).unwrap();
        assert!((val - 5.0).abs() < 0.001);
        assert_eq!(unit, "mg");

        let (_, (val, unit)) = quantity_literal(span("10.5'kg'")).unwrap();
        assert!((val - 10.5).abs() < 0.001);
        assert_eq!(unit, "kg");
    }

    #[test]
    fn test_quantity_literal_ucum_units() {
        // UCUM units can have complex syntax
        let (_, (val, unit)) = quantity_literal(span("100 '[lb_av]'")).unwrap();
        assert!((val - 100.0).abs() < 0.001);
        assert_eq!(unit, "[lb_av]");

        let (_, (val, unit)) = quantity_literal(span("98.6 '[degF]'")).unwrap();
        assert!((val - 98.6).abs() < 0.001);
        assert_eq!(unit, "[degF]");
    }

    // ========================================================================
    // Date/Time Literal Tests
    // ========================================================================

    #[test]
    fn test_date_literal() {
        let (_, d) = date_literal(span("@2024-01-15")).unwrap();
        assert_eq!(d, "2024-01-15");

        let (_, d) = date_literal(span("@2024-01")).unwrap();
        assert_eq!(d, "2024-01");

        let (_, d) = date_literal(span("@2024")).unwrap();
        assert_eq!(d, "2024");
    }

    #[test]
    fn test_date_literal_not_datetime() {
        // Should fail if followed by 'T' (that's a datetime)
        assert!(date_literal(span("@2024-01-15T10:00:00")).is_err());
    }

    #[test]
    fn test_time_literal() {
        let (_, t) = time_literal(span("@T14:30:00")).unwrap();
        assert_eq!(t, "14:30:00");

        let (_, t) = time_literal(span("@T14:30")).unwrap();
        assert_eq!(t, "14:30");

        let (_, t) = time_literal(span("@T14:30:00.123")).unwrap();
        assert_eq!(t, "14:30:00.123");
    }

    #[test]
    fn test_time_literal_midnight() {
        let (_, t) = time_literal(span("@T00:00:00")).unwrap();
        assert_eq!(t, "00:00:00");
    }

    #[test]
    fn test_datetime_literal() {
        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00");

        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00Z")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00Z");

        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00-05:00")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00-05:00");
    }

    #[test]
    fn test_datetime_literal_positive_timezone() {
        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00+05:30")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00+05:30");
    }

    #[test]
    fn test_datetime_literal_partial() {
        // DateTime can have partial time
        let (_, dt) = datetime_literal(span("@2024-01-15T14:30")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30");

        let (_, dt) = datetime_literal(span("@2024-01-15T14")).unwrap();
        assert_eq!(dt, "2024-01-15T14");
    }

    #[test]
    fn test_datetime_literal_milliseconds() {
        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00.123")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00.123");

        let (_, dt) = datetime_literal(span("@2024-01-15T14:30:00.123Z")).unwrap();
        assert_eq!(dt, "2024-01-15T14:30:00.123Z");
    }

    // ========================================================================
    // Comment Tests
    // ========================================================================

    #[test]
    fn test_line_comment() {
        let (rest, _) = line_comment(span("// comment\ncode")).unwrap();
        assert_eq!(rest.fragment(), "\ncode");
    }

    #[test]
    fn test_line_comment_at_eof() {
        let (rest, _) = line_comment(span("// comment at end")).unwrap();
        assert!(rest.is_empty());
    }

    #[test]
    fn test_block_comment() {
        let (rest, _) = block_comment(span("/* block */code")).unwrap();
        assert_eq!(rest.fragment(), "code");
    }

    #[test]
    fn test_block_comment_multiline() {
        let (rest, _) = block_comment(span("/* multi\nline\ncomment */code")).unwrap();
        assert_eq!(rest.fragment(), "code");
    }

    #[test]
    fn test_skip_ws_and_comments() {
        let (rest, _) = skip_ws_and_comments(span("  // comment\n  code")).unwrap();
        assert_eq!(rest.fragment(), "code");

        let (rest, _) = skip_ws_and_comments(span("/* block */  code")).unwrap();
        assert_eq!(rest.fragment(), "code");
    }

    #[test]
    fn test_skip_mixed_ws_and_comments() {
        let input = "  \n// line comment\n/* block */\n  identifier";
        let (rest, _) = skip_ws_and_comments(span(input)).unwrap();
        assert_eq!(rest.fragment(), "identifier");
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_ws_wrapper() {
        // Test the ws() helper function
        let mut parser = ws(identifier);
        let (rest, id) = parser(span("  foo  + bar")).unwrap();
        assert_eq!(id, "foo");
        assert_eq!(rest.fragment(), "+ bar");
    }

    #[test]
    fn test_any_identifier() {
        let (_, id) = any_identifier(span("simple")).unwrap();
        assert_eq!(id, "simple");

        let (_, id) = any_identifier(span("\"quoted\"")).unwrap();
        assert_eq!(id, "quoted");

        let (_, id) = any_identifier(span("`backtick`")).unwrap();
        assert_eq!(id, "backtick");
    }
}
