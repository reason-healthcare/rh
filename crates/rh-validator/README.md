# FHIR Validator

A high-performance FHIR (Fast Healthcare Interoperability Resources) validation library written in Rust. Validates FHIR R4 resources against StructureDefinition profiles with comprehensive support for cardinality, types, FHIRPath invariants, ValueSet bindings, and extensions.

**Version:** 0.2.0  
**Status:** Production Ready (Phases 0-10 Complete)

## Features

### ✅ Implemented (v0.2.0)

#### Core Validation (Phases 0-8)
- **Profile-based validation** - US Core, IPS, QI-Core, custom profiles
- **Hybrid architecture** - Leverages proven `rh-snapshot` library (100% FHIR compliant)
- **Cardinality checking** - Min/max occurrence validation with detailed paths
- **Type validation** - FHIR primitives and complex types
- **FHIRPath invariants** - Full FHIRPath 2.0 support with context tracking
- **ValueSet binding** - Required, extensible, preferred strength support
- **Extension validation** - Standard and complex extension structures
- **Slicing validation** - Discriminator-based slice matching
- **Auto-detection** - Profiles from `meta.profile` or resource type

#### CLI Integration (Phase 9)
- **Command-line tool** - `rh validate resource` and `rh validate batch`
- **Multiple formats** - text, json, operationoutcome
- **Batch processing** - NDJSON support for large datasets
- **14 integration tests** - Comprehensive CLI test coverage

#### OperationOutcome Output (Phase 10)
- **FHIR R4 compliance** - Standard OperationOutcome resource format
- **21 IssueType codes** - Full FHIR IssueType ValueSet mapping
- **Severity levels** - Error, warning, information
- **NDJSON batch output** - One OperationOutcome per resource
- **17 tests** - Complete OperationOutcome test coverage

#### Performance
- **LRU caching** - Profile snapshots and compiled rules
- **100x faster** - Cached validations ~1-5ms vs ~50-100ms initial
- **>99% cache hit rate** - Typical workload performance
- **Batch optimization** - Efficient multi-resource validation

### 📋 Test Coverage

- **Total tests:** 1,287 passing
- **Validator tests:** 132 (phases 0-10)
- **CLI tests:** 21 (resource and batch validation)
- **OperationOutcome tests:** 17 (unit and integration)
- **Zero failures** - All tests passing

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rh-validator = "0.2"
```

Basic usage:

```rust
use rh_validator::FhirValidator;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    // Create validator
    let validator = FhirValidator::new(None)?;

    // Validate a resource
    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{"family": "Doe", "given": ["John"]}],
        "gender": "male"
    });

    let result = validator.validate(&patient)?;

    if result.is_valid() {
        println!("✓ Resource is valid");
    } else {
        for issue in result.issues() {
            println!("✗ {}: {}", issue.severity, issue.message);
        }
    }

    Ok(())
}
```

## Usage

### Profile Validation

```rust
use rh_validator::FhirValidator;
use serde_json::json;

// Load validator with US Core profiles
let validator = FhirValidator::new(Some("~/.fhir/packages"))?;

// Validate against US Core Patient profile
let patient = json!({
    "resourceType": "Patient",
    "meta": {
        "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
    },
    "identifier": [{"system": "http://example.org", "value": "123"}],
    "name": [{"family": "Doe", "given": ["John"]}],
    "gender": "male"
});

let result = validator.validate(&patient)?;
```

### Explicit Profile URL

```rust
// Validate against specific profile
let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
let result = validator.validate_with_profile(&patient, profile_url)?;
```

### Batch Validation

```rust
// Validate multiple resources efficiently
let resources = vec![patient1, patient2, observation1];

for resource in resources {
    let result = validator.validate(&resource)?;
    if !result.is_valid() {
        for issue in result.issues() {
            eprintln!("{}: {}", issue.severity, issue.message);
        }
    }
}

// Check cache performance
let (p_hits, p_misses, p_rate, r_hits, r_misses, r_rate) = validator.cache_metrics();
println!("Cache hit rate: {:.1}%", p_rate * 100.0);
```

### OperationOutcome Output

```rust
// Convert to FHIR OperationOutcome
let result = validator.validate(&patient)?;
let operation_outcome = result.to_operation_outcome();

// Serialize to JSON
let json = serde_json::to_string_pretty(&operation_outcome)?;
println!("{}", json);

