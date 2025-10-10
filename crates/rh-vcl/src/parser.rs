//! VCL parser implementation using nom
//!
//! This module provides the main parsing functionality for VCL expressions.

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0},
    combinator::{all_consuming, map, opt, recognize, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use crate::{
    ast::*,
    error::{VclError, VclResult},
};

/// Parse a complete VCL expression
pub fn parse_vcl(input: &str) -> VclResult<VclExpression> {
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        return Err(VclError::parse_error("Empty VCL expression", 0, input));
    }

    match all_consuming(vcl)(trimmed_input) {
        Ok((_, expr)) => Ok(expr),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let position = trimmed_input.len() - e.input.len();
            let error_message = match e.code {
                nom::error::ErrorKind::Tag => "Unexpected token or character",
                nom::error::ErrorKind::Char => "Expected specific character",
                nom::error::ErrorKind::Many1 => "Expected one or more elements",
                nom::error::ErrorKind::Alt => "No valid alternative found",
                nom::error::ErrorKind::TakeWhile1 => "Expected valid characters",
                nom::error::ErrorKind::SeparatedList => "Invalid list format",
                _ => "Parse error",
            };

            // Try to provide context about what was being parsed
            let context_message = if position < 10 {
                format!("{error_message} near start of expression")
            } else {
                let start = position.saturating_sub(10);
                let context = &trimmed_input[start..position.min(trimmed_input.len())];
                format!("{error_message} after '{context}'")
            };

            Err(VclError::parse_error(context_message, position, input))
        }
        Err(nom::Err::Incomplete(_)) => Err(VclError::UnexpectedEof {
            position: trimmed_input.len(),
        }),
    }
}

// Grammar implementation following the ANTLR grammar

/// vcl: expr EOF
fn vcl(input: &str) -> IResult<&str, VclExpression> {
    map(expr, |expr| VclExpression { expr })(input)
}

/// expr: subExpr (conjunction | disjunction | exclusion )?
fn expr(input: &str) -> IResult<&str, Expression> {
    map(
        pair(sub_expr, opt(alt((conjunction, disjunction, exclusion)))),
        |(sub_expr, operation)| Expression {
            sub_expr,
            operation,
        },
    )(input)
}

/// subExpr: systemUri? (simpleExpr | OPEN expr CLOSE)
fn sub_expr(input: &str) -> IResult<&str, SubExpression> {
    map(
        pair(
            opt(system_uri),
            alt((
                map(simple_expr, SubExpressionContent::Simple),
                map(delimited(ws(char('(')), expr, ws(char(')'))), |e| {
                    SubExpressionContent::Nested(Box::new(e))
                }),
            )),
        ),
        |(system_uri, content)| SubExpression {
            system_uri,
            content,
        },
    )(input)
}

/// conjunction: (COMMA subExpr)+
fn conjunction(input: &str) -> IResult<&str, Operation> {
    map(
        many1(preceded(ws(char(',')), sub_expr)),
        Operation::Conjunction,
    )(input)
}

/// disjunction: (SEMI subExpr)+
fn disjunction(input: &str) -> IResult<&str, Operation> {
    map(
        many1(preceded(ws(char(';')), sub_expr)),
        Operation::Disjunction,
    )(input)
}

/// exclusion: DASH subExpr
fn exclusion(input: &str) -> IResult<&str, Operation> {
    map(preceded(ws(char('-')), sub_expr), Operation::Exclusion)(input)
}

/// simpleExpr: STAR | code | filter | includeVs
fn simple_expr(input: &str) -> IResult<&str, SimpleExpression> {
    alt((
        map(include_vs, SimpleExpression::IncludeValueSet),
        map(filter, SimpleExpression::Filter),
        value(SimpleExpression::Wildcard, ws(char('*'))),
        map(code, SimpleExpression::Code),
    ))(input)
}

/// includeVs: IN (URI | systemUri)
fn include_vs(input: &str) -> IResult<&str, IncludeValueSet> {
    preceded(
        ws(char('^')),
        alt((
            map(system_uri, IncludeValueSet::SystemUri),
            map(uri, IncludeValueSet::Uri),
        )),
    )(input)
}

