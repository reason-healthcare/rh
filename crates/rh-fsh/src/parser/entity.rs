//! FSH entity parsers — parse top-level FSH entities (Profile, Extension, Instance, etc.)

use crate::error::FshError;
use crate::parser::ast::*;
use crate::parser::lexer::*;
use crate::parser::rules::*;
use crate::parser::span::{SourceLocation, Span};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::char,
    combinator::{map, opt},
    IResult,
};

type VsComponentRulesResult<'a> =
    IResult<Span<'a>, (Vec<Spanned<VsComponentRule>>, Vec<Spanned<CaretValueRule>>)>;
type ConceptRulesResult<'a> =
    IResult<Span<'a>, (Vec<Spanned<ConceptRule>>, Vec<Spanned<CaretValueRule>>)>;

// ============================================================================
// Public entry point
// ============================================================================

/// Parse an entire FSH document
pub fn parse_document(input: Span<'_>, source_name: String) -> Result<FshDocument, FshError> {
    let mut entities = Vec::new();
    let mut remaining = input;

    while let Ok((inp, _)) = trivia(remaining) {
        remaining = inp;

        if remaining.fragment().is_empty() {
            break;
        }

        match parse_entity(remaining) {
            Ok((inp, entity)) => {
                entities.push(entity);
                remaining = inp;
            }
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                return Err(FshError::Parse {
                    message: format!("Unexpected input: {:?}", e.code),
                    location: remaining.location(),
                });
            }
            Err(nom::Err::Incomplete(_)) => break,
        }
    }

    Ok(FshDocument {
        entities,
        source_name,
    })
}

// ============================================================================
// Entity dispatcher
// ============================================================================

fn parse_entity(input: Span<'_>) -> IResult<Span<'_>, Spanned<FshEntity>> {
    alt((
        map(parse_alias, |a| Spanned::new(FshEntity::Alias(a.0), a.1)),
        map(parse_profile, |p| {
            Spanned::new(FshEntity::Profile(p.0), p.1)
        }),
        map(parse_extension, |e| {
            Spanned::new(FshEntity::Extension(e.0), e.1)
        }),
        map(parse_logical, |l| {
            Spanned::new(FshEntity::Logical(l.0), l.1)
        }),
        map(parse_resource_def, |r| {
            Spanned::new(FshEntity::Resource(r.0), r.1)
        }),
        map(parse_instance, |i| {
            Spanned::new(FshEntity::Instance(i.0), i.1)
        }),
        map(parse_value_set, |vs| {
            Spanned::new(FshEntity::ValueSet(vs.0), vs.1)
        }),
        map(parse_code_system, |cs| {
            Spanned::new(FshEntity::CodeSystem(cs.0), cs.1)
        }),
        map(parse_invariant, |inv| {
            Spanned::new(FshEntity::Invariant(inv.0), inv.1)
        }),
        map(parse_param_rule_set, |prs| {
            Spanned::new(FshEntity::ParamRuleSet(prs.0), prs.1)
        }),
        map(parse_rule_set, |rs| {
            Spanned::new(FshEntity::RuleSet(rs.0), rs.1)
        }),
        map(parse_mapping, |m| {
            Spanned::new(FshEntity::Mapping(m.0), m.1)
        }),
    ))(input)
}

// ============================================================================
// Alias
// ============================================================================

fn parse_alias(input: Span<'_>) -> IResult<Span<'_>, (Alias, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_ALIAS)(input)?;
    let (input, _) = take_while1(|c| c == ':' || c == ' ' || c == '\t')(input)?;
    let (input, name) = alias_name(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = alt((
        quoted_string,
        map(take_while1(|c: char| !c.is_whitespace()), |s: Span<'_>| {
            s.fragment().to_string()
        }),
    ))(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    Ok((input, (Alias { name, value }, loc)))
}

// ============================================================================
// Profile
// ============================================================================

fn parse_profile(input: Span<'_>) -> IResult<Span<'_>, (Profile, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_PROFILE)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, metadata) = parse_sd_metadata(input, name)?;
    let (input, rules) = parse_sd_rules(input)?;

    Ok((input, (Profile { metadata, rules }, loc)))
}

// ============================================================================
// Extension
// ============================================================================

fn parse_extension(input: Span<'_>) -> IResult<Span<'_>, (Extension, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_EXTENSION)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, mut metadata) = parse_sd_metadata(input, name)?;
    // FSH spec: extensions without an explicit Parent default to Extension
    if metadata.parent.is_none() {
        metadata.parent = Some("Extension".to_string());
    }
    let (input, rules) = parse_sd_rules(input)?;

    Ok((input, (Extension { metadata, rules }, loc)))
}

