//! FHIRPath parser implementation using nom

use crate::ast::*;
use crate::error::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::{many_m_n, separated_list0, separated_list1},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use rust_decimal::Decimal;

const MAX_DELIMITER_NESTING: usize = 12;

/// FHIRPath expression parser
pub struct FhirPathParser;

impl FhirPathParser {
    /// Create a new FHIRPath parser
    pub fn new() -> Self {
        Self
    }

    /// Parse a FHIRPath expression from a string.
    ///
    /// Expressions may nest delimiters up to 12 levels and AST nodes up to 64 levels.
    /// Sibling arguments and selector elements do not add to the depth of each other.
    pub fn parse(&self, input: &str) -> FhirPathResult<FhirPathExpression> {
        let stripped = strip_comments(input)?;
        let input = stripped.as_str();
        match parse_expression(input.trim()) {
            Ok((remaining, expr)) => {
                let remaining = remaining.trim();
                if remaining.is_empty() {
                    expr.check_depth()?;
                    Ok(FhirPathExpression { root: expr })
                } else {
                    Err(FhirPathError::SyntaxError {
                        line: 1,
                        column: input.len() - remaining.len(),
                        message: format!("Unexpected characters: {remaining}"),
                    })
                }
            }
            Err(nom::Err::Failure(error)) if error.code == nom::error::ErrorKind::TooLarge => {
                Err(FhirPathError::syntax_error(
                    1,
                    input.len() - error.input.len(),
                    EXPRESSION_DEPTH_ERROR,
                ))
            }
            Err(e) => Err(FhirPathError::SyntaxError {
                line: 1,
                column: 0,
                message: format!("Parse error: {e:?}"),
            }),
        }
    }
}

impl Default for FhirPathParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Remove `//` line comments and `/* */` block comments, preserving the
/// contents of string literals (`'…'`) and delimited identifiers (`` `…` ``).
fn strip_comments(input: &str) -> FhirPathResult<String> {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();
    let mut delimiters = Vec::new();
    while let Some((i, c)) = chars.next() {
        match c {
            '\'' | '`' => {
                let delim = c;
                out.push(c);
                let mut escaped = false;
                for (_, sc) in chars.by_ref() {
                    out.push(sc);
                    if escaped {
                        escaped = false;
                    } else if sc == '\\' {
                        escaped = true;
                    } else if sc == delim {
                        break;
                    }
                }
            }
            '/' if matches!(chars.peek(), Some((_, '/'))) => {
                // line comment — skip to end of line (keep the newline)
                for (_, sc) in chars.by_ref() {
                    if sc == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '/' if matches!(chars.peek(), Some((_, '*'))) => {
                chars.next(); // consume '*'
                let mut closed = false;
                let mut prev_star = false;
                for (_, sc) in chars.by_ref() {
                    if prev_star && sc == '/' {
                        closed = true;
                        break;
                    }
                    prev_star = sc == '*';
                }
                if !closed {
                    return Err(FhirPathError::SyntaxError {
                        line: 1,
                        column: i,
                        message: "Unterminated block comment".to_string(),
                    });
                }
                out.push(' ');
            }
            '(' | '[' | '{' => {
                if delimiters.len() == MAX_DELIMITER_NESTING {
                    return Err(FhirPathError::syntax_error(
                        1,
                        i,
                        "Expression delimiter nesting exceeds 12 levels",
                    ));
                }
                delimiters.push(match c {
                    '(' => ')',
                    '[' => ']',
                    _ => '}',
                });
                out.push(c);
            }
            ')' | ']' | '}' => {
                if delimiters.pop() != Some(c) {
                    return Err(FhirPathError::syntax_error(
                        1,
                        i,
                        "Unexpected closing delimiter",
                    ));
                }
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    Ok(out)
}

// Whitespace handling
fn ws<'a, F, O>(inner: F) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

fn keyword(word: &'static str) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input| {
        let (rest, token) = tag(word)(input)?;
        if rest
            .as_bytes()
            .first()
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        } else {
            Ok((rest, token))
        }
    }
}

fn depth_failure(input: &str) -> nom::Err<nom::error::Error<&str>> {
    nom::Err::Failure(nom::error::Error::new(
        input,
        nom::error::ErrorKind::TooLarge,
    ))
}

fn chain_tail<'a, P, T>(
    parser: P,
) -> impl Parser<&'a str, Output = Vec<T>, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = T, Error = nom::error::Error<&'a str>>,
{
    many_m_n(0, MAX_EXPRESSION_DEPTH, parser)
}

fn finish_chain<T>(
    input: &str,
    first: Expression,
    rest: impl ExactSizeIterator<Item = T>,
    combine: impl FnMut(Expression, T) -> Expression,
) -> IResult<&str, Expression> {
    if rest.len() == 0 {
        return Ok((input, first));
    }
    if rest.len() >= MAX_EXPRESSION_DEPTH {
        return Err(depth_failure(input));
    }
    let expression = rest.fold(first, combine);
    expression.check_depth().map_err(|_| depth_failure(input))?;
    Ok((input, expression))
}

// Parse expression (top level)
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_implies_expression(input)
}

// Parse implies expressions (lowest precedence)
fn parse_implies_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_or_expression(input)?;
    let (input, rest) =
        chain_tail(preceded(ws(keyword("implies")), parse_or_expression)).parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, expr| {
        Expression::Implies {
            left: Box::new(acc),
            right: Box::new(expr),
        }
    })
}

