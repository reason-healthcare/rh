//! CQL Statement and Library Parser
//!
//! Parses CQL library structure including:
//! - Library header (library, using, include)
//! - Terminology definitions (codesystem, valueset, code, concept)
//! - Parameter and context definitions
//! - Expression and function definitions

use super::ast::*;
use super::expression::{expression, parse_type_specifier};
use super::lexer::{any_identifier, keyword, skip_ws_and_comments, string_literal, ws};
use super::span::Span;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt, value},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

// ============================================================================
// Library Header
// ============================================================================

/// Parse library identifier: `library Name version 'x.x.x'`
pub fn parse_library_identifier(input: Span<'_>) -> IResult<Span<'_>, LibraryIdentifier> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("library")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = parse_qualified_name(input)?;
    let (input, version) = opt(preceded(ws(keyword("version")), string_literal))(input)?;

    Ok((input, LibraryIdentifier { name, version }))
}

fn parse_qualified_name(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, first) = any_identifier(input)?;
    let (input, rest) = many0(preceded(char('.'), any_identifier))(input)?;

    let mut name = first;
    for part in rest {
        name.push('.');
        name.push_str(&part);
    }
    Ok((input, name))
}

// ============================================================================
// Using Definition
// ============================================================================

/// Parse using definition: `using FHIR version '4.0.1'`
pub fn parse_using_def(input: Span<'_>) -> IResult<Span<'_>, UsingDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("using")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, model_name) = any_identifier(input)?;
    let (input, version) = opt(preceded(ws(keyword("version")), string_literal))(input)?;

    Ok((
        input,
        UsingDef {
            model_name,
            version,
            location: None,
        },
    ))
}

// ============================================================================
// Include Definition
// ============================================================================

/// Parse include definition: `include Library.Name version 'x' called Alias`
pub fn parse_include_def(input: Span<'_>) -> IResult<Span<'_>, IncludeDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("include")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, path) = parse_qualified_name(input)?;
    let (input, version) = opt(preceded(ws(keyword("version")), string_literal))(input)?;
    let (input, alias) = opt(preceded(ws(keyword("called")), any_identifier))(input)?;

    Ok((
        input,
        IncludeDef {
            path,
            version,
            alias,
            location: None,
        },
    ))
}

// ============================================================================
// CodeSystem Definition
// ============================================================================

/// Parse codesystem definition: `codesystem Name: 'oid' version 'x'`
pub fn parse_codesystem_def(input: Span<'_>) -> IResult<Span<'_>, CodeSystemDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("codesystem")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, id) = string_literal(input)?;
    let (input, version) = opt(preceded(ws(keyword("version")), string_literal))(input)?;

    Ok((
        input,
        CodeSystemDef {
            name,
            id,
            version,
            access,
            location: None,
        },
    ))
}

// ============================================================================
// ValueSet Definition
// ============================================================================

/// Parse valueset definition: `valueset Name: 'oid' version 'x' codesystems { CS1, CS2 }`
pub fn parse_valueset_def(input: Span<'_>) -> IResult<Span<'_>, ValueSetDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("valueset")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, id) = string_literal(input)?;
    let (input, version) = opt(preceded(ws(keyword("version")), string_literal))(input)?;
    let (input, codesystems) = opt(preceded(
        ws(keyword("codesystems")),
        delimited(ws(char('{')), parse_codesystem_list, ws(char('}'))),
    ))(input)?;

    Ok((
        input,
        ValueSetDef {
            name,
            id,
            version,
            codesystems: codesystems.unwrap_or_default(),
            access,
            location: None,
        },
    ))
}

fn parse_codesystem_list(input: Span<'_>) -> IResult<Span<'_>, Vec<String>> {
    let (input, first) = any_identifier(input)?;
    let (input, rest) = many0(preceded(ws(char(',')), any_identifier))(input)?;

    let mut list = vec![first];
    list.extend(rest);
    Ok((input, list))
}

// ============================================================================
// Code Definition
// ============================================================================

/// Parse code definition: `code Name: 'code' from CodeSystem display 'Display'`
pub fn parse_code_def(input: Span<'_>) -> IResult<Span<'_>, CodeDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("code")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, code) = string_literal(input)?;
    let (input, _) = ws(keyword("from"))(input)?;
    let (input, codesystem) = any_identifier(input)?;
    let (input, display) = opt(preceded(ws(keyword("display")), string_literal))(input)?;

    Ok((
        input,
        CodeDef {
            name,
            code,
            codesystem,
            display,
            access,
            location: None,
        },
    ))
}

