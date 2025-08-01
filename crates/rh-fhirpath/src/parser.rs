//! FHIRPath parser implementation using nom

use crate::ast::*;
use crate::error::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

// Parse compound duration literal (number + precision unit)
// Examples: "6 months", "2 years", "24 hours"
fn parse_compound_duration_literal(input: &str) -> IResult<&str, Literal> {
    let (input, number_str) = recognize(tuple((
        opt(char('-')),
        digit1,
        opt(tuple((char('.'), digit1))),
    )))(input)?;

    let (input, _) = multispace0(input)?; // Allow space between number and precision

    let (input, precision) = alt((
        // Plural forms first (to avoid partial matches)
        map(tag("years"), |_| DateTimePrecision::Year),
        map(tag("months"), |_| DateTimePrecision::Month),
        map(tag("weeks"), |_| DateTimePrecision::Week),
        map(tag("days"), |_| DateTimePrecision::Day),
        map(tag("hours"), |_| DateTimePrecision::Hour),
        map(tag("minutes"), |_| DateTimePrecision::Minute),
        map(tag("seconds"), |_| DateTimePrecision::Second),
        map(tag("milliseconds"), |_| DateTimePrecision::Millisecond),
        // Singular forms
        map(tag("year"), |_| DateTimePrecision::Year),
        map(tag("month"), |_| DateTimePrecision::Month),
        map(tag("week"), |_| DateTimePrecision::Week),
        map(tag("day"), |_| DateTimePrecision::Day),
        map(tag("hour"), |_| DateTimePrecision::Hour),
        map(tag("minute"), |_| DateTimePrecision::Minute),
        map(tag("second"), |_| DateTimePrecision::Second),
        map(tag("millisecond"), |_| DateTimePrecision::Millisecond),
    ))(input)?;

    // Parse the number
    let count = if number_str.contains('.') {
        // For fractional durations, we'll need to handle this as a floating-point operation
        number_str.parse::<f64>().unwrap_or(0.0) as i64
    } else {
        number_str.parse::<i64>().unwrap_or(0)
    };

    // Create a special compound duration quantity with the precision unit as the "unit"
    Ok((
        input,
        Literal::Quantity {
            value: count as f64,
            unit: Some(format!("{precision}")),
        },
    ))
}

/// FHIRPath expression parser
pub struct FhirPathParser;

impl FhirPathParser {
    /// Create a new FHIRPath parser
    pub fn new() -> Self {
        Self
    }

    /// Parse a FHIRPath expression from a string
    pub fn parse(&self, input: &str) -> FhirPathResult<FhirPathExpression> {
        match parse_expression(input.trim()) {
            Ok((remaining, expr)) => {
                let remaining = remaining.trim();
                if remaining.is_empty() {
                    Ok(FhirPathExpression { root: expr })
                } else {
                    Err(FhirPathError::SyntaxError {
                        line: 1,
                        column: input.len() - remaining.len(),
                        message: format!("Unexpected characters: {remaining}"),
                    })
                }
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

// Whitespace handling
fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O> + 'a,
{
    delimited(multispace0, inner, multispace0)
}

// Parse expression (top level)
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_or_expression(input)
}

// Parse OR/XOR expressions (lowest precedence)
fn parse_or_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_implies_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("or"), tag("xor")))),
        parse_implies_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
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
        }),
    ))
}

// Parse implies expressions (between and and or)
fn parse_implies_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_and_expression(input)?;
    let (input, rest) = many0(preceded(ws(tag("implies")), parse_and_expression))(input)?;

    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, expr| Expression::Implies {
                left: Box::new(acc),
                right: Box::new(expr),
            }),
    ))
}

// Parse AND expressions
fn parse_and_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_type_expression(input)?;
    let (input, rest) = many0(preceded(ws(tag("and")), parse_type_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| Expression::And {
            left: Box::new(acc),
            right: Box::new(expr),
        }),
    ))
}

