# rh-cql

CQL (Clinical Quality Language) capabilities for the RH ecosystem.

## Overview

This crate provides:

- **ELM Types**: Complete Rust type definitions for ELM (Expression Logical Model)
- **ELM JSON Parsing**: Deserialize ELM JSON into strongly-typed Rust structures
- **ModelInfo Types**: FHIR and custom model definitions for type resolution
- **Model Providers**: In-memory model management (WASM-compatible)
- **DataType System**: Type checking, compatibility, and implicit conversions
- **CQL Execution** *(planned)*: Evaluate ELM expressions against FHIR data

## Status

🚧 **Under Development** - Phase 1 (Foundation) complete; parser and execution planned.

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

### ✅ Phase 1: Foundation (Complete)

#### ELM Type Definitions
Full Rust type definitions for ELM, supporting:
- **Library structure**: Identifier, usings, includes, parameters, statements
- **220+ expression types**: Literals, operators, queries, retrieves, clinical operations
- **Type specifiers**: Named, list, interval, tuple, choice types
- **Annotations**: CQL-to-ELM translator info, errors, and tags

#### ModelInfo Types
- **ModelInfo**: Data model definitions (FHIR, QDM, custom models)
- **TypeInfo**: Simple types, classes, profiles, intervals, lists, tuples, choices
- **ClassInfo**: Elements, primary code paths, retrievability

#### Model Provider System
- **ModelInfoProvider trait**: Type resolution interface
- **MemoryModelInfoProvider**: In-memory model storage (WASM-compatible)
- **Built-in FHIR R4**: Pre-loaded model with core resource types
- **Statistics tracking**: Cache hit/miss metrics

#### DataType System
- **SystemType**: All CQL primitives (Boolean, Integer, Long, Decimal, String, Date, DateTime, Time, Quantity, Code, Concept, etc.)
- **DataType**: Model types, List, Interval, Tuple, Choice, type parameters
- **Subtype checking**: Integer <: Long <: Decimal, Date <: DateTime
- **Implicit conversions**: With conversion function mapping

### 🚧 Planned Features

- [ ] Phase 2: CQL Parser (nom-based)
- [ ] Phase 3: Library Resolution
- [ ] Phase 4: Semantic Analysis & Translation
- [ ] Phase 5+: Execution Engine

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

### Using ModelInfo Providers

```rust
use rh_cql::{fhir_r4_provider, ModelInfoProvider};

// Get the built-in FHIR R4 provider
let provider = fhir_r4_provider();

// Resolve types
if let Some(patient) = provider.resolve_class("FHIR", Some("4.0.1"), "Patient") {
    println!("Patient has {} elements", patient.element.len());
    println!("Primary code path: {:?}", patient.primary_code_path);
}

// Get patient class via helper
if let Some(patient) = provider.get_patient_class("FHIR", Some("4.0.1")) {
    println!("Found patient class: {:?}", patient.name);
}
```

### DataType System

```rust
use rh_cql::datatype::{DataType, SystemType};

// Create types
let int_type = DataType::integer();
let decimal_type = DataType::decimal();
let list_of_patients = DataType::list(DataType::model("FHIR", "Patient"));

// Check type compatibility
assert!(int_type.is_subtype_of(&decimal_type));  // Integer <: Decimal
assert!(int_type.can_convert_to(&decimal_type)); // Implicit conversion available

// Find common type
let common = int_type.common_type(&decimal_type); // -> Decimal
```

## Examples

Run the examples to explore the API:

```bash
# Parse a complete ELM library
cargo run --example parse_elm

# Work with Query expressions
cargo run --example elm_query

# ModelInfo providers and type resolution
cargo run --example model_provider

# DataType system and type checking
cargo run --example datatype_system
```

## Related Crates

- [`rh-foundation`](../rh-foundation) - Core types (MemoryStore for WASM-compatible caching)
- [`rh-fhirpath`](../rh-fhirpath) - FHIRPath evaluation (CQL builds on FHIRPath)
- [`rh-validator`](../rh-validator) - FHIR validation

## References

- [CQL Specification](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
- [CQL-to-ELM Translator (Java)](https://github.com/cqframework/clinical_quality_language)
- [FHIR Clinical Reasoning](https://hl7.org/fhir/clinicalreasoning-module.html)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
