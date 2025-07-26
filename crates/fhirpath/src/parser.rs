//! FHIRPath parser implementation using nom

use crate::ast::*;
use crate::error::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

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
    let (input, first) = parse_and_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("or"), tag("xor")))),
        parse_and_expression,
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

// Parse AND expressions
fn parse_and_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_equality_expression(input)?;
    let (input, rest) = many0(preceded(ws(tag("and")), parse_equality_expression))(input)?;

    Ok((
        input,
        rest.into_iter().fold(first, |acc, expr| Expression::And {
            left: Box::new(acc),
            right: Box::new(expr),
        }),
    ))
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
    let (input, first) = parse_union_expression(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((tag("div"), tag("mod"), tag("*"), tag("/")))),
        parse_union_expression,
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
        parse_string_literal,
        parse_number_literal,
    ))(input)
}

// Parse string literal
fn parse_string_literal(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('\'')(input)?;
    let (input, content) = take_while1(|c| c != '\'')(input)?;
    let (input, _) = char('\'')(input)?;
    Ok((input, Literal::String(content.to_string())))
}

// Parse number literal
fn parse_number_literal(input: &str) -> IResult<&str, Literal> {
    let (input, number_str) = recognize(tuple((digit1, opt(tuple((char('.'), digit1))))))(input)?;

    if number_str.contains('.') {
        // Parse as float
        if let Ok(num) = number_str.parse::<f64>() {
            Ok((input, Literal::Number(num)))
        } else {
            Ok((input, Literal::Number(0.0)))
        }
    } else {
        // Parse as integer
        if let Ok(num) = number_str.parse::<i64>() {
            Ok((input, Literal::LongNumber(num)))
        } else {
            Ok((input, Literal::LongNumber(0)))
        }
    }
}

// Parse identifier - simplified version
fn parse_identifier(input: &str) -> IResult<&str, String> {
    let (input, ident) = recognize(tuple((
        alt((alpha1, tag("_"))),
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
}
