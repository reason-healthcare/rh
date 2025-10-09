# FFQ - FHIR F### DSL Specification
- 🌐 **Language Agnostic**: Designed to be implementable in any programming language
- 📖 **Human Readable**: Intuitive syntax for FHIR terminology filtering
- 📋 **Formal Grammar**: ANTLR grammar for cross-language implementations Query Language

> ⚠️ **Experimental**: This DSL proposal is currently in experimental development and is not ready for production use.

FFQ (FHIR Filter Query) is a domain-specific language (DSL) proposal for expressing FHIR terminology filter queries in a human-readable format. This repository contains:

- **Language Specification**: A proposed DSL for FHIR terminology filtering (documented in this README)
- **Reference Implementation**: A Rust parser and FHIR ValueSet.compose translator
- **Future Work**: ANTLR grammar for language-agnostic implementation

The goal is to provide a standardized, intuitive syntax for building FHIR terminology queries that can be implemented across different programming languages and platforms.

## Features

### DSL Specification
-  **Formal Grammar**: ANTLR grammar definition for cross-language implementation
-  **Language Agnostic**: Designed to be implementable in any programming language
-  **Human Readable**: Intuitive syntax for FHIR terminology filtering

### Reference Implementation (Rust)
-  **Complete Grammar Support**: Parses the full FFQ syntax specification
- ️ **AST Generation**: Creates structured Abstract Syntax Trees from filter expressions
-  **FHIR Translation**: Converts parsed queries to FHIR ValueSet.compose JSON
- ️ **Alias Support**: Handles system and ValueSet aliases for cleaner queries
-  **Comprehensive Testing**: Well-tested with multiple real-world examples


## Quick Start (Reference Implementation)

### CLI Usage

```bash
# Parse FFQ expression and show AST
cargo run --bin rh -- ffq parse 'sct: << 73211009'

# Translate to FHIR ValueSet.compose JSON
cargo run --bin rh -- ffq translate 'sct: << 73211009'

# Interactive REPL with alias support
cargo run --bin rh -- ffq repl

# Parse from file
cargo run --bin rh -- ffq parse --file examples/01_basic_hierarchy.ffq

# Translate from file with pretty formatting
cargo run --bin rh -- ffq translate --file examples/02_property_filters.ffq
```

### Basic Example

The simplest way to get started is with a hierarchy query:

```bash
cargo run --bin rh -- ffq translate 'http://snomed.info/sct: << 73211009'
```

This will parse the FFQ expression and show the resulting FHIR JSON that can be used in ValueSet resources.

## 📚 Comprehensive Examples

The [`examples/`](examples/) directory contains detailed FFQ usage patterns and real-world scenarios:

### Basic Usage
- **[Basic Hierarchy](examples/01_basic_hierarchy.ffq)** - Hierarchy operators (`<`, `<<`) and versioned systems
- **[Property Filters](examples/02_property_filters.ffq)** - Property equality, membership, and PropertyIn expansion  
- **[Aliases](examples/03_aliases.ffq)** - System and ValueSet aliases for cleaner queries

### Advanced Operations
- **[Minus Operations](examples/04_minus_operations.ffq)** - Exclusions using minus (`-`) operator
- **[NOT Operations](examples/05_not_operations.ffq)** - Negation using NOT (`!`) operator
- **[Complex Expressions](examples/06_complex_expressions.ffq)** - Multi-level combinations and nested logic

### Real-World Use Cases  
- **[SNOMED CT Examples](examples/07_snomed_examples.ffq)** - Clinical findings, procedures, and medical concepts
- **[LOINC Examples](examples/08_loinc_examples.ffq)** - Laboratory tests and measurements
- **[Medication Examples](examples/09_medication_examples.ffq)** - Pharmaceutical products and drug hierarchies

### Running Examples

```bash
# Parse and view AST
cargo run --bin rh -- ffq parse --file examples/01_basic_hierarchy.ffq

# Translate to FHIR ValueSet.compose JSON  
cargo run --bin rh -- ffq translate --file examples/02_property_filters.ffq

# Interactive REPL (copy/paste any example)
cargo run --bin rh -- ffq repl
```

See the **[Examples README](examples/README.md)** for detailed documentation and expected outputs.

## Grammar Reference

FFQ supports the complete FHIR terminology filter query language:

### Basic Structure
```
[headers] expression
```

### System References
- **URI**: `http://snomed.info/sct`
- **With version**: `http://snomed.info/sct|20250131`
- **Alias**: `sct` (when defined in headers)

### Hierarchy Operations
- **Descendants**: `< 22298006` (children only)
- **Descendants or self**: `<< 22298006` (includes the concept itself)
- **Is-a relationship**: `isa 22298006`

### Property Filters
- **Equality**: `component = "Glucose"`
- **Membership**: `method in ("Automated count", "Manual count")`
- **Regular expressions**: `display ~ /diabetes/`
- **Existence**: `has component`

### ValueSet Operations
- **Direct reference**: `in vs(https://example.org/ValueSet/example)`
- **Alias reference**: `in #diabetes`

### Boolean Operations
- **AND**: `<< 73211009 & component = "Glucose"`
- **OR**: `<< 73211009 | << 44054006`
- **NOT**: `! << 73211009`
- **MINUS**: `<< 73211009 - << 44054006`

### Aliases
```
@alias sct = http://snomed.info/sct|20250131
@alias diabetes = vs(https://example.org/fhir/ValueSet/diabetes)
```