// Output:
// {
//   "resourceType": "OperationOutcome",
//   "issue": [
//     {
//       "severity": "error",
//       "code": "required",
//       "diagnostics": "Missing required field 'identifier'",
//       "location": ["Patient.identifier"],
//       "expression": ["Patient.identifier"]
//     }
//   ]
// }
```

## Examples

The crate includes 5 comprehensive examples in the [`examples/`](examples/) directory:

- **`basic_validation.rs`** - Simple validation with minimal Patient resources
- **`profile_validation.rs`** - US Core Patient profile validation
- **`batch_validation.rs`** - Efficient multi-resource validation with caching
- **`operation_outcome.rs`** - FHIR OperationOutcome output formats
- **`custom_workflow.rs`** - Custom validation workflows with filtering and reporting

Run examples:

```bash
cargo run --example basic_validation
cargo run --example profile_validation
cargo run --example batch_validation
```
- **`patient_validation.rs`** - Complete Patient resource validation examples

Run any example with:
```bash
cargo run -p rh-validator --example basic_validation
```

Test all examples:
```bash
just test-examples
```

## FHIR Test Cases Integration

The validator includes integration with the official FHIR R4 test suite (~570 test cases) to ensure spec compliance. Test cases are automatically downloaded and cached on first run.

### Running FHIR Test Cases

**Quick test (5 cases):**
```bash
# Using cargo
cargo test --features fhir-test-cases -p rh-validator -- --nocapture

# Using just (from workspace root)
just test-fhir

# Using just (from validator directory)
cd crates/rh-validator
just test-fhir
```

**Extended test suites:**
```bash
# 50 test cases
just test-fhir-50

# 100 test cases
just test-fhir-100

# ALL R4 test cases (~570 tests, takes several minutes)
just test-fhir-all
```

**Filter by module:**
```bash
# Test specific modules
just test-fhir-module profile
just test-fhir-module general
just test-fhir-module questionnaire
```

### Test Output

Tests produce detailed results including:
- Individual test pass/fail status
- Error diagnostics for failed validations
- Summary statistics (pass rate, error categories)
- Comparison table vs other validators (Java RI, Hapi, .NET)

Example output:
```
╔════════════════════════════════════════════════════════════════════╗
║ FHIR Validation Test Suite - 5 Tests                              ║
╚════════════════════════════════════════════════════════════════════╝

✓ test-case-1: Patient resource validation passed
✗ test-case-2: Missing required identifier field
✓ test-case-3: Observation with valid coding
...

Summary: 4/5 passed (80.0%)
```

### Test Case Features

- **Auto-download:** Test cases downloaded from hl7.org on first run
- **Caching:** Downloaded files cached in `target/fhir-test-cases/`
- **570 R4 tests:** Comprehensive coverage of FHIR validation scenarios
- **Module filtering:** Run specific test modules (profile, general, etc.)
- **Configurable:** Adjust test count via `TestRunConfig`

### Implementation Status

- ✅ Test case download and parsing
- ✅ Test runner with batch execution
- ✅ Module filtering and configuration
- ✅ Detailed result reporting
- 🚧 CI integration (planned)
- 🚧 Performance benchmarking (planned)

See [TODO.md](TODO.md) Phase 12 for detailed progress.

## Command Line Interface

The validator integrates with the `rh` CLI tool:

```bash
# Validate JSON syntax from file
rh validate json --input patient.json

# Validate from stdin with JSON output
echo '{"resourceType": "Patient"}' | rh validate json --format json

# Validate multiple resources (NDJSON)
rh validate json --input bundle.ndjson --multiple

# Strict mode (exit with error on validation failure)
rh validate json --input data.json --strict

# Custom validation parameters
rh validate json --input deep.json --max-depth 20 --stats
```

### CLI Options

- `--input, -i`: Input file path (reads from stdin if not provided)
- `--format, -f`: Output format (`text` or `json`)
- `--multiple`: Process as NDJSON (multiple JSON documents)
- `--max-depth`: Maximum allowed nesting depth (default: 100)
- `--stats`: Show detailed statistics for valid JSON
- `--strict`: Exit with non-zero code on validation failure

## Validation Layers

### 1. Syntax Validation (✅ Implemented)

The first layer ensures the input is valid JSON:

- **JSON parsing** with detailed error messages
- **Structural integrity** checks (balanced brackets, proper escaping)
- **Schema validation** (empty keys, infinite numbers, excessive nesting)
- **Performance safeguards** (string length limits, depth limits)

Example errors:
- `JSON syntax error: expected ',' or ']' at line 5, column 12`
- `Schema validation error: Maximum nesting depth of 100 exceeded`

### 2. FHIR Resource Validation (🚧 Planned)

Validates that JSON represents a valid FHIR resource:

- **Resource type recognition** (Patient, Observation, etc.)
- **Required field validation** (resourceType, mandatory elements)
- **Data type conformance** (strings, integers, dates, codes)
- **FHIR version compatibility** (R4, R5 differences)
- **Reference format validation** (Resource/id, #fragment)

Expected validation capabilities:
```rust
// Future API
let fhir_validator = FhirValidator::new("4.0.1")?;
let result = fhir_validator.validate_resource(json_content)?;
```

### 3. Profile Validation (🚧 Planned)

Advanced conformance validation against StructureDefinitions:

- **Profile conformance** (US Core, IPA, custom profiles)
- **Cardinality enforcement** (0..1, 1..*, specific ranges)
- **Terminology binding** validation with terminology servers
- **FHIRPath constraint** evaluation (invariants)
- **Extension validation** (known extensions, custom extensions)
- **Slice discrimination** and validation

Expected validation workflow:
```rust
// Future API
let profile_validator = ProfileValidator::new()
    .with_structure_definition(us_core_patient_profile)?
    .with_terminology_server("https://tx.fhir.org")?;

