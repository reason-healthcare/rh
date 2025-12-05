# CQL-to-ELM Translator Implementation Plan

## Overview

Implementation plan for `rh-cql`, a Rust CQL-to-ELM translator library integrated with rh CLI, and targeting WASM.

## Phases

### Phase 1: Foundation (ELM Types & Model Info)
**Goal**: Define ELM output types and model resolution infrastructure

- [x] **1.1 ELM Type Definitions**
  - Define all ELM expression types with serde serialization
  - Library, ExpressionDef, all Expression variants
  - TypeSpecifier variants (Named, List, Interval, Tuple, Choice)
  - Implement JSON serialization matching ELM JSON schema

- [x] **1.2 ModelInfo Types**
  - Parse ModelInfo XML schema
  - Define ModelInfo, TypeInfo, ClassInfo structs
  - System model (primitives: Boolean, Integer, Decimal, String, Date, DateTime, Time, Quantity)

- [x] **1.3 Model Manager**
  - ModelInfoProvider trait (in rh-cql)
  - MemoryStore pattern in rh-foundation (reusable for packages, models, libraries)
  - Implement MemoryModelInfoProvider using foundation pattern
  - Bundle FHIR R4 model info (fhir_r4_model_info, fhir_r4_provider)
  - Type resolution by name (resolve_type, resolve_class)
  - *Note: Memory provider pattern in rh-foundation for WASM compatibility and reuse across crates*

- [x] **1.4 DataType System**
  - Internal DataType enum (System, Model, List, Interval, Tuple, Choice, TypeParameter)
  - SystemType enum with all CQL primitives (Any, Boolean, Integer, Long, Decimal, String, Date, DateTime, Time, Quantity, Ratio, Code, Concept, Vocabulary)
  - Type compatibility checking (is_subtype_of, can_convert_to)
  - Common type resolution (common_type)
  - Implicit conversion map (Integer→Long→Decimal, Date→DateTime, Code→Concept)

### Phase 2: CQL Parser
**Goal**: Parse CQL source into AST using `nom` (consistent with rh-fhirpath)

- [x] **2.1 Parser Foundation**
  - Use `nom` parser combinators (same as rh-fhirpath)
  - Translate CQL grammar (CQL version 1.5.3) from ANTLR4/spec to nom combinators
  - Leverage patterns established in rh-fhirpath parser
  - Created parser module structure (mod.rs, lexer.rs, span.rs, ast.rs)
  - Source location tracking with Span type implementing nom traits