// Parse OR/XOR expressions
fn parse_or_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_and_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((keyword("or"), keyword("xor")))),
        parse_and_expression,
    ))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, (op, expr)| {
        let operator = match op {
            "or" => OrOperator::Or,
            "xor" => OrOperator::Xor,
            _ => OrOperator::Or,
        };
        Expression::Or {
            left: Box::new(acc),
            operator,
            right: Box::new(expr),
        }
    })
}

// Parse AND expressions
fn parse_and_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_membership_expression(input)?;
    let (input, rest) =
        chain_tail(preceded(ws(keyword("and")), parse_membership_expression)).parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, expr| {
        Expression::And {
            left: Box::new(acc),
            right: Box::new(expr),
        }
    })
}

fn parse_type_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_additive_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((keyword("is"), keyword("as")))),
        parse_type_specifier,
    ))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, (op, type_spec)| {
        let operator = match op {
            "is" => TypeOperator::Is,
            "as" => TypeOperator::As,
            _ => TypeOperator::Is,
        };
        Expression::Type {
            left: Box::new(acc),
            operator,
            type_specifier: type_spec,
        }
    })
}

// Parse type specifier (qualified name like System.String)
fn parse_type_specifier(input: &str) -> IResult<&str, TypeSpecifier> {
    let (input, qualified_name) = separated_list1(ws(char('.')), parse_identifier).parse(input)?;
    Ok((input, TypeSpecifier { qualified_name }))
}

fn parse_membership_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_equality_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((
            map(keyword("contains"), |_| MembershipOperator::Contains),
            map(keyword("in"), |_| MembershipOperator::In),
        ))),
        parse_equality_expression,
    ))
    .parse(input)?;
    finish_chain(input, first, rest.into_iter(), |left, (operator, right)| {
        Expression::Membership {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    })
}

fn parse_equality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_inequality_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((
            map(tag("!="), |_| EqualityOperator::NotEqual),
            map(tag("!~"), |_| EqualityOperator::NotEquivalent),
            map(tag("="), |_| EqualityOperator::Equal),
            map(tag("~"), |_| EqualityOperator::Equivalent),
        ))),
        parse_inequality_expression,
    ))
    .parse(input)?;
    finish_chain(input, first, rest.into_iter(), |left, (operator, right)| {
        Expression::Equality {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    })
}

// Parse inequality expressions (<, <=, >, >=)
fn parse_inequality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_union_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((tag("<="), tag(">="), tag("<"), tag(">")))),
        parse_union_expression,
    ))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, (op, expr)| {
        let operator = match op {
            "<" => InequalityOperator::LessThan,
            "<=" => InequalityOperator::LessThanOrEqual,
            ">" => InequalityOperator::GreaterThan,
            ">=" => InequalityOperator::GreaterThanOrEqual,
            _ => InequalityOperator::LessThan,
        };
        Expression::Inequality {
            left: Box::new(acc),
            operator,
            right: Box::new(expr),
        }
    })
}

// Parse additive expressions (+, -, &)
fn parse_additive_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_multiplicative_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((tag("+"), tag("-"), tag("&")))),
        parse_multiplicative_expression,
    ))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, (op, expr)| {
        let operator = match op {
            "+" => AdditiveOperator::Add,
            "-" => AdditiveOperator::Subtract,
            "&" => AdditiveOperator::Concatenate,
            _ => AdditiveOperator::Add,
        };
        Expression::Additive {
            left: Box::new(acc),
            operator,
            right: Box::new(expr),
        }
    })
}

