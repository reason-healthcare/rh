//! FSH Lexer Utilities
//!
//! Low-level nom combinators for FSH token recognition:
//! whitespace, comments, identifiers, string literals, numbers, paths, coded values.

use crate::parser::ast::{FshFlag, FshPath, FshPathSegment, FshValue};
use crate::parser::span::Span;
use nom::bytes::complete::take_while_m_n;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize, value},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

// ============================================================================
// FSH Keywords
// ============================================================================

pub const KW_PROFILE: &str = "Profile";
pub const KW_EXTENSION: &str = "Extension";
pub const KW_LOGICAL: &str = "Logical";
pub const KW_RESOURCE: &str = "Resource";
pub const KW_INSTANCE: &str = "Instance";
pub const KW_VALUE_SET: &str = "ValueSet";
pub const KW_CODE_SYSTEM: &str = "CodeSystem";
pub const KW_INVARIANT: &str = "Invariant";
pub const KW_RULE_SET: &str = "RuleSet";
pub const KW_MAPPING: &str = "Mapping";
pub const KW_ALIAS: &str = "Alias";

/// All entity keywords — used to detect end of a block
pub const ENTITY_KEYWORDS: &[&str] = &[
    KW_PROFILE,
    KW_EXTENSION,
    KW_LOGICAL,
    KW_RESOURCE,
    KW_INSTANCE,
    KW_VALUE_SET,
    KW_CODE_SYSTEM,
    KW_INVARIANT,
    KW_RULE_SET,
    KW_MAPPING,
    KW_ALIAS,
];

// ============================================================================
// Whitespace / comment helpers
// ============================================================================

/// Skip spaces and tabs (NOT newlines)
pub fn ws(input: Span<'_>) -> IResult<Span<'_>, ()> {
    // Accept ASCII space, tab, and non-breaking space (U+00A0) which some FSH files use
    value(
        (),
        take_while(|c: char| c == ' ' || c == '\t' || c == '\u{00A0}'),
    )(input)
}

/// Parse a line comment: `// ...` to end of line
pub fn line_comment(input: Span<'_>) -> IResult<Span<'_>, ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    Ok((input, ()))
}

/// Parse a block comment: `/* ... */`
pub fn block_comment(input: Span<'_>) -> IResult<Span<'_>, ()> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    Ok((input, ()))
}

/// Skip any combination of whitespace (including newlines) and comments
pub fn trivia(input: Span<'_>) -> IResult<Span<'_>, ()> {
    value(
        (),
        many0(alt((
            value((), take_while1(|c: char| c.is_whitespace())),
            line_comment,
            block_comment,
        ))),
    )(input)
}

// ============================================================================
// Primitives
// ============================================================================

/// Parse an FSH identifier: `[A-Za-z_][A-Za-z0-9_\-\.]*`
pub fn identifier(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, id) = recognize(pair(
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'),
    ))(input)?;
    Ok((input, id.fragment().to_string()))
}

/// Parse an FSH alias name: optional `$` prefix followed by an identifier
pub fn alias_name(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, dollar) = opt(char('$'))(input)?;
    let (input, id) = identifier(input)?;
    if dollar.is_some() {
        Ok((input, format!("${}", id)))
    } else {
        Ok((input, id))
    }
}

/// Parse a quoted string: `"..."` with basic escape handling
pub fn quoted_string(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('"')(input)?;
    let mut result = String::new();
    let mut remaining = input;
    loop {
        let (inp, chunk) = take_while(|c| c != '"' && c != '\\')(remaining)?;
        result.push_str(chunk.fragment());
        remaining = inp;
        if remaining.fragment().starts_with('"') {
            let (inp, _) = char('"')(remaining)?;
            remaining = inp;
            break;
        } else if remaining.fragment().starts_with('\\') {
            let frag = remaining.fragment();
            if frag.len() < 2 {
                break;
            }
            let escaped = frag.chars().nth(1).unwrap_or('\\');
            match escaped {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                c => {
                    result.push('\\');
                    result.push(c);
                }
            }
            remaining = remaining.slice(2..);
        } else {
            break;
        }
    }
    Ok((remaining, result))
}

use nom::Slice;