- [x] **2.2 Lexer**
  - CQL keywords and operators (130+ keywords)
  - Literals (string, number, date/time, quantity)
  - Identifiers and comments (line //, block /* */)
  - Source location tracking (Span, SourceLocation, SourceRange)

- [x] **2.3 CQL AST Types**
  - Library header (library, using, include, codesystem, valueset, code, concept, parameter, context)
  - Expression definitions and function definitions
  - All expression types (70+ types)
  - Type specifiers (Named, List, Interval, Tuple, Choice)

- [x] **2.4 Parser Implementation**
  - Expression parser with full operator precedence (10 levels)
  - Statement parser for library structure and definitions
  - Library parser integrating all components
  - CqlParser with parse() and parse_expression() methods
  - Comprehensive tests and cql_parser example

### Phase 3: Library Resolution
**Goal**: Resolve library dependencies

- [x] **3.1 LibrarySourceProvider Trait**
  - LibraryIdentifier (name + version key)
  - LibrarySource (source code with metadata)
  - LibrarySourceProvider trait (get_source, has_library, list_libraries, find_by_name)
  - MemoryLibrarySourceProvider using MemoryStore
  - FileLibrarySourceProvider for filesystem (non-WASM)
  - CompositeLibrarySourceProvider for layered providers

- [x] **3.2 CompiledLibrary**
  - Wrapper for ELM Library with convenient lookup methods
  - Usings, includes, parameters, expressions lookup
  - Code systems, value sets, codes, concepts lookup
  - Expression lookup by name and context
  - Public/private filtering
  - DefinitionRef enum for type-safe definition access
  - FunctionRef placeholder for function overload resolution

- [x] **3.3 LibraryManager**
  - Library caching using MemoryStore
  - Dependency resolution with resolve() and resolve_all()
  - Circular dependency detection (detect_cycle)
  - Topological sort for compilation order
  - Dependency graph generation
  - LibraryError enum for error handling

### Phase 4: Semantic Analysis & Translation
**Goal**: Translate CQL AST to typed ELM

- [x] **4.1 Preprocessor**
  - Collect LibraryInfo from AST (name, version, identifier)
  - Extract definitions without type resolution (expressions, functions, parameters)
  - Extract model and library dependencies for resolution
  - Extract terminology definitions (code systems, value sets, codes, concepts)
  - Definition lookup and classification (DefinitionKind)
  - Access control tracking (public/private)
  - Trigger library resolution via library_dependency_ids()

- [x] **4.2 LibraryBuilder Core**
  - Symbol and Scope types for symbol table management
  - SymbolKind enum (expression, function, parameter, terminology, alias, let, operand, library, context)
  - FunctionSignature for overload resolution with operand types
  - Context management (Patient, Practitioner, etc.) - auto-defines context symbol
  - Nested scope stack with shadowing semantics
  - Model registration (using declarations) with URI mapping
  - Included library management with qualified lookup
  - Identifier resolution through scope chain (innermost to outermost)
  - Qualified identifier resolution (library.name, model.type, alias.property)
  - Function overload resolution by name
  - Error and warning collection (BuilderError enum)
  - Integration with Preprocessor via from_library_info()

- [x] **4.3 Type Resolution**
  - Resolve type specifiers to DataTypes
  - Type inference for expressions
  - TypeBuilder (DataType ↔ TypeSpecifier)

- [x] **4.4 Operator Resolution**
  - System operator signatures (arithmetic, comparison, logical, string, list, interval, datetime)
  - Overload resolution with scoring (exact match, conversion, subtype)
  - Implicit conversions (Integer→Long→Decimal, Date→DateTime, Code→Concept)
  - Generic operator support (List<T>, Interval<T> operations)
  - Type parameter binding and substitution for result types
  - OperatorResolver with resolve_unary, resolve_binary, resolve_ternary
  - 21 tests + operator_resolver example

- [x] **4.5 Expression Translation** ✅
  - a. Literals (24 tests)
  - b. References - ExpressionRef, FunctionRef, ParameterRef (45 tests)
  - c. Operators - unary, binary, nary (101 tests)
  - d. Queries - source, where, return, sort (118 tests)
  - e. Retrieve expressions (131 tests)
  - f. Conditionals - if, case (142 tests)
  - g. Type operators - as, is, convert (165 tests)

- [x] **4.6 Function Resolution** ✅
  - System functions (nullary, aggregate, list, nullological, arithmetic, string, interval, clinical, type conversion)
  - User-defined functions (FunctionRef, FunctionDef translation)
  - Fluent function syntax (value.function(args) → function(value, args))
  - 46 new tests (211 total translator tests) + function_resolver example

### Phase 5: System Operators
**Goal**: Implement all CQL system operators

- [ ] **5.1 Arithmetic**: Add, Subtract, Multiply, Divide, Modulo, Negate, Abs, Ceiling, Floor, Truncate, Round, Ln, Exp, Log, Power, etc.
- [ ] **5.2 Comparison**: Equal, Equivalent, Less, Greater, LessOrEqual, GreaterOrEqual
- [ ] **5.3 Logical**: And, Or, Not, Implies, Xor
- [ ] **5.4 String**: Concatenate, Length, Upper, Lower, Substring, StartsWith, EndsWith, Matches, ReplaceMatches, etc.
- [ ] **5.5 DateTime**: Date/Time construction, component extraction, arithmetic, comparison
- [ ] **5.6 Interval**: Contains, In, Includes, IncludedIn, Overlaps, Meets, Start, End, Width, etc.
- [ ] **5.7 List**: First, Last, IndexOf, Contains, Includes, Union, Intersect, Except, Flatten, Distinct, Sort, etc.
- [ ] **5.8 Aggregate**: Count, Sum, Min, Max, Avg, Median, Mode, StdDev, Variance, AllTrue, AnyTrue
- [ ] **5.9 Type**: As, Is, Convert, ToBoolean, ToInteger, ToDecimal, ToString, ToDate, ToDateTime, ToTime, ToQuantity
- [ ] **5.10 Null**: IsNull, IsTrue, IsFalse, Coalesce

### Phase 6: Compiler Options & Output
**Goal**: Complete translation with options

- [ ] **6.1 CompilerOptions**
  - Annotation options
  - Semantic options (list/interval promotion)
  - Error severity levels

- [ ] **6.2 Output Generation**
  - JSON ELM output
  - Optional: XML ELM output
  - Annotation/locator inclusion

- [ ] **6.3 Error Reporting**
  - Detailed error messages
  - Source locations
  - Warning levels

### Phase 7: WASM & Integration
**Goal**: WASM build and JavaScript API

- [ ] **7.1 WASM Build**
  - wasm-bindgen setup
  - Bundle size optimization
  - Feature flags for optional components

- [ ] **7.2 JavaScript API**
  - `translateCql(source, options)` → ELM JSON
  - `validateCql(source)` → errors/warnings
  - Library registration API

- [ ] **7.3 Integration**
  - Consider rh-fhirpath integration for expression evaluation
  - FHIR Library resource support

### Phase 8: Testing & Validation
**Goal**: Ensure compatibility with Java/Kotlin reference implementation and CQL specification

#### 8A. Official CQL Conformance Tests (https://cql.hl7.org/tests.html)

The CQL specification provides an official conformance test suite at **https://cql.hl7.org/tests.zip**.

**Format:**
- XML files with test cases (same format as FHIRPath spec tests)
- Each test defines: expression, expected result, optional context
- XSD schema included for validation
- Tests should run with UTC timestamp to avoid timezone issues

**Test Categories (from spec):**
| Category | Description |
|----------|-------------|
| Aggregate Functions | Count, Sum, Min, Max, Avg, etc. |
| Aggregate Operator | aggregate clause in queries |
| Arithmetic Functions | Abs, Ceiling, Floor, Ln, Log, etc. |
| Comparison Operators | =, !=, <, >, <=, >=, ~, !~ |
| Conditional Operators | If, Case |
| Date/Time Operators | Date/Time construction, arithmetic, components |
| Errors And Messaging Operators | Message function |
| Interval Operators | Contains, In, Overlaps, Meets, etc. |
| List Operators | First, Last, Union, Intersect, etc. |
| Logical Operators | And, Or, Not, Xor, Implies |
| Nullological Operators | IsNull, IsTrue, IsFalse, Coalesce |
| String Operators | Concat, Substring, Length, etc. |
| Type Operators | As, Is, Convert, ToInteger, etc. |
| Types | Type system tests |
| Value Literals and Selectors | Literal parsing, constructors |

**Implementation:**
```rust
// Download and cache official tests
const CQL_TESTS_URL: &str = "https://cql.hl7.org/tests.zip";

// Test case structure (from XSD)
struct CqlTestCase {
    name: String,
    expression: String,
    expected: ExpectedResult,
    context: Option<String>,
    invalid: Option<bool>,  // Expects error
}

// Run against our translator + evaluator
fn run_conformance_test(test: &CqlTestCase) -> TestResult;
```

#### 8B. Reference Implementation Tests (cqframework/clinical_quality_language)

**CQL Input Files:**
```
Src/java/cql-to-elm/src/jvmTest/resources/org/cqframework/cql/cql2elm/
├── OperatorTests/           # Operator-specific tests
│   ├── ArithmeticOperators.cql
│   ├── ComparisonOperators.cql
│   ├── ListOperators.cql
│   ├── IntervalOperators.cql
│   ├── DateTimeOperators.cql
│   ├── StringOperators.cql
│   ├── LogicalOperators.cql
│   ├── TypeOperators.cql
│   ├── AggregateOperators.cql
│   └── NullologicalOperators.cql
├── LibraryTests/            # Library resolution tests
├── SignatureTests/          # Function signature tests
├── fhir/r4/                 # FHIR R4 specific tests
└── ErrorTests/              # Expected error cases
    ├── ParameterTestInvalid.cql    (17 expected errors)
    └── InvalidEquality.cql         (1 expected error)
```

**Expected ELM Output (Golden Files):**
```
Src/java/elm/src/jvmTest/resources/org/cqframework/cql/elm/
├── serializing/
│   ├── CMS146v2_Expected_SignatureLevel_None.json
│   ├── CMS146v2_Expected_SignatureLevel_All.json
│   └── ... (multiple signature levels)
└── ElmDeserialize/
    ├── fhir/*.json
    └── qdm/*.json
```

**Engine Test Manifests (XML):**
```
Src/java/engine-fhir/src/test/resources/org/hl7/fhirpath/cql/
├── CqlArithmeticFunctionsTest.xml
├── CqlComparisonOperatorsTest.xml
├── CqlListOperatorsTest.xml
├── CqlIntervalOperatorsTest.xml
├── CqlDateTimeOperatorsTest.xml
└── ... (per-operator test definitions with expected values)
```

#### Test Implementation Tasks

- [ ] **8.1 Test Infrastructure**
  - Download and cache official CQL spec tests (tests.zip)
  - Download and cache reference implementation tests
  - XML test manifest parser
  - ELM JSON comparison utilities (structural equality, ignoring order)

- [ ] **8.2 CQL Spec Conformance Tests**
  - Parse tests.zip XML format
  - Run each category systematically
  - Track pass rate per category
  - **Target: 100% conformance with spec tests**

- [ ] **8.3 Parser Tests**
  - Test each grammar construct against OperatorTests/*.cql
  - Verify AST structure matches expected patterns
  - Error recovery and location tracking tests

- [ ] **8.4 Translation Tests (Golden File Comparison)**
  - Translate CQL → ELM JSON
  - Compare against `CMS146v2_Expected_SignatureLevel_*.json`
  - Test with different CompilerOptions (SignatureLevel, annotations, etc.)
  - Track agreement percentage (like rh-validator)

- [ ] **8.5 Error Case Tests**
  - `ParameterTestInvalid.cql` → expect 17 errors
  - `InvalidEquality.cql` → expect 1 error
  - Verify error messages match reference implementation

- [ ] **8.6 Library Resolution Tests**
  - Multi-library compilation
  - Include/reference resolution
  - Circular dependency detection

- [ ] **8.7 Real-World CQL**
  - eCQM measures (CMS146, CMS149, etc.)
  - CDS Hooks libraries
  - WHO SMART Guidelines

- [ ] **8.8 Test Runner Infrastructure**
  - Create `tests/cql_tests/` module (similar to rh-validator's fhir_test_cases)
  - Implement test fixture downloader for:
    - CQL spec tests (https://cql.hl7.org/tests.zip)
    - Reference implementation tests (cqframework GitHub)
  - XML test manifest parser (reuse FHIRPath test format knowledge)
  - Test result comparison utilities
  - Summary reporter with pass/fail counts by category
  - Feature flag: `cql-tests` to enable conformance tests

- [ ] **8.9 Benchmarking**
  - Use `criterion` for benchmarks (consistent with rh-validator)
  - Create `benches/` directory with:
    - `translation_bench.rs` - CQL-to-ELM translation performance
    - `evaluation_bench.rs` - CQL/ELM evaluation performance
  - Benchmark scenarios:
    - Simple expression translation (literals, operators)
    - Complex query translation (from/where/return)
    - Full library translation (CMS146, etc.)
    - Expression evaluation against FHIR data
    - Batch evaluation (multiple patients)
  - Track metrics: throughput (ops/sec), latency (p50/p95/p99)
  - Compare WASM vs native performance

#### Test Runner Implementation
```rust
// tests/cql_tests/mod.rs
#[cfg(feature = "cql-tests")]
pub mod cql_tests {
    mod downloader;    // Download and cache test fixtures
    mod parser;        // Parse XML test manifests
    mod runner;        // Execute tests and collect results
    mod reporter;      // Summary reporting
    
    // Download CQL spec tests (tests.zip) on first run
    // Download reference implementation tests
    // Compare translation output
    // Track conformance percentage by category
    // Report disagreements
}

// Example test
#[test]
#[cfg(feature = "cql-tests")]
fn test_cql_spec_conformance() {
    let runner = CqlTestRunner::new();
    let results = runner.run_spec_tests();
    runner.print_summary(&results);
    assert!(results.pass_rate() > 0.95, "Conformance below 95%");
}
```

#### Benchmark Setup
```rust
// benches/translation_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn translation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cql_translation");
    
    // Simple expression
    group.bench_function("simple_arithmetic", |b| {
        b.iter(|| translate("1 + 2 * 3"))
    });
    
    // Complex query
    group.bench_function("fhir_query", |b| {
        b.iter(|| translate(PATIENT_QUERY_CQL))
    });
    
    // Full library (CMS146)
    group.bench_function("cms146_library", |b| {
        b.iter(|| translate(CMS146_CQL))
    });
    
    group.finish();
}

fn evaluation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cql_evaluation");
    
    // Pre-translated ELM
    let elm = translate(PATIENT_QUERY_CQL);
    let patient_data = load_test_patient();
    
    group.bench_function("evaluate_patient_query", |b| {
        b.iter(|| evaluate(&elm, &patient_data))
    });
    
    // Batch evaluation
    let patients = load_test_patients(100);
    group.bench_function("batch_100_patients", |b| {
        b.iter(|| evaluate_batch(&elm, &patients))
    });
    
    group.finish();
}

criterion_group!(benches, translation_benchmarks, evaluation_benchmarks);
criterion_main!(benches);
```

#### Conformance Tracking

**CQL Spec Tests (https://cql.hl7.org/tests.zip):**
| Category | Status | Pass/Total |
|----------|--------|------------|
| Aggregate Functions | ⬜ | 0/? |
| Aggregate Operator | ⬜ | 0/? |
| Arithmetic Functions | ⬜ | 0/? |
| Comparison Operators | ⬜ | 0/? |
| Conditional Operators | ⬜ | 0/? |
| Date/Time Operators | ⬜ | 0/? |
| Errors And Messaging | ⬜ | 0/? |
| Interval Operators | ⬜ | 0/? |
| List Operators | ⬜ | 0/? |
| Logical Operators | ⬜ | 0/? |
| Nullological Operators | ⬜ | 0/? |
| String Operators | ⬜ | 0/? |
| Type Operators | ⬜ | 0/? |
| Types | ⬜ | 0/? |
| Value Literals | ⬜ | 0/? |
| **TOTAL** | ⬜ | **0/?** |

**Reference Implementation Alignment:**
| Category | Status | Pass/Total |
|----------|--------|------------|
| OperatorTests/*.cql | ⬜ | 0/? |
| ELM Golden Files | ⬜ | 0/? |
| Library Resolution | ⬜ | 0/? |
| Error Cases | ⬜ | 0/? |
| Real-World eCQMs | ⬜ | 0/? |

## Dependencies

```toml
[dependencies]
# Workspace crates - reuse existing functionality
rh-fhirpath = { path = "../rh-fhirpath" }   # FHIRPath parsing & evaluation
rh-foundation = { path = "../rh-foundation" } # Package loading, HTTP, config, MemoryProvider pattern

# Parser and serialization
nom = { workspace = true }        # Parser combinators (same as rh-fhirpath)
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
indexmap = { workspace = true }   # Ordered maps for ELM
chrono = { workspace = true }     # Date/time handling
tokio = { workspace = true }      # Async runtime for library loading
tracing = { workspace = true }    # Logging/tracing

[dev-dependencies]
criterion = "0.5"                 # Benchmarking
tempfile = "3.0"                  # Test fixtures
tokio = { workspace = true, features = ["rt-multi-thread", "macros"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"

[features]
default = []
cql-tests = []                    # Enable conformance test suite

[[bench]]
name = "translation_bench"
harness = false

[[bench]]
name = "evaluation_bench"
harness = false
```

## Milestones

| Milestone | Description | Target |
|-----------|-------------|--------|
| M1 | ELM types + JSON serialization | Week 1-2 |
| M2 | CQL parser (basic expressions) | Week 3-4 |
| M3 | Simple CQL → ELM translation | Week 5-6 |
| M4 | Library resolution | Week 7 |
| M5 | Full operator support | Week 8-10 |
| M6 | Test runner + benchmarks | Week 11 |
| M7 | WASM build | Week 12 |
| M8 | Production ready | Week 13+ |

## Open Questions (All Resolved)

1. **Parser choice**: ✅ RESOLVED - Use `nom`
   - Consistent with rh-fhirpath implementation
   - Proven WASM compatibility in this codebase
   - No build step required, team familiarity

2. **Model bundling**: ✅ RESOLVED - Use existing FHIR ModelInfo
   - Use the standard FHIR ModelInfo from [CQF Common IG](https://www.fhir.org/guides/cqf/common/4.0.1/artifacts.html)
   - Key artifacts: `FHIR-ModelInfo`, `FHIRHelpers`, `FHIRCommon`
   - ModelInfo is now published in [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/using-modelinfo.html)
   - Canonical reference: `http://hl7.org/fhir/uv/cql/Library/FHIR-ModelInfo|4.0.1`
   - Support derived ModelInfo for IG-specific profiles (US Core, QI-Core, etc.)
   - Load ModelInfo from FHIR packages via rh-foundation PackageLoader

3. **FHIRPath integration**: ✅ RESOLVED - Reuse rh-fhirpath
   - CQL path expressions are FHIRPath-based
   - Reuse `rh-fhirpath` crate directly for:
     - Path expression parsing (`FhirPathParser`)
     - Path evaluation (`FhirPathEvaluator`, `EvaluationContext`)
     - AST types (`Expression`, `Invocation`, etc.)
   - CQL-specific operators extend FHIRPath in the translator
   - Shared WASM build infrastructure

4. **Async support**: ✅ RESOLVED - Use rh-foundation patterns
   - Follow patterns from [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/index.html)
   - Reuse `rh-foundation::loader::PackageLoader` for library/model loading
   - PackageLoader already handles:
     - Async FHIR package download from npm registries
     - Authentication (bearer tokens)
     - Tarball extraction and caching
     - Configurable timeout/retry
   - Library resolution via FHIR Library resources in packages
   - ModelInfo loading from `Library/<model>-ModelInfo` convention

## References

### CQL/ELM Specifications
- [CQL Specification](https://cql.hl7.org/)
- [ELM Specification](https://cql.hl7.org/elm.html)
- [CQL Grammar](https://github.com/cqframework/clinical_quality_language/blob/master/Src/grammar/cql.g4)
- [CQL Conformance Tests](https://cql.hl7.org/tests.html)

### Reference Implementation
- [Java Translator](https://github.com/cqframework/clinical_quality_language/tree/master/Src/java/cql-to-elm)
- [ELM JSON Schema](https://github.com/cqframework/clinical_quality_language/tree/master/Src/java/elm/schema)

### FHIR Integration
- [Using CQL with FHIR IG](https://build.fhir.org/ig/HL7/cql-ig/en/index.html) - Canonical patterns for CQL+FHIR
- [Using ModelInfo](https://build.fhir.org/ig/HL7/cql-ig/en/using-modelinfo.html) - ModelInfo conformance requirements
- [CQF Common IG](https://www.fhir.org/guides/cqf/common/4.0.1/artifacts.html) - FHIR ModelInfo, FHIRHelpers, FHIRCommon
- [FHIR-ModelInfo](https://build.fhir.org/ig/HL7/cql-ig/en/Library-FHIR-ModelInfo.html) - Canonical FHIR R4 ModelInfo
- [FHIRHelpers](https://build.fhir.org/ig/HL7/cql-ig/en/Library-FHIRHelpers.html) - Type conversion functions

### Workspace Crates (Reuse)
- `rh-fhirpath` - FHIRPath parsing and evaluation
- `rh-foundation` - Package loading, HTTP, configuration, MemoryProvider pattern (WASM-compatible in-memory storage)