/// systemUri: OPEN URI (PIPE version)? CLOSE
fn system_uri(input: &str) -> IResult<&str, SystemUri> {
    map(
        delimited(
            ws(char('(')),
            pair(uri, opt(preceded(char('|'), version_string))),
            ws(char(')')),
        ),
        |(uri, version)| SystemUri {
            uri,
            version: version.map(|v| v.to_string()),
        },
    )(input)
}

/// Parse version string (alphanumeric, dots, dashes, underscores)
fn version_string(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')(input)
}

/// Basic token parsers
/// Parse whitespace wrapper
fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

/// URI: [a-zA-Z]+ [:] [a-zA-Z0-9?=:;&_%+-.@#$^!{}/]+ ('|' ~[|()] *)?
fn uri(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((
            take_while1(|c: char| c.is_ascii_alphabetic()),
            char(':'),
            take_while1(|c: char| {
                c.is_ascii_alphanumeric() || "?=:;&_%+-.@#$^!{}/".contains(c) && c != '|'
            }),
        ))),
        |s: &str| s.to_string(),
    )(input)
}

/// SCODE: [a-zA-Z0-9] [-_a-zA-Z0-9]*
fn scode(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_ascii_alphanumeric()),
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        )),
        |s: &str| s.to_string(),
    )(input)
}

