//! FSH rule parsers — parse `* ...` lines inside entity blocks

use crate::parser::ast::*;
use crate::parser::lexer::*;
use crate::parser::span::Span;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list1,
    IResult,
};

// ============================================================================
// SD Rule dispatcher
// ============================================================================

/// Parse one `* <rule>` line as an SD rule
pub fn parse_sd_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<SdRule>> {
    let (input, _) = trivia(input)?;
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;

    // Try each rule type in order of specificity
    let (input, rule) = alt((
        map(parse_insert_rule_inner, SdRule::Insert),
        map(parse_caret_value_rule_inner, SdRule::CaretValue),
        map(parse_obeys_rule_inner, SdRule::Obeys),
        map(parse_add_element_rule_inner, SdRule::AddElement),
        map(parse_contains_rule_inner, SdRule::Contains),
        map(parse_only_rule_inner, SdRule::Only),
        map(parse_binding_rule_inner, SdRule::Binding),
        map(parse_assignment_rule_inner, SdRule::Assignment),
        map(parse_card_rule_inner, SdRule::Card),
        map(parse_flag_rule_inner, SdRule::Flag),
        map(parse_path_rule_inner, SdRule::Path),
    ))(input)?;

    // consume rest of line
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    Ok((input, Spanned::new(rule, loc)))
}

/// Parse one `* <rule>` line as an Instance rule
pub fn parse_instance_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<InstanceRule>> {
    let (input, _) = trivia(input)?;
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;

    let (input, rule) = alt((
        map(parse_insert_rule_inner, InstanceRule::Insert),
        map(parse_assignment_rule_inner, InstanceRule::Assignment),
    ))(input)?;

    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    Ok((input, Spanned::new(rule, loc)))
}

/// Parse a VS component rule
pub fn parse_vs_component_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<VsComponentRule>> {
    let (input, _) = trivia(input)?;
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;

    // Check for exclude
    let (input, inclusion) = if input.fragment().starts_with("exclude") {
        let (inp, _) = tag("exclude")(input)?;
        let (inp, _) = ws(inp)?;
        (inp, false)
    } else if input.fragment().starts_with("include") {
        let (inp, _) = tag("include")(input)?;
        let (inp, _) = ws(inp)?;
        (inp, true)
    } else {
        (input, true)
    };

    // Parse `codes from system X` or `codes from valueset Y` or `system#code` list
    let rule = parse_vs_component_body(input, inclusion)?;

    let (input, _) = take_while(|c| c != '\n')(rule.0)?;
    let (input, _) = opt(char('\n'))(input)?;

    Ok((input, Spanned::new(rule.1, loc)))
}

