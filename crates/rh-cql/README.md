# rh-cql

CQL (Clinical Quality Language) capabilities for the RH ecosystem.

## Overview

This crate provides:

- **Three-Stage CQL Compiler**: Parse → Semantic Analysis (Typed AST) → ELM Emission
- **ELM Types**: Complete Rust type definitions for ELM (Expression Logical Model)
- **ELM JSON Output**: Serialize ELM to JSON (compatible with reference implementation)
- **Source Maps**: CQL source span → ELM node correlation via `compile_to_elm_with_sourcemap`
- **Explain Output**: Human-readable parse tree and compilation details (`explain_parse`, `explain_compile`)
- **ELM Evaluator**: Pure Rust evaluation engine (`evaluate_elm`, `evaluate_elm_with_trace`)
- **ModelInfo Types**: FHIR and custom model definitions for type resolution
- **DataType System**: Type checking, compatibility, and implicit conversions

## Status

✅ **Core Complete** — Three-stage pipeline (Parse → Semantic Analysis → ELM Emit) implemented.
Source maps, explain mode, and ELM evaluation engine are all functional.

## Quick Start

```rust
use rh_cql::{compile, CompilerOptions};

let source = r#"
    library Example version '1.0.0'
    define Greeting: 'Hello, CQL!'
    define Answer: 42
"#;

let result = compile(source, None).unwrap();
assert!(result.is_success());

// Get the ELM as JSON
let json = result.to_json().unwrap();
println!("{}", json);
```

## Background