let result = profile_validator.validate(patient_json)?;
```

## Error Types and Reporting

### Current Error Types

```rust
pub enum ValidationError {
    JsonSyntax { message: String, line: usize, column: usize },
    Schema { message: String },
}
```

### Planned Error Types

```rust
pub enum ValidationError {
    // Syntax errors
    JsonSyntax { message: String, line: usize, column: usize },
    Schema { message: String },

    // FHIR resource errors
    MissingField { field: String, path: String },
    InvalidResourceType { found: String, expected: Vec<String> },
    InvalidFieldValue { field: String, value: String, reason: String },
    VersionMismatch { expected: String, found: String },

    // Profile validation errors
    CardinalityViolation { path: String, expected: String, found: usize },
    TerminologyBinding { path: String, code: String, system: String },
    FhirPathConstraint { path: String, expression: String, message: String },
    ProfileConformance { profile: String, element: String, issue: String },
}
```

## Integration Examples

### Healthcare System Integration

```rust
use rh_validator::{FhirValidator, ValidationConfig, ValidationSeverity};

// Configure validator for production use
let config = ValidationConfig {
    fhir_version: "4.0.1".to_string(),
    strict_mode: true,
    validate_terminology: true,
    terminology_server: Some("https://tx.fhir.org".to_string()),
    profiles: vec![
        "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
    ],
};

let validator = FhirValidator::new(config)?;

// Validate incoming FHIR data
let validation_result = validator.validate_resource(&incoming_json)?;
match validation_result.severity() {
    ValidationSeverity::Valid => process_resource(&incoming_json),
    ValidationSeverity::Warning => {
        log_warnings(&validation_result.warnings());
        process_resource(&incoming_json);
    },
    ValidationSeverity::Error => {
        return Err(format!("Validation failed: {:?}", validation_result.errors()));
    }
}
```

### Batch Processing Pipeline

```rust
use rh_validator::JsonValidator;
use std::fs::File;
use std::io::{BufRead, BufReader};

let validator = JsonValidator::new();
let file = File::open("fhir_bundle.ndjson")?;
let reader = BufReader::new(file);

let mut valid_count = 0;
let mut errors = Vec::new();

for (line_num, line) in reader.lines().enumerate() {
    let line = line?;
    match validator.validate(&line)? {
        ValidationResult::Valid => valid_count += 1,
        ValidationResult::Invalid(line_errors) => {
            errors.push((line_num + 1, line_errors));
        }
    }
}

println!("Processed {} valid resources", valid_count);
if !errors.is_empty() {
    println!("Found {} invalid resources:", errors.len());
    for (line_num, line_errors) in errors {
        println!("  Line {}: {} errors", line_num, line_errors.len());
    }
}
```

## Performance Considerations

### Current Performance Features

- **Streaming JSON parsing** with minimal memory allocation
- **Configurable depth limits** to prevent stack overflow
- **Early termination** on first syntax error (when appropriate)
- **Batch processing** support for large datasets

### Planned Optimizations

- **Profile caching** for repeated validation scenarios
- **Parallel validation** for independent resources
- **Incremental validation** for large bundles
- **Memory-mapped file** support for extremely large datasets
- **Validation result caching** for identical resources

## FHIR Version Support

### Planned Support Matrix

| FHIR Version | Status | Notes |
|--------------|--------|-------|
| DSTU2 | 🚧 Planned | Legacy support |
| STU3 | 🚧 Planned | Limited support |
| R4 (4.0.1) | 🚧 Planned | Primary target |
| R4B (4.3.0) | 🚧 Planned | Full support |
| R5 (5.0.0) | 🚧 Planned | Latest standard |

### Version-Specific Features

- **Automatic version detection** from resource metadata
- **Cross-version compatibility** warnings
- **Version-specific validation rules** and constraints
- **Migration assistance** for version upgrades

## Related Projects

- **rh-codegen** - Generate Rust types from FHIR StructureDefinitions
- **rh-fhirpath** - FHIRPath expression evaluation for constraints
- **rh-cli** - Command-line interface for FHIR processing tools

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Roadmap

### Phase 1: Foundation (✅ Complete)
- [x] JSON syntax validation
- [x] CLI integration
- [x] Error reporting system
- [x] Basic testing framework

### Phase 2: FHIR Basics (In Progress)
- [ ] Resource type validation
- [ ] Element validation
- [ ] Reference format checking

### Phase 3: Profile Validation (Planned)
- [ ] StructureDefinition parsing
- [ ] Cardinality validation
- [ ] Data type validation
- [ ] FHIRPath constraint evaluation

### Phase 4: Advanced Features (Planned)
- [ ] Terminology server integration
- [ ] Bundle validation
- [ ] Custom validation rules
- [ ] Performance optimization

### Phase 5: Production Features (Future)
- [ ] Validation caching
- [ ] Parallel processing
- [ ] Integration APIs
- [ ] Monitoring and metrics