/// Parse a triple-quoted multiline string: `"""..."""`
pub fn multiline_string(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = tag("\"\"\"")(input)?;
    let (input, content) = take_until("\"\"\"")(input)?;
    let (input, _) = tag("\"\"\"")(input)?;
    Ok((input, content.fragment().to_string()))
}

/// Parse an integer (optional leading `-` then digits)
pub fn integer(input: Span<'_>) -> IResult<Span<'_>, i64> {
    let (input, s) = recognize(pair(opt(char('-')), digit1))(input)?;
    let n: i64 = s.fragment().parse().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;
    Ok((input, n))
}

/// Parse a decimal: digits `.` digits
pub fn decimal(input: Span<'_>) -> IResult<Span<'_>, f64> {
    let (input, s) = recognize(tuple((opt(char('-')), digit1, char('.'), digit1)))(input)?;
    let n: f64 = s.fragment().parse().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;
    Ok((input, n))
}

/// Parse a cardinality expression: `min..max` where max can be `*`
/// Returns `(Option<u32>, Option<String>)`
pub fn cardinal(input: Span<'_>) -> IResult<Span<'_>, (Option<u32>, Option<String>)> {
    let (input, min_str) = take_while(|c: char| c.is_ascii_digit())(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, max_str) = alt((
        map(char('*'), |_| "*".to_string()),
        map(take_while1(|c: char| c.is_ascii_digit()), |s: Span<'_>| {
            s.fragment().to_string()
        }),
    ))(input)?;

    let min = if min_str.fragment().is_empty() {
        None
    } else {
        Some(min_str.fragment().parse::<u32>().unwrap_or(0))
    };
    let max = if max_str == "*" { None } else { Some(max_str) };
    Ok((input, (min, max)))
}

// ============================================================================
// Path parsing
// ============================================================================

/// Parse an FSH path like `component.value[x]` or `name[0]`
pub fn fsh_path(input: Span<'_>) -> IResult<Span<'_>, FshPath> {
    // A path is a series of segments separated by '.'
    // Segments can be: name, name[index], name[slice], name[x] (choice), extension(url)

    // Handle '.' alone as the root path indicator (e.g., `* . MS` means root element flags)
    if input.fragment().starts_with('.') {
        let rest = &input.fragment()[1..];
        if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace()) {
            let (input, _) = char('.')(input)?;
            return Ok((
                input,
                FshPath {
                    segments: Vec::new(),
                },
            ));
        }
    }

    let (input, first) = path_segment(input)?;
    let mut segments = vec![first];
    let mut remaining = input;

    loop {
        if !remaining.fragment().starts_with('.') {
            break;
        }
        let (inp, _) = char('.')(remaining)?;
        match path_segment(inp) {
            Ok((inp2, seg)) => {
                segments.push(seg);
                remaining = inp2;
            }
            Err(_) => {
                // No more valid segments after '.'
                break;
            }
        }
    }

    Ok((remaining, FshPath { segments }))
}

/// Parse an FSH path-segment identifier (no `.` allowed — `.` is the path separator)
fn path_identifier(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, id) = recognize(pair(
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
    ))(input)?;
    Ok((input, id.fragment().to_string()))
}

fn path_segment(input: Span<'_>) -> IResult<Span<'_>, FshPathSegment> {
    // extension(url) syntax
    if input.fragment().starts_with("extension(") {
        let (input, _) = tag("extension(")(input)?;
        let (input, url) = take_while(|c| c != ')')(input)?;
        let (input, _) = char(')')(input)?;
        return Ok((input, FshPathSegment::Extension(url.fragment().to_string())));
    }

    // Use path_identifier (no '.') to avoid consuming the path separator
    let (input, name) = path_identifier(input)?;

    if input.fragment().starts_with('[') {
        let (input, _) = char('[')(input)?;
        // could be index (digits), slice name, or x (choice type)
        if input.fragment().starts_with('+') || input.fragment().starts_with('=') {
            // slice operators, treat as slice
            let (input, s) = take_while(|c| c != ']')(input)?;
            let (input, _) = char(']')(input)?;
            return Ok((
                input,
                FshPathSegment::Slice {
                    element: name.to_string(),
                    slice: s.fragment().to_string(),
                },
            ));
        }
        let (input, content) = take_while(|c| c != ']')(input)?;
        let (input, _) = char(']')(input)?;
        let content_str = content.fragment();
        if content_str == "x" || content_str.ends_with("[x]") {
            return Ok((input, FshPathSegment::ChoiceType(name)));
        }
        if content_str.parse::<u32>().is_ok() {
            // Named numeric index (e.g., extension[0]) — preserve element name in Slice
            return Ok((
                input,
                FshPathSegment::Slice {
                    element: name.to_string(),
                    slice: content_str.to_string(),
                },
            ));
        }
        // Slice: store element name separately from slice name for proper FHIR path rendering
        return Ok((
            input,
            FshPathSegment::Slice {
                element: name.to_string(),
                slice: content_str.to_string(),
            },
        ));
    }

    Ok((input, FshPathSegment::Name(name)))
}

// ============================================================================
// Coded value
// ============================================================================

/// Parse `system#code "display"?` or `#code "display"?`
pub fn coded_value(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    // Optional system part before '#'
    let (input, system_part) = opt(take_while(|c| {
        c != '#' && c != ' ' && c != '\n' && c != '\t'
    }))(input)?;
    let has_hash = input.fragment().starts_with('#');
    if !has_hash {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }
    let (input, _) = char('#')(input)?;
    let (input, code) = take_while1(|c: char| !c.is_whitespace() && c != '"')(input)?;
    let (input, _) = ws(input)?;
    let (input, display) = opt(quoted_string)(input)?;

    let system = system_part.and_then(|s| {
        let frag = s.fragment();
        if frag.is_empty() {
            None
        } else {
            Some(frag.to_string())
        }
    });

    Ok((
        input,
        FshValue::Code {
            system,
            code: code.fragment().to_string(),
            display,
        },
    ))
}

// ============================================================================
// Flag parsing
// ============================================================================

/// Parse a single FSH flag character
pub fn fsh_flag(input: Span<'_>) -> IResult<Span<'_>, FshFlag> {
    alt((
        value(FshFlag::MustSupport, tag("MS")),
        value(FshFlag::SummaryElement, tag("SU")),
        value(FshFlag::Modifier, tag("?!")),
        value(FshFlag::Normative, tag("N")),
        value(FshFlag::TrialUse, tag("TU")),
        value(FshFlag::Draft, tag("D")),
    ))(input)
}

/// Parse a list of flags separated by whitespace
pub fn flags_list(input: Span<'_>) -> IResult<Span<'_>, Vec<FshFlag>> {
    let mut flags = Vec::new();
    let mut remaining = input;
    loop {
        let (inp, _) = ws(remaining)?;
        match fsh_flag(inp) {
            Ok((inp2, flag)) => {
                flags.push(flag);
                remaining = inp2;
            }
            Err(_) => break,
        }
    }
    Ok((remaining, flags))
}

// ============================================================================
// Value parsing
// ============================================================================

/// Parse any FSH value (used in assignment rules)
pub fn fsh_value(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    // Try date/dateTime first (YYYY-MM-DD[T...]) before integer/decimal
    if let Ok((inp, date_val)) = parse_date_or_datetime(input) {
        return Ok((inp, date_val));
    }

    alt((
        parse_ratio,
        parse_quantity,
        parse_reference,
        parse_canonical,
        map(multiline_string, FshValue::Str),
        map(quoted_string, FshValue::Str),
        map(tag("true"), |_| FshValue::Bool(true)),
        map(tag("false"), |_| FshValue::Bool(false)),
        coded_value,
        map(decimal, FshValue::Decimal),
        map(integer, FshValue::Integer),
        // $alias-name as a canonical reference (used in parameterized rule sets and templates)
        parse_alias_as_canonical,
        // Bare identifier — inline instance reference (e.g., contained[+] = myInstance)
        |input| {
            let (input, s) = identifier(input)?;
            Ok((input, FshValue::InstanceRef(s.to_string())))
        },
    ))(input)
}

/// Parse `$identifier` as a canonical reference (for use in value positions referencing aliases).
fn parse_alias_as_canonical(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    let (input, _) = char('$')(input)?;
    let (input, name) = identifier(input)?;
    Ok((input, FshValue::Canonical(format!("${}", name))))
}

/// Parse a date (YYYY-MM-DD) or dateTime (YYYY-MM-DDTHH:...) value
fn parse_date_or_datetime(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    let frag = input.fragment();
    // Must start with 4 digits then a dash
    if frag.len() < 10 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )));
    }
    let bytes = frag.as_bytes();
    // Check YYYY-MM-DD pattern
    for byte in bytes.iter().take(4) {
        if !byte.is_ascii_digit() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeWhileMN,
            )));
        }
    }
    if bytes[4] != b'-' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )));
    }
    if !bytes[5].is_ascii_digit() || !bytes[6].is_ascii_digit() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )));
    }
    if bytes[7] != b'-' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )));
    }
    if !bytes[8].is_ascii_digit() || !bytes[9].is_ascii_digit() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )));
    }

    // Consume the YYYY-MM-DD base
    let (input, base) = take_while_m_n(10, 10, |_: char| true)(input)?;
    let base_str = base.fragment();

    // Consume any trailing time/timezone chars (not whitespace, not ) or ,)
    let (input, extra) =
        take_while(|c: char| c != ' ' && c != '\n' && c != '\t' && c != ')' && c != ',')(input)?;
    let extra_str = extra.fragment();

    let full = format!("{}{}", base_str, extra_str);
    if full.contains('T') {
        Ok((input, FshValue::DateTime(full)))
    } else {
        Ok((input, FshValue::Date(full)))
    }
}

