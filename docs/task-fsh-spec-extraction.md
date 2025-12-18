# Task: FSH Specification Extraction for LLM Coding Agent

## Objective

Create a reusable script that extracts the FHIR Shorthand (FSH) specification from the official guide and transforms it into a format optimized for an LLM coding agent working on the rh-fsh compiler implementation.

## Background

The FSH specification at https://build.fhir.org/ig/HL7/fhir-shorthand/ is the authoritative reference for the language. However, it is designed for human readers in HTML format. For an LLM agent implementing a compiler, we need a consolidated, structured reference that:

1. Captures ALL syntactic rules without ambiguity
2. Includes concrete examples for each construct
3. Documents edge cases and special handling
4. Is formatted for efficient token usage

## Source URLs

| Section | URL | Content |
|---------|-----|---------|
| Overview | https://build.fhir.org/ig/HL7/fhir-shorthand/overview.html | Concepts and terminology |
| Reference | https://build.fhir.org/ig/HL7/fhir-shorthand/reference.html | Complete language reference |

## Primary Deliverable: Extraction Script

Write a script (`extract_fsh_spec.rs` or `extract_fsh_spec.py`) that:

1. **Fetches** the FSH specification pages
2. **Parses** the HTML structure
3. **Extracts** relevant content (definitions, examples, grammar)
4. **Cleans** boilerplate (navigation, footers, balloting info)
5. **Restructures** into LLM-optimized Markdown
6. **Outputs** `fsh-language-spec.md`

### Script Requirements

```
Sectioning Strategy:
- Split output by logical FSH concepts (## Aliases, ## Rules, ## Slicing)
- For every syntactic element (*, ->, from), capture definition + adjacent example

Example Extraction:
- Identify all <pre> or <code> blocks with FSH syntax
- Pair examples immediately with explanatory text
- Format as concept/explanation/example triplets

Grammar Isolation:
- Locate grammar appendix/section
- Extract EBNF or production rules
- Cross-reference with SUSHI's ANTLR grammar

Cleanup:
- Remove HL7 boilerplate (footers, ballot banners, history tables)
- Remove navigation sidebars
- Convert relative URLs to absolute URLs
- Preserve semantic structure
```

## Secondary Deliverable: Output Format

The script should produce `fsh-language-spec.md` with the following sections:

### 1. Token Definitions

Extract all lexical tokens with their exact patterns:

```markdown
## Tokens

### Keywords (Declaration)
| Token | Pattern | Example |
|-------|---------|---------|
| KW_ALIAS | `Alias` `:` | `Alias: $SCT = http://snomed.info/sct` |
| KW_PROFILE | `Profile` `:` | `Profile: MyPatient` |
...

### Keywords (Metadata)
| Token | Pattern | Context |
|-------|---------|---------|
| KW_PARENT | `Parent` `:` | Profile, Extension, Logical, Resource |
| KW_ID | `Id` `:` | All except Alias, Invariant |
...

### Operators and Delimiters
| Token | Pattern | Usage |
|-------|---------|-------|
| STAR | `\n` `*` ` ` | Rule prefix |
| EQUAL | `=` | Assignment |
| ARROW | `->` | Mapping |
...

### Literals
| Token | Pattern | Examples |
|-------|---------|----------|
| STRING | `"` chars `"` | `"Hello World"` |
| MULTILINE_STRING | `"""` chars `"""` | Multi-line descriptions |
| NUMBER | `[+-]?[0-9]+...` | `42`, `-3.14`, `1e10` |
| DATETIME | ISO format | `2024-01-15`, `2024-01-15T10:30:00Z` |
| CODE | `system#code "display"` | `http://loinc.org#12345-6 "Test"` |
...
```

### 2. Entity Definitions

For each FSH entity type, document:

```markdown
## Profile

### Declaration
```fsh
Profile: <name>
```

### Allowed Keywords
| Keyword | Required | Description |
|---------|----------|-------------|
| Parent | Yes | Base resource or profile |
| Id | No | Identifier (max 64 chars, lowercase with hyphens) |
| Title | No | Human-readable short name |
| Description | No | Detailed description |

### Allowed Rules
- CardRule
- FlagRule
- ValueSetRule
- FixedValueRule
- ContainsRule
- OnlyRule
- ObeysRule
- CaretValueRule
- InsertRule
- PathRule