// Parse type expressions (is, as)
fn parse_type_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_equality_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("is"), tag("as")))),
        parse_type_specifier,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, type_spec)| {
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
        }),
    ))
}

// Parse type specifier (qualified name like System.String)
fn parse_type_specifier(input: &str) -> IResult<&str, TypeSpecifier> {
    let (input, qualified_name) = separated_list1(tag("."), parse_identifier)(input)?;
    Ok((input, TypeSpecifier { qualified_name }))
}

// Parse equality and membership expressions (same precedence level)
fn parse_equality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_inequality_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            tag("!="),
            tag("!~"),
            tag("="),
            tag("~"),
            tag("contains"),
            tag("in"),
        ))),
        parse_inequality_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| match op {
            "=" => Expression::Equality {
                left: Box::new(acc),
                operator: EqualityOperator::Equal,
                right: Box::new(expr),
            },
            "~" => Expression::Equality {
                left: Box::new(acc),
                operator: EqualityOperator::Equivalent,
                right: Box::new(expr),
            },
            "!=" => Expression::Equality {
                left: Box::new(acc),
                operator: EqualityOperator::NotEqual,
                right: Box::new(expr),
            },
            "!~" => Expression::Equality {
                left: Box::new(acc),
                operator: EqualityOperator::NotEquivalent,
                right: Box::new(expr),
            },
            "in" => Expression::Membership {
                left: Box::new(acc),
                operator: MembershipOperator::In,
                right: Box::new(expr),
            },
            "contains" => Expression::Membership {
                left: Box::new(acc),
                operator: MembershipOperator::Contains,
                right: Box::new(expr),
            },
            _ => Expression::Equality {
                left: Box::new(acc),
                operator: EqualityOperator::Equal,
                right: Box::new(expr),
            },
        }),
    ))
}

// Parse inequality expressions (<, <=, >, >=)
fn parse_inequality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_additive_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("<="), tag(">="), tag("<"), tag(">")))),
        parse_additive_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
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
        }),
    ))
}

// Parse additive expressions (+, -, &)
fn parse_additive_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_multiplicative_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("+"), tag("-"), tag("&")))),
        parse_multiplicative_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
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
        }),
    ))
}

// Parse multiplicative expressions (*, /, div, mod)
fn parse_multiplicative_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_unary_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("div"), tag("mod"), tag("*"), tag("/")))),
        parse_unary_expression,
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, (op, expr)| {
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
        }),
    ))
}

// Parse unary expressions (-, not)
fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(ws(char('-')), parse_unary_expression), |expr| {
            Expression::Polarity {
                operator: PolarityOperator::Minus,
                operand: Box::new(expr),
            }
        }),
        parse_union_expression,
    ))(input)
}

// Parse union expressions
fn parse_union_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_invocation_expression(input)?;
    let (input, rest) = many0(preceded(ws(char('|')), parse_invocation_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| Expression::Union {
            left: Box::new(acc),
            right: Box::new(expr),
        }),
    ))
}

// Parse invocation expressions (member access and indexing)
fn parse_invocation_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_term(input)?;
    let (input, rest) = many0(alt((
        map(preceded(ws(char('.')), parse_invocation), |inv| {
            InvocationOrIndex::Invocation(inv)
        }),
        map(
            delimited(ws(char('[')), parse_expression, ws(char(']'))),
            InvocationOrIndex::Index,
        ),
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, item| match item {
            InvocationOrIndex::Invocation(inv) => Expression::Invocation {
                left: Box::new(acc),
                invocation: inv,
            },
            InvocationOrIndex::Index(idx) => Expression::Indexer {
                left: Box::new(acc),
                index: Box::new(idx),
            },
        }),
    ))
}

#[derive(Debug)]
enum InvocationOrIndex {
    Invocation(Invocation),
    Index(Expression),
}

// Parse term
fn parse_term(input: &str) -> IResult<&str, Expression> {
    alt((
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
    ))(input)
}