// Parse multiplicative expressions (*, /, div, mod)
fn parse_multiplicative_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_unary_expression(input)?;
    let (input, rest) = chain_tail((
        ws(alt((keyword("div"), keyword("mod"), tag("*"), tag("/")))),
        parse_unary_expression,
    ))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, (op, expr)| {
        let operator = match op {
            "*" => MultiplicativeOperator::Multiply,
            "/" => MultiplicativeOperator::Divide,
            "div" => MultiplicativeOperator::Div,
            "mod" => MultiplicativeOperator::Mod,
            _ => MultiplicativeOperator::Multiply,
        };
        Expression::Multiplicative {
            left: Box::new(acc),
            operator,
            right: Box::new(expr),
        }
    })
}

fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    let mut prefixes = Vec::new();
    let mut rest = input;
    let (rest, operand) = loop {
        if let Ok(result) = parse_minimum_integer(rest) {
            break result;
        }
        let operator = ws(alt((
            map(char('-'), |_| PolarityOperator::Minus),
            map(char('+'), |_| PolarityOperator::Plus),
        )))
        .parse(rest);
        let Ok((next, operator)) = operator else {
            break parse_invocation_expression(rest)?;
        };
        if prefixes.len() + 1 >= MAX_EXPRESSION_DEPTH {
            return Err(depth_failure(rest));
        }
        prefixes.push(operator);
        rest = next;
    };
    finish_chain(
        rest,
        operand,
        prefixes.into_iter().rev(),
        |operand, operator| Expression::Polarity {
            operator,
            operand: Box::new(operand),
        },
    )
}

fn parse_union_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_type_expression(input)?;
    let (input, rest) = chain_tail(preceded(ws(char('|')), parse_type_expression)).parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, expr| {
        Expression::Union {
            left: Box::new(acc),
            right: Box::new(expr),
        }
    })
}

// Parse invocation expressions (member access and indexing)
fn parse_invocation_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_term(input)?;
    let (input, rest) = chain_tail(alt((
        map(preceded(ws(char('.')), parse_invocation), |inv| {
            InvocationOrIndex::Invocation(inv)
        }),
        map(
            delimited(ws(char('[')), parse_expression, ws(char(']'))),
            InvocationOrIndex::Index,
        ),
    )))
    .parse(input)?;

    finish_chain(input, first, rest.into_iter(), |acc, item| match item {
        InvocationOrIndex::Invocation(inv) => Expression::Invocation {
            left: Box::new(acc),
            invocation: inv,
        },
        InvocationOrIndex::Index(idx) => Expression::Indexer {
            left: Box::new(acc),
            index: Box::new(idx),
        },
    })
}

#[derive(Debug)]
enum InvocationOrIndex {
    Invocation(Invocation),
    Index(Expression),
}

// Parse term
fn parse_term(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_instance_selector,
        map(parse_literal, |lit| Expression::Term(Term::Literal(lit))),
        map(parse_invocation, |inv| {
            Expression::Term(Term::Invocation(inv))
        }),
        map(parse_external_constant, |name| {
            Expression::Term(Term::ExternalConstant(name))
        }),
        map(
            delimited(ws(char('(')), parse_expression, ws(char(')'))),
            |expr| Expression::Term(Term::Parenthesized(Box::new(expr))),
        ),
    ))
    .parse(input)
}

fn parse_instance_selector(input: &str) -> IResult<&str, Expression> {
    let (input, type_specifier) = parse_type_specifier(input)?;
    let (input, elements) = delimited(
        ws(char('{')),
        alt((
            map(ws(char(':')), |_| Vec::new()),
            separated_list1(
                ws(char(',')),
                (parse_identifier, preceded(ws(char(':')), parse_expression)),
            ),
        )),
        ws(char('}')),
    )
    .parse(input)?;
    Ok((
        input,
        Expression::Term(Term::InstanceSelector {
            type_specifier,
            elements,
        }),
    ))
}

// Parse invocation (function call or member access)
fn parse_invocation(input: &str) -> IResult<&str, Invocation> {
    alt((
        map(keyword("$this"), |_| Invocation::This),
        map(keyword("$index"), |_| Invocation::Index),
        map(keyword("$total"), |_| Invocation::Total),
        parse_function_or_member,
    ))
    .parse(input)
}