### Examples
```fsh
Profile: USCorePatient
Parent: Patient
Id: us-core-patient
Title: "US Core Patient Profile"
Description: "Defines constraints on Patient for US Core."
* identifier 1..*
* identifier.system 1..1
* name 1..* MS
* name.family 1..1 MS
* gender 1..1 MS
* birthDate MS
```

### Edge Cases
- Parent is required; omitting it is a parse error
- Parent can reference another Profile (not just base resources)
- Cannot use inline extensions (use standalone extension syntax)
```

### 3. Rule Definitions

For each rule type, document the complete syntax:

```markdown
## CardRule (Cardinality Rule)

### Syntax
```
* <path> <cardinality>
```

### Cardinality Format
- `min..max` where min is integer, max is integer or `*`
- Examples: `0..1`, `1..1`, `0..*`, `1..*`, `2..5`

### Path Syntax
- Simple: `status`
- Nested: `name.given`
- Indexed: `name[0].given`
- Sliced: `component[bloodPressure]`
- Extension: `extension[birthsex]`

### Examples
```fsh
* identifier 1..*
* name.given 1..*
* contact 0..3
* address 0..*
```

### Semantic Rules
- Cannot widen cardinality (only constrain)
- min must be <= max
- If max is 0, element is prohibited
```

### 4. Path Syntax Reference

Complete documentation of FSH path expressions:

```markdown
## Path Syntax

### Basic Paths
| Type | Pattern | Example |
|------|---------|---------|
| Simple | `element` | `status` |
| Nested | `parent.child` | `name.given` |
| Indexed | `element[n]` | `name[0]` |
| Soft Index | `element[+]` or `element[=]` | `name[+]` |

### Slice Paths
| Type | Pattern | Example |
|------|---------|---------|
| Named Slice | `array[sliceName]` | `component[systolic]` |
| Re-slice | `array[slice][reslice]` | `component[bp][systolic]` |
| Extension | `extension[name]` | `extension[birthsex]` |

### Type Choice Paths
| Pattern | Example | Note |
|---------|---------|------|
| `element[x]Type` | `onsetDateTime` | NOT `onset[dateTime]` |
| `element[Type]` | `value[Quantity]` | For references |

### Caret Paths (Metadata)
| Pattern | Target | Example |
|---------|--------|---------|
| `^property` | Current element | `^short = "Brief"` |
| `path ^property` | Specific element | `code ^binding.description = "..."` |
```

### 5. Value Formats

Document all value literal formats:

```markdown
## Value Formats

### Primitive Values
| Type | Format | Examples |
|------|--------|----------|
| string | `"text"` | `"Hello"` |
| boolean | `true` / `false` | `true` |
| integer | `[+-]?[0-9]+` | `42`, `-7` |
| decimal | `[+-]?[0-9]+.[0-9]+` | `3.14` |
| date | `YYYY-MM-DD` | `2024-01-15` |
| dateTime | ISO 8601 | `2024-01-15T10:30:00Z` |

### Code Values
| Format | Example |
|--------|---------|
| Code only | `#active` |
| System#code | `http://hl7.org/fhir/status#active` |
| With display | `$SCT#12345 "Finding"` |

### Quantity Values
| Format | Example |
|--------|---------|
| UCUM | `70 'kg'` |
| Non-UCUM | `3 http://example.org#tablets` |

### Reference Values
| Format | Example |
|--------|---------|
| Type | `Reference(Patient)` |
| Multiple | `Reference(Patient or Practitioner)` |
| Instance | `Reference(MyPatientInstance)` |

### Canonical Values
| Format | Example |
|--------|---------|
| By name | `Canonical(MyProfile)` |
| With version | `Canonical(MyProfile|1.0.0)` |
```

### 6. Reserved Words and Symbols

Complete list with context:

```markdown
## Reserved Words

### Context-Free Reserved
- `true`, `false`
- `from`, `contains`, `named`, `and`, `or`, `only`, `obeys`
- `include`, `exclude`, `codes`, `where`, `valueset`, `system`
- `insert`, `contentReference`