// ============================================================================
// Concept Definition
// ============================================================================

/// Parse concept definition: `concept Name: { Code1, Code2 } display 'Display'`
pub fn parse_concept_def(input: Span<'_>) -> IResult<Span<'_>, ConceptDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("concept")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, _) = ws(char('{'))(input)?;
    let (input, codes) = parse_codesystem_list(input)?;
    let (input, _) = ws(char('}'))(input)?;
    let (input, display) = opt(preceded(ws(keyword("display")), string_literal))(input)?;

    Ok((
        input,
        ConceptDef {
            name,
            codes,
            display,
            access,
            location: None,
        },
    ))
}

// ============================================================================
// Parameter Definition
// ============================================================================

/// Parse parameter definition: `parameter Name Type default value`
pub fn parse_parameter_def(input: Span<'_>) -> IResult<Span<'_>, ParameterDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("parameter")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, type_specifier) = opt(ws(parse_type_specifier))(input)?;
    let (input, default) = opt(preceded(ws(keyword("default")), expression))(input)?;

    Ok((
        input,
        ParameterDef {
            name,
            type_specifier,
            default,
            access,
            location: None,
        },
    ))
}

// ============================================================================
// Context Definition
// ============================================================================

/// Parse context definition: `context Patient`
pub fn parse_context_def(input: Span<'_>) -> IResult<Span<'_>, ContextDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("context")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;

    Ok((
        input,
        ContextDef {
            name,
            location: None,
        },
    ))
}

// ============================================================================
// Expression Definition
// ============================================================================

/// Parse expression definition: `define Name: expression`
pub fn parse_expression_def(input: Span<'_>) -> IResult<Span<'_>, ExpressionDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let start_loc = input.location();
    let (input, access) = parse_access_modifier(input)?;
    let (input, _) = keyword("define")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char(':'))(input)?;
    let (input, expr) = expression(input)?;

    Ok((
        input,
        ExpressionDef {
            name,
            expression: expr,
            access,
            location: Some(start_loc),
        },
    ))
}

// ============================================================================
// Function Definition
// ============================================================================

/// Parse function definition: `define function Name(params) returns Type: body`
pub fn parse_function_def(input: Span<'_>) -> IResult<Span<'_>, FunctionDef> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, access) = parse_access_modifier(input)?;
    let (input, fluent) = opt(keyword("fluent"))(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("define")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, _) = keyword("function")(input)?;
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, name) = any_identifier(input)?;
    let (input, _) = ws(char('('))(input)?;
    let (input, parameters) = parse_function_parameters(input)?;
    let (input, _) = ws(char(')'))(input)?;
    let (input, return_type) = opt(preceded(ws(keyword("returns")), parse_type_specifier))(input)?;

    // Check for external or body
    let (input, external_or_body) = alt((
        map(ws(keyword("external")), |_| ExternalOrBody::External),
        map(pair(ws(char(':')), expression), |(_, e)| {
            ExternalOrBody::Body(Box::new(e))
        }),
    ))(input)?;

    let (external, body) = match external_or_body {
        ExternalOrBody::External => (true, None),
        ExternalOrBody::Body(e) => (false, Some(*e)),
    };

    Ok((
        input,
        FunctionDef {
            name,
            parameters,
            return_type,
            body,
            fluent: fluent.is_some(),
            external,
            access,
            location: None,
        },
    ))
}

enum ExternalOrBody {
    External,
    Body(Box<Expression>),
}

