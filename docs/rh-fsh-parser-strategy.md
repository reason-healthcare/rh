# rh-fsh Parser Strategy Analysis

## Overview

This document analyzes the options for implementing the FSH parser in Rust, evaluating whether to use the existing ANTLR4 grammar from SUSHI or take an alternative approach.

## The SUSHI ANTLR Grammar

### Grammar Files

SUSHI uses ANTLR4 with two grammar files:

| File | Size | Purpose |
|------|------|---------|
| `FSH.g4` | 7.2KB | Parser rules (syntax structure) |
| `FSHLexer.g4` | 5.9KB | Lexer rules (tokenization) |
| `MiniFSH.g4` | 335B | Subset grammar for inline FSH |
| `MiniFSHLexer.g4` | 673B | Lexer for MiniFSH |

### Grammar Characteristics

**Lexer Modes:**
The FSH lexer uses ANTLR mode switching for context-sensitive tokenization:
- `RULESET_OR_INSERT` - After `RuleSet:` or `insert` keywords
- `PARAM_RULESET_OR_INSERT` - For parameterized RuleSet references
- `LIST_OF_CONTEXTS` - For Extension context lists
- `LIST_OF_CODES` - For Characteristics code lists

**Parser Structure:**
```antlr
doc:     entity* EOF;
entity:  alias | profile | extension | invariant | instance
       | valueSet | codeSystem | ruleSet | paramRuleSet
       | mapping | logical | resource;
```

**Key Complexity Points:**
1. The `STAR` token matches newline + whitespace + `*` + space (rule delimiter)
2. `CODE` token handles `system#code` with optional quoted display text
3. `SEQUENCE` is a catch-all for non-whitespace (very permissive)
4. Reserved words like `from`, `contains`, `and` can appear in paths via `mostAlphaKeywords`

### Grammar Quirks to Note

```antlr
// Star requires newline before asterisk
STAR: ([\r\n] | LINE_COMMENT) WS* '*' [ \u00A0];

// Code can include quoted strings with escapes
CODE: SEQUENCE? '#' (SEQUENCE | CONCEPT_STRING);
CONCEPT_STRING: '"' (NONWS_STR | '\\"' | '\\\\')+ (WS (NONWS_STR | '\\"' | '\\\\')+)* '"';

// Path segments can be keywords
name: SEQUENCE | NUMBER | DATETIME | TIME | mostAlphaKeywords;
path: SEQUENCE | NUMBER | DATETIME | TIME | mostAlphaKeywords;
```

## Option Analysis

### Option 1: antlr4rust (Port ANTLR Grammar)

