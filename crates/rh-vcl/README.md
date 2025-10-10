# rh-vcl - ValueSet Compose Language Parser

[![Crates.io](https://img.shields.io/crates/v/rh-vcl)](https://crates.io/crates/rh-vcl)
[![Documentation](https://docs.rs/rh-vcl/badge.svg)](https://docs.rs/rh-vcl)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

A Rust parser for the ValueSet Compose Language (VCL), a domain-specific language for expressing ValueSet CLDs in a compact syntax suitable for use within a URL and enabling a new family of implicit ValueSet URIs that are usable across all code systems.

VCL is inspired by SNOMED CT's Expression Constraint Language (ECL) and is defined in the [FHIR IG Guidance](https://build.fhir.org/ig/FHIR/ig-guidance/vcl.html).

## Features

- ✅ **Complete VCL Grammar Support** - Parses all VCL constructs according to the official grammar
- ✅ **Rich AST** - Detailed Abstract Syntax Tree with convenient utility methods
- ✅ **FHIR ValueSet Translation** - Convert VCL expressions to FHIR ValueSet.compose structures
- ✅ **Comprehensive Error Handling** - Meaningful error messages with position information
- ✅ **Zero-Copy Parsing** - Efficient parsing using nom with minimal allocations
- ✅ **Serde Support** - Serialize/deserialize parsed VCL expressions and FHIR structures to/from JSON
- ✅ **WASM Compatible** - Can be compiled to WebAssembly for use in web applications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rh-vcl = "0.1.0"
```

## Quick Start

```rust
use rh_vcl::{parse_vcl, translate_vcl_string_to_fhir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a VCL expression
    let vcl_str = "(http://snomed.info/sct)status = \"active\"";
    let expression = parse_vcl(vcl_str)?;
    
    println!("Parsed VCL expression successfully!");
    println!("System URIs: {:?}", expression.system_uris());
    println!("Codes: {:?}", expression.codes());
    
    // Translate to FHIR ValueSet.compose
    let fhir_compose = translate_vcl_string_to_fhir(vcl_str)?;
    println!("FHIR compose: {}", serde_json::to_string_pretty(&fhir_compose)?);
    
    Ok(())
}
```

## VCL Language Support

This parser supports all VCL language features:

### Basic Expressions
- **Wildcards**: `*` (matches all codes)
- **Simple codes**: `123456` or `"quoted code"`
- **System URIs**: `(http://snomed.info/sct)123456`

### Operators
- **Conjunction** (AND): `code1, code2, code3`
- **Disjunction** (OR): `code1; code2; code3`  
- **Exclusion** (NOT): `* - inactive`

### Filters
- **Equality**: `status = "active"`
- **Is-a relationships**: `category << 123456`
- **Descendant-of**: `type < 456789`
- **Regular expressions**: `code / "^[A-Z]+$"`
- **Set operations**: `status ^ {active, inactive}`
- **Of operations**: `*.category`, `{code1, code2}.property`

### Complex Expressions
- **Nested expressions**: `(code1, code2); code3`
- **ValueSet inclusions**: `^http://example.org/valueset`
- **Filter lists**: `{status = "active", category << 123456}`

## Examples

### Basic Usage

```rust
use rh_vcl::parse_vcl;

// Simple code
let expr = parse_vcl("123456")?;

// Code with system URI
let expr = parse_vcl("(http://snomed.info/sct)123456")?;

// Wildcard
let expr = parse_vcl("*")?;

// Property filter
let expr = parse_vcl("status = \"active\"")?;

// Is-a relationship
let expr = parse_vcl("category << 123456")?;
```

### Complex Expressions

```rust
use rh_vcl::parse_vcl;

// Conjunction with system URI
let expr = parse_vcl("(http://snomed.info/sct)status = \"active\", category << 123456")?;

// Exclusion
let expr = parse_vcl("* - {inactive, \"not applicable\"}")?;

// Nested expressions
let expr = parse_vcl("(code1, code2); (code3, code4)")?;

// Of operations
let expr = parse_vcl("*.category")?;
let expr = parse_vcl("{code1, code2}.property")?;
```

### AST Analysis

```rust
use rh_vcl::parse_vcl;

let expr = parse_vcl("(http://snomed.info/sct)status = \"active\", category << 123456")?;

// Check for wildcards
if expr.contains_wildcards() {
    println!("Expression contains wildcards");
}

// Get all system URIs
let uris = expr.system_uris();
println!("System URIs: {:?}", uris);

// Get all codes
let codes = expr.codes();
println!("Codes: {:?}", codes);

// Serialize to JSON
let json = serde_json::to_string_pretty(&expr)?;
println!("JSON: {}", json);
```

### VCL to FHIR Translation

```rust
use rh_vcl::{VclTranslator, translate_vcl_string_to_fhir};

// Simple translation using convenience function
let fhir_compose = translate_vcl_string_to_fhir("(http://snomed.info/sct)123456")?;
println!("{}", serde_json::to_string_pretty(&fhir_compose)?);

// Custom translator with default system
let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
let vcl_expr = parse_vcl("status = \"active\", category << 123456")?;
let fhir_compose = translator.translate(&vcl_expr)?;

// Results in FHIR ValueSet.compose with multiple includes
assert_eq!(fhir_compose.include.len(), 2);
```

### Error Handling

```rust
use rh_vcl::{parse_vcl, VclError};

match parse_vcl("invalid expression $") {
    Ok(expr) => println!("Parsed: {:?}", expr),
    Err(VclError::ParseError { message, position, .. }) => {
        println!("Parse error at position {}: {}", position, message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Grammar Reference

The parser implements the complete VCL grammar:

```antlr
vcl         : expr EOF ;
expr        : subExpr (conjunction | disjunction | exclusion )? ;
subExpr     : systemUri? (simpleExpr | OPEN expr CLOSE);
conjunction : (COMMA subExpr)+ ;
disjunction : (SEMI subExpr)+ ;
exclusion   : DASH subExpr ;
simpleExpr  : STAR | code | filter | includeVs ;

includeVs   : IN (URI | systemUri) ;
systemUri   : OPEN URI CLOSE;
filter      : property (EQ | IS_A | DESC_OF | ...) value
            | (code | codeList | STAR | URI | filterList) DOT property ;

code        : SCODE | QUOTED_VALUE ;
```

See the [FHIR IG Guidance](https://build.fhir.org/ig/FHIR/ig-guidance/vcl.html) for the complete grammar specification.

## VCL to FHIR Translation

This crate can translate VCL expressions into FHIR ValueSet.compose structures:

### Translation Rules

| VCL Construct | FHIR Mapping | Example |
|---------------|--------------|---------|
| Simple code | `include.concept` | `123456` → `{"concept": [{"code": "123456"}]}` |
| Wildcard `*` | `include` (all codes) | `*` → `{"system": "..."}` |
| System URI | `include.system` | `(http://snomed.info/sct)code` → `{"system": "http://snomed.info/sct"}` |
| Property filter | `include.filter` | `status = "active"` → `{"filter": [{"property": "status", "op": "=", "value": "active"}]}` |
| Conjunction `,` | Multiple `include` | `code1, code2` → `{"include": [{...}, {...}]}` |
| Disjunction `;` | Single `include` with multiple concepts | `code1; code2` → `{"include": [{"concept": [...]}]}` |
| Exclusion `-` | `exclude` entry | `* - inactive` → `{"include": [...], "exclude": [...]}` |
| ValueSet inclusion | `include.valueSet` | `^http://example.org/vs` → `{"valueSet": ["..."]}` |

### Filter Operator Mapping

| VCL Operator | FHIR Operator | Description |
|--------------|---------------|-------------|
| `=` | `=` | Equals |
| `<<` | `is-a` | Is-a (subsumption) |
| `~<<` | `is-not-a` | Is-not-a |
| `<` | `descendent-of` | Descendant-of |
| `^` | `in` | In (member of) |
| `~^` | `not-in` | Not-in |
| `>>` | `generalizes` | Generalizes |
| `<!` | `child-of` | Child-of |
| `!!<` | `descendent-leaf` | Descendant-leaf |
| `?` | `exists` | Exists |
| `/` | `regex` | Regular expression |

## WASM Support

This crate can be compiled to WebAssembly for use in web applications:

```toml
[dependencies]
rh-vcl = { version = "0.1.0", features = ["wasm"] }
```

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](../../CONTRIBUTING.md) file for guidelines.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Related Projects

This crate is part of the [Reason Healthcare](https://github.com/reason-healthcare/rh) monorepo, which includes:

- **rh-codegen** - FHIR code generation from StructureDefinitions
- **rh-fhirpath** - FHIRPath expression evaluation
- **rh-validator** - FHIR resource validation
- **rh-loader** - FHIR package and resource loading