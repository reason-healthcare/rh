# FHIRPath Parser and Evaluator Engine in Rust

A comprehensive Rust implementation of a FHIRPath expression parser and evaluator for FHIR resources.

## Background

FHIRPath is a path-based navigation and extraction language for FHIR resources, defined by the HL7 FHIR specification. This crate provides a complete implementation with:

- **Parser**: Converts FHIRPath expressions into an Abstract Syntax Tree (AST) using nom parser combinators
- **Evaluator**: Comprehensive evaluation of FHIRPath expressions against FHIR resources
- **Type Safety**: Rust-native type checking and error handling with comprehensive error types
- **Performance**: Optimized for production use with efficient parsing and evaluation

## Current Implementation Status

[See implementation status](implementation.md)

## Usage

### CLI

You can use the `rh` cli. For help:
```
cargo run -p rh -- fhirpath --help
```

There is also an interactive REPL:
```
cargo run -p rh -- fhirpath repl
```

### Rust

```rust
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
use serde_json::json;

let patient = json!({
    "resourceType": "Patient",
    "name": [{"given": ["John"], "family": "Doe"}],
    "birthDate": "1974-12-25"
});

let parser = FhirPathParser::new();
let evaluator = FhirPathEvaluator::new();
let context = EvaluationContext::new(patient);

// Extract patient name
let expr = parser.parse("name.given").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// Date arithmetic
let expr = parser.parse("now() - 30 days").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();

// String operations
let expr = parser.parse("name.family.upper()").unwrap();
let result = evaluator.evaluate(&expr, &context).unwrap();
```

[See the examples directory](./examples/) for complete, runnable examples with detailed output and explanations.

### Unit Conversion

[See details about unit support and unit conversion](units.md)

### Error Handling

The system provides clear error messages for:
- **Incompatible units**: `Cannot add quantities with incompatible units: Some("kg") and Some("m")`
- **Division by zero**: `Division by zero`
- **Unknown units**: `Unknown unit: xyz`
- **Temperature conversion errors**: `Unknown temperature unit: xyz`
- **Mixed unit types**: `Cannot add quantities with incompatible units: Some("Cel") and Some("kg")`

See `examples/unit_conversion_example.rs` and `examples/temperature_conversion_example.rs` for comprehensive examples of all supported conversions and operations.

## Grammar Support

This implementation is based on the official FHIRPath grammar from:
https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4

The parser is implemented using the `nom` parser combinator library, providing a pure Rust implementation without requiring build-time code generation. The parser supports the complete core FHIRPath syntax including:

- Path expressions with proper precedence
- All literal value types
- Function invocations with parameters
- Boolean and arithmetic logic
- Comparison operations
- Union operations
- Collection and string manipulation

## Testing

The implementation includes comprehensive tests:

- **Unit tests**: tests covering parser and evaluator including temporal literals
- **Integration tests**: real-world usage examples including arithmetic, comparisons, membership, collection functions, filtering functions, string manipulation, math functions, array indexing, union operations, and temporal literals
- **Parser coverage**: All core syntax elements parse successfully including collection, filtering, string function calls, array indexing, union operations, and temporal literals
- **Evaluator coverage**: Literals (including temporal), member access, array indexing (including nested indexing), union operations (including mixed types and nested unions), arithmetic, comparison, membership, collection functions, filtering operations, and string manipulation
- **Edge case coverage**: Out-of-bounds indexing, empty collection handling, single-element array preservation, union with empty values, temporal literal parsing validation

### Run Tests

```bash
# Run all tests
cargo test --package fhirpath

# Run with output
cargo test --package fhirpath -- --nocapture
```

## Contributing

Contributions are welcome!

### Development Guidelines

Follow the workspace conventions:
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Write comprehensive tests for new features
- Update documentation for API changes
- Follow conventional commit message format

## References

- [FHIRPath Specification](http://hl7.org/fhirpath/)
- [FHIRPath Grammar](https://build.fhir.org/ig/HL7/FHIRPath/fhirpath.g4)
- [FHIR R4 Documentation](http://hl7.org/fhir/R4/)

## License

This project is licensed under the MIT OR Apache-2.0 license.