// ============================================================================
// Logical
// ============================================================================

fn parse_logical(input: Span<'_>) -> IResult<Span<'_>, (Logical, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_LOGICAL)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, metadata) = parse_sd_metadata(input, name)?;
    let (input, rules) = parse_sd_rules(input)?;

    Ok((input, (Logical { metadata, rules }, loc)))
}

// ============================================================================
// Resource
// ============================================================================

fn parse_resource_def(input: Span<'_>) -> IResult<Span<'_>, (ResourceDef, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_RESOURCE)(input)?;
    // Make sure it's "Resource:" not "RuleSet" etc.
    if !input.fragment().starts_with(':')
        && !input.fragment().starts_with(' ')
        && !input.fragment().starts_with('\t')
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, metadata) = parse_sd_metadata(input, name)?;
    let (input, rules) = parse_sd_rules(input)?;

    Ok((input, (ResourceDef { metadata, rules }, loc)))
}

// ============================================================================
// Instance
// ============================================================================

fn parse_instance(input: Span<'_>) -> IResult<Span<'_>, (Instance, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_INSTANCE)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, instance_of) = parse_instance_of(input)?;

    let mut remaining = input;
    let mut usage: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;

    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty()
            || starts_with_entity_kw(peek.fragment())
            || peek.fragment().starts_with('*')
        {
            remaining = peek;
            break;
        }
        let mut matched = false;
        if usage.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Usage", peek) {
                if val.is_some() {
                    usage = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && title.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Title", peek) {
                if val.is_some() {
                    title = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && description.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Description", peek) {
                if val.is_some() {
                    description = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched {
            break;
        }
    }

    let metadata = InstanceMetadata {
        name,
        instance_of,
        usage,
        title,
        description,
    };

    let (remaining, rules) = parse_instance_rules(remaining)?;

    Ok((remaining, (Instance { metadata, rules }, loc)))
}

fn parse_instance_of(input: Span<'_>) -> IResult<Span<'_>, String> {
    let (input, _) = trivia(input)?;
    let (input, _) = tag("InstanceOf")(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, val) = alt((
        quoted_string,
        map(
            take_while1(|c: char| !c.is_whitespace() && c != '\n'),
            |s: Span<'_>| s.fragment().to_string(),
        ),
    ))(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    Ok((input, val))
}

// ============================================================================
// ValueSet
// ============================================================================

fn parse_value_set(input: Span<'_>) -> IResult<Span<'_>, (ValueSet, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_VALUE_SET)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let mut remaining = input;
    let mut id: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;

    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty()
            || starts_with_entity_kw(peek.fragment())
            || peek.fragment().starts_with('*')
        {
            remaining = peek;
            break;
        }
        let mut matched = false;
        if id.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Id", peek) {
                if val.is_some() {
                    id = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && title.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Title", peek) {
                if val.is_some() {
                    title = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && description.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Description", peek) {
                if val.is_some() {
                    description = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched {
            break;
        }
    }

    let metadata = VsMetadata {
        name,
        id,
        title,
        description,
    };
    let (remaining, (components, caret_rules)) = parse_vs_component_rules(remaining)?;

    Ok((
        remaining,
        (
            ValueSet {
                metadata,
                components,
                caret_rules,
            },
            loc,
        ),
    ))
}

// ============================================================================
// CodeSystem
// ============================================================================

fn parse_code_system(input: Span<'_>) -> IResult<Span<'_>, (CodeSystem, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_CODE_SYSTEM)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let mut remaining = input;
    let mut id: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;

    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty()
            || starts_with_entity_kw(peek.fragment())
            || peek.fragment().starts_with('*')
        {
            remaining = peek;
            break;
        }
        let mut matched = false;
        if id.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Id", peek) {
                if val.is_some() {
                    id = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && title.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Title", peek) {
                if val.is_some() {
                    title = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && description.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Description", peek) {
                if val.is_some() {
                    description = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched {
            break;
        }
    }

    let metadata = CsMetadata {
        name,
        id,
        title,
        description,
    };
    let (remaining, (concepts, caret_rules)) = parse_concept_rules(remaining)?;

    Ok((
        remaining,
        (
            CodeSystem {
                metadata,
                concepts,
                caret_rules,
            },
            loc,
        ),
    ))
}

// ============================================================================
// Invariant
// ============================================================================

fn parse_invariant(input: Span<'_>) -> IResult<Span<'_>, (Invariant, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_INVARIANT)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, description) = opt_meta_kv("Description", input)?;
    let (input, expression) = opt_meta_kv("Expression", input)?;
    let (input, severity) = opt_meta_kv("Severity", input)?;
    let (input, xpath) = opt_meta_kv("XPath", input)?;

    Ok((
        input,
        (
            Invariant {
                name,
                description,
                expression,
                severity,
                xpath,
            },
            loc,
        ),
    ))
}

// ============================================================================
// RuleSet
// ============================================================================

fn parse_rule_set(input: Span<'_>) -> IResult<Span<'_>, (RuleSet, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_RULE_SET)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    let (input, rules) = parse_sd_rules(input)?;
    Ok((input, (RuleSet { name, rules }, loc)))
}

// ============================================================================
// ParamRuleSet
// ============================================================================

fn parse_param_rule_set(input: Span<'_>) -> IResult<Span<'_>, (ParamRuleSet, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_RULE_SET)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    // parameters in parentheses
    let (input, _) = char('(')(input)?;
    let (input, params_raw) = take_while(|c| c != ')')(input)?;
    let (input, _) = char(')')(input)?;
    let params: Vec<String> = params_raw
        .fragment()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    // Capture raw rule lines for parameter substitution later
    let raw_rules = collect_raw_rule_lines(input.fragment());

    let (input, rules) = parse_sd_rules(input)?;
    Ok((
        input,
        (
            ParamRuleSet {
                name,
                params,
                rules,
                raw_rules,
            },
            loc,
        ),
    ))
}

/// Collect raw text of `* ...` rule lines, stopping at entity keywords or blank entity boundary.
/// Each returned string includes the leading `* ` and rest of the rule line.
fn collect_raw_rule_lines(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        // Stop at entity keyword boundaries
        if starts_with_entity_kw(trimmed) {
            break;
        }
        if trimmed.starts_with('*') {
            result.push(trimmed.to_string());
        }
    }
    result
}

// ============================================================================
// Mapping
// ============================================================================

fn parse_mapping(input: Span<'_>) -> IResult<Span<'_>, (Mapping, SourceLocation)> {
    let loc = input.location();
    let (input, _) = tag(KW_MAPPING)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n')(input)?;
    let (input, _) = opt(char('\n'))(input)?;

    let (input, source) = opt_meta_kv("Source", input)?;
    let (input, target) = opt_meta_kv("Target", input)?;
    let (input, id) = opt_meta_kv("Id", input)?;
    let (input, title) = opt_meta_kv("Title", input)?;
    let (input, description) = opt_meta_kv("Description", input)?;

    let metadata = MappingMetadata {
        name,
        source,
        target,
        id,
        title,
        description,
    };

    let (input, rules) = parse_mapping_rules(input)?;

    Ok((input, (Mapping { metadata, rules }, loc)))
}

// ============================================================================
// Shared metadata helpers
// ============================================================================

fn parse_sd_metadata(input: Span<'_>, name: String) -> IResult<Span<'_>, SdMetadata> {
    let mut remaining = input;
    let mut parent: Option<String> = None;
    let mut id: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut characteristics: Vec<String> = Vec::new();

    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty()
            || starts_with_entity_kw(peek.fragment())
            || peek.fragment().starts_with('*')
        {
            remaining = peek;
            break;
        }
        let mut matched = false;
        if parent.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Parent", peek) {
                if val.is_some() {
                    parent = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && id.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Id", peek) {
                if val.is_some() {
                    id = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && title.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Title", peek) {
                if val.is_some() {
                    title = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && description.is_none() {
            if let Ok((inp, val)) = opt_meta_kv("Description", peek) {
                if val.is_some() {
                    description = val;
                    remaining = inp;
                    matched = true;
                }
            }
        }
        if !matched && characteristics.is_empty() {
            if let Ok((inp, Some(v))) = opt_meta_kv("Characteristics", peek) {
                characteristics = v.split(',').map(|s| s.trim().to_string()).collect();
                remaining = inp;
                matched = true;
            }
        }
        if !matched {
            break;
        }
    }

    Ok((
        remaining,
        SdMetadata {
            name,
            parent,
            id,
            title,
            description,
            characteristics,
        },
    ))
}

/// Try to parse an optional metadata key-value pair.
/// Returns (remaining, Option<value>). Never fails — just returns None.
fn opt_meta_kv<'a>(key: &'static str, input: Span<'a>) -> IResult<Span<'a>, Option<String>> {
    // Peek ahead to see if this line starts with the key
    let mut peek = input;
    // skip whitespace/newlines
    if let Ok((p, _)) = trivia(peek) {
        peek = p;
    }

    if peek.fragment().starts_with(key) {
        // Check that it's followed by whitespace or ':'
        let rest = &peek.fragment()[key.len()..];
        if rest.starts_with(':') || rest.starts_with(' ') || rest.starts_with('\t') {
            // Not a keyword that would start the next entity
            match meta_key_value(key, peek) {
                Ok((inp, val)) => return Ok((inp, Some(val))),
                Err(_) => return Ok((input, None)),
            }
        }
    }
    Ok((input, None))
}

// ============================================================================
// Rule collection helpers
// ============================================================================

fn parse_sd_rules(input: Span<'_>) -> IResult<Span<'_>, Vec<Spanned<SdRule>>> {
    let mut rules = Vec::new();
    let mut remaining = input;
    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty() {
            remaining = peek;
            break;
        }
        if starts_with_entity_kw(peek.fragment()) {
            remaining = peek;
            break;
        }
        if !peek.fragment().starts_with('*') {
            // Skip unrecognized lines
            let (inp, _) = take_while(|c| c != '\n')(peek)?;
            let (inp, _) = opt(char('\n'))(inp)?;
            remaining = inp;
            continue;
        }
        match parse_sd_rule(peek) {
            Ok((inp, rule)) => {
                rules.push(rule);
                remaining = inp;
            }
            Err(_) => {
                // skip the line
                let (inp, _) = take_while(|c| c != '\n')(peek)?;
                let (inp, _) = opt(char('\n'))(inp)?;
                remaining = inp;
            }
        }
    }
    Ok((remaining, rules))
}

fn parse_instance_rules(input: Span<'_>) -> IResult<Span<'_>, Vec<Spanned<InstanceRule>>> {
    let mut rules = Vec::new();
    let mut remaining = input;
    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty() {
            remaining = peek;
            break;
        }
        if starts_with_entity_kw(peek.fragment()) {
            remaining = peek;
            break;
        }
        if !peek.fragment().starts_with('*') {
            let (inp, _) = take_while(|c| c != '\n')(peek)?;
            let (inp, _) = opt(char('\n'))(inp)?;
            remaining = inp;
            continue;
        }
        match parse_instance_rule(peek) {
            Ok((inp, rule)) => {
                rules.push(rule);
                remaining = inp;
            }
            Err(_) => {
                let (inp, _) = take_while(|c| c != '\n')(peek)?;
                let (inp, _) = opt(char('\n'))(inp)?;
                remaining = inp;
            }
        }
    }
    Ok((remaining, rules))
}

fn parse_vs_component_rules(input: Span<'_>) -> VsComponentRulesResult<'_> {
    let mut components = Vec::new();
    let mut caret_rules: Vec<Spanned<CaretValueRule>> = Vec::new();
    let mut remaining = input;
    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty() {
            remaining = peek;
            break;
        }
        if starts_with_entity_kw(peek.fragment()) {
            remaining = peek;
            break;
        }
        if !peek.fragment().starts_with('*') {
            let (inp, _) = take_while(|c| c != '\n')(peek)?;
            let (inp, _) = opt(char('\n'))(inp)?;
            remaining = inp;
            continue;
        }
        // Check if this is a caret rule (* ^key = value) — parse and capture, don't treat as component
        let after_star = peek.fragment().trim_start_matches('*').trim_start();
        if after_star.starts_with('^') {
            use crate::parser::rules::parse_caret_value_rule_pub;
            if let Ok((inp, rule)) = parse_caret_value_rule_pub(peek) {
                caret_rules.push(rule);
                remaining = inp;
                continue;
            }
        }
        match parse_vs_component_rule(peek) {
            Ok((inp, rule)) => {
                let has_content = !rule.value.concepts.is_empty()
                    || !rule.value.from_vs.is_empty()
                    || !rule.value.filters.is_empty()
                    || rule.value.system.is_some();
                if has_content {
                    components.push(rule);
                }
                remaining = inp;
            }
            Err(_) => {
                let (inp, _) = take_while(|c| c != '\n')(peek)?;
                let (inp, _) = opt(char('\n'))(inp)?;
                remaining = inp;
            }
        }
    }
    Ok((remaining, (components, caret_rules)))
}

fn parse_concept_rules(input: Span<'_>) -> ConceptRulesResult<'_> {
    let mut concepts = Vec::new();
    let mut caret_rules: Vec<Spanned<CaretValueRule>> = Vec::new();
    let mut remaining = input;
    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty() {
            remaining = peek;
            break;
        }
        if starts_with_entity_kw(peek.fragment()) {
            remaining = peek;
            break;
        }
        if !peek.fragment().starts_with('*') {
            let (inp, _) = take_while(|c| c != '\n')(peek)?;
            let (inp, _) = opt(char('\n'))(inp)?;
            remaining = inp;
            continue;
        }
        // Check if this is a caret rule (* ^key = value)
        let after_star = peek.fragment().trim_start_matches('*').trim_start();
        if after_star.starts_with('^') {
            use crate::parser::rules::parse_caret_value_rule_pub;
            if let Ok((inp, rule)) = parse_caret_value_rule_pub(peek) {
                caret_rules.push(rule);
                remaining = inp;
                continue;
            }
        }
        match parse_concept_rule(peek) {
            Ok((inp, rule)) => {
                concepts.push(rule);
                remaining = inp;
            }
            Err(_) => {
                let (inp, _) = take_while(|c| c != '\n')(peek)?;
                let (inp, _) = opt(char('\n'))(inp)?;
                remaining = inp;
            }
        }
    }
    Ok((remaining, (concepts, caret_rules)))
}

fn parse_mapping_rules(input: Span<'_>) -> IResult<Span<'_>, Vec<Spanned<MappingRule>>> {
    let mut rules = Vec::new();
    let mut remaining = input;
    loop {
        let (peek, _) = trivia(remaining).unwrap_or((remaining, ()));
        if peek.fragment().is_empty() {
            remaining = peek;
            break;
        }
        if starts_with_entity_kw(peek.fragment()) {
            remaining = peek;
            break;
        }
        if !peek.fragment().starts_with('*') {
            let (inp, _) = take_while(|c| c != '\n')(peek)?;
            let (inp, _) = opt(char('\n'))(inp)?;
            remaining = inp;
            continue;
        }
        match parse_mapping_rule(peek) {
            Ok((inp, rule)) => {
                rules.push(rule);
                remaining = inp;
            }
            Err(_) => {
                let (inp, _) = take_while(|c| c != '\n')(peek)?;
                let (inp, _) = opt(char('\n'))(inp)?;
                remaining = inp;
            }
        }
    }
    Ok((remaining, rules))
}