fn parse_vs_component_body(input: Span<'_>, inclusion: bool) -> IResult<Span<'_>, VsComponentRule> {
    // Try "codes from system" syntax
    if input.fragment().starts_with("codes") {
        let (input, _) = tag("codes")(input)?;
        let (input, _) = ws(input)?;
        let (input, _) = tag("from")(input)?;
        let (input, _) = ws(input)?;
        return parse_vs_from_clause(input, inclusion, Vec::new());
    }

    // Otherwise parse a list of coded concepts
    let (input, concepts) = parse_vs_concept_list(input)?;
    let (input, _) = ws(input)?;

    // Optional from clause
    if input.fragment().starts_with("from") {
        let (input, _) = tag("from")(input)?;
        let (input, _) = ws(input)?;
        return parse_vs_from_clause(input, inclusion, concepts);
    }

    // Determine system from first concept if it has one
    let system = concepts.first().and_then(|c| c.system.clone());
    Ok((
        input,
        VsComponentRule {
            inclusion,
            system,
            from_vs: Vec::new(),
            concepts,
            filters: Vec::new(),
        },
    ))
}

fn parse_vs_concept_list(input: Span<'_>) -> IResult<Span<'_>, Vec<VsConceptRef>> {
    let mut concepts = Vec::new();
    let mut remaining = input;
    loop {
        let (inp, _) = ws(remaining)?;
        if inp.fragment().starts_with("from")
            || inp.fragment().starts_with('\n')
            || inp.fragment().is_empty()
        {
            remaining = inp;
            break;
        }
        match coded_value(inp) {
            Ok((
                inp2,
                FshValue::Code {
                    system,
                    code,
                    display,
                },
            )) => {
                concepts.push(VsConceptRef {
                    code,
                    system,
                    display,
                });
                remaining = inp2;
                // skip comma separator if present
                let (inp2, _) = ws(remaining)?;
                if inp2.fragment().starts_with(',') {
                    let (inp2, _) = char(',')(inp2)?;
                    remaining = inp2;
                } else {
                    remaining = inp2;
                    break;
                }
            }
            _ => break,
        }
    }
    Ok((remaining, concepts))
}

fn parse_vs_from_clause(
    input: Span<'_>,
    inclusion: bool,
    concepts: Vec<VsConceptRef>,
) -> IResult<Span<'_>, VsComponentRule> {
    let mut system: Option<String> = None;
    let mut from_vs: Vec<String> = Vec::new();
    let mut filters: Vec<VsFilter> = Vec::new();
    let mut remaining = input;

    // could be "system X" or "valueset Y" or "system X and valueset Y" etc.
    loop {
        let (inp, _) = ws(remaining)?;
        if inp.fragment().starts_with("system") {
            let (inp, _) = tag("system")(inp)?;
            let (inp, _) = ws(inp)?;
            let (inp, sys) = take_while1(|c: char| !c.is_whitespace())(inp)?;
            let sys = sys.fragment().to_string();
            system = Some(sys);
            remaining = inp;
        } else if inp.fragment().starts_with("valueset") {
            let (inp, _) = tag("valueset")(inp)?;
            let (inp, _) = ws(inp)?;
            let (inp, vs) = alt((quoted_string, identifier))(inp)?;
            from_vs.push(vs);
            remaining = inp;
        } else if inp.fragment().starts_with("and") {
            let (inp, _) = tag("and")(inp)?;
            remaining = inp;
        } else if inp.fragment().starts_with("where") {
            let (inp, _) = tag("where")(inp)?;
            let (inp, _) = ws(inp)?;
            let (inp, f) = parse_vs_filter(inp)?;
            filters.push(f);
            remaining = inp;
        } else {
            break;
        }
    }

    Ok((
        remaining,
        VsComponentRule {
            inclusion,
            system,
            from_vs,
            concepts,
            filters,
        },
    ))
}

fn parse_vs_filter(input: Span<'_>) -> IResult<Span<'_>, VsFilter> {
    let (input, property) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, op) = alt((
        map(tag("regex"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("="), |s: Span<'_>| s.fragment().to_string()),
        map(tag("!="), |s: Span<'_>| s.fragment().to_string()),
        map(tag("is-a"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("descendent-of"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("is-not-a"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("in"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("not-in"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("generalizes"), |s: Span<'_>| s.fragment().to_string()),
        map(tag("exists"), |s: Span<'_>| s.fragment().to_string()),
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = alt((
        quoted_string,
        // Handle #code filter values (e.g., `concept is-a #64940007`)
        parse_hash_code_value,
        identifier,
    ))(input)?;
    Ok((
        input,
        VsFilter {
            property,
            op,
            value,
        },
    ))
}

/// Parse a `#code` value used in VS filters (e.g., `concept is-a #64940007`)
fn parse_hash_code_value(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('#')(input)?;
    let (input, code) = take_while1(|c: char| !c.is_whitespace())(input)?;
    Ok((input, code.fragment().to_string()))
}

/// Parse a concept rule: `* #code "display" "definition"`
pub fn parse_concept_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<ConceptRule>> {
    let (input, _) = trivia(input)?;
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;

    // Parse hierarchy parents (prefixed with '#' indented)
    // For simplicity: just parse the main concept
    let (input, _) = char('#')(input)?;
    let (input, code) = take_while1(|c: char| !c.is_whitespace() && c != '"')(input)?;
    // Allow trivia (including newlines) before display — FSH supports multi-line concepts:
    //   * #code
    //       "display"
    //       "definition"
    let (input, _) = trivia(input)?;
    let (input, display) = opt(quoted_string)(input)?;
    let (input, _) = trivia(input)?;
    let (input, definition) = opt(quoted_string)(input)?;

    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    Ok((
        input,
        Spanned::new(
            ConceptRule {
                code: code.fragment().to_string(),
                display,
                definition,
                hierarchy: Vec::new(),
            },
            loc,
        ),
    ))
}

/// Parse a mapping rule: `* path -> "map" "comment"? language?`
pub fn parse_mapping_rule(input: Span<'_>) -> IResult<Span<'_>, Spanned<MappingRule>> {
    let (input, _) = trivia(input)?;
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;

    // Optional path before '->'
    let (input, path) = opt(parse_path_before_arrow)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = ws(input)?;
    let (input, map_str) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, comment) = opt(quoted_string)(input)?;
    let (input, _) = ws(input)?;
    let (input, language) = opt(map(
        take_while1(|c: char| !c.is_whitespace() && c != '\n'),
        |s: Span<'_>| s.fragment().to_string(),
    ))(input)?;

    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    Ok((
        input,
        Spanned::new(
            MappingRule {
                path,
                map: map_str,
                comment,
                language,
            },
            loc,
        ),
    ))
}

fn parse_path_before_arrow(input: Span<'_>) -> IResult<Span<'_>, FshPath> {
    // Parse a path that doesn't contain '->'
    let (input, path) = fsh_path(input)?;
    // Verify next non-ws is '->'
    let (check, _) = ws(input)?;
    if check.fragment().starts_with("->") {
        Ok((input, path))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

// ============================================================================
// Individual rule parsers (inner = after the '*' has been consumed)
// ============================================================================

fn parse_card_rule_inner(input: Span<'_>) -> IResult<Span<'_>, CardRule> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, (min, max)) = cardinal(input)?;
    let (input, _) = ws(input)?;
    let (input, flags) = flags_list(input)?;
    Ok((
        input,
        CardRule {
            path,
            min,
            max,
            flags,
        },
    ))
}

fn parse_flag_rule_inner(input: Span<'_>) -> IResult<Span<'_>, FlagRule> {
    // Parse one or more paths separated by `and` keyword, then flags
    let (input, first_path) = fsh_path(input)?;
    let mut paths = vec![first_path];

    let mut remaining = input;
    loop {
        let (after_ws, _) = ws(remaining)?;
        if after_ws.fragment().starts_with("and ") || after_ws.fragment().starts_with("and\t") {
            let (after_and, _) = tag("and")(after_ws)?;
            let (after_ws2, _) = ws(after_and)?;
            match fsh_path(after_ws2) {
                Ok((after_path, path)) => {
                    paths.push(path);
                    remaining = after_path;
                }
                Err(_) => break,
            }
        } else {
            remaining = after_ws;
            break;
        }
    }

    let (input, _) = ws(remaining)?;
    let (input, flags) = flags_list(input)?;
    if flags.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many1,
        )));
    }
    Ok((input, FlagRule { paths, flags }))
}

fn parse_binding_rule_inner(input: Span<'_>) -> IResult<Span<'_>, BindingRule> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = ws(input)?;
    let (input, vs) = alt((quoted_string, identifier))(input)?;
    let (input, _) = ws(input)?;
    // Optional strength in parentheses: (required), (extensible), etc.
    let (input, strength) = opt(parse_binding_strength)(input)?;
    Ok((
        input,
        BindingRule {
            path,
            value_set: vs,
            strength,
        },
    ))
}

fn parse_binding_strength(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = char('(')(input)?;
    let (input, s) = take_while(|c| c != ')')(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, s.fragment().to_string()))
}

fn parse_assignment_rule_inner(input: Span<'_>) -> IResult<Span<'_>, AssignmentRule> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = fsh_value(input)?;
    // Check for optional (exactly) suffix
    let (input, _) = ws(input)?;
    let (input, exactly) = if input.fragment().starts_with("(exactly)") {
        let (inp, _) = tag("(exactly)")(input)?;
        (inp, true)
    } else {
        (input, false)
    };
    Ok((
        input,
        AssignmentRule {
            path,
            value,
            exactly,
        },
    ))
}

fn parse_contains_rule_inner(input: Span<'_>) -> IResult<Span<'_>, ContainsRule> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("contains")(input)?;
    let (input, _) = trivia(input)?;
    let (input, items) = separated_list1(
        |inp| {
            let (inp, _) = trivia(inp)?;
            let (inp, _) = tag("and")(inp)?;
            let (inp, _) = trivia(inp)?;
            Ok((inp, ()))
        },
        parse_contains_item,
    )(input)?;
    Ok((input, ContainsRule { path, items }))
}

fn parse_contains_item(input: Span<'_>) -> IResult<Span<'_>, ContainsItem> {
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;

    // Optional "named X"
    let (input, alias) = if input.fragment().starts_with("named") {
        let (inp, _) = tag("named")(input)?;
        let (inp, _) = ws(inp)?;
        let (inp, alias) = identifier(inp)?;
        (inp, Some(alias))
    } else {
        (input, None)
    };

    let (input, _) = ws(input)?;
    let (input, cardinality) = opt(cardinal)(input)?;
    let (min, max) = cardinality.unwrap_or((None, None));

    // Consume optional flag keywords (MS, SU, ?!, N, TU, D) that may follow the cardinality
    let (input, _) = ws(input)?;
    let (input, _) = opt(parse_flags_inline)(input)?;

    Ok((
        input,
        ContainsItem {
            name,
            alias,
            min,
            max,
        },
    ))
}

/// Consume any flag keywords appearing inline (MS, SU, ?!, N, TU, D) without capturing them.
/// Used inside contains items to absorb flags that we don't store on the slice itself.
fn parse_flags_inline(input: Span<'_>) -> IResult<Span<'_>, ()> {
    let flag_kws = ["MS", "SU", "?!", "N", "TU", "D"];
    let mut remaining = input;
    let mut consumed = false;
    loop {
        let mut matched = false;
        for kw in &flag_kws {
            if remaining.fragment().starts_with(kw) {
                let rest = &remaining.fragment()[kw.len()..];
                if rest.is_empty()
                    || rest.starts_with(' ')
                    || rest.starts_with('\t')
                    || rest.starts_with('\n')
                    || rest.starts_with('\r')
                {
                    let (inp, _) = tag(*kw)(remaining)?;
                    let (inp, _) = ws(inp)?;
                    remaining = inp;
                    matched = true;
                    consumed = true;
                    break;
                }
            }
        }
        if !matched {
            break;
        }
    }
    if consumed {
        Ok((remaining, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn parse_only_rule_inner(input: Span<'_>) -> IResult<Span<'_>, OnlyRule> {
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("only")(input)?;
    let (input, _) = ws(input)?;
    let (input, types) = separated_list1(
        |inp| {
            let (inp, _) = ws(inp)?;
            let (inp, _) = tag("or")(inp)?;
            let (inp, _) = ws(inp)?;
            Ok((inp, ()))
        },
        alt((parse_reference_type, map(identifier, |s| s))),
    )(input)?;
    Ok((input, OnlyRule { path, types }))
}

fn parse_reference_type(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = tag("Reference(")(input)?;
    let (input, inner) = take_while(|c| c != ')')(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, format!("Reference({})", inner.fragment())))
}

fn parse_obeys_rule_inner(input: Span<'_>) -> IResult<Span<'_>, ObeysRule> {
    // optional path, then 'obeys', then invariant list
    let (input, path) = opt(parse_path_before_obeys)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("obeys")(input)?;
    let (input, _) = ws(input)?;
    let (input, invariants) = separated_list1(
        |inp| {
            let (inp, _) = ws(inp)?;
            let (inp, _) = tag("and")(inp)?;
            let (inp, _) = ws(inp)?;
            Ok((inp, ()))
        },
        identifier,
    )(input)?;
    Ok((input, ObeysRule { path, invariants }))
}

fn parse_path_before_obeys(input: Span<'_>) -> IResult<Span<'_>, FshPath> {
    let (input, path) = fsh_path(input)?;
    let (check, _) = ws(input)?;
    if check.fragment().starts_with("obeys") {
        Ok((input, path))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn parse_caret_value_rule_inner(input: Span<'_>) -> IResult<Span<'_>, CaretValueRule> {
    // Optional path before the caret
    let (input, path) = opt(parse_path_before_caret)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('^')(input)?;
    let (input, caret_path_raw) = take_while1(|c: char| !c.is_whitespace() && c != '=')(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = fsh_value(input)?;
    Ok((
        input,
        CaretValueRule {
            path,
            caret_path: caret_path_raw.fragment().to_string(),
            value,
        },
    ))
}

fn parse_path_before_caret(input: Span<'_>) -> IResult<Span<'_>, FshPath> {
    let (input, path) = fsh_path(input)?;
    let (check, _) = ws(input)?;
    if check.fragment().starts_with('^') {
        Ok((input, path))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )))
    }
}

fn parse_insert_rule_inner(input: Span<'_>) -> IResult<Span<'_>, InsertRule> {
    let (input, _) = tag("insert")(input)?;
    let (input, _) = ws(input)?;
    let (input, rule_set) = identifier(input)?;
    let (input, _) = ws(input)?;
    // Optional params in parentheses
    let (input, params) = if input.fragment().starts_with('(') {
        let (inp, _) = char('(')(input)?;
        let (inp, p) = separated_list1(
            |i| {
                let (i, _) = ws(i)?;
                let (i, _) = char(',')(i)?;
                let (i, _) = ws(i)?;
                Ok((i, ()))
            },
            map(take_while(|c| c != ',' && c != ')'), |s: Span<'_>| {
                s.fragment().trim().to_string()
            }),
        )(inp)?;
        let (inp, _) = char(')')(inp)?;
        (inp, p)
    } else {
        (input, Vec::new())
    };
    Ok((input, InsertRule { rule_set, params }))
}

fn parse_add_element_rule_inner(input: Span<'_>) -> IResult<Span<'_>, AddElementRule> {
    let (input, _) = tag("add")(input)?;
    let (input, _) = ws(input)?;
    let (input, path) = fsh_path(input)?;
    let (input, _) = ws(input)?;
    let (input, (min_opt, max_opt)) = cardinal(input)?;
    let min = min_opt.unwrap_or(0);
    let max = max_opt.unwrap_or_else(|| "*".to_string());
    let (input, _) = ws(input)?;
    let (input, types) = separated_list1(
        |inp| {
            let (inp, _) = ws(inp)?;
            let (inp, _) = tag("or")(inp)?;
            let (inp, _) = ws(inp)?;
            Ok((inp, ()))
        },
        identifier,
    )(input)?;
    let (input, _) = ws(input)?;
    let (input, short) = alt((quoted_string, rest_of_line))(input)?;
    Ok((
        input,
        AddElementRule {
            path,
            min,
            max,
            types,
            short,
        },
    ))
}

fn parse_path_rule_inner(input: Span<'_>) -> IResult<Span<'_>, PathRule> {
    let (input, path) = fsh_path(input)?;
    Ok((input, PathRule { path }))
}

/// Parse a standalone caret value rule line (starting with `*`) for use in VS/CS contexts.
/// Returns a Spanned<CaretValueRule> including source location.
pub fn parse_caret_value_rule_pub(input: Span<'_>) -> IResult<Span<'_>, Spanned<CaretValueRule>> {
    let loc = input.location();
    let (input, _) = char('*')(input)?;
    let (input, _) = ws(input)?;
    let (input, rule) = parse_caret_value_rule_inner(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    Ok((input, Spanned::new(rule, loc)))
}