// Parse function call or simple member access
fn parse_function_or_member(input: &str) -> IResult<&str, Invocation> {
    let (input, name) = parse_identifier(input)?;
    let (input, params) = if name == "sort" {
        opt(delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_sort_parameter),
            ws(char(')')),
        ))
        .parse(input)?
    } else {
        opt(delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_expression),
            ws(char(')')),
        ))
        .parse(input)?
    };

    if let Some(parameters) = params {
        Ok((input, Invocation::Function { name, parameters }))
    } else {
        Ok((input, Invocation::Member(name)))
    }
}

fn parse_sort_parameter(input: &str) -> IResult<&str, Expression> {
    let (input, expr) = parse_expression(input)?;
    let (input, direction) = opt(ws(alt((keyword("asc"), keyword("desc"))))).parse(input)?;
    if let Some(direction) = direction {
        Ok((
            input,
            Expression::SortKey {
                key: Box::new(expr),
                descending: direction == "desc",
            },
        ))
    } else {
        Ok((input, expr))
    }
}

// Parse external constant
fn parse_external_constant(input: &str) -> IResult<&str, String> {
    preceded(
        ws(char('%')),
        alt((parse_identifier, |s| parse_delimited_text(s, '\''))),
    )
    .parse(input)
}

// Parse literal values
fn parse_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map((char('{'), multispace0, char('}')), |_| Literal::Null),
        map(keyword("true"), |_| Literal::Boolean(true)),
        map(keyword("false"), |_| Literal::Boolean(false)),
        parse_datetime_literal, // Try datetime first (longer pattern)
        parse_time_literal,     // Then time
        parse_date_literal,     // Then date (shorter pattern)
        parse_quantity_literal, // Try quantity before string and number
        parse_string_literal,
        parse_number_literal,
    ))
    .parse(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, Literal> {
    map(|text| parse_delimited_text(text, '\''), Literal::String).parse(input)
}

/// Strings and delimited identifiers use the same UTF-16 escape syntax.
fn parse_delimited_text(input: &str, delimiter: char) -> IResult<&str, String> {
    let (mut rest, _) = char(delimiter)(input)?;
    let mut output = String::new();
    while let Some(c) = rest.chars().next() {
        rest = &rest[c.len_utf8()..];
        if c == delimiter {
            return Ok((rest, output));
        }
        if c != '\\' {
            output.push(c);
            continue;
        }
        let Some(escape) = rest.chars().next() else {
            break;
        };
        rest = &rest[escape.len_utf8()..];
        match escape {
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            'f' => output.push('\u{c}'),
            'u' => {
                if let Some(code_unit) = hex_code_unit(rest) {
                    rest = &rest[4..];
                    let mut units = [code_unit, 0];
                    let count = if (0xd800..=0xdbff).contains(&code_unit) {
                        if let Some(low) = rest.strip_prefix("\\u").and_then(hex_code_unit) {
                            units[1] = low;
                            rest = &rest[6..];
                        }
                        2
                    } else {
                        1
                    };
                    let scalar = char::decode_utf16(units[..count].iter().copied())
                        .next()
                        .and_then(Result::ok)
                        .ok_or_else(|| {
                            nom::Err::Failure(nom::error::Error::new(
                                rest,
                                nom::error::ErrorKind::Escaped,
                            ))
                        })?;
                    output.push(scalar);
                } else {
                    output.push('u');
                }
            }
            other => output.push(other),
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )))
}

fn hex_code_unit(input: &str) -> Option<u16> {
    let digits = input.get(..4)?;
    if !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u16::from_str_radix(digits, 16).ok()
}

fn parse_number_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map_res((digit1, char('L')), |(digits, _): (&str, char)| {
            digits.parse::<i64>().map(Literal::LongNumber)
        }),
        map_res(recognize((digit1, char('.'), digit1)), |digits| {
            Decimal::from_str_exact(digits).map(Literal::Number)
        }),
        map_res(digit1, |digits: &str| {
            digits
                .parse::<i32>()
                .map(|number| Literal::Integer(i64::from(number)))
        }),
    ))
    .parse(input)
}