// Parse invocation (function call or member access)
fn parse_invocation(input: &str) -> IResult<&str, Invocation> {
    alt((
        map(tag("$this"), |_| Invocation::This),
        map(tag("$index"), |_| Invocation::Index),
        map(tag("$total"), |_| Invocation::Total),
        parse_function_or_member,
    ))(input)
}

// Parse function call or simple member access
fn parse_function_or_member(input: &str) -> IResult<&str, Invocation> {
    let (input, name) = parse_identifier(input)?;
    let (input, params) = opt(delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), parse_expression),
        ws(char(')')),
    ))(input)?;

    if let Some(parameters) = params {
        Ok((input, Invocation::Function { name, parameters }))
    } else {
        Ok((input, Invocation::Member(name)))
    }
}

// Parse external constant
fn parse_external_constant(input: &str) -> IResult<&str, String> {
    preceded(char('%'), parse_identifier)(input)
}

// Parse literal values
fn parse_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(tag("{}"), |_| Literal::Null),
        map(tag("true"), |_| Literal::Boolean(true)),
        map(tag("false"), |_| Literal::Boolean(false)),
        parse_datetime_literal,          // Try datetime first (longer pattern)
        parse_time_literal,              // Then time
        parse_date_literal,              // Then date (shorter pattern)
        parse_quantity_literal,          // Try quantity before string and number
        parse_compound_duration_literal, // Try compound duration (number + precision)
        parse_datetime_precision_literal, // Try datetime precision keywords
        parse_string_literal,
        parse_number_literal,
    ))(input)
}

// Parse string literal
fn parse_string_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('\'')(input)?;
    let (input, content) = take_while(|c| c != '\'')(input)?;
    let (input, _) = char('\'')(input)?;
    Ok((input, Literal::String(content.to_string())))
}

// Parse number literal
fn parse_number_literal(input: &str) -> IResult<&str, Literal> {
    let (input, number_str) = recognize(tuple((digit1, opt(tuple((char('.'), digit1))))))(input)?;

    // Check for Long suffix
    let (input, has_long_suffix) = opt(char('L'))(input)?;

    if has_long_suffix.is_some() {
        // Explicit Long literal with 'L' suffix
        if number_str.contains('.') {
            // Cannot have decimal with Long suffix - this is an error case
            // For now, parse as whole number part only
            let whole_part = number_str.split('.').next().unwrap_or("0");
            if let Ok(num) = whole_part.parse::<i64>() {
                Ok((input, Literal::LongNumber(num)))
            } else {
                Ok((input, Literal::LongNumber(0)))
            }
        } else {
            // Integer with L suffix - parse as Long
            if let Ok(num) = number_str.parse::<i64>() {
                Ok((input, Literal::LongNumber(num)))
            } else {
                Ok((input, Literal::LongNumber(0)))
            }
        }
    } else if number_str.contains('.') {
        // Parse as float (no L suffix, contains decimal)
        if let Ok(num) = number_str.parse::<f64>() {
            Ok((input, Literal::Number(num)))
        } else {
            Ok((input, Literal::Number(0.0)))
        }
    } else {
        // Parse as integer (no L suffix, no decimal) - use Integer type
        if let Ok(num) = number_str.parse::<i64>() {
            Ok((input, Literal::Integer(num)))
        } else {
            Ok((input, Literal::Integer(0)))
        }
    }
}

// Parse date literal (@YYYY, @YYYY-MM, or @YYYY-MM-DD)
fn parse_date_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;

    // Check if there's more date information
    let (input, month_day) = opt(tuple((
        preceded(
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ),
        opt(preceded(
            char('-'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        )),
    )))(input)?;

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

// Parse datetime literal (@YYYY-MM-DDTHH:mm:ss or @YYYY-MM-DDTHH:mm:ssZ or @YYYY-MM-DDTHH:mm:ss+HH:mm)
fn parse_datetime_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, month) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, day) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char('T')(input)?;
    let (input, hour) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, minute) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, second) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;

    // Optional timezone information
    let (input, timezone) = opt(alt((
        recognize(char('Z')),
        recognize(tuple((
            alt((char('+'), char('-'))),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            char(':'),
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        ))),
    )))(input)?;

    let datetime_str = if let Some(tz) = timezone {
        format!("{year}-{month}-{day}T{hour}:{minute}:{second}{tz}")
    } else {
        format!("{year}-{month}-{day}T{hour}:{minute}:{second}")
    };

    Ok((input, Literal::DateTime(datetime_str)))
}