fn parse_ratio(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    // numerator ':' denominator where each part is a quantity or integer
    let (input, num) = alt((
        parse_quantity,
        map(decimal, FshValue::Decimal),
        map(integer, FshValue::Integer),
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, den) = alt((
        parse_quantity,
        map(decimal, FshValue::Decimal),
        map(integer, FshValue::Integer),
    ))(input)?;
    Ok((
        input,
        FshValue::Ratio {
            numerator: Box::new(num),
            denominator: Box::new(den),
        },
    ))
}

fn parse_quantity(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    let (input, val) = decimal(input)?;
    let (input, _) = take_while1(|c| c == ' ' || c == '\t')(input)?;
    // unit can be quoted or bare
    let (input, unit) = alt((
        quoted_string,
        map(
            take_while1(|c: char| !c.is_whitespace() && c != '\n'),
            |s: Span<'_>| s.fragment().to_string(),
        ),
    ))(input)?;
    Ok((input, FshValue::Quantity { value: val, unit }))
}

fn parse_reference(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    let (input, _) = tag("Reference(")(input)?;
    let (input, target) = take_while(|c| c != ')')(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, FshValue::Reference(target.fragment().to_string())))
}

fn parse_canonical(input: Span<'_>) -> IResult<Span<'_>, FshValue> {
    let (input, _) = tag("Canonical(")(input)?;
    let (input, target) = take_while(|c| c != ')')(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, FshValue::Canonical(target.fragment().to_string())))
}