/// The signed minimum magnitude exceeds the maximum positive literal.
fn parse_minimum_integer(input: &str) -> IResult<&str, Expression> {
    let (rest, (digits, suffix)) =
        preceded(ws(char('-')), (digit1, opt(char('L')))).parse(input)?;
    let literal = match (digits.parse::<u64>(), suffix) {
        (Ok(9_223_372_036_854_775_808), Some('L')) => Literal::LongNumber(i64::MIN),
        (Ok(2_147_483_648), None) => Literal::Integer(i64::from(i32::MIN)),
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )))
        }
    };
    let next = rest.trim_start();
    if next.starts_with(['.', '[', '\''])
        || next.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
            && ![
                "is", "as", "and", "or", "xor", "implies", "in", "contains", "div", "mod",
            ]
            .iter()
            .any(|word| keyword(word)(next).is_ok())
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    Ok((rest, Expression::Term(Term::Literal(literal))))
}

// Parse date literal (@YYYY, @YYYY-MM, or @YYYY-MM-DD)
fn parse_date_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;

    // Check if there's more date information
    let (input, month_day) = opt((
        preceded(
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ),
        opt(preceded(
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        )),
    ))
    .parse(input)?;

    // Ensure this doesn't match datetime by checking for 'T'
    if input.starts_with('T') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    let date_str = match month_day {
        Some((month, Some(day))) => format!("{year}-{month}-{day}"),
        Some((month, None)) => format!("{year}-{month}"),
        None => year.to_string(),
    };

    Ok((input, Literal::Date(date_str)))
}