## Examples

### 1. Simple Hierarchy Query

**Input:**
```
http://snomed.info/sct: < 22298006
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "concept",
          "op": "descendent-of",
          "value": "22298006"
        }
      ]
    }
  ]
}
```

### 2. Complex Query with Version and Properties

**Input:**
```
http://snomed.info/sct|20250731: << 404684003 & associatedMorphology = << 49755003
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "version": "20250731",
      "filter": [
        {
          "property": "concept",
          "op": "is-a",
          "value": "404684003"
        },
        {
          "property": "associatedMorphology",
          "op": "=",
          "value": "<< 49755003"
        }
      ]
    }
  ]
}
```

### 3. LOINC Query with Property Filters

**Input:**
```
http://loinc.org: component = "Glucose" & method in ("Automated count","Manual count")
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://loinc.org",
      "filter": [
        {
          "property": "component",
          "op": "=",
          "value": "Glucose"
        },
        {
          "property": "method",
          "op": "in",
          "value": "Automated count,Manual count"
        }
      ]
    }
  ]
}
```

### 4. Complex Query with Aliases and Set Operations

**Input:**
```
@alias sct = http://snomed.info/sct|20250131
@alias dm = vs(https://example.org/fhir/ValueSet/diabetes)
sct: << 73211009 | sct: in #dm - sct: << 44054006
```

**FHIR Output:**
```json
{
  "include": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "concept",
          "op": "is-a",
          "value": "73211009"
        }
      ]
    },
    {
      "system": "http://snomed.info/sct",
      "valueSet": [
        "https://example.org/fhir/ValueSet/diabetes"
      ]
    }
  ],
  "exclude": [
    {
      "system": "http://snomed.info/sct",
      "filter": [
        {
          "property": "concept",
          "op": "is-a",
          "value": "44054006"
        }
      ]
    }
  ]
}
```

## DSL Architecture

### Language Specification
The FFQ DSL specification is documented in this README with plans for a formal ANTLR grammar definition to enable consistent implementation across different programming languages and platforms.

### Reference Implementation Architecture

This Rust implementation demonstrates the three main components needed for any FFQ processor:

#### 1. Parser (`nom`-based)
- **Lexical Analysis**: Tokenizes input into meaningful components
- **Syntax Analysis**: Builds AST using recursive descent parsing
- **Error Handling**: Provides detailed parse error information

#### 2. AST (Abstract Syntax Tree)
- **Expression Types**: Clauses, boolean operations, grouping
- **Term Types**: Hierarchy, property filters, membership, existence  
- **Header Types**: System and ValueSet aliases

#### 3. FHIR Translator
- **AST → FHIR**: Converts parsed expressions to FHIR ValueSet.compose
- **Alias Resolution**: Resolves system and ValueSet references
- **JSON Serialization**: Outputs valid FHIR JSON

### Implementation Guidelines
Other language implementations should follow this general architecture:
1. **Parse** FFQ syntax into an AST representation
2. **Validate** the AST against FFQ semantic rules
3. **Translate** the AST to target format (e.g., FHIR ValueSet.compose)

## Supported FHIR Filter Operators

| FFQ Operator | FHIR Operator | Description |
|--------------|---------------|-------------|
| `=` | `=` | Equals |
| `<<` | `is-a` | Is-a (includes concept) |
| `<` | `descendent-of` | Descendant of (excludes concept) |
| `isa` | `is-a` | Is-a relationship |
| `in (...)` | `in` | Value is in set |
| `~ /pattern/` | `regex` | Regular expression match |
| `has prop` | `exists` | Property exists |

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- ✅ Parser functionality with all grammar features
- ✅ AST generation accuracy
- ✅ FHIR translation correctness
- ✅ Alias resolution
- ✅ Error handling

## Dependencies

### Reference Implementation (Rust)
- **nom**: Parser combinator library
- **serde**: Serialization framework  
- **serde_json**: JSON serialization

### Language Specification  
- **ANTLR**: Planned formal grammar definition (language-agnostic)

## Limitations

As an experimental project, FFQ has some current limitations:

- **Complex Boolean Logic**: Some nested boolean operations may not translate optimally
- **Error Messages**: Parse error messages could be more user-friendly  
- **Performance**: Not optimized for large-scale production use
- **Validation**: Limited validation of FHIR terminology concepts
- **Extensions**: No support for custom filter operators

## Contributing

This is an experimental DSL proposal. We welcome:

- **Language Design**: Feedback on FFQ syntax and semantics  
- **Grammar Improvements**: Enhancements to the ANTLR specification
- **Alternative Implementations**: FFQ parsers in other programming languages
- **Use Cases**: Real-world FHIR terminology filtering scenarios
- **Tooling**: Editors, validators, and other FFQ development tools

## Implementation in Other Languages

A planned ANTLR grammar will enable generating FFQ parsers for many programming languages including Java, Python, JavaScript, C#, C++, and others. In the meantime, implementers can use this Rust reference implementation and the grammar specification in this README as a guide. We encourage community implementations following the reference architecture outlined above.

## License

[Add your license information here]

## Related Resources

- [FHIR Terminology Module](https://hl7.org/fhir/terminology-module.html)
- [FHIR ValueSet Resource](https://hl7.org/fhir/valueset.html)
- [FHIR Filter Operations](https://hl7.org/fhir/valueset-filter-operator.html)
