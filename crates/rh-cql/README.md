# rh-cql

CQL (Clinical Quality Language) capabilities for the RH ecosystem.

## Overview

This crate provides:

- **ELM Types**: Complete Rust type definitions for ELM (Expression Logical Model)
- **ELM JSON Parsing**: Deserialize ELM JSON into strongly-typed Rust structures
- **CQL Execution** *(planned)*: Evaluate ELM expressions against FHIR data

## Status

🚧 **Under Development** - ELM type definitions are complete; execution engine is planned.

## Background

[CQL (Clinical Quality Language)](https://cql.hl7.org/) is an HL7 standard for expressing clinical logic. It's used extensively in:

- Clinical Decision Support (CDS)
- Electronic Clinical Quality Measures (eCQMs)
- FHIR Clinical Reasoning resources (Library, Measure, PlanDefinition)

CQL compiles to ELM (Expression Logical Model), a structured representation that can be executed against patient data.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  CQL Source │ ──▶ │  ELM (JSON) │ ──▶ │  Execution  │
│    (.cql)   │     │   or XML    │     │   Engine    │
└─────────────┘     └─────────────┘     └─────────────┘
      │                   │                   │
      └───────────────────┴───────────────────┘
                    rh-cql
```

## Features

### ✅ ELM Type Definitions (Complete)

Full Rust type definitions for ELM, supporting:
- **Library structure**: Identifier, usings, includes, parameters, statements
- **220+ expression types**: Literals, operators, queries, retrieves, clinical operations
- **Type specifiers**: Named, list, interval, tuple, choice types
- **Annotations**: CQL-to-ELM translator info, errors, and tags

### 🚧 Planned Features

- [ ] Core ELM expression evaluation
- [ ] FHIR data provider integration
- [ ] FHIRPath interop (via `rh-fhirpath`)
- [ ] CQL-to-ELM translation
- [ ] Terminology service integration

## Usage

### Parsing ELM JSON

```rust
use rh_cql::elm::{Library, Expression};

// Parse an ELM library from JSON
let json = r#"{
    "identifier": { "id": "MyLibrary", "version": "1.0.0" },
    "statements": {
        "def": [{
            "name": "IsAdult",
            "expression": {
                "type": "GreaterOrEqual",
                "operand": [
                    { "type": "CalculateAge", "operand": { "type": "Property", "path": "birthDate" } },
                    { "type": "Literal", "valueType": "{urn:hl7-org:elm-types:r1}Integer", "value": "18" }
                ]
            }
        }]
    }
}"#;

let library: Library = serde_json::from_str(json)?;
println!("Library: {:?}", library.identifier);
```

### Working with Expressions

```rust
use rh_cql::elm::{Expression, Literal, QName};

// Parse an expression
let expr: Expression = serde_json::from_str(r#"{"type": "Literal", "value": "42"}"#)?;

// Create expressions programmatically
let literal = Expression::Literal(Literal {
    value_type: Some(QName("{urn:hl7-org:elm-types:r1}Integer".into())),
    value: Some("42".into()),
    ..Default::default()
});

// Serialize back to JSON
let json = serde_json::to_string(&literal)?;
```

## Examples

Run the examples to see ELM parsing in action:

```bash
# Parse a complete ELM library
cargo run --example parse_elm

# Work with Query expressions
cargo run --example elm_query
```

## Related Crates

- [`rh-fhirpath`](../rh-fhirpath) - FHIRPath evaluation (CQL builds on FHIRPath)
- [`rh-validator`](../rh-validator) - FHIR validation
- [`rh-foundation`](../rh-foundation) - Core types and utilities

## References

- [CQL Specification](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
- [CQL-to-ELM Translator (Java)](https://github.com/cqframework/clinical_quality_language)
- [FHIR Clinical Reasoning](https://hl7.org/fhir/clinicalreasoning-module.html)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