### Context-Sensitive
These are reserved in specific positions but can appear in paths:
- `MS`, `SU`, `TU`, `N`, `D` (flags)
- `example`, `preferred`, `extensible`, `required` (binding strengths)
- `exactly` (fixed value modifier)

### Reserved Patterns
- `Reference(...)` - Reference type
- `Canonical(...)` - Canonical reference
- `CodeableReference(...)` - R5 codeable reference
- `(example)`, `(preferred)`, etc. - Binding strengths
```

## Script Implementation Guide

### Step 1: Agent Writes the Script

The agent should write a script with this structure:

```python
# extract_fsh_spec.py (example structure)

import requests
from bs4 import BeautifulSoup
from pathlib import Path

SOURCES = [
    "https://build.fhir.org/ig/HL7/fhir-shorthand/reference.html",
    "https://build.fhir.org/ig/HL7/fhir-shorthand/overview.html",
]

ANTLR_GRAMMAR = [
    "https://raw.githubusercontent.com/FHIR/sushi/master/antlr/src/main/antlr/FSH.g4",
    "https://raw.githubusercontent.com/FHIR/sushi/master/antlr/src/main/antlr/FSHLexer.g4",
]

def fetch_and_parse(url: str) -> BeautifulSoup:
    """Fetch HTML and return parsed soup."""
    ...

def extract_sections(soup: BeautifulSoup) -> dict:
    """Extract logical sections from the spec."""
    ...

def extract_code_examples(soup: BeautifulSoup) -> list:
    """Find all FSH code blocks with context."""
    ...

def clean_boilerplate(soup: BeautifulSoup) -> BeautifulSoup:
    """Remove nav, footers, ballot info."""
    ...

def format_as_markdown(sections: dict, examples: list) -> str:
    """Structure into LLM-optimized Markdown."""
    ...

def main():
    output = []
    for url in SOURCES:
        soup = fetch_and_parse(url)
        soup = clean_boilerplate(soup)
        sections = extract_sections(soup)
        examples = extract_code_examples(soup)
        output.append(format_as_markdown(sections, examples))

    # Add ANTLR grammar reference
    for grammar_url in ANTLR_GRAMMAR:
        output.append(f"## ANTLR Grammar: {grammar_url.split('/')[-1]}\n")
        output.append(f"```antlr\n{requests.get(grammar_url).text}\n```\n")

    Path("fsh-language-spec.md").write_text("\n".join(output))

if __name__ == "__main__":
    main()
```

### Step 2: Agent Runs and Iterates

1. Run the script: `python extract_fsh_spec.py`
2. Review output quality
3. Adjust extraction logic as needed
4. Re-run until output is satisfactory

### Step 3: Cross-Reference with ANTLR Grammar

The script should fetch and include the ANTLR grammar from:
- `FSH.g4` - Parser rules
- `FSHLexer.g4` - Lexer rules

These serve as the executable specification to validate extracted rules against.

### Step 4: Validate Output

Test the extracted spec by:
- Checking all FSH constructs are documented
- Verifying examples are correctly paired with explanations
- Confirming ANTLR grammar is aligned with prose descriptions

## Quality Criteria

The extraction script should produce output that enables an LLM agent to:

1. **Parse any valid FSH** - All syntax constructs documented
2. **Reject invalid FSH** - Error conditions specified
3. **Generate correct FHIR** - Semantic mappings clear
4. **Match SUSHI behavior** - Edge cases documented

## Output Locations

```
/Users/sjm/src/rh/scripts/extract_fsh_spec.py   # The extraction script
/Users/sjm/src/rh/docs/fsh-language-spec.md      # The generated spec
```

## Estimated Effort

| Phase | Work | LLM Inference Cost |
|-------|------|-------------------|
| Write script | Agent writes ~200 lines | One-time |
| Debug/iterate | Fix edge cases | Minimal |
| Re-run on spec updates | Execute script | Zero |

Total: Lower cost than inference-based extraction, with better results.

## Notes for Agent

- The FSH specification uses "normative" and "trial use" (TU) markers
- Trial Use features may change in future versions
- Some features are FHIR version specific (R4 vs R5)
- The grammar files are "informational" - spec text takes precedence on conflicts
- **The script is a deliverable** - commit it to version control
- **Re-run periodically** - FSH spec may be updated