// Parse datetime literal — partial-precision per FHIRPath spec:
//   DATETIME: '@' DATE 'T' (TIME TZ?)?
//   DATE:     YYYY ('-' MM ('-' DD)?)?
//   TIME:     hh (':' mm (':' ss ('.' fff)?)?)?
//   TZ:       'Z' | ('+'|'-') HH ':' MM
// Examples accepted: @2015T, @2015-02T, @2015-02-04T, @2014-01-01T08,
// @2018-03-01T10:30, @2012-04-15T15:30:31.0, @1973-12-25T00:00:00.000+10:00
fn parse_datetime_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;

    // Optional month, then optional day (only after month).
    let (input, month) = opt(preceded(
        char('-'),
        take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
    ))
    .parse(input)?;
    let (input, day) = if month.is_some() {
        opt(preceded(
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };

    // The 'T' is what distinguishes a DateTime from a Date literal.
    let (input, _) = char('T')(input)?;

    // Optional time: hh, optional :mm, optional :ss, optional .fff (each gated on its predecessor).
    let (input, hour) = opt(take_while_m_n(2, 2, |c: char| c.is_ascii_digit())).parse(input)?;
    let (input, minute) = if hour.is_some() {
        opt(preceded(
            char(':'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, second) = if minute.is_some() {
        opt(preceded(
            char(':'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, fraction) = if second.is_some() {
        opt(preceded(
            char('.'),
            take_while1(|c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };

    // Optional timezone — only valid once we have at least an hour.
    let (input, timezone) = if hour.is_some() {
        opt(alt((
            recognize(char('Z')),
            recognize((
                alt((char('+'), char('-'))),
                take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
                char(':'),
                take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            )),
        )))
        .parse(input)?
    } else {
        (input, None)
    };

    let mut s = String::with_capacity(32);
    s.push_str(year);
    if let Some(m) = month {
        s.push('-');
        s.push_str(m);
    }
    if let Some(d) = day {
        s.push('-');
        s.push_str(d);
    }
    s.push('T');
    if let Some(h) = hour {
        s.push_str(h);
    }
    if let Some(m) = minute {
        s.push(':');
        s.push_str(m);
    }
    if let Some(sec) = second {
        s.push(':');
        s.push_str(sec);
    }
    if let Some(f) = fraction {
        s.push('.');
        s.push_str(f);
    }
    if let Some(tz) = timezone {
        s.push_str(tz);
    }

    Ok((input, Literal::DateTime(s)))
}

// Parse time literal — partial-precision per FHIRPath spec:
//   TIME_LITERAL: '@' 'T' TIME
//   TIME:         hh (':' mm (':' ss ('.' fff)?)?)?
// Examples accepted: @T14, @T10:30, @T10:30:00, @T10:30:00.000
fn parse_time_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, _) = char('T')(input)?;
    let (input, hour) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, minute) = opt(preceded(
        char(':'),
        take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
    ))
    .parse(input)?;
    let (input, second) = if minute.is_some() {
        opt(preceded(
            char(':'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, fraction) = if second.is_some() {
        opt(preceded(
            char('.'),
            take_while1(|c: char| c.is_ascii_digit()),
        ))
        .parse(input)?
    } else {
        (input, None)
    };

    let mut s = String::with_capacity(16);
    s.push('T');
    s.push_str(hour);
    if let Some(m) = minute {
        s.push(':');
        s.push_str(m);
    }
    if let Some(sec) = second {
        s.push(':');
        s.push_str(sec);
    }
    if let Some(f) = fraction {
        s.push('.');
        s.push_str(f);
    }

    Ok((input, Literal::Time(s)))
}

fn parse_quantity_literal(input: &str) -> IResult<&str, Literal> {
    let (rest, (digits, _, unit)) = (
        recognize((digit1, opt((char('.'), digit1)))),
        multispace0,
        alt((
            |text| parse_delimited_text(text, '\''),
            |text| {
                let (rest, unit) = alpha1(text)?;
                match unit {
                    "year" | "years" | "month" | "months" | "week" | "weeks" | "day" | "days"
                    | "hour" | "hours" | "minute" | "minutes" | "second" | "seconds"
                    | "millisecond" | "milliseconds" => {
                        Ok((rest, unit.trim_end_matches('s').to_string()))
                    }
                    _ => Err(nom::Err::Error(nom::error::Error::new(
                        text,
                        nom::error::ErrorKind::Tag,
                    ))),
                }
            },
        )),
    )
        .parse(input)?;
    let value = Decimal::from_str_exact(digits).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;
    Ok((
        rest,
        Literal::Quantity {
            value,
            unit: (!unit.is_empty()).then_some(unit),
        },
    ))
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    if input.starts_with('`') {
        return parse_delimited_text(input, '`');
    }
    map(
        recognize((
            take_while_m_n(1, 1, |c: char| c.is_ascii_alphabetic() || c == '_'),
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
        )),
        str::to_owned,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_parsing() {
        let parser = FhirPathParser::new();
        let result = parser.parse("Patient");
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_member_access() {
        let parser = FhirPathParser::new();
        let result = parser.parse("Patient.name");
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let expr = result.unwrap();
        if let Expression::Invocation { left, invocation } = &expr.root {
            assert!(matches!(
                &**left,
                Expression::Term(Term::Invocation(Invocation::Member(_)))
            ));
            assert!(matches!(invocation, Invocation::Member(_)));
        } else {
            panic!("Expected invocation expression");
        }
    }

    #[test]
    fn test_literal_values() {
        let parser = FhirPathParser::new();

        // Boolean literal
        let result = parser.parse("true");
        assert!(result.is_ok());

        // String literal
        let result = parser.parse("'hello'");
        assert!(result.is_ok());

        // Number literal
        let result = parser.parse("42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_call() {
        let parser = FhirPathParser::new();
        let result = parser.parse("length()");
        assert!(result.is_ok());

        let expr = result.unwrap();
        if let Expression::Term(Term::Invocation(Invocation::Function { name, parameters })) =
            &expr.root
        {
            assert_eq!(name, "length");
            assert!(parameters.is_empty());
        } else {
            panic!("Expected function invocation");
        }
    }

    #[test]
    fn test_complex_expression() {
        let parser = FhirPathParser::new();
        let result = parser.parse("Patient.name[0].given");
        assert!(
            result.is_ok(),
            "Failed to parse complex expression: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_union_expression() {
        let parser = FhirPathParser::new();
        let result = parser.parse("name.given | name.family");
        assert!(result.is_ok());
    }

    #[test]
    fn test_equality_expression() {
        let parser = FhirPathParser::new();
        let result = parser.parse("name.use = 'official'");
        assert!(result.is_ok());
    }

    #[test]
    fn test_comparison_expressions() {
        let parser = FhirPathParser::new();

        // Test all comparison operators
        let expressions = vec![
            "5 > 3",
            "age >= 18",
            "weight < 100",
            "height <= 200",
            "'apple' < 'banana'",
            "score >= average",
        ];

        for expr_str in expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed comparison: {expr_str} -> {expr}");
                    // Verify it's an Inequality expression
                    if let Expression::Inequality { .. } = expr.root {
                        // Success
                    } else {
                        panic!("Expected Inequality expression for {expr_str}");
                    }
                }
                Err(e) => {
                    panic!("Failed to parse comparison {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_comparison_precedence() {
        let parser = FhirPathParser::new();

        // Test that arithmetic operators bind tighter than comparison operators
        let result = parser.parse("2 + 3 > 4");
        assert!(result.is_ok());
        let expr = result.unwrap();

        // Should parse as (2 + 3) > 4, not 2 + (3 > 4)
        if let Expression::Inequality {
            left,
            operator,
            right: _,
        } = expr.root
        {
            // Left side should be an additive expression
            if let Expression::Additive { .. } = left.as_ref() {
                // Correct precedence
            } else {
                panic!("Expected left side to be additive expression");
            }
            assert!(matches!(operator, InequalityOperator::GreaterThan));
        } else {
            panic!("Expected inequality expression");
        }
    }

    #[test]
    fn test_membership_operators() {
        let parser = FhirPathParser::new();

        // Test basic membership expressions
        let expressions = [
            "value in collection",
            "name contains 'John'",
            "item in list",
            "collection contains element",
        ];

        for expr_str in expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed membership: {expr_str} -> {expr}");
                    // Verify it's a Membership expression
                    if let Expression::Membership { .. } = expr.root {
                        // Success
                    } else {
                        panic!("Expected Membership expression for {expr_str}");
                    }
                }
                Err(e) => {
                    panic!("Failed to parse membership {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_membership_precedence() {
        let parser = FhirPathParser::new();

        // Test that membership has correct precedence (same as equality, left-associative)
        let result = parser.parse("a = b in collection");
        assert!(result.is_ok());
        let expr = result.unwrap();

        println!("Parsed expression: {:?}", expr.root);

        // Should parse as (a = b) in collection due to left-associativity
        if let Expression::Membership {
            left,
            operator,
            right: _,
        } = expr.root
        {
            // Left side should be an equality expression
            if let Expression::Equality { .. } = left.as_ref() {
                // Correct precedence
            } else {
                panic!("Expected left side to be equality expression, got: {left:?}");
            }
            assert!(matches!(operator, MembershipOperator::In));
        } else {
            panic!("Expected membership expression, got: {:?}", expr.root);
        }
    }

    #[test]
    fn test_date_time_literals() {
        let parser = FhirPathParser::new();

        // Test date literals
        let date_expressions = ["@2023-01-01", "@1990-12-25", "@2025-07-26"];

        for expr_str in date_expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed date: {expr_str} -> {expr}");
                    // Verify it's a Date literal
                    if let Expression::Term(Term::Literal(Literal::Date(date))) = expr.root {
                        assert!(!date.is_empty());
                        assert!(date.contains('-'));
                    } else {
                        panic!("Expected Date literal for {expr_str}, got: {:?}", expr.root);
                    }
                }
                Err(e) => {
                    panic!("Failed to parse date {expr_str}: {e:?}");
                }
            }
        }

        // Test datetime literals
        let datetime_expressions = [
            "@2023-01-01T12:30:45",
            "@2023-01-01T00:00:00Z",
            "@2023-01-01T12:30:45+05:30",
            "@2023-01-01T12:30:45-08:00",
        ];

        for expr_str in datetime_expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed datetime: {expr_str} -> {expr}");
                    // Verify it's a DateTime literal
                    if let Expression::Term(Term::Literal(Literal::DateTime(datetime))) = expr.root
                    {
                        assert!(!datetime.is_empty());
                        assert!(datetime.contains('T'));
                        assert!(datetime.contains(':'));
                    } else {
                        panic!(
                            "Expected DateTime literal for {expr_str}, got: {:?}",
                            expr.root
                        );
                    }
                }
                Err(e) => {
                    panic!("Failed to parse datetime {expr_str}: {e:?}");
                }
            }
        }

        // Test time literals
        let time_expressions = ["@T12:30:45", "@T00:00:00", "@T23:59:59"];

        for expr_str in time_expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed time: {expr_str} -> {expr}");
                    // Verify it's a Time literal
                    if let Expression::Term(Term::Literal(Literal::Time(time))) = expr.root {
                        assert!(!time.is_empty());
                        assert!(time.starts_with('T'));
                        assert!(time.contains(':'));
                    } else {
                        panic!("Expected Time literal for {expr_str}, got: {:?}", expr.root);
                    }
                }
                Err(e) => {
                    panic!("Failed to parse time {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_quantity_literals() {
        let parser = FhirPathParser::new();

        // Test basic quantity literals
        let quantity_expressions: Vec<(&str, Decimal, Option<String>)> = vec![
            (
                "5'mg'",
                Decimal::from_str_exact("5.0").unwrap(),
                Some("mg".to_string()),
            ),
            (
                "10.5'kg'",
                Decimal::from_str_exact("10.5").unwrap(),
                Some("kg".to_string()),
            ),
            (
                "100'mmHg'",
                Decimal::from_str_exact("100.0").unwrap(),
                Some("mmHg".to_string()),
            ),
            (
                "0.25'L'",
                Decimal::from_str_exact("0.25").unwrap(),
                Some("L".to_string()),
            ),
            ("42''", Decimal::from_str_exact("42.0").unwrap(), None),
            // Test quantity literals with space before unit
            (
                "15 'mm[Hg]'",
                Decimal::from_str_exact("15.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            (
                "37.2 'Cel'",
                Decimal::from_str_exact("37.2").unwrap(),
                Some("Cel".to_string()),
            ),
            (
                "5 'mg'",
                Decimal::from_str_exact("5.0").unwrap(),
                Some("mg".to_string()),
            ),
            (
                "2.5 'kg'",
                Decimal::from_str_exact("2.5").unwrap(),
                Some("kg".to_string()),
            ),
        ];

        for (expr_str, expected_value, expected_unit) in quantity_expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed quantity: {expr_str} -> {expr}");
                    // Verify it's a Quantity literal
                    if let Expression::Term(Term::Literal(Literal::Quantity { value, unit })) =
                        expr.root
                    {
                        assert_eq!(value, expected_value, "Value mismatch for {expr_str}");
                        assert_eq!(unit, expected_unit, "Unit mismatch for {expr_str}");
                    } else {
                        panic!(
                            "Expected Quantity literal for {expr_str}, got: {:?}",
                            expr.root
                        );
                    }
                }
                Err(e) => {
                    panic!("Failed to parse quantity {expr_str}: {e:?}");
                }
            }
        }

        // Test negative quantity (should parse as polarity expression)
        let result = parser.parse("-2.5'degC'");
        match result {
            Ok(expr) => {
                println!("✓ Parsed negative quantity: -2.5'degC' -> {expr}");
                // Should be a polarity expression containing a quantity
                if let Expression::Polarity { operator, operand } = expr.root {
                    assert!(matches!(operator, PolarityOperator::Minus));
                    if let Expression::Term(Term::Literal(Literal::Quantity { value, unit })) =
                        operand.as_ref()
                    {
                        assert_eq!(*value, Decimal::from_str_exact("2.5").unwrap());
                        assert_eq!(*unit, Some("degC".to_string()));
                    } else {
                        panic!(
                            "Expected Quantity literal in polarity expression, got: {operand:?}"
                        );
                    }
                } else {
                    panic!(
                        "Expected Polarity expression for -2.5'degC', got: {:?}",
                        expr.root
                    );
                }
            }
            Err(e) => {
                panic!("Failed to parse negative quantity -2.5'degC': {e:?}");
            }
        }

        // Test UCUM units (examples from healthcare)
        let ucum_expressions = [
            "37'Cel'",     // Celsius temperature
            "120'mm[Hg]'", // Millimeters of mercury
            "2.5'mg/kg'",  // Milligrams per kilogram
            "1'wk'",       // Week (calendar duration)
            "30'd'",       // Day (calendar duration)
            "6'mo'",       // Month (calendar duration)
            "2'a'",        // Year (calendar duration)
        ];

        for expr_str in ucum_expressions {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed UCUM quantity: {expr_str} -> {expr}");
                    // Verify it's a Quantity literal
                    if let Expression::Term(Term::Literal(Literal::Quantity { value: _, unit })) =
                        expr.root
                    {
                        assert!(
                            unit.is_some(),
                            "UCUM unit should not be empty for {expr_str}"
                        );
                    } else {
                        panic!(
                            "Expected Quantity literal for {expr_str}, got: {:?}",
                            expr.root
                        );
                    }
                }
                Err(e) => {
                    panic!("Failed to parse UCUM quantity {expr_str}: {e:?}");
                }
            }
        }
    }
}