// Parse time literal (@Thh:mm:ss)
fn parse_time_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('@')(input)?;
    let (input, _) = char('T')(input)?;
    let (input, hour) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, minute) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, second) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)?;

    let time_str = format!("T{hour}:{minute}:{second}");
    Ok((input, Literal::Time(time_str)))
}

// Parse quantity literal (number followed by single-quoted unit)
// Examples: 5'mg', 10.5'kg', 100'mmHg', 15 'mm[Hg]' (with optional space)
fn parse_quantity_literal(input: &str) -> IResult<&str, Literal> {
    // Parse the numeric value (integer or decimal)
    let (input, number_str) = recognize(tuple((
        opt(char('-')), // Optional negative sign
        digit1,
        opt(tuple((char('.'), digit1))),
    )))(input)?;

    // Parse optional whitespace between number and unit
    let (input, _) = multispace0(input)?;

    // Parse the unit string (single-quoted)
    let (input, _) = char('\'')(input)?;
    let (input, unit) = take_while(|c| c != '\'')(input)?;
    let (input, _) = char('\'')(input)?;

    // Convert the number string to f64 (quantities are always decimal)
    let value = number_str.parse::<f64>().unwrap_or(0.0); // Fallback for invalid numbers

    // Unit is optional per FHIR spec (can be empty string)
    let unit_opt = if unit.is_empty() {
        None
    } else {
        Some(unit.to_string())
    };

    Ok((
        input,
        Literal::Quantity {
            value,
            unit: unit_opt,
        },
    ))
}

// Parse datetime precision literal
// Examples: 'year', 'years', 'month', 'months', 'week', 'weeks', etc.
fn parse_datetime_precision_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        // Plural forms first (to avoid partial matches)
        map(tag("years"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Year)
        }),
        map(tag("months"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Month)
        }),
        map(tag("weeks"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Week)
        }),
        map(tag("days"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Day)
        }),
        map(tag("hours"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Hour)
        }),
        map(tag("minutes"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Minute)
        }),
        map(tag("seconds"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Second)
        }),
        map(tag("milliseconds"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Millisecond)
        }),
        // Singular forms
        map(tag("year"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Year)
        }),
        map(tag("month"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Month)
        }),
        map(tag("week"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Week)
        }),
        map(tag("day"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Day)
        }),
        map(tag("hour"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Hour)
        }),
        map(tag("minute"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Minute)
        }),
        map(tag("second"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Second)
        }),
        map(tag("millisecond"), |_| {
            Literal::DateTimePrecision(DateTimePrecision::Millisecond)
        }),
    ))(input)
}

// Parse identifier - simplified version
fn parse_identifier(input: &str) -> IResult<&str, String> {
    let (input, ident) = recognize(tuple((
        alt((alpha1, recognize(char('_')))),
        opt(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
    )))(input)?;
    Ok((input, ident.to_string()))
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
        let quantity_expressions = [
            ("5'mg'", 5.0, Some("mg".to_string())),
            ("10.5'kg'", 10.5, Some("kg".to_string())),
            ("100'mmHg'", 100.0, Some("mmHg".to_string())),
            ("0.25'L'", 0.25, Some("L".to_string())),
            ("42''", 42.0, None), // Empty unit
            // Test quantity literals with space before unit
            ("15 'mm[Hg]'", 15.0, Some("mm[Hg]".to_string())),
            ("37.2 'Cel'", 37.2, Some("Cel".to_string())),
            ("5 'mg'", 5.0, Some("mg".to_string())),
            ("2.5 'kg'", 2.5, Some("kg".to_string())),
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
                        assert_eq!(*value, 2.5);
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