fn parse_function_parameters(input: Span<'_>) -> IResult<Span<'_>, Vec<FunctionParameter>> {
    let (input, first) = opt(parse_function_parameter)(input)?;

    match first {
        Some(param) => {
            let (input, rest) = many0(preceded(ws(char(',')), parse_function_parameter))(input)?;
            let mut params = vec![param];
            params.extend(rest);
            Ok((input, params))
        }
        None => Ok((input, vec![])),
    }
}

fn parse_function_parameter(input: Span<'_>) -> IResult<Span<'_>, FunctionParameter> {
    let (input, name) = any_identifier(input)?;
    let (input, type_specifier) = opt(ws(parse_type_specifier))(input)?;

    Ok((
        input,
        FunctionParameter {
            name,
            type_specifier,
        },
    ))
}

// ============================================================================
// Statement (Expression or Function Definition)
// ============================================================================

/// Parse a statement (expression or function definition)
pub fn parse_statement(input: Span<'_>) -> IResult<Span<'_>, Statement> {
    let (input, _) = skip_ws_and_comments(input)?;

    // Peek ahead to determine if this is a function or expression definition
    // Function definitions have: [access] [fluent] define function
    // Expression definitions have: [access] define Name:

    // Try function first (since it's more specific)
    if let Ok((remaining, func_def)) = parse_function_def(input) {
        return Ok((remaining, Statement::FunctionDef(func_def)));
    }

    // Try expression definition
    if let Ok((remaining, expr_def)) = parse_expression_def(input) {
        return Ok((remaining, Statement::ExpressionDef(expr_def)));
    }

    // Fallback error
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

// ============================================================================
// Access Modifier
// ============================================================================

fn parse_access_modifier(input: Span<'_>) -> IResult<Span<'_>, AccessModifier> {
    let (input, _) = skip_ws_and_comments(input)?;
    let (input, modifier) = opt(alt((
        value(AccessModifier::Public, keyword("public")),
        value(AccessModifier::Private, keyword("private")),
    )))(input)?;
    // Skip whitespace after the modifier
    let (input, _) = skip_ws_and_comments(input)?;

    Ok((input, modifier.unwrap_or_default()))
}

// ============================================================================
// Full Library Parser
// ============================================================================

/// A terminology definition (codesystem, valueset, code, or concept)
#[derive(Debug, Clone)]
enum TerminologyDef {
    CodeSystem(CodeSystemDef),
    ValueSet(ValueSetDef),
    Code(CodeDef),
    Concept(ConceptDef),
}

/// Parse a single terminology definition (codesystem, valueset, code, or concept)
fn parse_terminology_def(input: Span<'_>) -> IResult<Span<'_>, TerminologyDef> {
    alt((
        map(parse_codesystem_def, TerminologyDef::CodeSystem),
        map(parse_valueset_def, TerminologyDef::ValueSet),
        map(parse_code_def, TerminologyDef::Code),
        map(parse_concept_def, TerminologyDef::Concept),
    ))(input)
}

/// Parse a complete CQL library
pub fn parse_library(input: Span<'_>) -> IResult<Span<'_>, Library> {
    let (input, _) = skip_ws_and_comments(input)?;

    // Parse library identifier (optional)
    let (input, identifier) = opt(parse_library_identifier)(input)?;

    // Parse using definitions
    let (input, usings) = many0(parse_using_def)(input)?;

    // Parse include definitions
    let (input, includes) = many0(parse_include_def)(input)?;

    // Parse terminology definitions in any order (codesystem, valueset, code, concept)
    let (input, terminology_defs) = many0(parse_terminology_def)(input)?;

    // Separate terminology definitions by type
    let mut codesystems = Vec::new();
    let mut valuesets = Vec::new();
    let mut codes = Vec::new();
    let mut concepts = Vec::new();

    for def in terminology_defs {
        match def {
            TerminologyDef::CodeSystem(cs) => codesystems.push(cs),
            TerminologyDef::ValueSet(vs) => valuesets.push(vs),
            TerminologyDef::Code(c) => codes.push(c),
            TerminologyDef::Concept(c) => concepts.push(c),
        }
    }

    // Parse parameter definitions
    let (input, parameters) = many0(parse_parameter_def)(input)?;

    // Parse context definitions
    let (input, contexts) = many0(parse_context_def)(input)?;

    // Parse statements (expression and function definitions)
    let (input, statements) = many0(parse_statement)(input)?;

    let (input, _) = skip_ws_and_comments(input)?;

    Ok((
        input,
        Library {
            identifier,
            usings,
            includes,
            codesystems,
            valuesets,
            codes,
            concepts,
            parameters,
            contexts,
            statements,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(s: &str) -> Span<'_> {
        Span::new(s)
    }

    // ========================================================================
    // Library Identifier Tests
    // ========================================================================

    #[test]
    fn test_library_identifier() {
        let (_, lib_id) =
            parse_library_identifier(span("library MyLibrary version '1.0.0'")).unwrap();
        assert_eq!(lib_id.name, "MyLibrary");
        assert_eq!(lib_id.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_library_identifier_no_version() {
        let (_, lib_id) = parse_library_identifier(span("library MyLibrary")).unwrap();
        assert_eq!(lib_id.name, "MyLibrary");
        assert_eq!(lib_id.version, None);
    }

    #[test]
    fn test_library_identifier_qualified() {
        let (_, lib_id) =
            parse_library_identifier(span("library CMS.Common version '1.0'")).unwrap();
        assert_eq!(lib_id.name, "CMS.Common");
    }

    // ========================================================================
    // Using Definition Tests
    // ========================================================================

    #[test]
    fn test_using_def() {
        let (_, using) = parse_using_def(span("using FHIR version '4.0.1'")).unwrap();
        assert_eq!(using.model_name, "FHIR");
        assert_eq!(using.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_using_def_no_version() {
        let (_, using) = parse_using_def(span("using QDM")).unwrap();
        assert_eq!(using.model_name, "QDM");
        assert_eq!(using.version, None);
    }

    // ========================================================================
    // Include Definition Tests
    // ========================================================================

    #[test]
    fn test_include_def() {
        let (_, incl) = parse_include_def(span(
            "include FHIRHelpers version '4.0.1' called FHIRHelpers",
        ))
        .unwrap();
        assert_eq!(incl.path, "FHIRHelpers");
        assert_eq!(incl.version, Some("4.0.1".to_string()));
        assert_eq!(incl.alias, Some("FHIRHelpers".to_string()));
    }

    #[test]
    fn test_include_def_no_alias() {
        let (_, incl) = parse_include_def(span("include Common.Helpers version '1.0'")).unwrap();
        assert_eq!(incl.path, "Common.Helpers");
        assert_eq!(incl.alias, None);
    }

    // ========================================================================
    // CodeSystem Definition Tests
    // ========================================================================

    #[test]
    fn test_codesystem_def() {
        let (_, cs) =
            parse_codesystem_def(span("codesystem LOINC: 'http://loinc.org' version '2.73'"))
                .unwrap();
        assert_eq!(cs.name, "LOINC");
        assert_eq!(cs.id, "http://loinc.org");
        assert_eq!(cs.version, Some("2.73".to_string()));
    }

    #[test]
    fn test_codesystem_def_private() {
        let (_, cs) =
            parse_codesystem_def(span("private codesystem Internal: 'urn:internal'")).unwrap();
        assert_eq!(cs.name, "Internal");
        assert_eq!(cs.access, AccessModifier::Private);
    }

    // ========================================================================
    // ValueSet Definition Tests
    // ========================================================================

    #[test]
    fn test_valueset_def() {
        let (_, vs) = parse_valueset_def(span(
            "valueset DiabetesCodes: 'http://example.org/vs/diabetes'",
        ))
        .unwrap();
        assert_eq!(vs.name, "DiabetesCodes");
        assert_eq!(vs.id, "http://example.org/vs/diabetes");
    }

    // ========================================================================
    // Code Definition Tests
    // ========================================================================

    #[test]
    fn test_code_def() {
        let (_, code) = parse_code_def(span(
            "code Glucose: '2345-7' from LOINC display 'Glucose [Mass/volume] in Serum'",
        ))
        .unwrap();
        assert_eq!(code.name, "Glucose");
        assert_eq!(code.code, "2345-7");
        assert_eq!(code.codesystem, "LOINC");
        assert_eq!(
            code.display,
            Some("Glucose [Mass/volume] in Serum".to_string())
        );
    }

    // ========================================================================
    // Parameter Definition Tests
    // ========================================================================

    #[test]
    fn test_parameter_def() {
        let (_, param) =
            parse_parameter_def(span("parameter MeasurementPeriod Interval<DateTime>")).unwrap();
        assert_eq!(param.name, "MeasurementPeriod");
        assert!(param.type_specifier.is_some());
    }

    #[test]
    fn test_parameter_def_with_default() {
        let (_, param) = parse_parameter_def(span("parameter Count Integer default 10")).unwrap();
        assert_eq!(param.name, "Count");
        assert!(param.default.is_some());
    }

    // ========================================================================
    // Context Definition Tests
    // ========================================================================

    #[test]
    fn test_context_def() {
        let (_, ctx) = parse_context_def(span("context Patient")).unwrap();
        assert_eq!(ctx.name, "Patient");
    }

    // ========================================================================
    // Expression Definition Tests
    // ========================================================================

    #[test]
    fn test_expression_def() {
        let (_, expr_def) = parse_expression_def(span("define InPatient: true")).unwrap();
        assert_eq!(expr_def.name, "InPatient");
        assert_eq!(expr_def.access, AccessModifier::Public);
    }

    #[test]
    fn test_expression_def_private() {
        let (_, expr_def) = parse_expression_def(span("private define Helper: 42")).unwrap();
        assert_eq!(expr_def.name, "Helper");
        assert_eq!(expr_def.access, AccessModifier::Private);
    }

    // ========================================================================
    // Function Definition Tests
    // ========================================================================

    #[test]
    fn test_function_def() {
        let (_, func) = parse_function_def(span(
            "define function Add(a Integer, b Integer) returns Integer: a + b",
        ))
        .unwrap();
        assert_eq!(func.name, "Add");
        assert_eq!(func.parameters.len(), 2);
        assert!(!func.external);
        assert!(func.body.is_some());
    }

    #[test]
    fn test_function_def_external() {
        let (_, func) = parse_function_def(span(
            "define function External(x Integer) returns String external",
        ))
        .unwrap();
        assert_eq!(func.name, "External");
        assert!(func.external);
        assert!(func.body.is_none());
    }

    #[test]
    fn test_function_def_fluent() {
        let (_, func) = parse_function_def(span(
            "fluent define function toAge(birthDate Date) returns Integer: 0",
        ))
        .unwrap();
        assert_eq!(func.name, "toAge");
        assert!(func.fluent);
    }

    // ========================================================================
    // Full Library Tests
    // ========================================================================

    #[test]
    fn test_parse_simple_library() {
        let source = r#"
            library Example version '1.0.0'
            using FHIR version '4.0.1'
            context Patient
            define InPatient: true
        "#;

        let (remaining, library) = parse_library(span(source)).unwrap();
        assert!(remaining.fragment().trim().is_empty());
        assert!(library.identifier.is_some());
        assert_eq!(library.identifier.as_ref().unwrap().name, "Example");
        assert_eq!(library.usings.len(), 1);
        assert_eq!(library.contexts.len(), 1);
        assert_eq!(library.statements.len(), 1);
    }

    #[test]
    fn test_parse_library_with_terminology() {
        let source = r#"
            library Terminology version '1.0'
            codesystem LOINC: 'http://loinc.org'
            valueset DiabetesCodes: 'http://example.org/vs/diabetes'
            code Glucose: '2345-7' from LOINC
        "#;

        let (_, library) = parse_library(span(source)).unwrap();
        assert_eq!(library.codesystems.len(), 1);
        assert_eq!(library.valuesets.len(), 1);
        assert_eq!(library.codes.len(), 1);
    }

    #[test]
    fn test_parse_library_with_multiple_definitions() {
        let source = r#"
            library Test version '1.0'
            
            define A: 1
            define B: 2
            define C: A + B
        "#;

        let (_, library) = parse_library(span(source)).unwrap();
        assert_eq!(library.statements.len(), 3);
    }
}