/// QUOTED_VALUE: '"' (~["\\] | '\\' ["\\])* '"'
fn quoted_value(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let mut result = String::new();
    let mut chars = input.chars();
    let mut consumed = 1; // for opening quote

    while let Some(ch) = chars.next() {
        consumed += ch.len_utf8();
        match ch {
            '"' => {
                // End of quoted string
                return Ok((&input[consumed - 1..], result));
            }
            '\\' => {
                // Escape sequence
                if let Some(escaped) = chars.next() {
                    consumed += escaped.len_utf8();
                    match escaped {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                } else {
                    // Backslash at end of input
                    result.push('\\');
                    break;
                }
            }
            _ => result.push(ch),
        }
    }

    // Unterminated quoted string
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

/// code: SCODE | QUOTED_VALUE
fn code(input: &str) -> IResult<&str, Code> {
    ws(alt((
        map(quoted_value, Code::Quoted),
        map(scode, Code::Simple),
    )))(input)
}

/// codeList: LCRLY code (COMMA code)+ RCRLY
fn code_list(input: &str) -> IResult<&str, Vec<Code>> {
    delimited(
        ws(char('{')),
        separated_list1(ws(char(',')), code),
        ws(char('}')),
    )(input)
}

// Filter parsing implementation

/// filter: (property ...) | ((code | codeList | STAR | URI | filterList) DOT property)
fn filter(input: &str) -> IResult<&str, Filter> {
    alt((property_filter, of_operation_filter))(input)
}

/// Property-based filter: property (EQ code | IS_A code | ...)
fn property_filter(input: &str) -> IResult<&str, Filter> {
    map(
        tuple((
            code, // property is just a code
            filter_operator,
            filter_value,
        )),
        |(property, operator, value)| Filter::PropertyFilter {
            property,
            operator,
            value,
        },
    )(input)
}

/// "Of" operation filter: (code|codeList|*|uri|filterList).property
fn of_operation_filter(input: &str) -> IResult<&str, Filter> {
    map(
        tuple((of_source, ws(char('.')), code)),
        |(source, _, property)| Filter::OfOperation { source, property },
    )(input)
}

/// Parse filter operators
fn filter_operator(input: &str) -> IResult<&str, FilterOperator> {
    ws(alt((
        value(FilterOperator::IsNotA, tag("~<<")),
        value(FilterOperator::IsA, tag("<<")),
        value(FilterOperator::DescendantLeaf, tag("!!<")),
        value(FilterOperator::ChildOf, tag("<!")),
        value(FilterOperator::DescendantOf, char('<')),
        value(FilterOperator::NotIn, tag("~^")),
        value(FilterOperator::In, char('^')),
        value(FilterOperator::Generalizes, tag(">>")),
        value(FilterOperator::Equals, char('=')),
        value(FilterOperator::Regex, char('/')),
        value(FilterOperator::Exists, char('?')),
    )))(input)
}

/// Parse filter values
fn filter_value(input: &str) -> IResult<&str, FilterValue> {
    ws(alt((
        map(filter_list, FilterValue::FilterList),
        map(code_list, FilterValue::CodeList),
        map(uri, FilterValue::Uri),
        map(quoted_value, FilterValue::String), // for regex patterns
        map(code, FilterValue::Code),
    )))(input)
}

/// Parse source for "of" operations
fn of_source(input: &str) -> IResult<&str, OfSource> {
    ws(alt((
        map(filter_list, OfSource::FilterList),
        map(code_list, OfSource::CodeList),
        map(uri, OfSource::Uri),
        value(OfSource::Wildcard, char('*')),
        map(code, OfSource::Code),
    )))(input)
}

/// filterList: LCRLY filter (COMMA filter)* RCRLY
fn filter_list(input: &str) -> IResult<&str, Vec<Filter>> {
    delimited(
        ws(char('{')),
        separated_list1(ws(char(',')), filter),
        ws(char('}')),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scode_parsing() {
        assert_eq!(scode("abc123").unwrap(), ("", "abc123".to_string()));
        assert_eq!(
            scode("A1_test-code").unwrap(),
            ("", "A1_test-code".to_string())
        );
        assert_eq!(scode("123abc").unwrap(), ("", "123abc".to_string()));
    }

    #[test]
    fn test_quoted_value_parsing() {
        assert_eq!(
            quoted_value("\"hello world\"").unwrap(),
            ("", "hello world".to_string())
        );
        assert_eq!(
            quoted_value("\"escaped \\\"quote\\\"\"").unwrap(),
            ("", "escaped \"quote\"".to_string())
        );
        assert_eq!(
            quoted_value("\"backslash \\\\\"").unwrap(),
            ("", "backslash \\".to_string())
        );
    }

    #[test]
    fn test_uri_parsing() {
        assert_eq!(
            uri("http://example.org/fhir").unwrap(),
            ("", "http://example.org/fhir".to_string())
        );
        assert_eq!(
            uri("https://terminology.hl7.org/CodeSystem/v3-ActCode").unwrap(),
            (
                "",
                "https://terminology.hl7.org/CodeSystem/v3-ActCode".to_string()
            )
        );
    }

    #[test]
    fn test_code_parsing() {
        let result = code("test123").unwrap();
        assert_eq!(result.1, Code::Simple("test123".to_string()));

        let result = code("\"quoted code\"").unwrap();
        assert_eq!(result.1, Code::Quoted("quoted code".to_string()));
    }

    #[test]
    fn test_simple_wildcard() {
        let result = parse_vcl("*").unwrap();
        assert_eq!(
            result.expr.sub_expr.content,
            SubExpressionContent::Simple(SimpleExpression::Wildcard)
        );
    }

    #[test]
    fn test_simple_code() {
        let result = parse_vcl("test123").unwrap();
        if let SubExpressionContent::Simple(SimpleExpression::Code(code)) =
            &result.expr.sub_expr.content
        {
            assert_eq!(code, &Code::Simple("test123".to_string()));
        } else {
            panic!("Expected simple code expression");
        }
    }

    #[test]
    fn test_filter_operator_parsing() {
        assert_eq!(filter_operator("=").unwrap().1, FilterOperator::Equals);
        assert_eq!(filter_operator("<<").unwrap().1, FilterOperator::IsA);
        assert_eq!(filter_operator("~<<").unwrap().1, FilterOperator::IsNotA);
        assert_eq!(
            filter_operator("<").unwrap().1,
            FilterOperator::DescendantOf
        );
        assert_eq!(filter_operator("^").unwrap().1, FilterOperator::In);
        assert_eq!(filter_operator("~^").unwrap().1, FilterOperator::NotIn);
        assert_eq!(
            filter_operator(">>").unwrap().1,
            FilterOperator::Generalizes
        );
        assert_eq!(filter_operator("<!").unwrap().1, FilterOperator::ChildOf);
        assert_eq!(
            filter_operator("!!<").unwrap().1,
            FilterOperator::DescendantLeaf
        );
        assert_eq!(filter_operator("?").unwrap().1, FilterOperator::Exists);
        assert_eq!(filter_operator("/").unwrap().1, FilterOperator::Regex);
    }

    #[test]
    fn test_property_filter() {
        let result = property_filter("status = active").unwrap().1;
        match result {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                assert_eq!(property, Code::Simple("status".to_string()));
                assert_eq!(operator, FilterOperator::Equals);
                match value {
                    FilterValue::Code(Code::Simple(code)) => assert_eq!(code, "active"),
                    _ => panic!("Expected simple code value"),
                }
            }
            _ => panic!("Expected property filter"),
        }
    }

    #[test]
    fn test_include_vs_parsing() {
        let result = parse_vcl("^http://example.org/valueset").unwrap();
        if let SubExpressionContent::Simple(SimpleExpression::IncludeValueSet(vs)) =
            &result.expr.sub_expr.content
        {
            match vs {
                IncludeValueSet::Uri(uri) => assert_eq!(uri, "http://example.org/valueset"),
                _ => panic!("Expected URI include"),
            }
        } else {
            panic!("Expected include valueset expression");
        }
    }

    #[test]
    fn test_system_uri() {
        let result = parse_vcl("(http://snomed.info/sct)123456").unwrap();
        assert!(result.expr.sub_expr.system_uri.is_some());
        let system_uri = result.expr.sub_expr.system_uri.unwrap();
        assert_eq!(system_uri.uri, "http://snomed.info/sct");
        assert_eq!(system_uri.version, None);

        if let SubExpressionContent::Simple(SimpleExpression::Code(code)) =
            &result.expr.sub_expr.content
        {
            assert_eq!(code, &Code::Simple("123456".to_string()));
        } else {
            panic!("Expected simple code expression");
        }
    }

    #[test]
    fn test_system_uri_with_version() {
        let result = parse_vcl("(http://snomed.info/sct|2025)123456").unwrap();
        assert!(result.expr.sub_expr.system_uri.is_some());
        let system_uri = result.expr.sub_expr.system_uri.unwrap();
        assert_eq!(system_uri.uri, "http://snomed.info/sct");
        assert_eq!(system_uri.version, Some("2025".to_string()));

        if let SubExpressionContent::Simple(SimpleExpression::Code(code)) =
            &result.expr.sub_expr.content
        {
            assert_eq!(code, &Code::Simple("123456".to_string()));
        } else {
            panic!("Expected simple code expression");
        }
    }

    #[test]
    fn test_system_uri_with_complex_version() {
        let result = parse_vcl("(http://loinc.org|v2.76)8302-2").unwrap();
        assert!(result.expr.sub_expr.system_uri.is_some());
        let system_uri = result.expr.sub_expr.system_uri.unwrap();
        assert_eq!(system_uri.uri, "http://loinc.org");
        assert_eq!(system_uri.version, Some("v2.76".to_string()));

        if let SubExpressionContent::Simple(SimpleExpression::Code(code)) =
            &result.expr.sub_expr.content
        {
            assert_eq!(code, &Code::Simple("8302-2".to_string()));
        } else {
            panic!("Expected simple code expression");
        }
    }

    #[test]
    fn test_conjunction() {
        let result = parse_vcl("code1, code2, code3").unwrap();
        if let Some(Operation::Conjunction(codes)) = &result.expr.operation {
            assert_eq!(codes.len(), 2); // First code is in sub_expr, rest in conjunction
                                        // Check that all are simple codes
            for sub_expr in codes {
                if let SubExpressionContent::Simple(SimpleExpression::Code(_)) = &sub_expr.content {
                    // Good
                } else {
                    panic!("Expected code in conjunction");
                }
            }
        } else {
            panic!("Expected conjunction operation");
        }
    }

    #[test]
    fn test_disjunction() {
        let result = parse_vcl("code1; code2; code3").unwrap();
        if let Some(Operation::Disjunction(codes)) = &result.expr.operation {
            assert_eq!(codes.len(), 2); // First code is in sub_expr, rest in disjunction
        } else {
            panic!("Expected disjunction operation");
        }
    }

    #[test]
    fn test_exclusion() {
        let result = parse_vcl("* - inactive").unwrap();
        if let Some(Operation::Exclusion(excluded)) = &result.expr.operation {
            if let SubExpressionContent::Simple(SimpleExpression::Code(code)) = &excluded.content {
                assert_eq!(code, &Code::Simple("inactive".to_string()));
            } else {
                panic!("Expected simple code in exclusion");
            }
        } else {
            panic!("Expected exclusion operation");
        }
    }

    #[test]
    fn test_nested_expression() {
        let result = parse_vcl("(code1, code2)").unwrap();
        if let SubExpressionContent::Nested(nested_expr) = &result.expr.sub_expr.content {
            // The nested expression should have a conjunction
            assert!(nested_expr.operation.is_some());
        } else {
            panic!("Expected nested expression");
        }
    }

    #[test]
    fn test_code_list() {
        let result = code_list("{code1, code2, \"quoted code\"}").unwrap();
        assert_eq!(result.1.len(), 3);
        assert_eq!(result.1[0], Code::Simple("code1".to_string()));
        assert_eq!(result.1[1], Code::Simple("code2".to_string()));
        assert_eq!(result.1[2], Code::Quoted("quoted code".to_string()));
    }

    #[test]
    fn test_complex_vcl_example() {
        // Test a real-world-like VCL expression
        let result =
            parse_vcl("(http://snomed.info/sct)status = \"active\", category << 123456").unwrap();

        // Should have a system URI on the main sub-expression
        assert!(result.expr.sub_expr.system_uri.is_some());

        // Should have a conjunction operation
        if let Some(Operation::Conjunction(exprs)) = &result.expr.operation {
            assert_eq!(exprs.len(), 1);

            // Second expression should be a filter
            if let SubExpressionContent::Simple(SimpleExpression::Filter(filter)) =
                &exprs[0].content
            {
                match filter {
                    Filter::PropertyFilter {
                        property,
                        operator,
                        value,
                    } => {
                        assert_eq!(property, &Code::Simple("category".to_string()));
                        assert_eq!(operator, &FilterOperator::IsA);
                        match value {
                            FilterValue::Code(Code::Simple(code)) => assert_eq!(code, "123456"),
                            _ => panic!("Expected simple code in filter value"),
                        }
                    }
                    _ => panic!("Expected property filter"),
                }
            } else {
                panic!("Expected filter in second expression");
            }
        } else {
            panic!("Expected conjunction");
        }
    }

    #[test]
    fn test_of_operation_filter() {
        let result = of_operation_filter("*.category").unwrap().1;
        match result {
            Filter::OfOperation { source, property } => {
                assert_eq!(source, OfSource::Wildcard);
                assert_eq!(property, Code::Simple("category".to_string()));
            }
            _ => panic!("Expected of operation filter"),
        }
    }

    // Error handling tests
    #[test]
    fn test_empty_input() {
        let result = parse_vcl("");
        assert!(result.is_err());
        match result.unwrap_err() {
            VclError::ParseError { message, .. } => {
                assert!(message.contains("Empty VCL expression"));
            }
            _ => panic!("Expected parse error for empty input"),
        }
    }

    #[test]
    fn test_whitespace_only() {
        let result = parse_vcl("   \t  ");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_operator() {
        let result = parse_vcl("code1 $ code2");
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_parentheses() {
        let result = parse_vcl("(code1, code2");
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_quoted_string() {
        let result = quoted_value("\"unclosed string");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_uri() {
        // URI must start with alphabetic characters followed by colon
        let result = uri("123://invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_code_list() {
        let result = parse_vcl("{code1, code2,}"); // trailing comma
        assert!(result.is_err());
    }

    #[test]
    fn test_display_code() {
        let simple_code = Code::Simple("test123".to_string());
        assert_eq!(format!("{simple_code}"), "test123");

        let quoted_code = Code::Quoted("quoted value".to_string());
        assert_eq!(format!("{quoted_code}"), "\"quoted value\"");
    }

    #[test]
    fn test_code_value() {
        let simple_code = Code::Simple("test123".to_string());
        assert_eq!(simple_code.value(), "test123");

        let quoted_code = Code::Quoted("quoted value".to_string());
        assert_eq!(quoted_code.value(), "quoted value");
    }
}
