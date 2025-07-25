# FHIRPath Parser and Evaluator

A Rust implementation of a FHIRPath expression parser and evaluator for FHIR resources.

## Overview

FHIRPath is a path-based navigation and extraction language that can be used to navigate and extract elements from FHIR resources. This crate provides:

- **Parser**: Converts FHIRPath expressions into an Abstract Syntax Tree (AST)
- **Evaluator**: Evaluates FHIRPath expressions against FHIR resources (basic implementation)
- **AST**: Type-safe representation of FHIRPath expressions

## Features

### Currently Implemented

- ✅ Basic expression parsing with nom parser combinators
- ✅ Member access (`Patient.name`, `Patient.name.given`)
- ✅ Array indexing (`Patient.name[0]`)
- ✅ Function calls (`name.count()`, `name.exists()`)
- ✅ Literal values (`true`, `false`, `42`, `'string'`, `{}`)
- ✅ Union expressions (`name.given | name.family`)
- ✅ Equality operators (`=`, `!=`, `~`, `!~`)
- ✅ Logical operators (`and`, `or`, `xor`)
- ✅ External constants (`%context`, `%resource`)
- ✅ Special variables (`$this`, `$index`, `$total`)
- ✅ Parenthesized expressions
- ✅ Basic evaluation for simple expressions

### TODO

- ⏳ Comparison operators (`<`, `<=`, `>`, `>=`)
- ⏳ Date/time literals (`@2023-01-01`, `@T12:00:00`)
- ⏳ Type operations (`is`, `as`)
- ⏳ Mathematical operations (`+`, `-`, `*`, `/`, `div`, `mod`)
- ⏳ String concatenation (`&`)
- ⏳ Built-in functions (where, select, count, exists, etc.)
- ⏳ Quantity literals with units
- ⏳ Complex evaluation contexts
- ⏳ Error reporting with line/column information

## Usage

### Basic Parsing

```rust
use fhirpath::FhirPathParser;

let parser = FhirPathParser::new();
let expression = parser.parse("Patient.name.given").unwrap();
println!("Parsed: {}", expression);
```

### Evaluation

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
use serde_json::json;

let patient = json!({
    "resourceType": "Patient",
    "name": [{
        "given": ["John"],
        "family": "Doe"
    }]
});

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();
let context = EvaluationContext::new(patient);

let expression = parser.parse("resourceType").unwrap();
let result = evaluator.evaluate(&expression, &context).unwrap();
println!("Result: {:?}", result);
```

## Grammar Support

This implementation is based on the official FHIRPath grammar from:
https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4

The parser supports most of the core FHIRPath syntax including:

- Path expressions
- Literal values
- Function invocations
- Boolean logic
- Comparison operations
- Union operations

## Architecture

### Parser (`parser.rs`)

Uses the `nom` parser combinator library to parse FHIRPath expressions into an AST. The parser is built with operator precedence in mind:

1. OR/XOR expressions (lowest precedence)
2. AND expressions
3. Equality expressions
4. Union expressions
5. Invocation expressions (member access, indexing)
6. Terms (literals, functions, parentheses)

### AST (`ast.rs`)

Defines a complete type-safe representation of FHIRPath expressions with proper Display implementations for debugging and serialization support.

### Evaluator (`evaluator.rs`)

Provides basic evaluation capabilities for FHIRPath expressions against JSON-based FHIR resources. Currently supports:

- Literal evaluation
- Simple member access
- Basic logical operations

### Error Handling (`error.rs`)

Comprehensive error types for parsing and evaluation errors with context information.

## Examples

```rust
// Simple member access
"Patient.name"

// Array indexing
"Patient.name[0].given"

// Function calls
"name.count()"
"name.exists()"
"name.empty()"

// Logical expressions
"active = true"
"name.exists() and birthDate.exists()"

// Union expressions
"name.given | name.family"

// Complex expressions
"name.where(use = 'official').given"
```

## Testing

Run the test suite:

```bash
cargo test --package fhirpath
```

Run integration tests with output:

```bash
cargo test --package fhirpath test_parser_examples -- --nocapture
```

## Contributing

This is an early implementation of FHIRPath in Rust. Contributions are welcome, especially for:

- Implementing missing operators and functions
- Improving evaluation performance
- Adding more comprehensive test cases
- Better error reporting

## License

This project is licensed under the MIT OR Apache-2.0 license.