// ============================================================================
// Line-level helpers
// ============================================================================

/// Consume to end of line (or EOF), returning the content
pub fn rest_of_line(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, content) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    Ok((input, content.fragment().trim().to_string()))
}

/// Check if a line is a metadata key-value: `Key: value`
pub fn meta_key_value<'a>(key: &'static str, input: Span<'a>) -> IResult<Span<'a>, String> {
    let (input, _) = trivia(input)?;
    let (input, _) = tag(key)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    // value may be a quoted string or identifier/url
    let (input, val) = alt((
        quoted_string,
        map(
            take_while1(|c: char| c != '\n' && c != '\r'),
            |s: Span<'_>| s.fragment().trim().to_string(),
        ),
    ))(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    Ok((input, val))
}

/// Returns true if the input starts with an entity keyword
pub fn starts_with_entity_kw(input: &str) -> bool {
    for kw in ENTITY_KEYWORDS {
        if let Some(rest) = input.strip_prefix(kw) {
            // Make sure it's not a prefix of a longer identifier
            if rest.is_empty()
                || rest.starts_with(':')
                || rest.starts_with(' ')
                || rest.starts_with('\t')
                || rest.starts_with('\n')
                || rest.starts_with('\r')
            {
                return true;
            }
        }
    }
    false
}