**Repository:** [rrevenantt/antlr4rust](https://github.com/rrevenantt/antlr4rust)

**Pros:**
- Direct port of official grammar ensures specification compliance
- Grammar updates from SUSHI can be applied easily
- ANTLR4 is well-documented with extensive tooling
- Works on stable Rust since v0.3

**Cons:**
- Requires Java toolchain for grammar compilation
- Runtime is 3-4x slower than hand-written parsers (see benchmarks)
- Not fully merged into mainline ANTLR4 (uses fork)
- Generated code is verbose and harder to customize
- Limited control over error recovery strategies

**Benchmark Reference (from antlr4rust README):**
```
Lexers:
  xmlparser:        1.86ms
  quick_xml:        1.46ms
  antlr_xml_lexer:  5.79ms  (3.1x slower)

Parsers:
  roxmltree:        4.93ms
  antlr_xml_full:   10.25ms (2.1x slower)
```

**Effort Estimate:** Medium (grammar porting + runtime integration)

### Option 2: pest (PEG Parser Generator)

**Repository:** [pest-parser/pest](https://github.com/pest-parser/pest)

**Pros:**
- Native Rust, no external toolchain
- Clean grammar syntax similar to ANTLR
- Good error messages out of the box
- Active development and community
- Simpler than LALRPOP for most grammars

**Cons:**
- PEG semantics differ from ANTLR's ALL(*) - may require grammar restructuring
- No typed AST generation (must write conversion layer manually)
- Ordered choice (`/`) vs alternation (`|`) can cause subtle bugs
- Slower than nom for complex grammars

**pest Grammar Example:**
```pest
doc = { SOI ~ entity* ~ EOI }
entity = { alias | profile | extension | instance | invariant | ... }

profile = {
    "Profile" ~ ":" ~ name ~
    sd_metadata* ~
    sd_rule*
}

name = @{ (ASCII_ALPHANUMERIC | "_")+ }
```

**Effort Estimate:** High (complete grammar rewrite + AST conversion)

### Option 3: LALRPOP (LR(1) Parser Generator)

**Repository:** [lalrpop/lalrpop](https://github.com/lalrpop/lalrpop)

**Pros:**
- Native Rust, integrated with Cargo
- Generates typed AST directly (most ergonomic)
- Used by major projects (RustPython, Solang)
- Grammar macros reduce repetition
- Excellent error messages during grammar development

**Cons:**
- LR(1) may not handle all FSH constructs without refactoring
- Default lexer is "toy quality" - need custom lexer for FSH
- Steeper learning curve than pest
- Grammar syntax differs significantly from ANTLR

**LALRPOP Grammar Example:**
```lalrpop
pub Doc: Vec<Entity> = Entity* => <>;

Entity: Entity = {
    <Alias> => Entity::Alias(<>),
    <Profile> => Entity::Profile(<>),
    // ...
};

Profile: Profile = {
    "Profile" ":" <name:Name> <meta:SdMetadata*> <rules:SdRule*> => {
        Profile { name, metadata: meta, rules }
    }
};
```

**Effort Estimate:** High (grammar rewrite + custom lexer)

### Option 4: Hand-Written Recursive Descent

**Pros:**
- Maximum control over parsing behavior
- Best error recovery and messages
- Optimal performance
- No build-time code generation
- Easier debugging

**Cons:**
- Highest implementation effort
- Must manually handle all grammar rules
- Harder to verify against specification
- More prone to implementation bugs

**Effort Estimate:** Very High (complete manual implementation)

## Recommendation

### Primary Recommendation: pest

**Rationale:**

1. **Balance of effort and control** - pest provides a middle ground between using antlr4rust (easiest but least control) and hand-writing (most control but highest effort).

2. **No external toolchain** - Unlike antlr4rust which requires Java, pest integrates cleanly into the Rust build system.

3. **Good enough performance** - For a compiler that processes text files, pest's performance is adequate. The bottleneck will likely be FHIR package loading, not parsing.

4. **Error message quality** - pest's automatic error messages with line/column information are good enough for initial development.

5. **Grammar readability** - pest grammars are readable and can be cross-referenced with the FSH specification.

### Migration Path

If pest proves insufficient (performance or expressiveness issues), the codebase can migrate to:
- LALRPOP (if typed AST generation becomes important)
- Hand-written parser (if performance becomes critical)

The AST types will be the same regardless of parser, so migration affects only the parsing layer.

### Alternative Consideration: antlr4rust for Specification Compliance

If strict SUSHI compatibility is the highest priority, consider antlr4rust:
- Same grammar = same parsing behavior
- Easier to track FSH specification changes
- Grammar differences from SUSHI become test failures rather than unnoticed bugs

However, this trades implementation convenience for specification fidelity.

## Implementation Approach for pest

### Phase 1: Lexer Foundation

Create a pest grammar that handles FSH's lexical structure:

```pest
// fsh.pest

WHITESPACE = _{ " " | "\t" }
COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* | "/*" ~ (!"*/" ~ ANY)* ~ "*/" }

// Keywords with colon
KW_ALIAS = { "Alias" ~ ":" }
KW_PROFILE = { "Profile" ~ ":" }
KW_EXTENSION = { "Extension" ~ ":" }
// ... etc

// The critical STAR token (rule delimiter)
STAR = { NEWLINE ~ WHITESPACE* ~ "*" ~ " " }

// Codes: system#code "display"
CODE = @{ SEQUENCE? ~ "#" ~ (SEQUENCE | CONCEPT_STRING) }
CONCEPT_STRING = @{ "\"" ~ (!"\"" ~ ANY | "\\\"")* ~ "\"" }

// Catch-all sequence
SEQUENCE = @{ (!WHITESPACE ~ !NEWLINE ~ ANY)+ }
```

### Phase 2: Parser Rules

Map ANTLR parser rules to pest:

```pest
doc = { SOI ~ entity* ~ EOI }

entity = {
    alias
  | profile
  | extension
  | invariant
  | instance
  | value_set
  | code_system
  | rule_set
  | param_rule_set
  | mapping
  | logical
  | resource
}

profile = {
    KW_PROFILE ~ name ~
    sd_metadata* ~
    sd_rule*
}

sd_metadata = { parent | id | title | description }
sd_rule = { card_rule | flag_rule | value_set_rule | fixed_value_rule | ... }
```

### Phase 3: AST Conversion

Create Rust types and conversion from pest parse tree:

```rust
// src/parser/ast.rs

pub struct FshDocument {
    pub entities: Vec<Entity>,
}

pub enum Entity {
    Alias(Alias),
    Profile(Profile),
    Extension(Extension),
    // ...
}

pub struct Profile {
    pub name: String,
    pub parent: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub rules: Vec<SdRule>,
    pub location: SourceLocation,
}
```

### Phase 4: Error Handling

Implement FSH-specific error types:

```rust
// src/error.rs

pub struct FshError {
    pub kind: FshErrorKind,
    pub message: String,
    pub location: SourceLocation,
    pub source_line: String,
}

pub enum FshErrorKind {
    ParseError,
    UnknownAlias(String),
    InvalidPath(String),
    CardinalityError,
    // ...
}
```

## Testing Strategy

### Unit Tests
- Test each grammar rule in isolation
- Compare parse trees against expected AST structures

### Integration Tests
- Parse real FSH files from FSH Finder repositories
- Compare parsed AST with SUSHI's behavior

### Differential Testing
- Run both rh-fsh and SUSHI on same input
- Compare generated FHIR artifacts

## Appendix: Full ANTLR Grammar Reference

The complete FSH.g4 grammar is available at:
https://github.com/FHIR/sushi/blob/master/antlr/src/main/antlr/FSH.g4

Key sections for implementation reference:
- Lines 1-15: Entity definitions
- Lines 16-30: Metadata keywords
- Lines 31-60: Rule definitions
- Lines 61-80: ValueSet components
- Lines 81-100: Helper rules (name, path, value, etc.)