[CQL (Clinical Quality Language)](https://cql.hl7.org/) is an HL7 standard for expressing clinical logic. It's used extensively in:

- Clinical Decision Support (CDS)
- Electronic Clinical Quality Measures (eCQMs)
- FHIR Clinical Reasoning resources (Library, Measure, PlanDefinition)

CQL compiles to ELM (Expression Logical Model), a structured representation that can be executed against patient data.

## Architecture

The compiler follows a strict three-stage pipeline:

```
┌─────────────┐    ┌─────────────────┐    ┌─────────────┐    ┌─────────────┐
│  CQL Source │ ──▶│  SemanticAnalyzer│──▶ │ ElmEmitter  │──▶ │  ELM JSON   │
│   parser/   │    │  semantics/      │    │  emit/      │    │ (+ SourceMap│
└─────────────┘    └─────────────────┘    └─────────────┘    │  optional)  │
                                                              └─────────────┘
                                                                     │
                                                              ┌──────▼──────┐
                                                              │  eval/      │
                                                              │ evaluate_elm│
                                                              └─────────────┘
```

See [ARCHITECTURE.md](./ARCHITECTURE.md) for full details.

## Public API

### Compiling CQL

The main entry point is the `compile()` function:

```rust
use rh_cql::{compile, CompilerOptions};

let source = r#"
    library MyLibrary version '1.0'
    using FHIR version '4.0.1'
    context Patient
    define IsAdult: AgeInYears() >= 18
"#;

// Compile with default options
let result = compile(source, None).unwrap();

if result.is_success() {
    println!("Compiled successfully!");
    
    // Access the ELM library
    let library = &result.library;
    if let Some(id) = &library.identifier {
        println!("Library: {} v{}", 
            id.id.as_deref().unwrap_or("?"),
            id.version.as_deref().unwrap_or("?"));
    }
} else {
    // Handle errors
    for err in &result.errors {
        println!("Error: {}", err.message());
    }
}
```

### Compiler Options

Customize compilation behavior:

```rust
use rh_cql::{compile, CompilerOptions, CompilerOption, SignatureLevel};

// Debug mode: includes annotations, locators, and result types
let options = CompilerOptions::debug();

// Strict mode: disables implicit conversions
let options = CompilerOptions::strict();

// Custom options
let options = CompilerOptions::new()
    .with_option(CompilerOption::EnableAnnotations)
    .with_option(CompilerOption::EnableLocators)
    .with_signature_level(SignatureLevel::All);

let result = compile(source, Some(options)).unwrap();
```

### Direct to JSON

Compile and serialize in one step:

```rust
use rh_cql::compile_to_json;

let source = "library Test version '1.0' define X: 42";

// Pretty-printed JSON
let json = compile_to_json(source, None, true).unwrap();

// Compact JSON
let compact = compile_to_json(source, None, false).unwrap();
```

### Validation Only

Check CQL syntax without generating full ELM:

```rust
use rh_cql::validate;

let source = "library Test version '1.0' define X: 42";
let result = validate(source, None).unwrap();

if result.is_valid() {
    println!("CQL is valid!");
} else {
    for err in &result.errors {
        println!("Error: {}", err.message());
    }
}
```

### Source Maps

Get a correlated CQL→ELM source map alongside the compiled output:

```rust
use rh_cql::compile_to_elm_with_sourcemap;

let source = "library Test version '1.0' define X: 1 + 2";
let result = compile_to_elm_with_sourcemap(source, None, None).unwrap();

if result.is_success() {
    let library = &result.library;
    let sm_json = result.source_map_json().unwrap();
    println!("{sm_json}");
}
```

### Explaining CQL

Human-readable parse tree and compilation details:

```rust
use rh_cql::{explain_parse, explain_compile};

let source = "library Test version '1.0' define X: 1 + 2";

// Parse tree
let parse_explanation = explain_parse(source).unwrap();
println!("{parse_explanation}");

// Semantic analysis details (resolved types, overloads, conversions)
let compile_explanation = explain_compile(source, None).unwrap();
println!("{compile_explanation}");
```

### Evaluating ELM

```rust
use rh_cql::{
    compile, evaluate_elm, evaluate_elm_with_trace,
    EvalContextBuilder, FixedClock, CqlDateTime,
};

let source = "library Test version '1.0' define X: 1 + 2";
let result = compile(source, None).unwrap();
assert!(result.is_success());

let clock = FixedClock::new(CqlDateTime {
    year: 2025, month: Some(1), day: Some(1),
    hour: Some(0), minute: Some(0), second: Some(0),
    millisecond: None, offset_seconds: Some(0),
});
let ctx = EvalContextBuilder::new(clock).build();

// Evaluate a named expression
let value = evaluate_elm(&result.library, "X", &ctx).unwrap();
println!("X = {value}"); // X = 3

// Evaluate with step-by-step trace
let (value, trace) = evaluate_elm_with_trace(&result.library, "X", &ctx).unwrap();
for event in &trace {
    println!("  op={} inputs={:?} output={}", event.op, event.inputs, event.output);
}
```

### Working with CompilationResult

```rust
use rh_cql::compile;

let result = compile(source, None).unwrap();

// Check status
result.is_success();      // true if no errors
result.has_warnings();    // true if warnings present
result.issue_count();     // total errors + warnings + messages

// Access issues
result.errors;            // Vec<CqlCompilerException>
result.warnings;          // Vec<CqlCompilerException>  
result.messages;          // Vec<CqlCompilerException>

// Get JSON output
let json = result.to_json().unwrap();           // Pretty-printed
let compact = result.to_compact_json().unwrap(); // Minified
```

## Features

### ✅ CQL-to-ELM Compiler (Three-Stage Pipeline)

- **CQL Parser**: pest PEG grammar, full CQL expression set
- **Semantic Analyzer**: Type checking, symbol resolution, operator overloading, implicit conversions
- **ELM Emitter**: Modular emit submodules (<500 lines each)
- **Source Maps**: CQL span → ELM node correlation (`compile_to_elm_with_sourcemap`)
- **JSON Output**: Serialize to JSON compatible with reference implementation

#### Design Decision: FHIRHelpers-Agnostic Translation

Unlike the Java reference translator, **rh-cql does not automatically insert FHIRHelpers conversion calls** during translation. This is an intentional design choice:

| Aspect | Reference Translator | rh-cql |
|--------|---------------------|--------|
| FHIR→CQL type conversions | Inserts `FHIRHelpers.ToString`, `ToConcept`, etc. | Leaves types as-is |
| CodeableConcept ~ Code | Wraps both in `ToConcept` calls | Direct comparison |
| Translator complexity | Must load FHIRHelpers definitions | Simpler, no external dependencies |
| Runtime responsibility | Minimal type handling | Must handle FHIR↔CQL coercion |

**Example - Comparing CodeableConcept to Code:**

CQL:
```cql
C.clinicalStatus ~ "Active Condition Code"
```

Reference translator output:
```json
{
  "type": "Equivalent",
  "operand": [
    {
      "name": "ToConcept",
      "libraryName": "FHIRHelpers",
      "type": "FunctionRef",
      "operand": [{ "path": "clinicalStatus", "scope": "C", "type": "Property" }]
    },
    {
      "type": "ToConcept",
      "operand": { "name": "Active Condition Code", "type": "CodeRef" }
    }
  ]
}
```

rh-cql output:
```json
{
  "type": "Equivalent",
  "operand": [
    { "path": "clinicalStatus", "scope": "C", "type": "Property" },
    { "name": "Active Condition Code", "type": "CodeRef" }
  ]
}
```

**Why this approach?**

1. **Separation of Concerns**: Translation converts syntax to structure; type coercion is a runtime concern
2. **Model Independence**: The translator can work with any data model, not just FHIR
3. **Runtime Flexibility**: Executors can implement FHIR type handling natively (potentially more efficiently)
4. **Simpler Translator**: No need to load/parse FHIRHelpers library during translation

The ELM output is valid per the ELM specification—the runtime/executor must understand how to evaluate equivalence between FHIR and CQL types.

### ✅ Explain Mode

- `explain_parse(source)` — AST summary with node types and source locations
- `explain_compile(source, options)` — Compilation narrative (resolved types, overloads, implicit conversions)
- `explain_eval(elm, trace)` — Evaluation trace explanation

### ✅ ELM Evaluation Engine

- `evaluate_elm(library, name, ctx)` — Evaluate a named expression
- `evaluate_elm_with_trace(library, name, ctx)` — Evaluation with `TraceEvent` trace
- Arithmetic, comparison, string, datetime, type-conversion operators
- Interval, list, and query evaluation
- Three-valued logic (null-safe And, Or, Not, etc.)
- Deterministic clock injection via `FixedClock` / `EvalContextBuilder`

### ✅ ELM Type Definitions

Full Rust type definitions for ELM, supporting:
- **Library structure**: Identifier, usings, includes, parameters, statements
- **220+ expression types**: Literals, operators, queries, retrieves, clinical operations
- **Type specifiers**: Named, list, interval, tuple, choice types
- **Annotations**: CQL-to-ELM translator info, errors, and tags

### ✅ ModelInfo & Type System

- **ModelInfo types**: Data model definitions (FHIR, QDM, custom)
- **Model providers**: In-memory model storage (WASM-compatible)
- **Built-in FHIR R4**: Pre-loaded model with core resource types
- **DataType system**: Type checking, subtyping, implicit conversions

### � Planned / In Progress

- [ ] WASM build for browser/Node.js usage
- [ ] Full CQL conformance parity (Phase F)
- [ ] Retrieve expression full model-driven translation
- [ ] Incremental compilation and caching

## Additional APIs

### Parsing ELM JSON

```rust
use rh_cql::elm::Library;

let json = r#"{"identifier": {"id": "MyLib", "version": "1.0"}}"#;
let library: Library = serde_json::from_str(json)?;
```

### Using ModelInfo Providers

```rust
use rh_cql::{fhir_r4_provider, ModelInfoProvider};

let provider = fhir_r4_provider();

if let Some(patient) = provider.resolve_class("FHIR", Some("4.0.1"), "Patient") {
    println!("Patient has {} elements", patient.element.len());
}
```

### DataType System

```rust
use rh_cql::datatype::{DataType, SystemType};

let int_type = DataType::integer();
let decimal_type = DataType::decimal();

// Check type compatibility
assert!(int_type.is_subtype_of(&decimal_type));  // Integer <: Decimal
```

## Examples

Run the examples to explore the API:

```bash
# Compile CQL to ELM (main public API)
cargo run --example compile_cql

# Parse existing ELM JSON
cargo run --example parse_elm

# Work with Query expressions  
cargo run --example elm_query

# ModelInfo providers and type resolution
cargo run --example model_provider

# DataType system and type checking
cargo run --example datatype_system

# Compiler options
cargo run --example compiler_options

# Library management
cargo run --example library_manager
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
